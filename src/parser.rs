use std::{fs, path::Path};

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

use crate::action::ActionRef;

lazy_static! {
    /// Captures: 1: Indentation, 2: The action string
    /// The [a-zA-Z0-0] ensures we start with a valid character, not 'uses:'
    static ref USES_REGEX: Regex = Regex::new(
        r"(?m)^\s*-?\s*uses:\s+([a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+)@([^\s#]+)"
    ).unwrap();
}

#[derive(Debug)]
pub struct WorkflowFile {
    pub path: String,
    pub content: String,
    pub actions: Vec<UsesLine>,
}

#[derive(Debug, Clone)]
pub struct UsesLine {
    pub line_number: usize,
    pub indent: String,
    pub action: ActionRef,
}

impl WorkflowFile {
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

    fn parse_uses_line(line: &str, line_number: usize) -> Option<UsesLine> {
        let captures = USES_REGEX.captures(line)?;

        let indent = captures.get(1)?.as_str().to_string();
        let raw_action = captures.get(2)?.as_str();

        // This will now handle "actions/checkout" by defaulting to "main"
        // because of the logic we put in ActionRef::parse
        let action = ActionRef::parse(raw_action).ok()?;

        if action.is_local() {
            return None;
        }

        Some(UsesLine {
            line_number,
            indent,
            action,
        })
    }

    pub fn unpinned_actions(&self) -> Vec<&UsesLine> {
        self.actions.iter().filter(|u| !u.action.is_sha).collect()
    }

    pub fn pinned_count(&self) -> usize {
        self.actions.iter().filter(|u| u.action.is_sha).count()
    }
}
