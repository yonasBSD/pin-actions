use std::fmt;

use serde::{Deserialize, Serialize};

/// Represents a GitHub Action reference
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionRef {
    /// The action repository (e.g., "actions/checkout")
    pub repository: String,

    /// The reference (tag, branch, or SHA)
    pub reference: String,

    /// Whether this is already a SHA
    pub is_sha: bool,
}

impl ActionRef {
    /// Parse an action string like "actions/checkout@v4"
    pub fn parse(action_str: &str) -> Option<Self> {
        let parts: Vec<&str> = action_str.split('@').collect();
        if parts.len() != 2 {
            return None;
        }

        let repository = parts[0].trim().to_string();
        let reference = parts[1].trim().to_string();

        // Check if it's already a SHA (40 hex characters)
        let is_sha = reference.len() == 40 && reference.chars().all(|c| c.is_ascii_hexdigit());

        Some(ActionRef {
            repository,
            reference,
            is_sha,
        })
    }

    /// Get the git URL for this action
    pub fn git_url(&self) -> String {
        format!("https://github.com/{}.git", self.repository)
    }

    /// Format as action@ref
    pub fn to_string(&self) -> String {
        format!("{}@{}", self.repository, self.reference)
    }

    /// Check if this is a local action (starts with ./)
    pub fn is_local(&self) -> bool {
        self.repository.starts_with("./")
    }
}

impl fmt::Display for ActionRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.repository, self.reference)
    }
}

/// Represents a pinned action with SHA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedAction {
    pub action: ActionRef,
    pub sha: String,
    pub original_ref: String,
}

impl PinnedAction {
    pub fn new(action: ActionRef, sha: String) -> Self {
        let original_ref = action.reference.clone();
        PinnedAction {
            action,
            sha,
            original_ref,
        }
    }

    /// Format as "action@sha # original_ref"
    pub fn format_uses_line(&self) -> String {
        format!(
            "{}@{} # {}",
            self.action.repository, self.sha, self.original_ref
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_action_ref() {
        let action = ActionRef::parse("actions/checkout@v4").unwrap();
        assert_eq!(action.repository, "actions/checkout");
        assert_eq!(action.reference, "v4");
        assert!(!action.is_sha);
    }

    #[test]
    fn test_parse_action_ref_with_sha() {
        let action =
            ActionRef::parse("actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11").unwrap();
        assert_eq!(action.repository, "actions/checkout");
        assert_eq!(action.reference, "b4ffde65f46336ab88eb53be808477a3936bae11");
        assert!(action.is_sha);
    }

    #[test]
    fn test_is_local() {
        let action = ActionRef::parse("./local-action@v1").unwrap();
        assert!(action.is_local());

        let action = ActionRef::parse("actions/checkout@v4").unwrap();
        assert!(!action.is_local());
    }

    #[test]
    fn test_git_url() {
        let action = ActionRef::parse("actions/checkout@v4").unwrap();
        assert_eq!(action.git_url(), "https://github.com/actions/checkout.git");
    }

    #[test]
    fn test_pinned_action_format() {
        let action = ActionRef::parse("actions/checkout@v4").unwrap();
        let pinned = PinnedAction::new(action, "abc123".to_string());
        assert_eq!(pinned.format_uses_line(), "actions/checkout@abc123 # v4");
    }
}
