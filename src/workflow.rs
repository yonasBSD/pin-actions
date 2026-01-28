use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};
use walkdir::WalkDir;

use crate::{
    action::{ActionRef, PinnedAction},
    git::GitResolver,
    parser::WorkflowFile,
};

/// Results from processing workflows
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResults {
    pub files_processed: usize,
    pub actions_found: usize,
    pub actions_pinned: usize,
    pub already_pinned: usize,
    pub errors: usize,
    pub pinned_actions: Vec<PinnedActionResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinnedActionResult {
    pub file: String,
    pub action: String,
    pub old_ref: String,
    pub sha: String,
}

/// Workflow processor
pub struct WorkflowProcessor {
    workflows_dir: PathBuf,
    dry_run: bool,
    backup: bool,
    concurrency: usize,
}

impl WorkflowProcessor {
    pub fn new(
        workflows_dir: PathBuf,
        dry_run: bool,
        backup: bool,
        _skip_pinned: bool,
        concurrency: usize,
    ) -> Self {
        Self {
            workflows_dir,
            dry_run,
            backup,
            concurrency,
        }
    }

    /// Process all workflow files
    pub async fn process(&self) -> Result<ProcessResults> {
        let resolver = GitResolver::new();

        // Find all workflow files
        let workflow_files = self.find_workflow_files()?;

        if workflow_files.is_empty() {
            info!("No workflow files found");
            return Ok(ProcessResults {
                files_processed: 0,
                actions_found: 0,
                actions_pinned: 0,
                already_pinned: 0,
                errors: 0,
                pinned_actions: Vec::new(),
            });
        }

        info!("Found {} workflow file(s)", workflow_files.len());

        // Parse all workflow files
        let mut parsed_workflows = Vec::new();
        for path in &workflow_files {
            match WorkflowFile::parse(path) {
                Ok(workflow) => parsed_workflows.push(workflow),
                Err(e) => {
                    error!("Failed to parse {}: {}", path.display(), e);
                    continue;
                },
            }
        }

        // Collect all unique actions that need pinning
        let mut actions_to_resolve = HashMap::new();
        let mut already_pinned = 0;

        for workflow in &parsed_workflows {
            already_pinned += workflow.pinned_count();

            for uses in workflow.unpinned_actions() {
                let key = uses.action.to_string();
                actions_to_resolve
                    .entry(key)
                    .or_insert_with(|| uses.action.clone());
            }
        }

        let actions_found = parsed_workflows
            .iter()
            .map(|w| w.actions.len())
            .sum::<usize>();

        if actions_to_resolve.is_empty() {
            info!("No actions need pinning");
            return Ok(ProcessResults {
                files_processed: parsed_workflows.len(),
                actions_found,
                actions_pinned: 0,
                already_pinned,
                errors: 0,
                pinned_actions: Vec::new(),
            });
        }

        info!("Resolving {} unique action(s)", actions_to_resolve.len());

        // Resolve SHAs with progress bar
        let progress = ProgressBar::new(actions_to_resolve.len() as u64);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );

        let actions_vec: Vec<ActionRef> = actions_to_resolve.values().cloned().collect();
        let results = resolver.batch_resolve(actions_vec, self.concurrency).await;

        let mut pinned_map = HashMap::new();
        let mut errors = 0;

        for (action, result) in results {
            progress.inc(1);
            match result {
                Ok(sha) => {
                    progress.set_message(format!("✓ {}", action.repository.green()));
                    debug!("Resolved {} → {}", action, sha);
                    pinned_map.insert(action.to_string(), PinnedAction::new(action, sha));
                },
                Err(e) => {
                    progress.set_message(format!("✗ {}", action.repository.red()));
                    warn!("Failed to resolve {}: {}", action, e);
                    errors += 1;
                },
            }
        }

        progress.finish_with_message("Resolution complete");

        // Rewrite workflow files
        let mut pinned_actions = Vec::new();
        let mut actions_pinned = 0;

        for workflow in parsed_workflows {
            if let Err(e) = self.rewrite_workflow(&workflow, &pinned_map, &mut pinned_actions) {
                error!("Failed to rewrite {}: {}", workflow.path, e);
                errors += 1;
            } else {
                actions_pinned += workflow.unpinned_actions().len();
            }
        }

        Ok(ProcessResults {
            files_processed: workflow_files.len(),
            actions_found,
            actions_pinned,
            already_pinned,
            errors,
            pinned_actions,
        })
    }

    /// Find all workflow YAML files
    fn find_workflow_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(&self.workflows_dir)
            .follow_links(false)
            .max_depth(1)
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yml" || ext == "yaml" {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }

        Ok(files)
    }

    /// Rewrite a workflow file with pinned actions
    fn rewrite_workflow(
        &self,
        workflow: &WorkflowFile,
        pinned_map: &HashMap<String, PinnedAction>,
        results: &mut Vec<PinnedActionResult>,
    ) -> Result<()> {
        let mut new_content = String::new();
        let lines: Vec<&str> = workflow.content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;

            // Find if this line contains an action we need to pin
            if let Some(uses) = workflow.actions.iter().find(|u| u.line_number == line_num) {
                let key = uses.action.to_string();

                if let Some(pinned) = pinned_map.get(&key) {
                    // Replace with pinned version
                    let new_line = format!("{}uses: {}", uses.indent, pinned.format_uses_line());
                    new_content.push_str(&new_line);
                    new_content.push('\n');

                    info!(
                        "  {} {} → {}",
                        "📌".cyan(),
                        uses.action.to_string().yellow(),
                        pinned.sha[..8].green()
                    );

                    results.push(PinnedActionResult {
                        file: workflow.path.clone(),
                        action: uses.action.repository.clone(),
                        old_ref: uses.action.reference.clone(),
                        sha: pinned.sha.clone(),
                    });
                } else {
                    // Keep original if we couldn't resolve
                    new_content.push_str(line);
                    new_content.push('\n');
                }
            } else {
                // Keep original line
                new_content.push_str(line);
                new_content.push('\n');
            }
        }

        // Remove trailing newline if original didn't have one
        if !workflow.content.ends_with('\n') {
            new_content.pop();
        }

        if self.dry_run {
            debug!("Dry run: would write to {}", workflow.path);
            return Ok(());
        }

        // Create backup if requested
        if self.backup {
            let backup_path = format!("{}.bak", workflow.path);
            fs::copy(&workflow.path, &backup_path)
                .with_context(|| format!("Failed to create backup at {}", backup_path))?;
            debug!("Created backup: {}", backup_path);
        }

        // Write the new content
        fs::write(&workflow.path, new_content)
            .with_context(|| format!("Failed to write to {}", workflow.path))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[tokio::test]
    async fn test_process_empty_directory() {
        let temp = TempDir::new().unwrap();
        let processor = WorkflowProcessor::new(temp.path().to_path_buf(), false, false, true, 10);

        let results = processor.process().await.unwrap();
        assert_eq!(results.files_processed, 0);
    }

    #[test]
    fn test_find_workflow_files() {
        let temp = TempDir::new().unwrap();
        let workflows_dir = temp.path().join(".github/workflows");
        fs::create_dir_all(&workflows_dir).unwrap();

        fs::write(workflows_dir.join("ci.yml"), "").unwrap();
        fs::write(workflows_dir.join("test.yaml"), "").unwrap();
        fs::write(workflows_dir.join("readme.md"), "").unwrap();

        let processor = WorkflowProcessor::new(workflows_dir, false, false, true, 10);

        let files = processor.find_workflow_files().unwrap();
        assert_eq!(files.len(), 2);
    }
}
