use std::env;

use colored::Colorize;

const USERNAME: &str = "TXP_USERNAME";
const PW: &str = "TXP_PW";
const CLIENT_ID: &str = "TXP_CLIENT_ID";
const CLIENT_SECRET: &str = "TXP_CLIENT_SECRET";
const DOMAIN: &str = "TXP_DOMAIN";
const AUDIENCE: &str = "TXP_AUDIENCE";

pub struct Input {
    pub(crate) username: Option<String>,
    pub(crate) pw: Option<String>,
    pub(crate) client_id: Option<String>,
    pub(crate) client_secret: Option<String>,
    pub(crate) domain: Option<String>,
    pub(crate) aud: Option<String>,
}

#[derive(Debug)]
pub struct PureInput {
    pub(crate) username: String,
    pub(crate) pw: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) domain: String,
    pub(crate) aud: String,
}

impl Input {
    pub fn convert_pure(self, command: &str) -> Result<PureInput, String> {
        let username: String = get_or_from_env(self.username, USERNAME, command)?;
        let pw: String = get_or_from_env(self.pw, PW, command)?;
        let client_id: String = get_or_from_env(self.client_id, CLIENT_ID, command)?;
        let client_secret: String = get_or_from_env(self.client_secret, CLIENT_SECRET, command)?;
        let aud: String = get_or_from_env(self.aud, AUDIENCE, command)?;
        let domain: String = get_or_from_env(self.domain, DOMAIN, command)?;

        Ok(PureInput {
            username,
            pw,
            client_id,
            client_secret,
            domain,
            aud,
        })
    }
}

fn get_or_from_env(tryable: Option<String>, key: &str, command: &str) -> Result<String, String> {
    if let Some(res) = tryable {
        Ok(res)
    } else {
        env::var(key).map_err(|_err| {
            format!(
                "
        {} 
        {} {}
        or set as an environment variable: export {key}=<value>
        ",
                "missing parameter:".red(),
                "provide it as an argument, get help with:",
                format!("tg {command} --help").magenta(),
            )
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn raw_to_pure_with_all_some() {
        let raw = Input {
            username: Some("hello".to_owned()),
            pw: Some("hello2".to_owned()),
            client_id: Some("hello".to_owned()),
            client_secret: Some("hello".to_owned()),
            domain: Some("hello".to_owned()),
            aud: Some("hello".to_owned()),
        }
        .convert_pure("");

        assert!(raw.is_ok(), "should have been okay {raw:?}");
    }
}
