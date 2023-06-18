use crate::parser::Toml;
use anyhow::Context;
use std::path::PathBuf;
use yansi::Paint;

pub const CONF_FILE: &str = "config.toml";

/// Load the config file.
///
/// The default path is the `config.toml` file in the same directory as the app. Alternative
/// path to config file can be provided as the first argument.
pub fn load_file() -> anyhow::Result<String> {
    let config_path: PathBuf = std::env::args()
        .nth(1)
        .unwrap_or(CONF_FILE.to_string())
        .into();
    if !config_path.exists() {
        anyhow::bail!(
            "Missing config file in path `{}`",
            Paint::blue(config_path.display())
        );
    }
    std::fs::read_to_string(config_path).with_context(|| "Unable to read file content to string.")
}

/// Config struct used in app
///
/// This is flattened form of the `Toml` struct
pub struct Config {
    pub id: String,
    pub state: String,
    pub details: String,
    pub start_timestamp: Option<String>,
    pub end_timestamp: Option<String>,
    pub l_img: Option<(String, String)>,
    pub s_img: Option<(String, String)>,
    pub first_label: Option<String>,
    pub first_url: Option<String>,
    pub second_label: Option<String>,
    pub second_url: Option<String>,
}

impl std::convert::From<Toml> for Config {
    fn from(v: Toml) -> Self {
        let l_img = if v.images.large_image.is_none() {
            None
        } else {
            Some((
                v.images.large_image.unwrap(),
                v.images.large_image_tooltip.unwrap_or_default(),
            ))
        };
        let s_img = if v.images.small_image.is_none() {
            None
        } else {
            Some((
                v.images.small_image.unwrap(),
                v.images.small_image_tooltip.unwrap_or_default(),
            ))
        };
        Config {
            id: v.id,
            state: v.state.state,
            details: v.state.details,
            start_timestamp: v.state.start_timestamp,
            end_timestamp: v.state.end_timestamp,
            l_img,
            s_img,
            first_label: v.buttons.first_label,
            first_url: v.buttons.first_url,
            second_label: v.buttons.second_label,
            second_url: v.buttons.second_url,
        }
    }
}
