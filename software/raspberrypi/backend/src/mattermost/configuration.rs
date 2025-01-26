use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct MatterMost {
    pub python: String,
    pub enable: bool,
    pub url: String,
    pub loginid: String,
    pub apitoken: String,
    pub scheme: String,
    pub port: u16,
}
