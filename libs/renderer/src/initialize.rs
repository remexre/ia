use crate::Renderer;
use libremexre::{err, errors::Result};
use log::info;
use std::sync::Arc;
use vulkano::{
    device::{Device, DeviceExtensions, Features, Queue},
    image::SwapchainImage,
    instance::{Instance, PhysicalDevice, PhysicalDeviceType},
    swapchain::{PresentMode, Surface, SurfaceTransform, Swapchain},
    sync::now,
};
use vulkano_win::VkSurfaceBuild;
use winit::{EventsLoop, Window, WindowBuilder};

impl Renderer {
    /// Creates a new `Renderer`.
    pub fn new() -> Result<(Renderer, EventsLoop)> {
        let instance = Instance::new(None, &vulkano_win::required_extensions(), None)?;

        let mut pds = PhysicalDevice::enumerate(&instance)
            .map(|pd| {
                let mem_bytes: usize = pd.memory_heaps().map(|h| h.size()).sum();
                let mem_gb = mem_bytes >> 30;
                let qf_count: usize = pd
                    .queue_families()
                    .filter(|qf| qf.supports_graphics())
                    .map(|qf| qf.queues_count())
                    .sum();
                let type_priority = match pd.ty() {
                    PhysicalDeviceType::DiscreteGpu => 5,
                    PhysicalDeviceType::IntegratedGpu => 4,
                    PhysicalDeviceType::VirtualGpu => 3,
                    PhysicalDeviceType::Cpu => 2,
                    PhysicalDeviceType::Other => 1,
                };
                let priority = if qf_count == 0 {
                    0
                } else {
                    (100 * type_priority) + (10 * mem_gb) + qf_count
                };
                (pd, priority)
            })
            .collect::<Vec<_>>();
        pds.sort_by_key(|&(_, priority)| priority);

        if pds.is_empty() {
            return Err(err!("No Vulkan devices found"));
        }

        info!("Found {} Vulkan devices:", pds.len());
        for (n, (pd, priority)) in pds.iter().enumerate() {
            info!("#{} (p={}): {}", n, priority, pd.name());
        }
        let pd = pds.remove(0).0;
        drop(pds);

        let mut qfs = pd
            .queue_families()
            .filter(|&qf| qf.supports_graphics())
            .collect::<Vec<_>>();
        qfs.sort_by_key(|qf| qf.queues_count());

        if qfs.is_empty() {
            return Err(err!("Primary Vulkan devices had no queue families"));
        }

        let qf = qfs.remove(0);
        drop(qfs);

        let device_exts = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        let (device, queues) = Device::new(pd, &Features::none(), &device_exts, Some((qf, 0.5)))?;

        let mut queues = queues.into_iter().collect::<Vec<_>>();
        if queues.is_empty() {
            return Err(err!("Primary Vulkan devices had no queues"));
        }
        let queue = queues.remove(0);
        drop(queues);

        let event_loop = EventsLoop::new();
        let surface = WindowBuilder::new()
            .with_title("ia")
            .build_vk_surface(&event_loop, instance.clone())?;

        let (swapchain, images) = make_swapchain(surface.clone(), device.clone(), &queue)?;

        let cleanup_future = now(device.clone());

        Ok((
            Renderer {
                device,
                images,
                instance,
                queue,
                surface,
                swapchain,

                cleanup_future: Some(Box::new(cleanup_future)),
            },
            event_loop,
        ))
    }

    /// Recreates the swapchain, to account for e.g. a new window size.
    pub(crate) fn recreate_swapchain(&mut self) -> Result<()> {
        let (swapchain, images) =
            make_swapchain(self.surface.clone(), self.device.clone(), &self.queue)?;
        self.swapchain = swapchain;
        self.images = images;
        Ok(())
    }
}

/// Creates the swapchain. Defined here to share code between `Renderer::new` and
/// `Renderer::recreate_swapchain`.
fn make_swapchain(
    surface: Arc<Surface<Window>>,
    device: Arc<Device>,
    queue: &Arc<Queue>,
) -> Result<(Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>)> {
    let window = surface.window();
    let caps = surface.capabilities(device.physical_device())?;
    let dims = window
        .get_inner_size()
        .map(|dims| dims.to_physical(window.get_hidpi_factor()).into())
        .map(|(w, h)| [w, h])
        .or_else(|| caps.current_extent)
        .ok_or_else(|| err!("window was closed"))?;
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;
    Swapchain::new(
        device,
        surface,
        caps.min_image_count,
        format,
        dims,
        1,
        caps.supported_usage_flags,
        queue,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        true,
        None,
    )
    .map_err(From::from)
}
