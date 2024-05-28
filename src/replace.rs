use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Base16Colors {
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    base0a: String,
    base0b: String,
    base0c: String,
    base0d: String,
    base0e: String,
    base0f: String,
}

pub fn replace(text: &str, colors: &Base16Colors) -> String {
    let regex = RegexBuilder::new(r"\$\{base[0][0-9a-f]}")
        .multi_line(true)
        .build()
        .unwrap();

    let replaced = regex.replace_all(text, |captures: &regex::Captures| {
        match &captures[0] {
            "${base00}" => colors.base00.clone(),
            "${base01}" => colors.base01.clone(),
            "${base02}" => colors.base02.clone(),
            "${base03}" => colors.base03.clone(),
            "${base04}" => colors.base04.clone(),
            "${base05}" => colors.base05.clone(),
            "${base06}" => colors.base06.clone(),
            "${base07}" => colors.base07.clone(),
            "${base08}" => colors.base08.clone(),
            "${base09}" => colors.base09.clone(),
            "${base0a}" => colors.base0a.clone(),
            "${base0b}" => colors.base0b.clone(),
            "${base0c}" => colors.base0c.clone(),
            "${base0d}" => colors.base0d.clone(),
            "${base0e}" => colors.base0e.clone(),
            "${base0f}" => colors.base0f.clone(),
            _ => String::new(), // Return empty string for unmatched pattern
        }
    });

    replaced.to_string()
}

#[cfg(test)]
mod test_replace {
    use super::*;

    fn setup_colors() -> Base16Colors {
        Base16Colors {
            base00: String::from("000000"),
            base01: String::from("111111"),
            base02: String::from("222222"),
            base03: String::from("333333"),
            base04: String::from("444444"),
            base05: String::from("555555"),
            base06: String::from("666666"),
            base07: String::from("777777"),
            base08: String::from("888888"),
            base09: String::from("999999"),
            base0a: String::from("AAAAAA"),
            base0b: String::from("BBBBBB"),
            base0c: String::from("CCCCCC"),
            base0d: String::from("DDDDDD"),
            base0e: String::from("EEEEEE"),
            base0f: String::from("FFFFFF"),
        }
    }

    #[test]
    fn one_line() {
        let input = "color: ${base02}";

        let expected = "color: 222222";

        let output = replace(input, &setup_colors());

        assert_eq!(expected.to_string(), output);
    }

    #[test]
    fn multi_line() {
        let input = r#"
        color1: #${base01};
        color2: #${base02};
        "#;

        let expected = r#"
        color1: #111111;
        color2: #222222;
        "#;

        let output = replace(input, &setup_colors());

        assert_eq!(expected.to_string(), output);
    }
}
