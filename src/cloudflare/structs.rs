use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DNSSettings {
    pub ipv4_only: Option<bool>,
    pub ipv6_only: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub comment: Option<String>,
    pub content: Option<String>,
    pub name: Option<String>,
    pub proxied: Option<bool>,
    pub settings: Option<DNSSettings>,
    pub ttl: Option<u32>,
}