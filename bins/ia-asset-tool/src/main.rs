// use assets::{Program, ProgramInner, ProgramSafetyPromise};
use libremexre::{catch, err, errors::Result};
use log::{error, warn};
use serde_cbor::to_vec;
use shaderc::{Compiler, ShaderKind};
use std::{
    fs::{read, read_to_string, write},
    path::PathBuf,
    process::exit,
    sync::Arc,
};
use structopt::StructOpt;

fn main() -> Result<()> {
    let options = Options::from_args();
    libremexre::init_logger(options.verbose, options.quiet);

    match options.subcommand {
        Subcommand::CompileShaders {
            vertex_shader,
            fragment_shader,
            shader_bundle,
        } => {
            let mut compiler =
                Compiler::new().ok_or_else(|| err!("Couldn't initialize shader compiler"))?;
            let vs = read_to_string(&vertex_shader)?;
            let fs = read_to_string(&fragment_shader)?;

            let r = catch(|| -> shaderc::Result<_> {
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
                Ok((vs, fs))
            });
            let (vs, fs) = match r {
                Ok(r) => r,
                Err(shaderc::Error::CompilationError(n, msg)) => {
                    error!("Failed to compile shaders ({}):\n\n{}", n, msg);
                    exit(1);
                }
                Err(err) => return Err(Box::new(err)),
            };

            unimplemented!();
            /*
            let program = Program::from(Arc::new(ProgramInner {
                vert_bytes: vs.as_binary_u8().to_owned(),
                frag_bytes: fs.as_binary_u8().to_owned(),
                promise: unsafe { ProgramSafetyPromise::i_promise() },
            }));
            let bundle = to_vec(&program)?;
            write(shader_bundle, &bundle)?;
            */

            if vs.get_num_warnings() > 0 {
                warn!("{}", vs.get_warning_messages());
            }
            if fs.get_num_warnings() > 0 {
                warn!("{}", fs.get_warning_messages());
            }
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
        #[structopt(short = "o", long = "output")]
        shader_bundle: PathBuf,
    },

    /// Parses and prints an IQM file.
    #[structopt(name = "parse-iqm")]
    ParseIQM { file: PathBuf },
}
