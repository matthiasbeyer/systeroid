use getopts::Options;
use std::env;
use std::path::PathBuf;
use systeroid_core::regex::Regex;
use systeroid_core::sysctl::display::DisplayType;
use systeroid_core::sysctl::DEFAULT_PRELOAD;

/// Help message for the arguments.
const HELP_MESSAGE: &str = r#"
Usage:
    {bin} [options] [variable[=value] ...]

Options:
{usage}

For more details see {bin}(8)."#;

/// Command-line arguments.
#[derive(Debug, Default)]
pub struct Args {
    /// Whether if the verbose logging is enabled.
    pub verbose: bool,
    /// Whether if the quiet mode is enabled.
    pub quiet: bool,
    /// Path of the Linux kernel documentation.
    pub kernel_docs: Option<PathBuf>,
    /// Display type of the variables.
    pub display_type: DisplayType,
    /// Whether if the unknown variable errors should be ignored.
    pub ignore_errors: bool,
    /// Do not pipe output into a pager.
    pub no_pager: bool,
    /// Whether if files are given to preload values.
    pub preload_files: bool,
    /// Pattern for matching the parameters.
    pub pattern: Option<Regex>,
    /// Whether if the documentation should be shown.
    pub explain_params: bool,
    /// Free string fragments.
    pub values: Vec<String>,
}

impl Args {
    /// Parses the command-line arguments.
    pub fn parse() -> Option<Self> {
        let mut opts = Options::new();
        opts.optflag("a", "all", "display all variables");
        opts.optflag("A", "", "alias of -a");
        opts.optflag("X", "", "alias of -a");
        opts.optflag("b", "binary", "print value without new line");
        opts.optflag("e", "ignore", "ignore unknown variables errors");
        opts.optflag("N", "names", "print variable names without values");
        opts.optflag("n", "values", "print only values of the given variable(s)");
        opts.optflag("p", "load", "read values from file");
        opts.optflag("f", "", "alias of -p");
        opts.optopt(
            "r",
            "pattern",
            "select setting that match expression",
            "<expression>",
        );
        opts.optflag("q", "quiet", "do not echo variable set");
        opts.optflag("d", "", "alias of -h");
        opts.optflag(
            "E",
            "explain",
            "provide a detailed explanation for a variable",
        );
        opts.optopt(
            "d",
            "docs",
            "set the path of the kernel documentation",
            "<path>",
        );
        opts.optflag("P", "no-pager", "do not pipe output into a pager");
        opts.optflag("v", "verbose", "enable the verbose logging");
        opts.optflag("h", "help", "display this help and exit");
        opts.optflag("V", "version", "output version information and exit");

        let env_args = env::args().collect::<Vec<String>>();
        let mut matches = opts
            .parse(&env_args[1..])
            .map_err(|e| eprintln!("error: `{}`", e))
            .ok()?;

        let preload_files = matches.opt_present("p") || matches.opt_present("f");
        let show_help = matches.opt_present("h") || matches.opt_present("d");
        let required_args_present = matches.opt_present("a")
            || matches.opt_present("A")
            || matches.opt_present("X")
            || !matches.free.is_empty()
            || matches.opt_present("E")
            || preload_files;

        if show_help || env_args.len() == 1 {
            let usage = opts.usage_with_format(|opts| {
                HELP_MESSAGE
                    .replace("{bin}", env!("CARGO_PKG_NAME"))
                    .replace(
                        "{usage}",
                        &opts
                            .filter(|msg| !msg.contains("alias of"))
                            .collect::<Vec<String>>()
                            .join("\n"),
                    )
            });
            println!("{}", usage);
            None
        } else if !required_args_present {
            println!(
                "{}: no variables specified\n\
                Try `{} --help' for more information.",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_NAME")
            );
            None
        } else if matches.opt_present("V") {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            None
        } else {
            let display_type = if matches.opt_present("N") {
                DisplayType::Name
            } else if matches.opt_present("n") {
                DisplayType::Value
            } else if matches.opt_present("b") {
                DisplayType::Binary
            } else {
                DisplayType::Default
            };
            if preload_files && matches.free.is_empty() {
                matches.free = vec![DEFAULT_PRELOAD.to_string()];
            }
            Some(Args {
                verbose: matches.opt_present("v"),
                quiet: matches.opt_present("q"),
                kernel_docs: matches.opt_str("d").map(PathBuf::from),
                display_type,
                ignore_errors: matches.opt_present("e"),
                no_pager: matches.opt_present("P"),
                preload_files,
                pattern: matches
                    .opt_str("r")
                    .map(|v| Regex::new(&v).expect("invalid regex")),
                explain_params: matches.opt_present("E"),
                values: matches.free,
            })
        }
    }
}
