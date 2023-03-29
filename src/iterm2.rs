use serde::{Deserialize, Serialize};

use crate::{VSCodeTerminalTheme, RGBAF32};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ITerm2TerminalTheme {
    #[serde(rename = "Ansi 0 Color")]
    pub ansi_0_color: ITerm2Color,

    #[serde(rename = "Ansi 1 Color")]
    pub ansi_1_color: ITerm2Color,

    #[serde(rename = "Ansi 2 Color")]
    pub ansi_2_color: ITerm2Color,

    #[serde(rename = "Ansi 3 Color")]
    pub ansi_3_color: ITerm2Color,

    #[serde(rename = "Ansi 4 Color")]
    pub ansi_4_color: ITerm2Color,

    #[serde(rename = "Ansi 5 Color")]
    pub ansi_5_color: ITerm2Color,

    #[serde(rename = "Ansi 6 Color")]
    pub ansi_6_color: ITerm2Color,

    #[serde(rename = "Ansi 7 Color")]
    pub ansi_7_color: ITerm2Color,

    #[serde(rename = "Ansi 8 Color")]
    pub ansi_8_color: ITerm2Color,

    #[serde(rename = "Ansi 9 Color")]
    pub ansi_9_color: ITerm2Color,

    #[serde(rename = "Ansi 10 Color")]
    pub ansi_10_color: ITerm2Color,

    #[serde(rename = "Ansi 11 Color")]
    pub ansi_11_color: ITerm2Color,

    #[serde(rename = "Ansi 12 Color")]
    pub ansi_12_color: ITerm2Color,

    #[serde(rename = "Ansi 13 Color")]
    pub ansi_13_color: ITerm2Color,

    #[serde(rename = "Ansi 14 Color")]
    pub ansi_14_color: ITerm2Color,

    #[serde(rename = "Ansi 15 Color")]
    pub ansi_15_color: ITerm2Color,

    #[serde(rename = "Background Color")]
    pub background_color: ITerm2Color,

    #[serde(rename = "Badge Color")]
    pub badge_color: ITerm2Color,

    #[serde(rename = "Bold Color")]
    pub bold_color: ITerm2Color,

    #[serde(rename = "Cursor Color")]
    pub cursor_color: ITerm2Color,

    #[serde(rename = "Cursor Guide Color")]
    pub cursor_guide_color: ITerm2Color,

    #[serde(rename = "Cursor Text Color")]
    pub cursor_text_color: ITerm2Color,

    #[serde(rename = "Foreground Color")]
    pub foreground_color: ITerm2Color,

    #[serde(rename = "Link Color")]
    pub link_color: ITerm2Color,

    #[serde(rename = "Selected Text Color")]
    pub selected_text_color: ITerm2Color,

    #[serde(rename = "Selection Color")]
    pub selection_color: ITerm2Color,
}

