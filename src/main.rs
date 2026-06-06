use anyhow::Context;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "opencc-cli", version, about = "OpenCC Command Line Interface")]
struct Cli {
    /// Source locale
    #[arg(short, long, default_value = "cn")]
    from: String,

    /// Target locale
    #[arg(short, long, default_value = "tw2")]
    to: String,

    /// Input file path (optional)
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Output file path (optional)
    #[arg(short, long)]
    output: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Prints the shell completion script to stdout
    Completions {
        /// The shell to generate completions for
        #[arg(value_parser = clap::value_parser!(Shell))]
        shell: Shell,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(Commands::Completions { shell }) = cli.command {
        let mut cmd = Cli::command();
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
