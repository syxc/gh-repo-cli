//! Command-line parsing that reproduces the Go build's Cobra behavior: the
//! same subcommands, flags, help text, error strings, suggestions and exit
//! codes. Help and usage texts mirror Cobra's templates so the output stays
//! identical when captured non-interactively.

use crate::commands;
use crate::config::Config;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

const ROOT_LONG: &str = "ghr is a CLI tool for analyzing GitHub repositories without API tokens.

It uses git clone instead of GitHub API, providing unlimited repository analysis.

Examples:
  ghr analyze facebook/react
  ghr search vuejs/core ref
  ghr structure facebook/react --depth 2
  ghr read facebook/react README.md";

const ROOT_USAGE_FULL: &str = "Usage:
  ghr [command]

Available Commands:
  analyze     Perform comprehensive analysis of a GitHub repository
  clean       Clean cached repositories
  help        Help about any command
  ls          List files in a directory
  read        Read a specific file from repository
  readme      Get repository README
  search      Search for code patterns in a repository
  structure   Get repository directory structure

Flags:
  -h, --help       help for ghr
      --no-cache   Bypass cache and re-clone
  -v, --version    version for ghr

Use \"ghr [command] --help\" for more information about a command.";

/// Cobra renders this variant from the `help` command, where the root's
/// default `-h`/`-v` flags have not been initialized yet.
const ROOT_USAGE_PLAIN: &str = "Usage:
  ghr [command]

Available Commands:
  analyze     Perform comprehensive analysis of a GitHub repository
  clean       Clean cached repositories
  help        Help about any command
  ls          List files in a directory
  read        Read a specific file from repository
  readme      Get repository README
  search      Search for code patterns in a repository
  structure   Get repository directory structure

Flags:
      --no-cache   Bypass cache and re-clone

Use \"ghr [command] --help\" for more information about a command.";

const HELP_LONG: &str = "Help provides help for any command in the application.
Simply type ghr help [path to command] for full details.";

const HELP_USAGE: &str = "Usage:
  ghr help [command] [flags]

Flags:
  -h, --help   help for help

Global Flags:
      --no-cache   Bypass cache and re-clone";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArgRule {
    Exact(usize),
    Range(usize, usize),
    Max(usize),
}

struct CommandSpec {
    name: &'static str,
    long: &'static str,
    usage: &'static str,
    args: ArgRule,
    flags: &'static [FlagDef],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlagKind {
    Help,
    NoCache,
    OutputStr,
    OutputBool,
    Depth,
    Ext,
    IgnoreCase,
    All,
    Old,
    Sections,
}

struct FlagDef {
    long: &'static str,
    short: Option<char>,
    /// Cobra renders flags in error messages as `"-d, --depth"` or, when the
    /// flag has no shorthand, as `"--no-cache"`.
    display: &'static str,
    kind: FlagKind,
}

const HELP_FLAG: FlagDef = FlagDef {
    long: "help",
    short: Some('h'),
    display: "-h, --help",
    kind: FlagKind::Help,
};

const NO_CACHE_FLAG: FlagDef = FlagDef {
    long: "no-cache",
    short: None,
    display: "--no-cache",
    kind: FlagKind::NoCache,
};

const COMMON_FLAGS: &[FlagDef] = &[HELP_FLAG, NO_CACHE_FLAG];

const ANALYZE_FLAGS: &[FlagDef] = &[
    HELP_FLAG,
    NO_CACHE_FLAG,
    FlagDef {
        long: "depth",
        short: Some('d'),
        display: "-d, --depth",
        kind: FlagKind::Depth,
    },
    FlagDef {
        long: "output",
        short: Some('o'),
        display: "-o, --output",
        kind: FlagKind::OutputStr,
    },
    FlagDef {
        long: "sections",
        short: None,
        display: "--sections",
        kind: FlagKind::Sections,
    },
];

const STRUCTURE_FLAGS: &[FlagDef] = &[
    HELP_FLAG,
    NO_CACHE_FLAG,
    FlagDef {
        long: "depth",
        short: Some('d'),
        display: "-d, --depth",
        kind: FlagKind::Depth,
    },
    FlagDef {
        long: "output",
        short: Some('o'),
        display: "-o, --output",
        kind: FlagKind::OutputStr,
    },
];

const SEARCH_FLAGS: &[FlagDef] = &[
    HELP_FLAG,
    NO_CACHE_FLAG,
    FlagDef {
        long: "ext",
        short: Some('e'),
        display: "-e, --ext",
        kind: FlagKind::Ext,
    },
    FlagDef {
        long: "ignore-case",
        short: Some('i'),
        display: "-i, --ignore-case",
        kind: FlagKind::IgnoreCase,
    },
    FlagDef {
        long: "output",
        short: Some('o'),
        display: "-o, --output",
        kind: FlagKind::OutputStr,
    },
];

const LS_FLAGS: &[FlagDef] = &[
    HELP_FLAG,
    NO_CACHE_FLAG,
    FlagDef {
        long: "depth",
        short: Some('d'),
        display: "-d, --depth",
        kind: FlagKind::Depth,
    },
];

const CLEAN_FLAGS: &[FlagDef] = &[
    HELP_FLAG,
    NO_CACHE_FLAG,
    FlagDef {
        long: "all",
        short: Some('a'),
        display: "-a, --all",
        kind: FlagKind::All,
    },
    FlagDef {
        long: "old",
        short: None,
        display: "--old",
        kind: FlagKind::Old,
    },
    FlagDef {
        long: "output",
        short: Some('o'),
        display: "-o, --output",
        kind: FlagKind::OutputBool,
    },
];

