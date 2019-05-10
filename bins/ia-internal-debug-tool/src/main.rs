use std::{error::Error, fs::read, path::PathBuf};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();
    libremexre::init_logger(options.verbose, options.quiet);

    match options.subcommand {
        Subcommand::ParseIQM { file } => {
            let data = read(file)?;
            match iqm::IQM::parse_from(&data) {
                Some(iqm) => println!("{:#?}", iqm),
                None => eprintln!("Failed to parse file"),
            }
        }
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Options {
    /// Silence all log output.
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Increase log verbosity (-v, -vv, -vvv, etc. supported).
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,

    /// The subcommand to run.
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    /// Parses and prints an IQM file.
    #[structopt(name = "parse-iqm")]
    ParseIQM { file: PathBuf },
}
