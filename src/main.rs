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
    let args: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();

    std::process::exit(cli::run(&args));
}
