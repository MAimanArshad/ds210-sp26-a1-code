// No-op fake colors for printing to terminal in Windows
// Termion does not support windows.
#[cfg(windows)]
pub enum Color {
    Yellow,
    Green,
    Red,
    Blue,
    Reset,
}

#[cfg(windows)]
pub use Color::*;

#[cfg(windows)]
pub fn Fg(_: Color) -> &'static str {
  ""
}

#[cfg(not(windows))]
pub use termion::color::*;