use std::sync::Mutex;

use crate::{
    misc::memory_bus::MemoryBus48, motherboards::Monarch64Motherboard,
    peripherals::storage::monad_boot_cartridge::MonadBootCartridge,
};

pub struct MonadMotherboard {
    pub cpu: Box<dyn crate::cpus::Monarch64CPU>,
    pub io_bus: Mutex<crate::misc::io_bus::IoBus>,
    pub boot_cartridge: Option<MonadBootCartridge>,
}

impl Monarch64Motherboard for MonadMotherboard {
    fn get_cpu(&self) -> &dyn crate::cpus::Monarch64CPU {
        &*self.cpu
    }

    fn get_cpu_mut(&mut self) -> &mut dyn crate::cpus::Monarch64CPU {
        &mut *self.cpu
    }

    fn set_cpu(&mut self, cpu: Box<dyn crate::cpus::Monarch64CPU>) {
        self.cpu = cpu;
    }

    fn run_cpu(&mut self, memory_bus: &Mutex<MemoryBus48>) {
        self.cpu.run_cpu(memory_bus, &self.io_bus);
    }

    fn init(&mut self, memory_bus: &Mutex<MemoryBus48>) {
        // First, we have to load the boot cartridge into RAM if it exists
        if let Some(cartridge) = &self.boot_cartridge {
            if cartridge.get_revision() > 0 {
                log::error!(
                    "Monad Motherboard: Unsupported boot cartridge revision: {}. No data loaded.",
                    cartridge.get_revision()
                );
            } else {
                let data = cartridge.get_data();
                if data.len() <= memory_bus.lock().unwrap().get_size() {
                    memory_bus.lock().unwrap().write_bytes(0, data);
                } else {
                    log::error!(
                        "Monad Motherboard: Boot cartridge data is larger than RAM size. No data loaded."
                    );
                }
            }
        } else {
            log::error!("Monad Motherboard: No boot cartridge inserted. No data loaded.");
        }
    }
}

impl MonadMotherboard {
    pub fn new(cpu: Box<dyn crate::cpus::Monarch64CPU>) -> Self {
        Self {
            cpu,
            boot_cartridge: None,
            io_bus: Mutex::new(crate::misc::io_bus::IoBus::new()),
        }
    }

    pub fn with_boot_cartridge(mut self, cartridge: MonadBootCartridge) -> Self {
        self.boot_cartridge = Some(cartridge);
        self
    }

    pub fn set_boot_cartridge(&mut self, cartridge: MonadBootCartridge) {
        self.boot_cartridge.replace(cartridge);
    }

    pub fn remove_boot_cartridge(&mut self) {
        self.boot_cartridge = None;
    }
}
