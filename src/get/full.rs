use crate::inputs::PureInput;

use super::get_token;

pub fn get(input: &PureInput) -> Result<String, String> {
    let answer = get_token(input)?;

    serde_json::to_string_pretty(&answer).map_err(|err| err.to_string())
}
