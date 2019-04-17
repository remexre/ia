//! A Vulkan-based renderer.
#![deny(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    missing_debug_implementations,
    missing_docs,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    safe_extern_statics,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unions_with_drop_fields,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    unused_results,
    while_true
)]

pub mod components;
mod draw;
mod initialize;

use derivative::Derivative;
use std::sync::Arc;
use vulkano::{
    device::{Device, Queue},
    image::SwapchainImage,
    instance::Instance,
    swapchain::{Surface, Swapchain},
    sync::GpuFuture,
};
use winit::{Event, EventsLoop, Window};

/// The main renderer value.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Renderer {
    device: Arc<Device>,
    event_loop: EventsLoop,
    #[derivative(Debug = "ignore")]
    images: Vec<Arc<SwapchainImage<Window>>>,
    instance: Arc<Instance>,
    queue: Arc<Queue>,
    surface: Arc<Surface<Window>>,
    swapchain: Arc<Swapchain<Window>>,

    #[derivative(Debug = "ignore")]
    cleanup_future: Option<Box<dyn GpuFuture>>,
}

impl Renderer {
    /// Runs the provided closure for each event.
    ///
    /// This also performs cleanup actions.
    pub fn poll_events<F: FnMut(Event)>(&mut self, cb: F) {
        self.cleanup_future.as_mut().unwrap().cleanup_finished();
        self.event_loop.poll_events(cb)
    }
}
