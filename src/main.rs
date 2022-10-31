// CRP - Custom Rich Presence client for Discord
//
// Repository: https://github.com/Yakiyo/crp
// Copyright 2022 Yakiyo. All rights reserved. MIT license.

use crp::load_conf;
use crp::run;

fn main() {
    // TODO: Check for new releases on running
    let config = match load_conf() {
		Ok(conf) => conf,
		Err(e) => {
			eprintln!("{e}");
			// This keeps the process running so the user can see the error output
			// instead of the terminal instantly closing
			loop {}
		}
	};

    if let Err(e) = run(&config) {
        eprintln!("Internal error, {e}");
        // This keeps the process running so the user can see the error output
		// instead of the terminal instantly closing
		loop {}
    };
}
