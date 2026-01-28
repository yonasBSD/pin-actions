use std::{fs, path::Path};

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

use crate::action::ActionRef;

lazy_static! {
    /// Regex to match uses: lines in workflows
    /// Matches: "uses: owner/repo@ref" and captures indentation, action, and ref
    static ref USES_REGEX: Regex = Regex::new(
        r"(?m)^\s*-?\s*uses:\s+([^@\s]+)@([^\s#]+)"
    ).unwrap();
}

/// A parsed workflow file
#[derive(Debug)]
pub struct WorkflowFile {
    pub path: String,
    pub content: String,
    pub actions: Vec<UsesLine>,
}

/// Represents a single "uses:" line in a workflow
#[derive(Debug, Clone)]
pub struct UsesLine {
    pub line_number: usize,
    pub indent: String,
    pub action: ActionRef,
}

impl WorkflowFile {
    /// Parse a workflow file and extract all action uses
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read workflow file: {}", path_str))?;

        let mut actions = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(uses) = Self::parse_uses_line(line, line_num + 1) {
                actions.push(uses);
            }
        }

        Ok(WorkflowFile {
            path: path_str,
            content,
            actions,
        })
    }

    /// Parse a single uses: line
    fn parse_uses_line(line: &str, line_number: usize) -> Option<UsesLine> {
        let captures = USES_REGEX.captures(line)?;

        // Extract indent (everything before "uses:")
        let indent = line.split("uses:").next()?.to_string();
        let repo = captures.get(1)?.as_str();
        let reference = captures.get(2)?.as_str();

        let action_str = format!("{}@{}", repo, reference);
        let action = ActionRef::parse(&action_str)?;

        // Skip local actions
        if action.is_local() {
            return None;
        }

        Some(UsesLine {
            line_number,
            indent,
            action,
        })
    }

    /// Get all actions that need pinning (not already SHAs)
    pub fn unpinned_actions(&self) -> Vec<&UsesLine> {
        self.actions
            .iter()
            .filter(|uses| !uses.action.is_sha)
            .collect()
    }

    /// Count actions that are already pinned
    pub fn pinned_count(&self) -> usize {
        self.actions
            .iter()
            .filter(|uses| uses.action.is_sha)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uses_line() {
        let line = "      - uses: actions/checkout@v4";
        let uses = WorkflowFile::parse_uses_line(line, 1).unwrap();

        assert_eq!(uses.indent, "      - ");
        assert_eq!(uses.action.repository, "actions/checkout");
        assert_eq!(uses.action.reference, "v4");
        assert!(!uses.action.is_sha);
    }

    #[test]
    fn test_parse_uses_line_with_sha() {
        let line = "      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11";
        let uses = WorkflowFile::parse_uses_line(line, 1).unwrap();

        assert!(uses.action.is_sha);
    }

    #[test]
    fn test_parse_uses_line_with_comment() {
        let line = "      - uses: actions/checkout@v4 # Comment";
        let uses = WorkflowFile::parse_uses_line(line, 1).unwrap();

        assert_eq!(uses.action.reference, "v4");
    }

    #[test]
    fn test_skip_local_action() {
        let line = "      - uses: ./local-action@v1";
        let uses = WorkflowFile::parse_uses_line(line, 1);

        assert!(uses.is_none());
    }

    #[test]
    fn test_workflow_file_content() {
        let yaml = r#"
name: CI
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v3
      - uses: actions/cache@b4ffde65f46336ab88eb53be808477a3936bae11
      - uses: ./local-action@v1
"#;

        let temp = tempfile::NamedTempFile::new().unwrap();
        fs::write(temp.path(), yaml).unwrap();

        let workflow = WorkflowFile::parse(temp.path()).unwrap();

        assert_eq!(workflow.actions.len(), 3); // Excludes local action
        assert_eq!(workflow.unpinned_actions().len(), 2);
        assert_eq!(workflow.pinned_count(), 1);
    }
}
