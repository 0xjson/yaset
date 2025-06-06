use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
struct CrtshEntry {
    name_value: String, 
}

pub fn enumerate(domain: &str) -> Result<HashSet<String>> {
    let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
    let entries: Vec<CrtshEntry> = reqwest::blocking::get(&url)
        .with_context(|| format!("Failed to query crt.sh for {}", domain))?
        .json()
        .with_context(|| "Failed to parse crt.sh response")?;

    // let subdomains: HashSet<String> = entries
    //     .into_iter()
    //     .flat_map(|entry| {
    //         entry
    //             .name_value
    //             .split('\n')
    //             .map(|s| s.trim().to_string())
    //             .collect::<Vec<String>>()
    //     })
    //     .filter(|s| !s.is_empty())
    //     .collect();
    //
    let mut subdomains = HashSet::new();

    for entry in entries {
        for name_value in entry.name_value.split('\n') {
            let name = name_value.trim();
            if name.is_empty() {
                continue;
            }

            // skip wildcard cert (*.domain.com)
            if name.starts_with("*.") {
                let base_domain = name.trim_start_matches("*.");
                subdomains.insert(base_domain.to_string());
                continue;
            }

            if name == format!("*.{}", domain) {
                continue;
            }

            subdomains.insert(name.to_string());
        }
    }

    Ok(subdomains)
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crtsh_enumerate() {
        // Note: this is a live API #[test]
        let result = enumerate("example.com");
        assert!(result.is_ok());
        let subdomains = result.unwrap();
        assert!(subdomains.contains("example.com"));
    }
}
