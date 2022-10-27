// CRP - Custom Rich Presence client for Discord
//
// Repository: https://github.com/Yakiyo/crp
// Copyright 2022 Yakiyo. All rights reserved. MIT license.

use crp::load_conf;
use crp::run;
use std::process;

fn main() {
    // TODO: Check for new releases on running
    let config = load_conf();

    if let Err(e) = run(&config) {
        eprintln!("Internal error, {e}");
        process::exit(1);
    };
}
