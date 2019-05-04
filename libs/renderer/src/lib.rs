//! A Vulkan-based renderer.
#![deny(
    bad_style,
    bare_trait_objects,
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
use ecstasy::{ComponentStore, System};
use log::error;
use std::sync::Arc;
use vulkano::{
    command_buffer::AutoCommandBufferBuilder,
    device::{Device, Queue},
    format::ClearValue,
    image::SwapchainImage,
    instance::Instance,
    swapchain::{Surface, Swapchain},
    sync::GpuFuture,
};
use winit::Window;

/// The main renderer value.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Renderer {
    device: Arc<Device>,
    #[derivative(Debug = "ignore")]
    images: Vec<Arc<SwapchainImage<Window>>>,
    instance: Arc<Instance>,
    queue: Arc<Queue>,
    surface: Arc<Surface<Window>>,
    swapchain: Arc<Swapchain<Window>>,

    #[derivative(Debug = "ignore")]
    cleanup_future: Option<Box<dyn GpuFuture + Send>>,
}

impl System for Renderer {
    fn run(&mut self, _cs: &ComponentStore, _dt: f32) {
        if let Some(cleanup_future) = self.cleanup_future.as_mut() {
            cleanup_future.cleanup_finished();
        }

        let device = self.device.clone();
        let queue = self.queue.clone();
        let queue_family = queue.family();
        self.draw(|image| {
            AutoCommandBufferBuilder::primary_one_time_submit(device, queue_family)?
                .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 0.5, 1.0]))?
                .build()
                .map_err(From::from)
        })
        .unwrap_or_else(|err| error!("{:?}\n{}", err, err));
    }
}
