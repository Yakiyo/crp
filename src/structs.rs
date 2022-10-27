//! Structs used to parse [config.toml] file

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub struct Config {
    pub ID: String,
    pub State: State,
    pub Images: Images,
    pub Buttons: Buttons,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub struct State {
    pub State: String,
    pub Details: String,
    #[serde(default = "def_val")]
    pub StartTimestamp: String,
    #[serde(default = "def_val")]
    pub EndTimestamp: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub struct Images {
    #[serde(default = "def_val")]
    pub LargeImage: String,
    #[serde(default = "def_val")]
    pub LargeImageTooltip: String,
    #[serde(default = "def_val")]
    pub SmallImage: String,
    #[serde(default = "def_val")]
    pub SmallImageTooltip: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub struct Buttons {
    #[serde(default = "def_val")]
    pub FirstLabel: String,
    #[serde(default = "def_val")]
    pub FirstUrl: String,
    #[serde(default = "def_val")]
    pub SecondLabel: String,
    #[serde(default = "def_val")]
    pub SecondUrl: String,
}

fn def_val() -> String {
    String::from("")
}
