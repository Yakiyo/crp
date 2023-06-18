use anyhow::bail;
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
    pub buttons: Buttons,
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
pub fn parse<S: AsRef<str>>(s: S) -> anyhow::Result<&'static mut Toml> {
    let toml = toml::from_str::<Toml>(s.as_ref())
        .context("Invalid config file")?
        .validate()
        .context("Invalid config file");
    match toml {
        Ok(e) => Ok(e),
        Err(e) => Err(e),
    }
}

impl Toml {
    fn validate(&mut self) -> anyhow::Result<&mut Toml> {
        if self.id.chars().all(|x| x.is_ascii_digit()) && self.id.chars().count() < 17 {
            bail!("ID parameter must only contain digits and be longer than 17 digits")
        }
        if self.images.large_image.is_some_and(|x| x.is_empty()) {
            self.images.large_image = None;
        }
        if self.images.small_image.is_some_and(|x| x.is_empty()) {
            self.images.small_image = None;
        }
        Ok(self)
    }
}