const COMMAND_SPECS: &[CommandSpec] = &[
    CommandSpec {
        name: "analyze",
        long: "Analyze a GitHub repository and display statistics about languages, file types, and structure.",
        usage: "Usage:
  ghr analyze <repo> [flags]

Flags:
  -d, --depth int         Tree depth for the structure section (default 2)
  -h, --help              help for analyze
  -o, --output string     Save output to file
      --sections string   Sections to report: info, languages, file-types, structure

Global Flags:
      --no-cache   Bypass cache and re-clone",
        args: ArgRule::Exact(1),
        flags: ANALYZE_FLAGS,
    },
    CommandSpec {
        name: "clean",
        long: "Clean cached repositories from the local cache directory.",
        usage: "Usage:
  ghr clean [repo] [flags]

Flags:
  -a, --all          Clean all cached repositories
  -h, --help         help for clean
      --old string   Remove cached repositories unused for this long (e.g. 30d)
  -o, --output       Clean output directory as well

Global Flags:
      --no-cache   Bypass cache and re-clone",
        args: ArgRule::Max(1),
        flags: CLEAN_FLAGS,
    },
    CommandSpec {
        name: "help",
        long: HELP_LONG,
        usage: HELP_USAGE,
        args: ArgRule::Max(usize::MAX),
        flags: COMMON_FLAGS,
    },
    CommandSpec {
        name: "ls",
        long: "List files in a directory of a GitHub repository.",
        usage: "Usage:
  ghr ls <repo> [path] [flags]

Flags:
  -d, --depth int   Maximum depth (default 1)
  -h, --help        help for ls

Global Flags:
      --no-cache   Bypass cache and re-clone",
        args: ArgRule::Range(1, 2),
        flags: LS_FLAGS,
    },
    CommandSpec {
        name: "read",
        long: "Read the contents of a specific file from a GitHub repository.",
        usage: "Usage:
  ghr read <repo> <file> [flags]

Flags:
  -h, --help   help for read

Global Flags:
      --no-cache   Bypass cache and re-clone",
        args: ArgRule::Exact(2),
        flags: COMMON_FLAGS,
    },
    CommandSpec {
        name: "readme",
        long: "Read the README file from a GitHub repository.",
        usage: "Usage:
  ghr readme <repo> [flags]

Flags:
  -h, --help   help for readme

Global Flags:
      --no-cache   Bypass cache and re-clone",
        args: ArgRule::Exact(1),
        flags: COMMON_FLAGS,
    },
    CommandSpec {
        name: "search",
        long: "Search for patterns in a GitHub repository's codebase.",
        usage: "Usage:
  ghr search <repo> <query> [flags]

Flags:
  -e, --ext string      Filter by file extension
  -h, --help            help for search
  -i, --ignore-case     Case insensitive search
  -o, --output string   Save output to file

Global Flags:
      --no-cache   Bypass cache and re-clone",
        args: ArgRule::Exact(2),
        flags: SEARCH_FLAGS,
    },
    CommandSpec {
        name: "structure",
        long: "Display the directory structure of a GitHub repository.",
        usage: "Usage:
  ghr structure <repo> [flags]

Flags:
  -d, --depth int       Maximum depth (default 3)
  -h, --help            help for structure
  -o, --output string   Save to file

Global Flags:
      --no-cache   Bypass cache and re-clone",
        args: ArgRule::Exact(1),
        flags: STRUCTURE_FLAGS,
    },
];

#[derive(Debug)]
pub enum Command {
    Analyze {
        repo: String,
        depth: i64,
        sections: AnalyzeSections,
        output: Option<String>,
    },
    Structure {
        repo: String,
        depth: i64,
        output: Option<String>,
    },
    Search {
        repo: String,
        query: String,
        ext: Option<String>,
        ignore_case: bool,
        output: Option<String>,
    },
    Read {
        repo: String,
        file: String,
    },
    Ls {
        repo: String,
        path: String,
        depth: i64,
    },
    Readme {
        repo: String,
    },
    Clean {
        repo: Option<String>,
        all: bool,
        output: bool,
        old: Option<String>,
    },
}

impl Command {
    fn usage(&self) -> &'static str {
        let name = match self {
            Command::Analyze { .. } => "analyze",
            Command::Structure { .. } => "structure",
            Command::Search { .. } => "search",
            Command::Read { .. } => "read",
            Command::Ls { .. } => "ls",
            Command::Readme { .. } => "readme",
            Command::Clean { .. } => "clean",
        };
        spec_by_name(name).map_or(HELP_USAGE, |spec| spec.usage)
    }
}

#[derive(Debug)]
enum Parsed {
    Version,
    Help(String),
    Command {
        command: Command,
        no_cache: bool,
    },
    /// Cobra-style failure: "Error:" line plus a usage block, exit code 1.
    Failure {
        message: String,
        usage: &'static str,
    },
    UnknownCommand {
        name: String,
        suggestions: Vec<&'static str>,
    },
    /// `help` with an unknown topic prints to stderr but exits 0, like Cobra.
    UnknownHelpTopic(Vec<String>),
}

pub fn run(args: &[String]) -> i32 {
    match parse(args) {
        Parsed::Version => {
            println!("ghr version {VERSION}");
            0
        }
        Parsed::Help(help) => {
            println!("{help}");
            0
        }
        Parsed::Command { command, no_cache } => dispatch(command, no_cache),
        Parsed::Failure { message, usage } => {
            report_usage_error(&message, usage);
            1
        }
        Parsed::UnknownCommand { name, suggestions } => {
            report_unknown_command(&name, &suggestions);
            1
        }
        Parsed::UnknownHelpTopic(topics) => {
            eprintln!("Unknown help topic [{}]", quote_topics(&topics));
            eprintln!("{ROOT_USAGE_PLAIN}");
            0
        }
    }
}

fn dispatch(command: Command, no_cache: bool) -> i32 {
    let config = Config::load();
    let usage = command.usage();
    match commands::run(command, &config, no_cache) {
        Ok(()) => 0,
        Err(message) => {
            eprintln!("Error: {message}");
            eprintln!("{usage}");
            eprintln!();
            1
        }
    }
}

fn report_usage_error(message: &str, usage: &str) {
    eprintln!("Error: {message}");
    eprintln!("{usage}");
    eprintln!();
}

