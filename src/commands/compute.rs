use crate::session::Region;

use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use std::collections::HashMap;
use std::path::PathBuf;

pub struct Compute<'a> {
    endpoint: String,
    token: &'a str,
    client: &'a Client,
}

impl<'a> Compute<'a> {
    pub fn new(tenant_id: &'a str, region: Region, token: &'a str, client: &'a Client) -> Self {
        let endpoint = format!(
            "https://compute.{}.conoha.io/v2/{}",
            region.to_url(),
            tenant_id
        );
        Compute {
            endpoint,
            token,
            client,
        }
    }

    pub fn vm_list(&self) -> Vec<Server> {
        let mut map: HashMap<String, Vec<Server>> = self
            .client
            .get(&self.url("/servers"))
            .header("X-Auth-Token", self.token)
            .send()
            .expect("Failed to get VM instance list.")
            .json()
            .expect("Failed parse JSON.");
        map.remove("servers").expect("Servers expected")
    }

    pub fn download_iso(&self, url: &str) {
        let res = self
            .client
            .post(&self.url("/iso-images"))
            .header("X-Auth-Token", self.token)
            .json(&json!({
                "iso-image": {
                    "url": url
                }
            }))
            .send()
            .expect("Failed to download ISO.");
        if !res.status().is_success() {
            panic!("Failed to download ISO.");
        }
    }

    pub fn list_iso(&self) -> Vec<IsoImage> {
        let mut map: HashMap<String, Vec<IsoImage>> = self
            .client
            .get(&self.url("/iso-images"))
            .header("X-Auth-Token", self.token)
            .send()
            .expect("Failed to get VM instance list.")
            .json()
            .expect("Failed parse JSON.");
        map.remove("iso-images").expect("ISO images expected")
    }

    fn url(&self, prefix: &str) -> String {
        format!("{}{}", self.endpoint, prefix)
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    id: String,
    links: Vec<Link>,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Link {
    href: String,
    rel: String,
}

#[derive(Debug, Deserialize)]
pub struct IsoImage {
    url: String,
    path: PathBuf,
    ctime: String,
    name: String,
    size: u64,
}

impl IsoImage {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
