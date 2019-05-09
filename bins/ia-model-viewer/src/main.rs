use ecstasy::Engine;
use libremexre::errors::Result;
use log::info;
use renderer::Renderer;
use std::path::PathBuf;
use structopt::StructOpt;
use winit::{Event, WindowEvent};

#[derive(Debug, StructOpt)]
struct Options {
    /// Silence all log output.
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Increase log verbosity (-v, -vv, -vvv, etc. supported).
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,

    /// The file to load a model from.
    model_file: PathBuf,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    stderrlog::new()
        .quiet(options.quiet)
        .verbosity(options.verbose)
        .init()?;

    // Start the renderer.
    let (renderer, mut event_loop) = Renderer::new()?;

    // Assemble the parts into the engine.
    let mut engine = Engine::new().build_par_pass().add(renderer).finish();

    let mut keep_running = true;
    while keep_running {
        event_loop.poll_events(|ev| match ev {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => keep_running = false,
            _ => info!("TODO: Handle event {:?}", ev),
        });
        engine.run_once();
    }

    Ok(())
}
