use assets::{irb::IRB, Assets};
use ecstasy::Engine;
use libremexre::errors::Result;
use log::info;
use renderer::init_renderer;
use structopt::StructOpt;
use winit::{Event, WindowEvent};

fn main() -> Result<()> {
    let options = Options::from_args();
    libremexre::init_logger(options.verbose + 1, options.quiet);

    // Start the renderer, load the assets, and assemble the parts into the engine.
    let (renderer, mut event_loop) = init_renderer!()?;
    let (assets, errs) = Assets::from_irb(IRB::load_from_file("")?, renderer.device());
    if !errs.is_empty() {
        let mut s = "Errors loading assets:".to_string();
        for err in errs {
            s += "\n";
            s += &err.to_string();
        }
        return Err(libremexre::err!("{}", s));
    }
    let mut engine = Engine::new(assets).build_par_pass().add(renderer).finish();

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
