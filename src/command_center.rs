use crate::{get, inputs::PureInput};
use clap::{Parser, Subcommand};

pub fn figure() -> Result<String, String> {
    let cli = Cli::parse();

    let result: Result<String, String> = match cli.command {
        Some(Commands::Get {
            username,
            pw,
            aud,
            id,
            secret,
            domain,
            command,
        }) => {
            let input = PureInput {
                username: username.clone(),
                pw: pw.clone(),
                client_id: id.clone(),
                client_secret: secret.clone(),
                domain: domain.clone(),
                aud: aud.clone(),
            };

            match &command {
                Some(Getcommands::Raw) => get::raw::get(&input),
                Some(Getcommands::Full) => get::full::get(&input),
                Some(Getcommands::File {
                    file_name,
                    value_name,
                }) => get::file::write(&input, value_name, file_name),
                None => Ok("try tg --help for information on how to use aditus".to_string()),
            }
        }
        Some(Commands::Token { path }) => Ok(path),

        None => Ok("try aditus --help for information on how to use aditus".to_string()),
    };
    result
}

/// the name: aditus means "access" | get access tokens, slice them, show them
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// [STABLE] get tokens from an OAuth identity provider
    Get {
        // a test user that you control
        #[arg(short, long, env = "TUS_USERNAME")]
        username: String,
        #[arg(short, long, env = "TUS_PW")]
        // password from that user
        pw: String,
        /// audience you are requesting for, aka. resource server
        #[arg(short, long, env = "TUS_AUD")]
        aud: String,
        /// client id of the app you want a token for
        #[arg(short, long, env = "TUS_ID")]
        id: String,
        /// client secret of the app you want a token for
        #[arg(short, long, env = "TUS_SECRET")]
        secret: String,
        /// domain of your provider, if you have one, the custom domain
        #[arg(short, long, env = "TUS_DOMAIN")]
        domain: String,

        #[command(subcommand)]
        command: Option<Getcommands>,
    },

    /// [PREVIEW] show a token as json
    Token {
        #[arg(short, long)]
        path: String,
    },
}

#[derive(Subcommand, Debug)]
enum Getcommands {
    /// raw accesstoken
    Raw,
    /// returns full answer from the provider
    Full,
    /// write the token to a file in a env file fashion
    File {
        /// write the token to a file in a .env file fashion
        #[arg(short, long, default_value = ".env")]
        file_name: String,
        /// write the token to a file in a env file fashion
        #[arg(short, long, default_value = "token")]
        value_name: String,
    },
}
