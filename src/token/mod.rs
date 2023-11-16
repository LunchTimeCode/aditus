use serde::{Deserialize, Serialize};

use crate::inputs::PureInput;

pub mod file;
pub mod full;
pub mod raw;

fn get_token(pure: &PureInput) -> Result<Answer, String> {
    let url = format!("https://{}/oauth/token", pure.domain);

    let data = &[
        ("grant_type", "password"),
        ("username", &pure.username),
        ("password", &pure.pw),
        ("audience", &pure.aud),
        ("client_id", &pure.client_id),
        ("client_secret", &pure.client_secret),
        ("scope", "openid"),
    ];

    let answer: Answer = ureq::post(&url)
        .send_form(data)
        .map_err(|err| {
            format!(
                "domain: {url} 
        with form data: {data:?} 
        returned: 
        {err}"
            )
        })?
        .into_json()
        .map_err(|err| format!("could not deserialize: {err}"))?;

    Ok(answer)
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    access_token: String,
    expires_in: i64,
    scope: String,
    token_type: String,
}
