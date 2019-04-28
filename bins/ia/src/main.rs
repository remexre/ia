use ecstasy::ComponentStore;
use libremexre::errors::Result;
use log::info;
use renderer::Renderer;
use structopt::StructOpt;
use winit::{Event, WindowEvent};

fn main() -> Result<()> {
    let options = Options::from_args();
    stderrlog::new()
        .quiet(options.quiet)
        .verbosity(options.verbose + 1)
        .init()?;

    let mut renderer = Renderer::new()?;

    let mut cs = ComponentStore::new();
    let mut keep_running = true;
    while keep_running {
        renderer.poll_events(|ev| match ev {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => keep_running = false,
            _ => info!("TODO: Handle event {:?}", ev),
        });
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
}
