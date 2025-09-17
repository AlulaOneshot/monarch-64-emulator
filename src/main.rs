use crate::system::Monarch64System;

pub mod cpus;
pub mod motherboards;
pub mod system;

pub fn main() {
    let system = Monarch64System::new();
}