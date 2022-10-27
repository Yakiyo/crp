# CRP - Custom Rich Presence Client for Discord 
[![CI](https://github.com/Yakiyo/crp/actions/workflows/ci.yml/badge.svg)](https://github.com/Yakiyo/crp) [![GitHub stars](https://img.shields.io/github/stars/Yakiyo/crp.svg?style=social&label=Star&maxAge=2592000)](https://github.com/Yakiyo/crp) [![GitHub issues](https://badgen.net/github/issues/Yakiyo/crp/)](https://github.com/Yakiyo/crp) [![GitHub Release](https://img.shields.io/github/release/Yakiyo/crp.svg?style=flat)](https://github.com/Yakiyo/crp) 

Lightweight, simple client for showing custom [Rich Presence](https://discord.com/rich-presence) on [discord](https://discord.com)

## Usage

- Register a new discord application
    - Go to discord [developers dashboard](https://discord.com/developers/applications/me).
    - Click on the "New Application" button at the top right. 
    - Give it a name. The name will be your rich presence name. i.e. if your application's name is "Dyno", it will show in your status as "**Playing** Dyno"
    - Copy your application ID.
    - Go to the "Rich Presence" section of the application (in the navigation bar on the left)
    - Add whatever image you want in the assets section.
- Download the latest release from [release's](https://github.com/Yakiyo/crp/releases) section. Download the zip of your operating system, unzip the file in a folder/directory of your choice.
- After unzipping the files, there should be a `config.toml` file (you might not see the `.toml` extention depending on your system). Edit the config file to your needs with any text editor and save it. See more in [config](#config)
- Run the crp file (crp.exe / crp). It should open a command prompt
    - If theres any error in your config file, it should show it
- Now discord should show your rich presence. 
> **Note**
>
> This will only work with the discord dekstop app. Due to limitations from discord, it is impossible to have rich presence on the browser version.

## Config
The `config.toml` file is in [Toml](https://toml.io) language, so the syntax follows its rules and specs.

The schema for the config file is as follows
```toml
ID = "" # Required

[State]
State = "" # Required
Details = "" # Required
StartTimestamp = ""
EndTimestamp = ""

[Images]
LargeImage = ""
SmallImage = ""
LargeImageTooltip = ""
SmallImageTooltip = ""

[Buttons]
FirstLabel = ""
FirstUrl = ""
SecondLabel = ""
SecondUrl = ""
```
If you miss any of the required field, the app doesn't run and instead warns you to check your config file again. All the other fields are optional, but there are some things to be kept in mind
- Image tool tips obviously wont work if you don't specify any image name
- If you specify a Button label, then a url for that button is required too. If you dont provide a url, it by default links to crp's [github repo](https://github.com/Yakiyo/crp)
- If you specify both the start and end time, the start gets preference over the end.
- If you do not need a functionality, its better to keep the value as just `""`, thought removing the entire key works. But don't keep the key name bare like `LargeImage = ` where the `""` are removed. Use `LargeImage = ""` instead. Anything extra added to the config file is ignored by the app.

### Images and Assets
For setting images, go to the [Discord Dev Portal](https://discord.com/developers/applications) and open your application's page, and open the Rich Presence section. There is a section named "Rich Presence Assets". Upload your image/photo there and give a name, for conveniences sake, don't put spaces in the names or too lengthy names. Give discord 5-10 mins to refresh your assets. Now open your config file. Put the name of your image in place of the `LargeImage` key and run the programme. It should show up.

## Building
- Install [Rust and Cargo](https://www.rust-lang.org/tools/install).
- Clone the github repository 
```bash
$ git clone https://github.com/Yakiyo/crp
```
- Build with cargo
```bash
$ cargo build
# or
$ cargo run
```

This project was inspired by PizzaBelly's [EasyRP](https://github.com/Pizzabelly/EasyRP)

## Author
**CRP** © [Yakiyo](https://github.com/Yakiyo). Authored and maintained by Yakiyo.

Released under [MIT](https://opensource.org/licenses/MIT) License
