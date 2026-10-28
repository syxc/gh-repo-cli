//! ghr — a lightweight CLI tool for analyzing GitHub repositories without API
//! tokens. It clones repositories with `git` instead of calling the GitHub API.

mod cache;
mod cli;
mod color;
mod commands;
mod config;
mod fsutil;
mod git;
mod json;
mod output;
mod search;

fn main() {
    let args: Result<Vec<String>, _> = std::env::args_os()
        .skip(1)
        .map(std::ffi::OsString::into_string)
        .collect();
    let Ok(args) = args else {
        eprintln!("Error: arguments must be valid UTF-8");
        std::process::exit(1);
    };

    std::process::exit(cli::run(&args));
}
