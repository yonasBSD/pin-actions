use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    // Captures: 1: repository, 2: optional reference (after @)
    // Matches everything up to @ or whitespace/comment
    static ref ACTION_RE: Regex = Regex::new(r"^([^@\s#]+)(?:@([^\s#]+))?").unwrap();
}

#[derive(Debug, thiserror::Error)]
pub enum ActionError {
    #[error("Invalid action format: {0}")]
    InvalidFormat(String),
}

/// Represents a GitHub Action reference
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionRef {
    pub repository: String,
    pub reference: String,
    pub is_sha: bool,
}

impl ActionRef {
    /// Parse action strings like "actions/checkout@v4" or "actions/checkout"
    pub fn parse(action_str: &str) -> Result<Self, ActionError> {
        let trimmed = action_str.trim();
        let caps = ACTION_RE
            .captures(trimmed)
            .ok_or_else(|| ActionError::InvalidFormat(trimmed.to_string()))?;

        let repository = caps.get(1).map(|m| m.as_str().to_string()).unwrap();

        // Default to "main" if no @ref is provided
        let reference = caps
            .get(2)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "main".to_string());

        let is_sha = reference.len() == 40 && reference.chars().all(|c| c.is_ascii_hexdigit());

        Ok(ActionRef {
            repository,
            reference,
            is_sha,
        })
    }

    /// Returns the GitHub git URL for the action
    pub fn git_url(&self) -> String {
        format!("https://github.com/{}.git", self.repository)
    }

    /// Checks if this is a local action reference (starts with ./)
    pub fn is_local(&self) -> bool {
        self.repository.starts_with("./")
    }
}

/// Implement Display to enable .to_string() and use in format strings
impl fmt::Display for ActionRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.repository, self.reference)
    }
}

/// Represents a pinned action with its resolved SHA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedAction {
    pub action: ActionRef,
    pub sha: String,
    pub original_ref: String,
}

impl PinnedAction {
    pub fn new(action: ActionRef, sha: String) -> Self {
        let original_ref = action.reference.clone();
        Self {
            action,
            sha,
            original_ref,
        }
    }

    /// Format as "action@sha # original_ref" for YAML output
    pub fn format_uses_line(&self) -> String {
        format!(
            "{}@{} # {}",
            self.action.repository, self.sha, self.original_ref
        )
    }
}