fn report_unknown_command(name: &str, suggestions: &[&'static str]) {
    eprintln!("Error: unknown command \"{name}\" for \"ghr\"");
    if !suggestions.is_empty() {
        eprintln!();
        eprintln!("Did you mean this?");
        for suggestion in suggestions {
            eprintln!("\t{suggestion}");
        }
        eprintln!();
    }
    eprintln!("Run 'ghr --help' for usage.");
}

fn parse(args: &[String]) -> Parsed {
    // Phase 1 mirrors Cobra's Find/stripFlags: at this stage only `--no-cache`
    // is a registered flag, so any other `--flag` or two-character `-x` is
    // assumed to take a value and swallows the next argument; `--` stops the
    // scan; lone dashes and empty strings are dropped.
    let mut commands: Vec<&str> = Vec::new();
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        index += 1;

        if arg == "--" {
            break;
        }

        let long_takes_value = arg.starts_with("--")
            && !arg.contains('=')
            && arg.strip_prefix("--") != Some("no-cache");
        let short_takes_value = arg.len() == 2 && arg.starts_with('-') && !arg.contains('=');
        if long_takes_value || short_takes_value {
            if args.len() - index <= 1 {
                break;
            }
            index += 1;
            continue;
        }

        if !arg.is_empty() && !arg.starts_with('-') {
            commands.push(arg);
        }
    }

    // Phase 2 dispatches on the first command candidate, like Cobra's
    // innerfind + legacyArgs.
    match commands.first() {
        None => parse_root_command(args),
        Some(name) => match spec_by_name(name) {
            Some(spec) => parse_subcommand(spec, &drop_first(args, name), false),
            None => Parsed::UnknownCommand {
                name: (*name).to_owned(),
                suggestions: suggestions_for(name),
            },
        },
    }
}

/// Removes the first occurrence of `name`, mirroring `argsMinusFirstX`.
fn drop_first<'a>(args: &'a [String], name: &str) -> Vec<&'a String> {
    let mut remaining = Vec::with_capacity(args.len());
    let mut removed = false;
    for arg in args {
        if !removed && arg == name {
            removed = true;
            continue;
        }
        remaining.push(arg);
    }

    remaining
}

/// Executes the root command: parses its flags, then renders help or version.
fn parse_root_command(args: &[String]) -> Parsed {
    let mut help = false;
    let mut version = false;
    let mut index = 0;

    while index < args.len() {
        let arg = args[index].as_str();

        if arg == "--" {
            break;
        }

        if let Some(long) = arg.strip_prefix("--") {
            if long.starts_with('-') || long.starts_with('=') {
                return Parsed::Failure {
                    message: format!("bad flag syntax: {arg}"),
                    usage: ROOT_USAGE_FULL,
                };
            }
            let (name, inline) = split_inline(long);
            match name {
                "help" => help = true,
                "version" => version = true,
                "no-cache" => match bool_value(inline, &NO_CACHE_FLAG) {
                    Ok(_) => {}
                    Err(message) => {
                        return Parsed::Failure {
                            message,
                            usage: ROOT_USAGE_FULL,
                        };
                    }
                },
                _ => {
                    return Parsed::Failure {
                        message: format!("unknown flag: --{name}"),
                        usage: ROOT_USAGE_FULL,
                    };
                }
            }
        } else if arg.len() > 1 && arg.starts_with('-') {
            let mut rest = &arg[1..];
            while let Some(short) = rest.chars().next() {
                match short {
                    'h' => help = true,
                    'v' => version = true,
                    _ => {
                        return Parsed::Failure {
                            message: format!("unknown shorthand flag: '{short}' in -{rest}"),
                            usage: ROOT_USAGE_FULL,
                        };
                    }
                }
                rest = &rest[short.len_utf8()..];
            }
        }
        // Anything else is a positional argument; the root command ignores it.

        index += 1;
    }

    if help {
        return Parsed::Help(root_help());
    }
    if version {
        return Parsed::Version;
    }

    // The root command is not runnable, so Cobra renders help.
    Parsed::Help(root_help())
}

fn parse_subcommand(spec: &'static CommandSpec, args: &[&String], root_no_cache: bool) -> Parsed {
    let mut flags = ParsedFlags::default();
    let mut no_cache = root_no_cache;
    let mut help = false;
    let mut positionals: Vec<String> = Vec::new();

    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();

        if arg == "--" {
            positionals.extend(args[index + 1..].iter().map(|arg| (*arg).clone()));
            break;
        }

        if let Some(long) = arg.strip_prefix("--") {
            if long.starts_with('-') || long.starts_with('=') {
                return Parsed::Failure {
                    message: format!("bad flag syntax: {arg}"),
                    usage: spec.usage,
                };
            }
            let (flag_name, inline) = split_inline(long);
            let Some(def) = spec.flags.iter().find(|def| def.long == flag_name) else {
                return Parsed::Failure {
                    message: format!("unknown flag: --{flag_name}"),
                    usage: spec.usage,
                };
            };
            match apply_flag(
                def,
                inline,
                args.get(index + 1).copied(),
                &mut flags,
                &mut no_cache,
                &mut help,
                spec,
            ) {
                Ok(consumed_next) => index += 1 + usize::from(consumed_next),
                Err(parsed) => return parsed,
            }
        } else if arg.len() > 1 && arg.starts_with('-') {
            match apply_short_cluster(
                spec,
                arg,
                args.get(index + 1).copied(),
                &mut flags,
                &mut no_cache,
                &mut help,
            ) {
                Ok(consumed_next) => index += 1 + usize::from(consumed_next),
                Err(parsed) => return parsed,
            }
        } else {
            positionals.push(arg.to_owned());
            index += 1;
        }
    }

    if spec.name == "help" {
        return parse_help_command(&positionals, help);
    }

    if help {
        return Parsed::Help(command_help(spec));
    }

    if let Err(message) = validate_args(spec, positionals.len()) {
        return Parsed::Failure {
            message,
            usage: spec.usage,
        };
    }

    let sections = match flags.sections.as_deref() {
        Some(raw) => match parse_sections(raw) {
            Ok(sections) => sections,
            Err(message) => {
                return Parsed::Failure {
                    message,
                    usage: spec.usage,
                };
            }
        },
        None => AnalyzeSections::all(),
    };

    Parsed::Command {
        command: build_command(spec, &positionals, flags, sections),
        no_cache,
    }
}

