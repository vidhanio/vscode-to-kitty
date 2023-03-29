use std::io::{self, Read};

use color_eyre::eyre;
use vscode_to_iterm2::{ITerm2TerminalTheme, VSCodeTerminalTheme};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let value = serde_json::from_str::<serde_json::Value>(&input)?;

    let color = value
        .get("colors")
        .ok_or_else(|| eyre::eyre!("no field `colors`"))?;

    let vscode_theme = serde_json::from_value::<VSCodeTerminalTheme>(color.clone())?;

    let iterm2_theme = ITerm2TerminalTheme::from(vscode_theme);

    plist::to_writer_xml(&mut io::stdout(), &iterm2_theme)?;

    Ok(())
}
