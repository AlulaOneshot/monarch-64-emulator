use std::sync::Mutex;

pub struct Monarch64System {
    pub motherboard: Box<dyn crate::motherboards::Monarch64Motherboard>,
    pub memory_bus: Mutex<crate::misc::memory_bus::MemoryBus48>,
}

impl Monarch64System {
    pub fn new(motherboard: Box<dyn crate::motherboards::Monarch64Motherboard>) -> Self {
        Self {
            motherboard,
            memory_bus: Mutex::new(crate::misc::memory_bus::MemoryBus48::new()),
        }
    }
}
