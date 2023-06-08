mod config;
mod parser;

/// The entry point to the function. This wraps the main logic
/// so that the main function only focuses on error handling
/// and pausing the thread
fn run() -> anyhow::Result<()> {
    let content = config::load_file()?;
    let config = parser::parse(content)?;
    println!("{config:#?}");
    Ok(())
}

fn main() {
    #[cfg(windows)]
    {
        // If ansi escape sequences are not supported, disable colors on windows
        if !yansi::Paint::enable_windows_ascii() {
            yansi::Paint::disable();
        }
    }

    if let Err(e) = run() {
        eprintln!("{} {:?}", yansi::Paint::red("[ERROR]"), e);
    }
    std::thread::park();
}
