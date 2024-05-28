use crate::replace::{replace, Base16Colors};
use std::{collections::HashMap, fs, io};

use serde::{Deserialize, Serialize};
use shellexpand::tilde;

use colored::Colorize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    paths: HashMap<String, String>,
    colors: Base16Colors,
}

pub fn parse_config(from: String) -> Result<Config, toml::de::Error> {
    toml::from_str(&from)
}

pub fn evaluate_config(config: Config) -> Result<(), io::Error> {
    for (from, destination) in config.paths {
        println!("{}", format!("reading {from}").green());
        let from = tilde(&from).to_string();

        let from = fs::read_to_string(from)?;

        let replaced = replace(&from, &config.colors);

        let destination = tilde(&destination).to_string();

        fs::write(destination, replaced)?;
    }

    Ok(())
}

pub fn read_config(path: String) -> Result<Config, Box<dyn std::error::Error>> {
    let path = tilde(&path).to_string();

    let config = fs::read_to_string(path)?;
    let result = parse_config(config)?;

    Ok(result)
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    fn colors() -> &'static str {
        r#"
        [colors]
        base00 = "FFFFFF"
        base01 = "FFFFFF"
        base02 = "FFFFFF"
        base03 = "FFFFFF"
        base04 = "FFFFFF"
        base05 = "FFFFFF"
        base06 = "FFFFFF"
        base07 = "FFFFFF"
        base08 = "FFFFFF"
        base09 = "FFFFFF"
        base0a = "FFFFFF"
        base0b = "FFFFFF"
        base0c = "FFFFFF"
        base0d = "FFFFFF"
        base0e = "FFFFFF"
        base0f = "FFFFFF"
        "#
    }

    #[test]
    fn only_valid_config() {
        let input = colors(); // paths field is missing

        let output = parse_config(input.to_string());

        assert!(output.err().is_some());
    }

    #[test]
    fn pass_valid_config() {
        let input = format!("[paths]\n 'k' = 'v' \n{}", colors());

        let output = parse_config(input.to_string());

        assert!(output.err().is_none());
    }

    #[test]
    fn path_is_string() {
        let input = format!("[paths]\n '~/path' = 10 \n{}", colors());

        let output = parse_config(input.to_string());

        assert!(output.err().is_some());
    }
}
