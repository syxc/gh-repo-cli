//! ANSI styling compatible with the Go build's `fatih/color`: colors turn off
//! for `NO_COLOR`, `TERM=dumb` or non-terminal stdout, colors reset with code
//! `0` and `Bold` resets with its specific reset code `22`.

use std::io::IsTerminal;
use std::sync::OnceLock;

pub fn blue(text: &str) -> String {
    styled(colors_disabled(), "34", "0", text)
}

pub fn green(text: &str) -> String {
    styled(colors_disabled(), "32", "0", text)
}

pub fn red(text: &str) -> String {
    styled(colors_disabled(), "31", "0", text)
}

pub fn yellow(text: &str) -> String {
    styled(colors_disabled(), "33", "0", text)
}

pub fn cyan(text: &str) -> String {
    styled(colors_disabled(), "36", "0", text)
}

pub fn gray(text: &str) -> String {
    styled(colors_disabled(), "90", "0", text)
}

pub fn bold(text: &str) -> String {
    styled(colors_disabled(), "1", "22", text)
}

fn styled(disabled: bool, code: &str, reset: &str, text: &str) -> String {
    if disabled {
        return text.to_owned();
    }

    format!("\x1b[{code}m{text}\x1b[{reset}m")
}

/// Matches `fatih/color`'s `NoColor`: decided once, from `NO_COLOR`, `TERM`
/// and whether stdout is a terminal.
fn colors_disabled() -> bool {
    static DISABLED: OnceLock<bool> = OnceLock::new();
    *DISABLED.get_or_init(|| {
        let term_dumb = std::env::var_os("TERM").is_some_and(|term| term == "dumb");
        non_empty_env("NO_COLOR") || term_dumb || !std::io::stdout().is_terminal()
    })
}

fn non_empty_env(key: &str) -> bool {
    std::env::var_os(key).is_some_and(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn styled_keeps_plain_text_when_colors_are_disabled() {
        assert_eq!(styled(true, "34", "0", "hi"), "hi");
    }

    #[test]
    fn styled_wraps_foreground_colors_with_reset_zero() {
        assert_eq!(styled(false, "34", "0", "hi"), "\x1b[34mhi\x1b[0m");
    }

    #[test]
    fn styled_resets_bold_with_its_own_reset_code() {
        assert_eq!(styled(false, "1", "22", "hi"), "\x1b[1mhi\x1b[22m");
    }
}
