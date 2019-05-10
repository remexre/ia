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
    image::SwapchainImage,
    instance::Instance,
    swapchain::{Surface, Swapchain, SwapchainCreationError},
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
    recreate_swapchain: bool,

    #[derivative(Debug = "ignore")]
    cleanup_future: Option<Box<dyn GpuFuture + Send>>,
}

impl System for Renderer {
    fn run(&mut self, _cs: &ComponentStore, _dt: f32) {
        if let Some(cleanup_future) = self.cleanup_future.as_mut() {
            cleanup_future.cleanup_finished();
        }

        if self.recreate_swapchain {
            let window = self.surface.window();
            let dims = if let Some(dims) = window.get_inner_size() {
                let dims: (u32, u32) = dims.to_physical(window.get_hidpi_factor()).into();
                [dims.0, dims.1]
            } else {
                error!("Could not get window inner size when recreating swapchain; was the window closed?");
                return;
            };
            let (swapchain, images) = match self.swapchain.recreate_with_dimension(dims) {
                Ok(r) => r,
                Err(SwapchainCreationError::UnsupportedDimensions) => return,
                Err(err) => {
                    error!("Failed to recreate swapchain: {:?} ({})", err, err);
                    return;
                }
            };
            self.swapchain = swapchain;
            self.images = images;
            self.recreate_swapchain = false;
        }

        let device = self.device.clone();
        let queue = self.queue.clone();
        let queue_family = queue.family();
        self.draw(|_image| {
            AutoCommandBufferBuilder::primary_one_time_submit(device, queue_family)?
                // .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 0.5, 1.0]))?
                .build()
                .map_err(From::from)
        })
    }
}
