use crate::modules::SubdomainEnumerator;
pub mod modules;

use anyhow::Result;
use std::collections::HashSet;

pub async fn enumerate_subdomains(domain: &str) -> Result<HashSet<String>> {
    let crtsh = modules::Crtsh;
    let chaos = modules::Chaos;

    let (crtsh_results, chaos_results) = tokio::join!(
        crtsh.enumerate(domain),
        chaos.enumerate(domain)
    );
    
    let mut subdomains: HashSet<String> = HashSet::new();

    if let Ok(results) = crtsh_results {
        subdomains.extend(results);
    }

    if let Ok(results) = chaos_results {
        subdomains.extend(results);
    }
    // additional filtering to remove any remaining wildcards or invalid entries
    let filtered: HashSet<String> = subdomains
        .into_iter()
        .filter(|s| {
            !s.starts_with('*') &&
            !s.is_empty() &&
            !s.starts_with('.') &&
            s.contains('.')
        })
        .collect();

    Ok(filtered)
}
