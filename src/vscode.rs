use serde::{de, Deserialize, Deserializer};

use crate::RGBAF32;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct VSCodeTerminalTheme {
    #[serde(rename = "terminal.ansiBlack")]
    #[serde(deserialize_with = "deserialize_hex")]
    pub ansi_black: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiBlue")]
    pub ansi_blue: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiBrightBlack")]
    pub ansi_bright_black: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiBrightBlue")]
    pub ansi_bright_blue: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiBrightCyan")]
    pub ansi_bright_cyan: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiBrightGreen")]
    pub ansi_bright_green: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiBrightMagenta")]
    pub ansi_bright_magenta: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiBrightRed")]
    pub ansi_bright_red: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiBrightWhite")]
    pub ansi_bright_white: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiBrightYellow")]
    pub ansi_bright_yellow: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiCyan")]
    pub ansi_cyan: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiGreen")]
    pub ansi_green: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiMagenta")]
    pub ansi_magenta: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiRed")]
    pub ansi_red: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiWhite")]
    pub ansi_white: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.ansiYellow")]
    pub ansi_yellow: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.background")]
    pub background: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminal.foreground")]
    pub foreground: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminalCursor.background")]
    pub cursor_background: RGBAF32,

    #[serde(deserialize_with = "deserialize_hex")]
    #[serde(rename = "terminalCursor.foreground")]
    pub cursor_foreground: RGBAF32,
}

fn deserialize_hex<'de, D>(deserializer: D) -> Result<RGBAF32, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    let string = string.trim_start_matches('#');

    let mut bytes = hex::decode(string)
        .map_err(de::Error::custom)?
        .into_iter()
        .map(|b| f32::from(b) / 255.0);

    let r = bytes
        .next()
        .ok_or_else(|| de::Error::missing_field("red"))?;

    let g = bytes
        .next()
        .ok_or_else(|| de::Error::missing_field("green"))?;

    let b = bytes
        .next()
        .ok_or_else(|| de::Error::missing_field("blue"))?;

    let a = bytes.next().unwrap_or(1.0);

    bytes.next().map_or_else(
        || Ok(()),
        |_| Err(de::Error::custom("too many bytes in hex color")),
    )?;

    Ok(RGBAF32::new(r, g, b, a))
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;
    use serde_json::Value;

    use super::*;

    #[test]
    fn works() {
        let json_str = include_str!("../tests/data/vscode-theme.json");
        let value = serde_json::from_str::<Value>(json_str).unwrap();
        let json_str = serde_json::to_string(&value["colors"]).unwrap();

        let theme = serde_json::from_str::<VSCodeTerminalTheme>(&json_str);

        eprintln!("{theme:#?}");

        assert_ok!(theme);
    }
}
