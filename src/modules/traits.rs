use anyhow::Result;
use std::collections::HashSet;

#[async_trait::async_trait]
pub trait SubdomainEnumerator {
    async fn enumerate(&self, domain: &str) -> Result<HashSet<String>>;
}
