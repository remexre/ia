use assets::AssetRequestExt;
use ecstasy::Engine;
use libremexre::errors::Result;
use log::info;
use renderer::init_renderer;
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
    #[structopt(short = "m", long = "model")]
    model_file: PathBuf,

    /// The file to load a shader bundle from.
    #[structopt(short = "s", long = "shaders")]
    shader_bundle_file: PathBuf,

    /// The file to load a texture from.
    #[structopt(short = "t", long = "texture")]
    texture_file: PathBuf,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    libremexre::init_logger(options.verbose, options.quiet);

    // Start the renderer.
    let (renderer, mut event_loop) = init_renderer!()?;

    // Assemble the parts into the engine.
    let mut engine = Engine::new().build_par_pass().add(renderer).finish();

    // Create the main entity.
    let entity = engine.store.new_entity();
    engine
        .store
        .request_asset::<assets::Model>(entity, options.model_file);
    engine
        .store
        .request_asset::<assets::Program>(entity, options.shader_bundle_file);
    engine
        .store
        .request_asset::<assets::Texture>(entity, options.texture_file);

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
