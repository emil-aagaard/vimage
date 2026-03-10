use std::{env, process::exit};

use vimage::{
    browser::{create_html, open_browser},
    config::Config,
    constants::HELP_MESSAGE,
    error::Error,
    path::{get_output_path, get_path},
};

/// Main function.
fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        exit(1);
    }
}

/// Main flow.
fn run() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("help") => {
            println!("{}", HELP_MESSAGE);
            Ok(())
        }
        Some(relative_path) => {
            let config = Config::from_iter(args.iter().skip(2))?;
            let path = get_path(relative_path)?;
            let output_path = get_output_path(&config.output_directory, &config.output_file_name)?;
            create_html(&path, &output_path, config.interval)?;
            open_browser(&output_path, &config.open_command)
        }
        None => Err(Error::NoArgs),
    }
}
