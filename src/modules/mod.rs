pub mod crtsh;

use anyhow::Result;
use std::collections::HashSet;

pub trait SubdomainEnumerator {
    fn enumerate(&self, domain: &str) -> Result<HashSet<String>>;
}

pub struct Crtsh;

impl SubdomainEnumerator for Crtsh {
    fn enumerate(&self, domain: &str) -> Result<HashSet<String>> {
        crtsh::enumerate(domain)
    }
}
