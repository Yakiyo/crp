mod structs;
use colored::Colorize;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::error::Error;
use std::fs;
use std::io;
use structs::Config;

/// Constant for config file
pub const CONF_FILE_NAME: &str = "config.toml";

// TODO: Generate config file if absent
/// Loads [Config] from config.toml
pub fn load_conf() -> Result<Config, &'static str> {
    let file = match fs::read_to_string(&CONF_FILE_NAME) {
        Ok(e) => e,
        _ => {
            return Err("\
		Missing file config.toml in directory.\n\
		Please copy the file from 'https://github.com/Yakiyo/crp/blob/main/config.toml'\
		and adjust it to your needs\n\
		Please close this window, fix your config file and restart the process again.")
        }
    };

    let conf: Config = match toml::from_str(&file) {
        Ok(c) => c,
        _ => {
            return Err("Invalid syntax in config file\n\
			Config Requirements:\n\
		 	• Make sure parameters ID, State & Details are present and have valid values\n\
		 	• Do not have any empty values. Either remove them completely or put \"\" instead\n\
		 	For any problems, open an issue in https://github.com/Yakiyo/crp/issues\n\
		 	Please close this window, fix your config file and restart the process again.")
        }
    };

    // Am not sure if discord application IDs are always 18 characters long. So
    // checking for 17 >= to be safe
    if !conf.ID.chars().all(char::is_numeric) || conf.ID.chars().count() < 17 {
        return Err("Error when parsing config file.\n\
		• ID must only contain numeric characters and be of 17-18 characters.");
    }
	if conf.ID == "01234567890123456789" {
		return Err("Please use a valid application ID. The default value '01234567890123456789' is not a valid application ID.")
	}
    Ok(conf)
}

/// Initiates the process. Connects to the client and stuff.
pub fn run(c: &Config) -> Result<(), Box<dyn Error>> {
    // Fix for this: https://github.com/Yakiyo/crp/issues/6
	colored::control::set_virtual_terminal(true).unwrap();
	println!("Connecting.......");

    let actv = activity::Activity::new()
        .state(&c.State.State)
        .details(&c.State.Details);

    // Simplify names for the sake of it
    let st = &c.State;
    let img = &c.Images;
    let butt = &c.Buttons; // :p

    // Create assets
    let mut assets = activity::Assets::new();

    if !img.LargeImage.is_empty() {
        assets = assets.large_image(&img.LargeImage);
        if !img.LargeImageTooltip.is_empty() {
            assets = assets.large_text(&img.LargeImageTooltip);
        }
    }
    if !img.SmallImage.is_empty() {
        assets = assets.small_image(&img.SmallImage);
        if !img.SmallImageTooltip.is_empty() {
            assets = assets.small_text(&img.SmallImageTooltip);
        }
    }

    // Create timestamps
    let mut ts = activity::Timestamps::new();
    if !st.StartTimestamp.is_empty() && st.StartTimestamp != "0" {
        ts = ts.start(st.StartTimestamp.parse::<i64>()?);
    } else if !st.EndTimestamp.is_empty() && st.EndTimestamp != "0" {
        ts = ts.end(st.EndTimestamp.parse::<i64>()?);
    }

    // Create buttons
    let mut buttons = Vec::new();
    if !butt.FirstLabel.is_empty() {
        // Discord doesn't even register the rich presence
        // if there is a label but no link. So default it to
        // the github repo link, if its emtpy
        let url = if butt.FirstUrl.is_empty() {
            "https://github.com/Yakiyo/crp"
        } else {
            &butt.FirstUrl
        };
        buttons.push(activity::Button::new(&butt.FirstLabel, url));
    }
    if !butt.SecondLabel.is_empty() {
        let url = if butt.SecondUrl.is_empty() {
            "https://github.com/Yakiyo/crp"
        } else {
            &butt.SecondUrl
        };
        buttons.push(activity::Button::new(&butt.SecondLabel, url));
    }

    let mut client = DiscordIpcClient::new(&c.ID)?;
    client.connect()?;
    client.set_activity(actv.assets(assets).timestamps(ts).buttons(buttons))?;

    // One hot cluster fuck of string interpolation :(
    println!(
        "{} ({})\n\
		{}: {}\n\
		{}: {}\n\
		{}: {}\n\
		{}: {}\n\
		{}: {} with {}: {}\n\
		{}: {} with {}: {}\n\
		",
        "Showing Presence".blue(),
        c.ID.bold(),
        "State".blue(),
        c.State.State,
        "Details".blue(),
        c.State.Details,
        "Start Timestamp".blue(),
        c.State.StartTimestamp,
        "End Timestamp".blue(),
        c.State.EndTimestamp,
        "Large Image".blue(),
        c.Images.LargeImage,
        "tooltip".blue(),
        c.Images.LargeImageTooltip,
        "Small Image".blue(),
        c.Images.SmallImage,
        "tooltip".blue(),
        c.Images.SmallImageTooltip,
    );
    // This keeps the process running. If the user types q/quit/exit
    // it'll close client and shut down. Else keep going. This is to
    // ensure the terminal doesn't close down. There is probably a
    // better process, but this is all i can think of for now
    // FIXME: Make this better?
    // TODO: Check for file change on loop, also use Sleep to pause between loops
    loop {
        let q = input("Press q/quit/exit to shutdown process:").to_lowercase();
        if q == "q" || q == "quit" || q == "exit" {
            break;
        } else {
            continue;
        }
    }

    println!("Shutting down client....");
    client.close()?;

    Ok(())
}

/// Take input from users through the command line
pub fn input(query: &str) -> String {
    println!("{query}");
    let mut res = String::new();
    io::stdin().read_line(&mut res).unwrap();

    // Trim it for any whitespaces and return a newly generated String
    String::from(res.trim())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_conf() {
        let conf = match load_conf() {
            Ok(o) => o,
            Err(e) => {
                panic!("{e}");
            }
        };
        assert_eq!(conf.State.Details, "Using CRP");
    }
}
