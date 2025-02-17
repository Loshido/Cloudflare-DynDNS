use reqwest::Client;
use serde_json::json;
use super::structs::Record;

pub async fn update_dns_record(
    body: Record,
    zone_id: &str,
    dns_record_id: &str,
    cloudflare_email: &str,
    cloudflare_api_key: &str
) -> bool {
    let client = Client::new();
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        zone_id, dns_record_id
    );

    let body = json!(body);

    let response = match client
        .patch(&url)
        .header("Content-Type", "application/json")
        .header("X-Auth-Email", cloudflare_email)
        .header("X-Auth-Key", cloudflare_api_key)
        .json(&body)
        .send()
        .await {
        Ok(v) => v,
        _ => return false
    };

        
    response.status().is_success()
}