impl Default for ITerm2TerminalTheme {
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unreadable_literal)]
    fn default() -> Self {
        let color_space = "sRGB".to_owned();

        Self {
            ansi_0_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            },
            ansi_1_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.789772,
                green: 0.106740855,
                blue: 0.0,
                alpha: 1.0,
            },
            ansi_2_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.0,
                green: 0.7626493,
                blue: 0.0,
                alpha: 1.0,
            },
            ansi_3_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.7805887,
                green: 0.7695802,
                blue: 0.0,
                alpha: 1.0,
            },
            ansi_4_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.0096368175,
                green: 0.14576238,
                blue: 0.78215533,
                alpha: 1.0,
            },
            ansi_5_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.79021937,
                green: 0.18889284,
                blue: 0.78154236,
                alpha: 1.0,
            },
            ansi_6_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.0,
                green: 0.7742459,
                blue: 0.7816564,
                alpha: 1.0,
            },
            ansi_7_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.781043,
                green: 0.7810429,
                blue: 0.7810429,
                alpha: 1.0,
            },
            ansi_8_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.40781933,
                green: 0.40781933,
                blue: 0.40781933,
                alpha: 1.0,
            },
            ansi_9_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 1.0,
                green: 0.4303358,
                blue: 0.40569735,
                alpha: 1.0,
            },
            ansi_10_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.3742693,
                green: 0.9800454,
                blue: 0.40609294,
                alpha: 1.0,
            },
            ansi_11_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.9995062,
                green: 0.98755413,
                blue: 0.40395972,
                alpha: 1.0,
            },
            ansi_12_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.4093805,
                green: 0.44484475,
                blue: 1.0,
                alpha: 1.0,
            },
            ansi_13_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 1.0,
                green: 0.4651724,
                blue: 1.0,
                alpha: 1.0,
            },
            ansi_14_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.37600303,
                green: 0.992616,
                blue: 1.0,
                alpha: 1.0,
            },
            ansi_15_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 1.0,
                green: 0.99999994,
                blue: 1.0,
                alpha: 1.0,
            },
            background_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            },
            badge_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 1.0,
                green: 0.1491003,
                blue: 0.0,
                alpha: 0.5,
            },
            bold_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 1.0,
                green: 0.99999994,
                blue: 1.0,
                alpha: 1.0,
            },
            cursor_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.781043,
                green: 0.7810429,
                blue: 0.7810429,
                alpha: 1.0,
            },
            cursor_guide_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.7021403,
                green: 0.9268138,
                blue: 1.0,
                alpha: 0.25,
            },
            cursor_text_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 1.0,
                green: 0.99999994,
                blue: 1.0,
                alpha: 1.0,
            },
            foreground_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.781043,
                green: 0.7810429,
                blue: 0.7810429,
                alpha: 1.0,
            },
            link_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.0,
                green: 0.35915294,
                blue: 0.73422706,
                alpha: 1.0,
            },
            selected_text_color: ITerm2Color {
                color_space: color_space.clone(),
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            },
            selection_color: ITerm2Color {
                color_space,
                red: 0.75813824,
                green: 0.86968446,
                blue: 1.0,
                alpha: 1.0,
            },
        }
    }
}

impl From<VSCodeTerminalTheme> for ITerm2TerminalTheme {
    fn from(theme: VSCodeTerminalTheme) -> Self {
        Self {
            ansi_0_color: theme.ansi_black.into(),
            ansi_1_color: theme.ansi_red.into(),
            ansi_2_color: theme.ansi_green.into(),
            ansi_3_color: theme.ansi_yellow.into(),
            ansi_4_color: theme.ansi_blue.into(),
            ansi_5_color: theme.ansi_magenta.into(),
            ansi_6_color: theme.ansi_cyan.into(),
            ansi_7_color: theme.ansi_white.into(),
            ansi_8_color: theme.ansi_bright_black.into(),
            ansi_9_color: theme.ansi_bright_red.into(),
            ansi_10_color: theme.ansi_bright_green.into(),
            ansi_11_color: theme.ansi_bright_yellow.into(),
            ansi_12_color: theme.ansi_bright_blue.into(),
            ansi_13_color: theme.ansi_bright_magenta.into(),
            ansi_14_color: theme.ansi_bright_cyan.into(),
            ansi_15_color: theme.ansi_bright_white.into(),
            background_color: theme.background.into(),
            cursor_color: theme.cursor_foreground.into(),
            cursor_text_color: theme.cursor_background.into(),
            foreground_color: theme.foreground.into(),
            selected_text_color: theme.cursor_background.into(),
            selection_color: theme.cursor_foreground.into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct ITerm2Color {
    #[serde(rename = "Color Space")]
    pub color_space: String,

    #[serde(rename = "Red Component")]
    pub red: f32,

    #[serde(rename = "Green Component")]
    pub green: f32,

    #[serde(rename = "Blue Component")]
    pub blue: f32,

    #[serde(rename = "Alpha Component")]
    pub alpha: f32,
}

impl From<RGBAF32> for ITerm2Color {
    fn from(color: RGBAF32) -> Self {
        Self {
            color_space: "sRGB".into(),
            red: color.r,
            green: color.g,
            blue: color.b,
            alpha: color.a,
        }
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;

    use super::*;

    #[test]
    fn works() {
        let plist_bytes = include_bytes!("../../../Downloads/Untitled.itermcolors");

        let theme = plist::from_bytes::<ITerm2TerminalTheme>(plist_bytes);

        eprintln!("{theme:#?}");

        assert_ok!(theme);
    }
}