fn parse_help_command(topics: &[String], help: bool) -> Parsed {
    if help {
        return Parsed::Help(command_help(spec_by_name("help").unwrap_or_else(|| {
            // Unreachable: the help command is part of COMMAND_SPECS.
            unreachable!("help command spec is always present")
        })));
    }

    let Some(first) = topics.first() else {
        return Parsed::Help(root_help());
    };

    match spec_by_name(first) {
        Some(spec) => Parsed::Help(command_help(spec)),
        None => Parsed::UnknownHelpTopic(topics.to_vec()),
    }
}

#[derive(Default)]
struct ParsedFlags {
    output: Option<String>,
    depth: Option<i64>,
    ext: Option<String>,
    ignore_case: bool,
    all: bool,
    output_dir: bool,
    old: Option<String>,
    sections: Option<String>,
}

/// Applies a long flag; returns whether the next argv entry was consumed as
/// the flag's value.
fn apply_flag(
    def: &FlagDef,
    inline: Option<&str>,
    next: Option<&String>,
    flags: &mut ParsedFlags,
    no_cache: &mut bool,
    help: &mut bool,
    spec: &'static CommandSpec,
) -> Result<bool, Parsed> {
    let flagged = |message: String| Parsed::Failure {
        message,
        usage: spec.usage,
    };

    match def.kind {
        FlagKind::Help => {
            *help = true;
            Ok(false)
        }
        FlagKind::NoCache => {
            *no_cache = bool_value(inline, def).map_err(flagged)?;
            Ok(false)
        }
        FlagKind::IgnoreCase => {
            flags.ignore_case = bool_value(inline, def).map_err(flagged)?;
            Ok(false)
        }
        FlagKind::All => {
            flags.all = bool_value(inline, def).map_err(flagged)?;
            Ok(false)
        }
        FlagKind::OutputBool => {
            flags.output_dir = bool_value(inline, def).map_err(flagged)?;
            Ok(false)
        }
        FlagKind::OutputStr => {
            let (value, consumed) = flag_value(inline, next, def).map_err(flagged)?;
            flags.output = Some(value);
            Ok(consumed)
        }
        FlagKind::Old => {
            let (value, consumed) = flag_value(inline, next, def).map_err(flagged)?;
            flags.old = Some(value);
            Ok(consumed)
        }
        FlagKind::Sections => {
            let (value, consumed) = flag_value(inline, next, def).map_err(flagged)?;
            flags.sections = Some(value);
            Ok(consumed)
        }
        FlagKind::Ext => {
            let (value, consumed) = flag_value(inline, next, def).map_err(flagged)?;
            flags.ext = Some(value);
            Ok(consumed)
        }
        FlagKind::Depth => {
            let (raw, consumed) = flag_value(inline, next, def).map_err(flagged)?;
            match parse_int_base0(&raw) {
                Ok(value) => {
                    flags.depth = Some(value);
                    Ok(consumed)
                }
                Err(reason) => Err(flagged(int_argument_message(&raw, def, reason))),
            }
        }
    }
}

/// Applies a shorthand cluster (`-io out.txt`), returning whether the next
/// argv entry was consumed as a value.
fn apply_short_cluster(
    spec: &'static CommandSpec,
    arg: &str,
    next: Option<&String>,
    flags: &mut ParsedFlags,
    no_cache: &mut bool,
    help: &mut bool,
) -> Result<bool, Parsed> {
    let flagged = |message: String| Parsed::Failure {
        message,
        usage: spec.usage,
    };

    let mut rest = &arg[1..];
    while let Some(short) = rest.chars().next() {
        let Some(def) = spec.flags.iter().find(|def| def.short == Some(short)) else {
            return Err(flagged(format!(
                "unknown shorthand flag: '{short}' in -{rest}"
            )));
        };

        let remainder = &rest[short.len_utf8()..];
        match def.kind {
            FlagKind::Help => {
                *help = true;
                rest = remainder;
            }
            FlagKind::NoCache | FlagKind::IgnoreCase | FlagKind::All | FlagKind::OutputBool => {
                let inline = remainder.strip_prefix('=');
                let value = bool_value(inline, def).map_err(flagged)?;
                match def.kind {
                    FlagKind::NoCache => *no_cache = value,
                    FlagKind::IgnoreCase => flags.ignore_case = value,
                    FlagKind::All => flags.all = value,
                    FlagKind::OutputBool => flags.output_dir = value,
                    _ => unreachable!("only boolean kinds reach this arm"),
                }
                if inline.is_some() {
                    return Ok(false);
                }
                rest = remainder;
            }
            FlagKind::OutputStr
            | FlagKind::Ext
            | FlagKind::Old
            | FlagKind::Sections
            | FlagKind::Depth => {
                let (raw, consumed_next) = if remainder.is_empty() {
                    let Some(value) = next else {
                        return Err(flagged(format!(
                            "flag needs an argument: '{short}' in -{rest}"
                        )));
                    };
                    (value.clone(), true)
                } else {
                    // pflag quirk: only "-f=value" (length > 2) is an
                    // assignment, so "-o=" assigns the literal "=".
                    let value = match remainder.strip_prefix('=') {
                        Some("") => "=",
                        Some(value) => value,
                        None => remainder,
                    };
                    (value.to_owned(), false)
                };

                if def.kind == FlagKind::Depth {
                    match parse_int_base0(&raw) {
                        Ok(value) => flags.depth = Some(value),
                        Err(reason) => {
                            return Err(flagged(int_argument_message(&raw, def, reason)));
                        }
                    }
                } else if def.kind == FlagKind::Ext {
                    flags.ext = Some(raw);
                } else if def.kind == FlagKind::Old {
                    flags.old = Some(raw);
                } else if def.kind == FlagKind::Sections {
                    flags.sections = Some(raw);
                } else {
                    flags.output = Some(raw);
                }

                return Ok(consumed_next);
            }
        }
    }

    Ok(false)
}

