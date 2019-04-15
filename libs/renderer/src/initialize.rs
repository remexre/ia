use crate::Renderer;
use libremexre::err;
use log::info;
use std::error::Error;
use vulkano::{
    device::{Device, DeviceExtensions, Features},
    instance::{Instance, InstanceExtensions, PhysicalDevice, PhysicalDeviceType},
};

impl Renderer {
    /// Creates a new `Renderer`.
    pub fn new() -> Result<Renderer, Box<dyn Error>> {
        let instance = Instance::new(None, &InstanceExtensions::none(), None)?;

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

        let (dev, queues) = Device::new(
            pd,
            &Features::none(),
            &DeviceExtensions::none(),
            Some((qf, 0.5)),
        )?;

        let mut queues = queues.into_iter().collect::<Vec<_>>();
        if queues.is_empty() {
            return Err(err!("Primary Vulkan devices had no queues"));
        }
        let queue = queues.remove(0);
        drop(queues);

        Ok(Renderer {
            dev,
            instance,
            queue,
        })
    }
}
