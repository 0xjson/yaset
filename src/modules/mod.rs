pub mod crtsh;
pub mod chaos;
pub mod traits;

use anyhow::Result;
use std::collections::HashSet;

pub use traits::SubdomainEnumerator;

pub struct Crtsh;
pub struct Chaos;

//trait SubdomainEnumerator {
//    fn enumerate(&self, domain: &str) -> Result<HashSet<String>>;
//}

#[async_trait::async_trait]
impl SubdomainEnumerator for Crtsh {
    async fn enumerate(&self, domain: &str) -> Result<HashSet<String>> {
        crtsh::enumerate(domain).await
    }
}

#[async_trait::async_trait]
impl SubdomainEnumerator for Chaos {
    async fn enumerate(&self, domain: &str) -> Result<HashSet<String>> {
        chaos::enumerate(domain).await
    }
}
