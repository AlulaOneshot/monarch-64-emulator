use crate::{
    motherboards::monad::MonadMotherboard,
    peripherals::storage::monad_boot_cartridge::MonadBootCartridge, system::Monarch64System,
};

pub mod cpus;
pub mod misc;
pub mod motherboards;
pub mod peripherals;
pub mod system;

pub fn main() {
    // First, we must initialize fern
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").expect("failed to open log file"))
        // Apply globally
        .apply()
        .expect("failed to initialize logging");

    let mut system = Monarch64System::new(Box::new(
        MonadMotherboard::new(Box::new(cpus::monad::MonadCPU::new())).with_boot_cartridge(
            MonadBootCartridge::new(include_bytes!("../test_boot_cartridge.bin")),
        ),
    ));

    system.motherboard.init(&system.memory_bus);

    system.motherboard.run_cpu(&system.memory_bus);
}