fn int_argument_message(raw: &str, def: &FlagDef, reason: &str) -> String {
    format!(
        "invalid argument \"{raw}\" for \"{}\" flag: strconv.ParseInt: parsing \"{raw}\": {reason}",
        def.display
    )
}

fn flag_value(
    inline: Option<&str>,
    next: Option<&String>,
    def: &FlagDef,
) -> Result<(String, bool), String> {
    if let Some(inline) = inline {
        return Ok((inline.to_owned(), false));
    }

    match next {
        Some(value) => Ok((value.clone(), true)),
        None => Err(format!("flag needs an argument: --{}", def.long)),
    }
}

fn bool_value(inline: Option<&str>, def: &FlagDef) -> Result<bool, String> {
    let Some(raw) = inline else {
        return Ok(true);
    };

    parse_bool(raw).ok_or_else(|| {
        format!(
            "invalid argument \"{raw}\" for \"{}\" flag: strconv.ParseBool: parsing \"{raw}\": invalid syntax",
            def.display
        )
    })
}

fn validate_args(spec: &CommandSpec, count: usize) -> Result<(), String> {
    match spec.args {
        ArgRule::Exact(expected) => {
            if count != expected {
                return Err(format!("accepts {expected} arg(s), received {count}"));
            }
        }
        ArgRule::Range(min, max) => {
            if count < min || count > max {
                return Err(format!(
                    "accepts between {min} and {max} arg(s), received {count}"
                ));
            }
        }
        ArgRule::Max(max) => {
            if count > max {
                return Err(format!("accepts at most {max} arg(s), received {count}"));
            }
        }
    }

    Ok(())
}

/// Output sections `analyze` reports; all are on unless `--sections` narrows
/// them. The JSON output contains only the selected sections.
// Independent output toggles: a four-variant enum per section would not make
// any combination invalid, so flags are the honest representation here.
#[expect(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalyzeSections {
    pub info: bool,
    pub languages: bool,
    pub file_types: bool,
    pub structure: bool,
}

impl AnalyzeSections {
    pub fn all() -> Self {
        Self {
            info: true,
            languages: true,
            file_types: true,
            structure: true,
        }
    }

    /// Whether the statistics sections need the directory tree to be read.
    pub fn needs_stats(self) -> bool {
        self.languages || self.file_types
    }
}

/// Parses `--sections` values such as `languages,file-types`.
fn parse_sections(raw: &str) -> Result<AnalyzeSections, String> {
    let mut sections = AnalyzeSections {
        info: false,
        languages: false,
        file_types: false,
        structure: false,
    };

    for name in raw
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
    {
        match name {
            "info" => sections.info = true,
            "languages" => sections.languages = true,
            "file-types" => sections.file_types = true,
            "structure" => sections.structure = true,
            other => {
                return Err(format!(
                    "invalid argument \"{other}\" for \"--sections\" flag: expected info, languages, file-types, structure"
                ));
            }
        }
    }

    if !(sections.info || sections.languages || sections.file_types || sections.structure) {
        return Err(format!(
            "invalid argument \"{raw}\" for \"--sections\" flag: select at least one of info, languages, file-types, structure"
        ));
    }

    Ok(sections)
}

fn build_command(
    spec: &'static CommandSpec,
    positionals: &[String],
    flags: ParsedFlags,
    sections: AnalyzeSections,
) -> Command {
    let repo = || {
        positionals
            .first()
            .expect("positional count is validated")
            .clone()
    };
    let second = || {
        positionals
            .get(1)
            .expect("positional count is validated")
            .clone()
    };

    match spec.name {
        "analyze" => Command::Analyze {
            repo: repo(),
            depth: flags.depth.unwrap_or(2),
            sections,
            output: flags.output,
        },
        "structure" => Command::Structure {
            repo: repo(),
            depth: flags.depth.unwrap_or(3),
            output: flags.output,
        },
        "search" => Command::Search {
            repo: repo(),
            query: second(),
            ext: flags.ext,
            ignore_case: flags.ignore_case,
            output: flags.output,
        },
        "read" => Command::Read {
            repo: repo(),
            file: second(),
        },
        "ls" => Command::Ls {
            repo: repo(),
            path: positionals
                .get(1)
                .cloned()
                .unwrap_or_else(|| ".".to_owned()),
            depth: flags.depth.unwrap_or(1),
        },
        "readme" => Command::Readme { repo: repo() },
        "clean" => Command::Clean {
            repo: positionals.first().cloned(),
            all: flags.all,
            output: flags.output_dir,
            old: flags.old,
        },
        other => unreachable!("help is handled before command construction: {other}"),
    }
}

fn spec_by_name(name: &str) -> Option<&'static CommandSpec> {
    COMMAND_SPECS.iter().find(|spec| spec.name == name)
}

fn command_help(spec: &CommandSpec) -> String {
    format!("{}\n\n{}", spec.long, spec.usage)
}

fn root_help() -> String {
    format!("{ROOT_LONG}\n\n{ROOT_USAGE_FULL}")
}

/// Cobra's suggestion rule: case-insensitive Levenshtein distance of at most
/// 2, or a case-insensitive prefix match.
/// Cobra's suggestion rule: case-insensitive Levenshtein distance of at most
/// 2, or a case-insensitive prefix match. The `help` command is excluded and
/// the order matches the Go build's registration order.
const SUGGESTABLE_COMMANDS: [&str; 7] = [
    "analyze",
    "clean",
    "read",
    "ls",
    "readme",
    "search",
    "structure",
];

