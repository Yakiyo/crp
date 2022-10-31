// CRP - Custom Rich Presence client for Discord
//
// Repository: https://github.com/Yakiyo/crp
// Copyright 2022 Yakiyo. All rights reserved. MIT license.

use crp::input;
use crp::load_conf;
use crp::run;
use std::process;
use std::{thread, time};

fn main() {
    // TODO: Check for new releases on running
    let config = match load_conf() {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("{e}");
            // This keeps the process running so the user can see the error output
            // instead of the terminal instantly closing
            let t = time::Duration::from_secs(5);
            thread::sleep(t);
            input("Press q/quit/exit to shutdown process:");
            process::exit(1);
        }
    };

    if let Err(e) = run(&config) {
        eprintln!("Internal error, {e}");
        // This keeps the process running so the user can see the error output
        // instead of the terminal instantly closing
        let t = time::Duration::from_secs(5);
        thread::sleep(t);
        input("Press q/quit/exit to shutdown process:");
        process::exit(1);
    };
}
