use std::sync::Mutex;

use crate::misc::memory_bus::MemoryBus48;

pub mod monad;

pub trait Monarch64Motherboard {
    fn get_cpu(&self) -> &dyn crate::cpus::Monarch64CPU;
    fn get_cpu_mut(&mut self) -> &mut dyn crate::cpus::Monarch64CPU;
    fn set_cpu(&mut self, cpu: Box<dyn crate::cpus::Monarch64CPU>);
    fn run_cpu(&mut self, memory_bus: &Mutex<MemoryBus48>);

    fn init(&mut self, memory_bus: &Mutex<MemoryBus48>);
}
