use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use colored::Colorize;

use crate::inputs::PureInput;

use super::get_token;

pub fn write(input: &PureInput, value_name: &str, file_name: &str) -> Result<String, String> {
    let answer = get_token(input)?;

    write_file(value_name, &answer.access_token, file_name)
}

fn write_file(value_name: &str, value: &str, file_name: &str) -> Result<String, String> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_name)
        .map_err(|err| err.to_string())?;

    let mut target = BufWriter::new(file);

    writeln!(target, "{value_name}={value}").map_err(|err| err.to_string())?;
    Ok("written to file".green().to_string())
}
