use ecstasy::Engine;
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

    // Create an asset loader, including bundled assets.
    // let mut asset_loader = assets::Loader::new();
    #[cfg(feature = "bundle_assets")]
    for file in <bundle_assets::Assets as packer::Packer>::list() {
        println!("{:?}", file);
    }

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

#[derive(Debug, StructOpt)]
struct Options {
    /// Silence all log output.
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Increase log verbosity (-v, -vv, -vvv, etc. supported).
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

#[cfg(feature = "bundle_assets")]
mod bundle_assets;