fn suggestions_for(typed: &str) -> Vec<&'static str> {
    let typed = typed.to_lowercase();
    SUGGESTABLE_COMMANDS
        .into_iter()
        .filter(|name| {
            levenshtein_distance(&typed, &name.to_lowercase()) <= 2
                || name.to_lowercase().starts_with(&typed)
        })
        .collect()
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut previous: Vec<usize> = (0..=b.len()).collect();
    let mut current = vec![0; b.len() + 1];

    for (i, char_a) in a.iter().enumerate() {
        current[0] = i + 1;
        for (j, char_b) in b.iter().enumerate() {
            let cost = usize::from(char_a != char_b);
            current[j + 1] = (previous[j + 1] + 1)
                .min(current[j] + 1)
                .min(previous[j] + cost);
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[b.len()]
}

/// Formats help topics like Go's `%#q` on a string slice: backtick quoting
/// with a space separator.
fn quote_topics(topics: &[String]) -> String {
    topics
        .iter()
        .map(|topic| {
            if topic.contains('`') || topic.contains('\\') || topic.chars().any(char::is_control) {
                format!("\"{topic}\"")
            } else {
                format!("`{topic}`")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn split_inline(flag: &str) -> (&str, Option<&str>) {
    match flag.split_once('=') {
        Some((name, value)) => (name, Some(value)),
        None => (flag, None),
    }
}

fn parse_bool(raw: &str) -> Option<bool> {
    match raw {
        "1" | "t" | "T" | "true" | "TRUE" | "True" => Some(true),
        "0" | "f" | "F" | "false" | "FALSE" | "False" => Some(false),
        _ => None,
    }
}

/// Parses integers like Go's `strconv.ParseInt(s, 0, 64)`: base prefixes
/// (`0x`, `0o`, `0b`), legacy octal with a leading zero, an optional sign,
/// and underscores as digit separators.
fn parse_int_base0(raw: &str) -> Result<i64, &'static str> {
    let (negative, digits) = match raw.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, raw.strip_prefix('+').unwrap_or(raw)),
    };

    let (base, digits, base_prefixed) = if digits.len() >= 3 {
        match &digits[..2] {
            "0x" | "0X" => (16u32, &digits[2..], true),
            "0o" | "0O" => (8, &digits[2..], true),
            "0b" | "0B" => (2, &digits[2..], true),
            _ if digits.starts_with('0') => (8, &digits[1..], true),
            _ => (10, digits, false),
        }
    } else if digits.starts_with('0') && digits.len() > 1 {
        (8, &digits[1..], true)
    } else {
        (10, digits, false)
    };

    if digits.is_empty() {
        return Err("invalid syntax");
    }

    let mut value: u64 = 0;
    // Underscores are separators: valid after a base prefix or between two
    // digits, never at the ends or doubled up.
    let mut underscore_allowed = base_prefixed;
    let mut last_was_underscore = false;
    for character in digits.chars() {
        if character == '_' {
            if !underscore_allowed {
                return Err("invalid syntax");
            }
            underscore_allowed = false;
            last_was_underscore = true;
            continue;
        }

        let Some(digit) = character.to_digit(base) else {
            return Err("invalid syntax");
        };
        value = value
            .checked_mul(u64::from(base))
            .and_then(|value| value.checked_add(u64::from(digit)))
            .ok_or("value out of range")?;
        underscore_allowed = true;
        last_was_underscore = false;
    }

    if last_was_underscore {
        return Err("invalid syntax");
    }

    // Like Go's ParseUint + negation: the negative range extends one step
    // further than the positive one (-9223372036854775808 is valid).
    if negative {
        if value > 1u64 << 63 {
            return Err("value out of range");
        }
        if value == 1u64 << 63 {
            return Ok(i64::MIN);
        }
        return Ok(-i64::try_from(value).map_err(|_| "value out of range")?);
    }

    i64::try_from(value).map_err(|_| "value out of range")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed_command(args: &[&str]) -> Command {
        let owned: Vec<String> = args.iter().map(|arg| (*arg).to_owned()).collect();
        match parse(&owned) {
            Parsed::Command { command, .. } => command,
            parsed => panic!("expected a command, got {parsed:?}"),
        }
    }

    fn failure_message(args: &[&str]) -> String {
        let owned: Vec<String> = args.iter().map(|arg| (*arg).to_owned()).collect();
        match parse(&owned) {
            Parsed::Failure { message, .. } => message,
            parsed => panic!("expected a failure, got {parsed:?}"),
        }
    }

    #[test]
    fn parse_maps_subcommands_with_defaults() {
        let Command::Structure { depth, .. } = parsed_command(&["structure", "a/b"]) else {
            panic!("expected structure");
        };
        assert_eq!(depth, 3);

        let Command::Ls { path, depth, .. } = parsed_command(&["ls", "a/b"]) else {
            panic!("expected ls");
        };
        assert_eq!((path.as_str(), depth), (".", 1));
    }

    #[test]
    fn parse_accepts_flags_before_and_after_the_subcommand() {
        assert!(matches!(
            parsed_command(&["--no-cache", "analyze", "a/b"]),
            Command::Analyze { .. }
        ));
        assert!(matches!(
            parsed_command(&["analyze", "--no-cache", "a/b"]),
            Command::Analyze { .. }
        ));
    }

    #[test]
    fn parse_reports_cobra_style_arg_count_errors() {
        assert_eq!(
            failure_message(&["analyze"]),
            "accepts 1 arg(s), received 0"
        );
        assert_eq!(
            failure_message(&["analyze", "a", "b"]),
            "accepts 1 arg(s), received 2"
        );
        assert_eq!(
            failure_message(&["ls"]),
            "accepts between 1 and 2 arg(s), received 0"
        );
        assert_eq!(
            failure_message(&["clean", "a", "b"]),
            "accepts at most 1 arg(s), received 2"
        );
    }

    #[test]
    fn parse_reports_unknown_commands_with_suggestions() {
        let owned = vec!["analze".to_owned()];
        match parse(&owned) {
            Parsed::UnknownCommand { name, suggestions } => {
                assert_eq!(name, "analze");
                assert_eq!(suggestions, vec!["analyze"]);
            }
            parsed => panic!("expected an unknown command, got {parsed:?}"),
        }
    }

    #[test]
    fn parse_reports_unknown_flags_and_shorthands() {
        assert_eq!(
            failure_message(&["analyze", "--bogus"]),
            "unknown flag: --bogus"
        );
        assert_eq!(
            failure_message(&["analyze", "x", "-iz"]),
            "unknown shorthand flag: 'i' in -iz"
        );
        assert_eq!(failure_message(&["--bogus"]), "unknown flag: --bogus");
    }

    #[test]
    fn parse_reports_missing_flag_values_like_pflag() {
        assert_eq!(
            failure_message(&["analyze", "x", "-o"]),
            "flag needs an argument: 'o' in -o"
        );
        assert_eq!(
            failure_message(&["analyze", "x", "--output"]),
            "flag needs an argument: --output"
        );
    }

    #[test]
    fn parse_reports_invalid_depth_like_strconv() {
        assert_eq!(
            failure_message(&["structure", "x", "-d", "abc"]),
            "invalid argument \"abc\" for \"-d, --depth\" flag: strconv.ParseInt: parsing \"abc\": invalid syntax"
        );
        assert_eq!(
            failure_message(&["structure", "x", "-d", "99999999999999999999"]),
            "invalid argument \"99999999999999999999\" for \"-d, --depth\" flag: strconv.ParseInt: parsing \"99999999999999999999\": value out of range"
        );
    }

    #[test]
    fn parse_int_base0_matches_strconv_base_zero() {
        assert_eq!(parse_int_base0("3"), Ok(3));
        assert_eq!(parse_int_base0("0"), Ok(0));
        assert_eq!(parse_int_base0("-1"), Ok(-1));
        assert_eq!(parse_int_base0("+5"), Ok(5));
        assert_eq!(parse_int_base0("0x10"), Ok(16));
        assert_eq!(parse_int_base0("0b101"), Ok(5));
        assert_eq!(parse_int_base0("0o17"), Ok(15));
        assert_eq!(parse_int_base0("1_0"), Ok(10));
        assert_eq!(parse_int_base0("0x_1"), Ok(1));
        assert_eq!(parse_int_base0("abc"), Err("invalid syntax"));
        assert_eq!(parse_int_base0(""), Err("invalid syntax"));
        assert_eq!(parse_int_base0("1_"), Err("invalid syntax"));
        assert_eq!(parse_int_base0("_1"), Err("invalid syntax"));
        assert_eq!(parse_int_base0("1__0"), Err("invalid syntax"));
        assert_eq!(
            parse_int_base0("99999999999999999999"),
            Err("value out of range")
        );
    }

    #[test]
    fn parse_maps_analyze_depth_with_default_two() {
        let Command::Analyze { depth, .. } = parsed_command(&["analyze", "a/b"]) else {
            panic!("expected analyze");
        };
        assert_eq!(depth, 2);

        let Command::Analyze { depth, .. } = parsed_command(&["analyze", "a/b", "-d", "4"]) else {
            panic!("expected analyze");
        };
        assert_eq!(depth, 4);
    }

    #[test]
    fn analyze_help_documents_the_depth_flag() {
        let spec = spec_by_name("analyze").unwrap();
        assert!(
            spec.usage.contains("--depth int"),
            "usage missing --depth: {}",
            spec.usage
        );
    }

    #[test]
    fn parse_sections_selects_a_subset() {
        let sections = parse_sections("languages,file-types").unwrap();

        assert!(!sections.info);
        assert!(sections.languages);
        assert!(sections.file_types);
        assert!(!sections.structure);
        assert!(sections.needs_stats());
    }

    #[test]
    fn parse_sections_rejects_unknown_names_and_empty_lists() {
        let unknown = parse_sections("languages,bogus").unwrap_err();
        assert!(unknown.contains("bogus"), "unexpected message: {unknown}");

        let empty = parse_sections("").unwrap_err();
        assert!(
            empty.contains("at least one"),
            "unexpected message: {empty}"
        );

        let commas = parse_sections(", ,").unwrap_err();
        assert!(
            commas.contains("at least one"),
            "unexpected message: {commas}"
        );
    }

    #[test]
    fn parse_sections_defaults_to_everything() {
        let Command::Analyze { sections, .. } = parsed_command(&["analyze", "a/b"]) else {
            panic!("expected analyze");
        };
        assert_eq!(sections, AnalyzeSections::all());

        let Command::Analyze { sections, .. } =
            parsed_command(&["analyze", "a/b", "--sections", "info"])
        else {
            panic!("expected analyze");
        };
        assert!(sections.info);
        assert!(!sections.structure);
        assert!(!sections.needs_stats());
    }

    #[test]
    fn analyze_help_documents_the_sections_flag() {
        let spec = spec_by_name("analyze").unwrap();
        assert!(
            spec.usage.contains("--sections string"),
            "usage missing --sections: {}",
            spec.usage
        );
    }

    #[test]
    fn parse_maps_clean_old_flag() {
        let Command::Clean { old, all, .. } = parsed_command(&["clean", "--old", "30d"]) else {
            panic!("expected clean");
        };
        assert_eq!(old.as_deref(), Some("30d"));
        assert!(!all);

        let Command::Clean { old, .. } = parsed_command(&["clean"]) else {
            panic!("expected clean");
        };
        assert_eq!(old, None);
    }

    #[test]
    fn clean_help_documents_the_old_flag() {
        let spec = spec_by_name("clean").unwrap();
        assert!(
            spec.usage.contains("--old string"),
            "usage missing --old: {}",
            spec.usage
        );
        assert!(spec.usage.contains("30d"));
    }

    #[test]
    fn parse_int_base0_accepts_the_minimum_i64_like_go() {
        assert_eq!(parse_int_base0("-9223372036854775808"), Ok(i64::MIN));
        assert_eq!(
            parse_int_base0("-9223372036854775809"),
            Err("value out of range")
        );
        assert_eq!(
            parse_int_base0("9223372036854775808"),
            Err("value out of range")
        );
    }

    #[test]
    fn lone_dash_and_empty_arguments_render_root_help() {
        assert!(matches!(parse(&["-".to_owned()]), Parsed::Help(_)));
        assert!(matches!(parse(&[String::new()]), Parsed::Help(_)));
    }

    #[test]
    fn unknown_root_flags_swallow_the_next_argument_like_cobra() {
        // Cobra's stripFlags treats an unknown long flag as value-taking, so
        // "analyze" is consumed and the first leftover token is the command.
        let owned: Vec<String> = ["--help", "analyze", "x"]
            .iter()
            .map(ToString::to_string)
            .collect();
        match parse(&owned) {
            Parsed::UnknownCommand { name, .. } => assert_eq!(name, "x"),
            parsed => panic!("expected unknown command, got {parsed:?}"),
        }

        let owned: Vec<String> = ["-v", "analyze", "a/b"]
            .iter()
            .map(ToString::to_string)
            .collect();
        match parse(&owned) {
            Parsed::UnknownCommand { name, .. } => assert_eq!(name, "a/b"),
            parsed => panic!("expected unknown command, got {parsed:?}"),
        }
    }

    #[test]
    fn flags_with_equals_do_not_swallow_the_command_name() {
        // A flag assignment is skipped entirely by Find, so the command error
        // surfaces before any flag value is validated.
        let owned: Vec<String> = ["--no-cache=x", "bogus"]
            .iter()
            .map(ToString::to_string)
            .collect();
        match parse(&owned) {
            Parsed::UnknownCommand { name, .. } => assert_eq!(name, "bogus"),
            parsed => panic!("expected unknown command, got {parsed:?}"),
        }
    }

    #[test]
    fn suggestions_follow_prefix_or_close_distance() {
        assert_eq!(suggestions_for("ana"), vec!["analyze"]);
        assert_eq!(suggestions_for("read"), vec!["read", "readme"]);
        assert_eq!(suggestions_for("zzzzzz"), Vec::<&str>::new());
    }

    #[test]
    fn help_topics_are_quoted_like_go_hash_q() {
        assert_eq!(quote_topics(&["bogus".to_owned()]), "`bogus`");
        assert_eq!(
            quote_topics(&["bogus".to_owned(), "extra".to_owned()]),
            "`bogus` `extra`"
        );
    }

    #[test]
    fn version_flag_before_subcommand_prints_version() {
        let owned = vec!["--version".to_owned(), "analyze".to_owned()];
        assert!(matches!(parse(&owned), Parsed::Version));

        let owned = vec!["-v".to_owned()];
        assert!(matches!(parse(&owned), Parsed::Version));
    }

    #[test]
    fn help_flag_wins_over_version() {
        let owned = vec!["--help".to_owned(), "--version".to_owned()];
        assert!(matches!(parse(&owned), Parsed::Help(_)));

        let owned = vec!["-vh".to_owned()];
        assert!(matches!(parse(&owned), Parsed::Help(_)));
    }

    #[test]
    fn double_dash_at_root_renders_help() {
        let owned = vec!["--".to_owned(), "analyze".to_owned()];
        assert!(matches!(parse(&owned), Parsed::Help(_)));
    }

    #[test]
    fn shorthand_clusters_split_values_like_pflag() {
        let Command::Search {
            ext, ignore_case, ..
        } = parsed_command(&["search", "a/b", "q", "-ie", ".js"])
        else {
            panic!("expected search");
        };
        assert!(ignore_case);
        assert_eq!(ext.as_deref(), Some(".js"));

        let Command::Analyze { output, .. } = parsed_command(&["analyze", "a/b", "-oout.json"])
        else {
            panic!("expected analyze");
        };
        assert_eq!(output.as_deref(), Some("out.json"));

        let Command::Analyze { output, .. } = parsed_command(&["analyze", "a/b", "-o=out.json"])
        else {
            panic!("expected analyze");
        };
        assert_eq!(output.as_deref(), Some("out.json"));

        // pflag quirk: a bare "-o=" assigns the literal "=".
        let Command::Analyze { output, .. } = parsed_command(&["analyze", "a/b", "-o="]) else {
            panic!("expected analyze");
        };
        assert_eq!(output.as_deref(), Some("="));
    }

    #[test]
    fn bool_flags_accept_explicit_values_only_via_equals() {
        let Command::Search { ignore_case, .. } =
            parsed_command(&["search", "a/b", "q", "--ignore-case=false"])
        else {
            panic!("expected search");
        };
        assert!(!ignore_case);

        assert_eq!(
            failure_message(&["search", "a/b", "q", "--ignore-case=maybe"]),
            "invalid argument \"maybe\" for \"-i, --ignore-case\" flag: strconv.ParseBool: parsing \"maybe\": invalid syntax"
        );
    }

    #[test]
    fn help_command_resolves_topics() {
        let owned: Vec<String> = vec!["help".to_owned(), "analyze".to_owned()];
        assert!(matches!(parse(&owned), Parsed::Help(_)));

        let owned: Vec<String> = vec!["help".to_owned()];
        assert!(matches!(parse(&owned), Parsed::Help(_)));

        let owned: Vec<String> = vec!["help".to_owned(), "bogus".to_owned(), "extra".to_owned()];
        match parse(&owned) {
            Parsed::UnknownHelpTopic(topics) => {
                assert_eq!(topics, vec!["bogus", "extra"]);
            }
            parsed => panic!("expected unknown help topic, got {parsed:?}"),
        }
    }

    #[test]
    fn every_spec_usage_lists_its_flags() {
        for spec in COMMAND_SPECS {
            assert!(
                spec.usage.contains("--no-cache"),
                "spec {} missing global flag",
                spec.name
            );
            assert!(
                spec.usage.starts_with("Usage:\n  ghr "),
                "spec {} has unexpected usage",
                spec.name
            );
        }
    }
}
