use anyhow::{Context, Result};
use reqwest::header;
use serde::Deserialize;
use std::collections::HashSet;
use std::env;

#[derive(Debug, Deserialize)]
struct ChaosResponse {
    subdomains: Vec<String>,
}

pub async fn enumerate(domain: &str) -> Result<HashSet<String>> {
    let api_key = match env::var("CHAOS_API_KEY") {
        Ok(key) => key,
        Err(_) => return Ok(HashSet::new()), //Return empty set if no API key
    };

    let client = reqwest::Client::new();
    let url = format!("https://dns.projectdiscovery.io/dns/{}/subdomains", domain);

    let response = client
        .get(&url)
        .header(header::AUTHORIZATION, api_key)
        .send()
        .await
        .with_context(|| format!("Failed to query Chaos API for {}", domain))?;
    
    let data: ChaosResponse = response
        .json()
        .await
        .with_context(|| "Failed to parse Chaos API response")?;

    //Convert subdomains to full domains and collect into HashSet
    let subdomains: HashSet<String> = data
        .subdomains
        .into_iter()
        .map(|sub| format!("{}.{}", sub, domain))
        .collect();
    
    Ok(subdomains)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chaos_enumerate() {
        // this test will only work if CHAOS_API_KEY is set
        if env::var("CHAOS_API_KEY").is_ok() {
            let result =  enumerate("example.com").await;
            assert!(result.is_ok());
            let subdomains = result.unwrap();
            assert!(subdomains.iter().any(|s| s.ends_with("example.com")));
        }
    }
}
