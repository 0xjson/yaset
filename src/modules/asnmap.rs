use anyhow::{Context, Result};
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
pub struct AsnEntry {
    pub first_ip: String,
    pub last_ip: String,
    pub asn: i64,
    pub country: String,
    pub org: String,
}

#[derive(Debug, Deserialize)]
struct AsnResponse(Vec<AsnEntry>);

pub async fn enumerate(domain: &str, api_key: &str) -> Result<Vec<AsnEntry>> {
    let base_domain = domain.trim_end_matches(".com");
    let url = format!("https://asn.projectdiscovery.io/api/v1/asnmap?org={}", base_domain);
    
    let response = reqwest::Client::new()
        .get(&url)
        .header("X-PDCP-KEY", api_key)
        .send()
        .await
        .with_context(|| format!("Failed to query ASN API for {}", domain))?;
    
    let response_bytes = response.bytes().await?;

    if let Ok(error_value) = serde_json::from_slice::<serde_json::Value>(&response_bytes) {
        if error_value.get("error").is_some() {
            return Ok(Vec::new());
        }
    }

    let data: AsnResponse = serde_json::from_slice(&response_bytes)
        .with_context(|| "Failed to parse ASN API response")?;

    Ok(data.0)
}

pub fn calculate_ip_blocks(entry: &AsnEntry) -> Vec<String> {
    let first_ip: IpAddr = entry.first_ip.parse().unwrap();
    let last_ip: IpAddr = entry.last_ip.parse().unwrap();
    
    // This is a simplified calculation - you might want to use a proper IP range calculation library
    // For production use, consider using the `ipnetwork` crate for more accurate calculations
    vec![
        format!("{}/24", first_ip),
        format!("{}/24", last_ip),
    ]
}
