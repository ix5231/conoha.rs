use std::{collections::HashMap, str::FromStr};

use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

use crate::commands::Compute;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Region {
    Tyo1,
    Tyo2,
    Syn1,
    Sjc1,
}

impl Region {
    pub fn to_url(&self) -> &'static str {
        match *self {
            Region::Tyo1 => "tyo1",
            Region::Tyo2 => "tyo2",
            Region::Syn1 => "syn1",
            Region::Sjc1 => "sjc1",
        }
    }
}

impl FromStr for Region {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tyo1" => Ok(Region::Tyo1),
            "tyo2" => Ok(Region::Tyo2),
            "syn1" => Ok(Region::Syn1),
            "sjc1" => Ok(Region::Sjc1),
            _ => Err("Couldn't parse region"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Auth<'a> {
    password_credentials: HashMap<&'static str, &'a str>,
    tenant_id: &'a str,
}

impl<'a> Auth<'a> {
    fn new(username: &'a str, password: &'a str, tenant_id: &'a str) -> Self {
        let mut map = HashMap::new();
        map.insert("username", username);
        map.insert("password", password);

        Auth {
            password_credentials: map,
            tenant_id,
        }
    }
}

pub struct Session {
    region: Region,
    tenant_id: String,
    user_id: String,
    user_pass: String,
    token: Option<String>,
    client: Client,
}

impl Session {
    pub fn new(region: Region, tenant_id: String, user_id: String, user_pass: String) -> Self {
        Session {
            region,
            tenant_id,
            user_id,
            user_pass,
            token: None,
            client: Client::new(),
        }
    }

    pub fn auth(&mut self) {
        let mut auth_data = HashMap::new();
        auth_data.insert(
            "auth",
            Auth::new(&self.user_id, &self.user_pass, &self.tenant_id),
        );
        let response: Value = self
            .client
            .post((self.url() + "v2.0/tokens").as_str())
            .json(&auth_data)
            .send()
            .expect("Failed to request token.")
            .json()
            .expect("Failed to parse JSON.");
        self.token = Some(
            response["access"]["token"]["id"]
                .as_str()
                .expect("No token id found.")
                .to_string(),
        );
    }

    pub fn compute(&self) -> Compute {
        Compute::new(
            self.tenant_id.as_str(),
            self.region,
            self.token.as_ref().expect("No token found."),
            &self.client,
        )
    }

    fn url(&self) -> String {
        format!("https://identity.{}.conoha.io/", self.region.to_url())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_region() {
        assert_eq!(Region::from_str("tyo1"), Ok(Region::Tyo1));
        assert_eq!(Region::from_str("Tyo1"), Ok(Region::Tyo1));
        assert_eq!(Region::from_str("TYO1"), Ok(Region::Tyo1));
        assert_eq!(Region::from_str("TYO3"), Err("Couldn't parse region"));
    }
}
