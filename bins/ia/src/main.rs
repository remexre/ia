use ecstasy::Engine;
use libremexre::errors::Result;
use log::info;
use renderer::init_renderer;
use structopt::StructOpt;
use winit::{Event, WindowEvent};

fn main() -> Result<()> {
    let options = Options::from_args();
    libremexre::init_logger(options.verbose + 1, options.quiet);

    // Create an asset loader, including bundled assets.
    // let mut asset_loader = assets::Loader::new();
    #[cfg(feature = "bundle_assets")]
    for file in <bundle_assets::Assets as packer::Packer>::list() {
        println!("{:?}", file);
    }

    // Start the renderer.
    let (renderer, mut event_loop) = init_renderer!()?;

    // Assemble the parts into the engine.
    let mut engine = Engine::new().build_par_pass().add(renderer).finish();

    let mut keep_running = true;
    while keep_running {
        event_loop.poll_events(|ev| {
            if let Event::WindowEvent { event: ev, .. } = ev {
                match ev {
                    WindowEvent::CloseRequested => keep_running = false,
                    ev => {
                        info!("TODO: Handle event {:?}", ev);
                    }
                }
            }
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
