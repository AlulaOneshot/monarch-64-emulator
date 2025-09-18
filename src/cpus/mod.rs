use std::sync::Mutex;

use crate::misc::memory_bus::MemoryBus48;

pub mod monad;

pub trait Monarch64CPU {
    fn execute_cycle(&mut self, memory_bus: &Mutex<MemoryBus48>);
    fn run_cpu(&mut self, memory_bus: &Mutex<MemoryBus48>);
}
