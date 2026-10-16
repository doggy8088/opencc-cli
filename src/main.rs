use anyhow::Context;
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use clap_complete::{Shell, generate};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

mod help;

const LOCALES: [&str; 7] = ["cn", "hk", "tw", "tw2", "twp", "jp", "t"];

#[derive(Parser, Debug)]
#[command(
    name = "opencc-cli",
    version,
    about = help::ABOUT,
    long_about = help::LONG_ABOUT,
    override_usage = help::USAGE,
    help_template = "{name} {version}\n{about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}",
    disable_help_subcommand = true,
    subcommand_value_name = "COMMAND"
)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "LOCALE",
        default_value = "cn",
        value_parser = LOCALES,
        hide_possible_values = true,
        help = "Locale of the input text",
        long_help = "Locale the input text is written in. Must match the input; it is not auto-detected.\nOne of: cn, tw2, twp, tw, hk, jp, t (see Locales below)."
    )]
    from: String,

    #[arg(
        short,
        long,
        value_name = "LOCALE",
        default_value = "tw2",
        value_parser = LOCALES,
        hide_possible_values = true,
        help = "Locale of the output text",
        long_help = "Locale to convert the text into. Use tw2 for Taiwan output (recommended).\nOne of: cn, tw2, twp, tw, hk, jp, t (see Locales below)."
    )]
    to: String,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Read from FILE instead of stdin",
        long_help = "Read input from FILE. Without this option, input is read from stdin."
    )]
    input: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Write to FILE instead of stdout (overwrites)",
        long_help = "Write output to FILE, creating or overwriting it. Without this option, output\ngoes to stdout. Must not be the same path as --input."
    )]
    output: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print a shell completion script to stdout
    #[command(
        long_about = "Print a shell completion script to stdout.\n\nExample: opencc-cli completions zsh > ~/.zfunc/_opencc-cli"
    )]
    Completions {
        /// Shell to generate completions for
        #[arg(value_parser = clap::value_parser!(Shell))]
        shell: Shell,
    },
}

fn cli_command() -> clap::Command {
    Cli::command()
        .after_help(help::after_help())
        .after_long_help(help::after_long_help())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::from_arg_matches(&cli_command().get_matches()).unwrap_or_else(|e| e.exit());

    if let Some(Commands::Completions { shell }) = cli.command {
        let mut cmd = cli_command();
        generate(shell, &mut cmd, "opencc-cli", &mut io::stdout());
        return Ok(());
    }

    let converter = opencc_rust::converter(&cli.from, &cli.to).with_context(|| {
        format!(
            "Failed to create converter from '{}' to '{}'",
            cli.from, cli.to
        )
    })?;

    let mut reader: Box<dyn BufRead> = match &cli.input {
        Some(path) => {
            let file = File::open(path)
                .with_context(|| format!("Failed to open input file '{}'", path.display()))?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut writer: Box<dyn Write> = match &cli.output {
        Some(path) => {
            let file = File::create(path)
                .with_context(|| format!("Failed to create output file '{}'", path.display()))?;
            Box::new(BufWriter::new(file))
        }
        None => Box::new(BufWriter::new(io::stdout())),
    };

    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = reader
            .read_line(&mut line)
            .context("Failed to read line from input")?;
        if bytes_read == 0 {
            break;
        }
        let converted = converter.convert(&line);
        writer
            .write_all(converted.as_bytes())
            .context("Failed to write to output")?;
    }

    writer.flush().context("Failed to flush output")?;

    Ok(())
}
