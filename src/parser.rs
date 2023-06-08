use anyhow::Context;
use serde::Deserialize;

/// Toml converted to a Rust struct
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Toml {
    #[serde(rename = "ID")]
    pub id: String,
    pub state: State,
    pub images: Images,
}

/// State
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    pub state: String,
    pub details: String,
    pub start_timestamp: Option<String>,
    pub end_timestamp: Option<String>,
}

/// Images
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Images {
    pub large_image: Option<String>,
    pub large_image_tooltip: Option<String>,
    pub small_image: Option<String>,
    pub small_image_tooltip: Option<String>,
}

/// Buttons
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Buttons {
    pub first_label: Option<String>,
    pub first_url: Option<String>,
    pub second_label: Option<String>,
    pub second_url: Option<String>,
}

/// Parse toml string to config struct
pub fn parse<S: AsRef<str>>(s: S) -> anyhow::Result<Toml> {
    toml::from_str(s.as_ref()).context("Invalid config file")
}
