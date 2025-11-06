use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub hosts: Vec<HostEntry>,
}

impl Default for Config {
    fn default() -> Self {
        Self { hosts: vec![] }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostEntry {
    pub name: String,
    pub user: String,
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub auth: Auth,
}

fn default_port() -> u16 { 22 }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Auth {
    /// Certificate / key-based auth
    Cert {
        /// Path to identity file, e.g. ~/.ssh/id_rsa
        identity_file: String
    },
    /// Password-based auth. Password is optional; if not present, SSH will prompt.
    Password {
        #[serde(skip_serializing_if = "Option::is_none")]
        password: Option<String>
    }
}
