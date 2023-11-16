#![warn(clippy::pedantic)]
use clap::{Parser, Subcommand};
use inputs::Input;

mod inputs;
mod token;

fn main() {
    let cli = Cli::parse();

    let input = Input {
        username: cli.username.clone(),
        pw: cli.pw.clone(),
        client_id: cli.client_id.clone(),
        client_secret: cli.client_secret.clone(),
        domain: cli.domain.clone(),
        aud: cli.aud.clone(),
    };

    // either take from input, from env or fail
    let pure = match input.convert_pure("") {
        Ok(pure) => pure,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1)
        }
    };

    let result = match &cli.command {
        Some(Commands::Raw) => token::raw::get(&pure),
        Some(Commands::Full) => token::full::get(&pure),
        Some(Commands::File {
            file_name,
            value_name,
        }) => token::file::write(&pure, value_name, file_name),
        None => Ok("try tg --help for information on how to use token_getter".to_string()),
    };

    match result {
        Ok(message) => println!("{message}"),
        Err(error_message) => {
            eprintln!("{error_message}");
            std::process::exit(1)
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    username: Option<String>,
    #[arg(short, long)]
    pw: Option<String>,
    /// audience you are requesting for
    #[arg(short, long)]
    aud: Option<String>,
    /// client id of the app you want a token for
    #[arg(short, long)]
    client_id: Option<String>,
    /// client secret of the app you want a token for
    #[arg(short, long)]
    client_secret: Option<String>,
    /// auth0 domain of your tenant, if possible the custom one
    #[arg(short, long)]
    domain: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// uses the username and pw together with client id and client secret to get a access token
    Raw,

    Full,

    File {
        #[arg(short, long)]
        file_name: String,
        #[arg(short, long)]
        value_name: String,
    },
}
