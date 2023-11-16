#![warn(clippy::pedantic)]
mod command_center;
mod get;
mod inputs;

fn main() {
    match command_center::figure() {
        Ok(message) => println!("{message}"),
        Err(error_message) => {
            eprintln!("{error_message}");
            std::process::exit(1)
        }
    }
}
