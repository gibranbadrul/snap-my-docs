use std::path::PathBuf;
use clap::{
    builder::{
        styling::{Ansi256Color, AnsiColor, Styles},
    },
    ArgAction, Parser,
};

const STYLES: Styles = Styles::styled()
    .header(Ansi256Color(203).on_default().bold())
    .usage(Ansi256Color(203).on_default().bold())
    .literal(AnsiColor::White.on_default())
    .placeholder(AnsiColor::White.on_default());

/// Command-line arguments to parse.
#[derive(Debug, Parser)]
#[command(
    version,
    author = clap::crate_authors!("\n"),
    about,
    help_template = "\
{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
    override_usage = "smd [FLAGS] [OPTIONS]",
    styles(STYLES),
    disable_help_flag = true,
    disable_version_flag = true,
)]
pub struct Args {
    /// Prints help information
    #[arg(
        short = 'h',
        long = "help",
        action = ArgAction::Help,
        global = true,
        help = "Prints help information",
        help_heading = "FLAGS"
    )]
    pub help: Option<bool>,

    /// Prints version information
    #[arg(
        short = 'V',
        long = "version",
        action = ArgAction::Version,
        global = true,
        help = "Prints version information",
        help_heading = "FLAGS"
    )]
    pub version: Option<bool>,

    /// Increases the logging verbosity
    #[arg(short, long, action = ArgAction::Count, alias = "debug", help_heading = Some("FLAGS"))]
    pub verbose: u8,

    /// Writes the default configuration file to `docs.json`
    #[arg(
        short,
        long,
        value_name = "CONFIG",
        num_args = 0..=1,
        required = false,
        help_heading = "OPTIONS"
    )]
    pub init: Option<Option<String>>,

    /// Sets the configuration file
    #[arg(
        short,
        long,
        env = "SMD_CONFIG",
        value_name = "PATH",
        default_value = "docs.json",
        help = "Sets the configuration file [env: SMD_CONFIG=]",
        help_heading = "OPTIONS"
    )]
    pub config: PathBuf,

    /// Writes output to the given file
    #[arg(
        short,
        long,
        env = "SMD_OUTPUT",
        value_name = "PATH",
        help = "Writes output to the given file [env: SMD_OUTPUT=]",
        help_heading = "OPTIONS"
    )]
    pub output: Option<PathBuf>,
}
