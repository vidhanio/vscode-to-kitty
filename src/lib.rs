#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod iterm2;
mod vscode;

pub use iterm2::ITerm2TerminalTheme;
pub use vscode::VSCodeTerminalTheme;

type RGBAF32 = rgb::RGBA<f32>;
