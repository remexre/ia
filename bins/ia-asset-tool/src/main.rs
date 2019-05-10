use libremexre::{err, errors::Result};
use shaderc::{Compiler, ShaderKind};
use std::{
    fs::{read, read_to_string},
    path::PathBuf,
};
use structopt::StructOpt;

fn main() -> Result<()> {
    let options = Options::from_args();
    libremexre::init_logger(options.verbose, options.quiet);

    match options.subcommand {
        Subcommand::CompileShaders {
            vertex_shader,
            fragment_shader,
        } => {
            let mut compiler =
                Compiler::new().ok_or_else(|| err!("Couldn't initialize shader compiler"))?;
            let vs = read_to_string(&vertex_shader)?;
            let fs = read_to_string(&fragment_shader)?;

            let vs = compiler.compile_into_spirv(
                &vs,
                ShaderKind::Vertex,
                &vertex_shader.display().to_string(),
                "main",
                None,
            )?;
            let fs = compiler.compile_into_spirv(
                &fs,
                ShaderKind::Fragment,
                &fragment_shader.display().to_string(),
                "main",
                None,
            )?;

            unimplemented!()
        }
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
    /// Compiles a vertex shader and a fragment shader into a shader bundle.
    #[structopt(name = "compile-shaders")]
    CompileShaders {
        vertex_shader: PathBuf,
        fragment_shader: PathBuf,
    },

    /// Parses and prints an IQM file.
    #[structopt(name = "parse-iqm")]
    ParseIQM { file: PathBuf },
}
