use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use git2::Repository;
use tokio::task;
use tracing::debug;

use crate::action::ActionRef;

/// Git resolver for fetching SHAs from remote repositories
#[derive(Clone)]
pub struct GitResolver {
    cache: Arc<Mutex<HashMap<String, String>>>,
}

impl GitResolver {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Resolve a reference to its SHA using git ls-remote
    pub async fn resolve_sha(&self, action: &ActionRef) -> Result<String> {
        let key = action.to_string();

        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            if let Some(sha) = cache.get(&key) {
                debug!("Cache hit for {}", key);
                return Ok(sha.clone());
            }
        }

        // Resolve via git
        let git_url = action.git_url();
        let reference = action.reference.clone();

        debug!("Resolving {} from {}", reference, git_url);

        let sha = task::spawn_blocking(move || Self::git_ls_remote(&git_url, &reference))
            .await
            .context("Failed to spawn git ls-remote task")??;

        // Cache the result
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(key, sha.clone());
        }

        Ok(sha)
    }

    /// Execute git ls-remote to get SHA
    fn git_ls_remote(url: &str, reference: &str) -> Result<String> {
        let repo = Repository::init_bare("/tmp/pin-actions-git")?;
        let mut remote = repo.remote_anonymous(url)?;

        // Try to fetch the reference
        let refs_to_fetch = vec![
            format!("refs/tags/{}", reference),
            format!("refs/heads/{}", reference),
            reference.to_string(),
        ];

        remote.connect(git2::Direction::Fetch)?;
        let remote_heads = remote.list()?;

        for ref_name in refs_to_fetch {
            if let Some(remote_head) = remote_heads.iter().find(|h| h.name() == ref_name) {
                let oid = remote_head.oid();
                return Ok(oid.to_string());
            }
        }

        // If no exact match, try partial match
        for remote_head in remote_heads {
            if remote_head.name().ends_with(&reference) {
                let oid = remote_head.oid();
                return Ok(oid.to_string());
            }
        }

        anyhow::bail!(
            "Could not resolve reference '{}' in repository '{}'",
            reference,
            url
        )
    }

    /// Batch resolve multiple actions concurrently
    pub async fn batch_resolve(
        &self,
        actions: Vec<ActionRef>,
        concurrency: usize,
    ) -> Vec<(ActionRef, Result<String>)> {
        use futures::stream::{self, StreamExt};

        stream::iter(actions)
            .map(|action| {
                let resolver = self.clone();
                async move {
                    let result = resolver.resolve_sha(&action).await;
                    (action, result)
                }
            })
            .buffer_unordered(concurrency)
            .collect()
            .await
    }
}

impl Default for GitResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_resolve_sha() {
        let resolver = GitResolver::new();
        let action = ActionRef::parse("actions/checkout@v4").unwrap();

        let result = resolver.resolve_sha(&action).await;
        assert!(result.is_ok());

        let sha = result.unwrap();
        assert_eq!(sha.len(), 40);
        assert!(sha.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_cache() {
        let resolver = GitResolver::new();
        let action = ActionRef::parse("actions/checkout@v4").unwrap();

        let sha1 = resolver.resolve_sha(&action).await.unwrap();
        let sha2 = resolver.resolve_sha(&action).await.unwrap();

        assert_eq!(sha1, sha2);
    }
}
