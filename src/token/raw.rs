use crate::inputs::PureInput;

use super::get_token;

pub fn get(input: &PureInput) -> Result<String, String> {
    let answer = get_token(input)?;

    Ok(answer.access_token)
}
