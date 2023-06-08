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
