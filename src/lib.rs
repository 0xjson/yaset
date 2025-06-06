use crate::modules::SubdomainEnumerator;
pub mod modules;

use anyhow::Result;
use std::collections::HashSet;

pub fn enumerate_subdomains(domain: &str) -> Result<HashSet<String>> {
    let mut subdomains = HashSet::new();

    let crtsh = modules::Crtsh;
    if let Ok(results) = crtsh.enumerate(domain) {
        subdomains.extend(results);
    }

    // additional filtering to remove any remaining wildcards or invalid entries
    let filtered: HashSet<String> = subdomains
        .into_iter()
        .filter(|s| !s.starts_with('*') && !s.is_empty())
        .collect();

    Ok(filtered)
}
