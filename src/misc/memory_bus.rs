pub struct MemoryBus48 {
    ram: [u8; 1024 * 4], // 4KB of RAM for simplicity
}

impl MemoryBus48 {
    pub fn new() -> Self {
        Self { ram: [0; 1024 * 4] }
    }

    pub fn dump_memory(&self, start: u64, length: usize) -> String {
        let end = (start as usize + length).min(self.ram.len());
        self.ram[start as usize..end]
            .iter()
            .enumerate()
            .map(|(i, byte)| format!("{:04X}: {:02X}", start as usize + i, byte))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn read_bytes(&self, address: u64, length: usize) -> &[u8] {
        if address + length as u64 <= self.ram.len() as u64 {
            &self.ram[address as usize..(address as usize + length)]
        } else {
            log::error!(
                "Monad Motherboard: Attempted to read beyond RAM bounds: {:#X} + {} bytes",
                address,
                length
            );
            &[]
        }
    }

    pub fn write_bytes(&mut self, base_address: u64, value: &[u8]) {
        let length = value.len();
        if base_address + length as u64 <= self.ram.len() as u64 {
            self.ram[base_address as usize..(base_address as usize + length)]
                .copy_from_slice(value);
        } else {
            log::error!(
                "Monad Motherboard: Attempted to write beyond RAM bounds: {:#X} + {} bytes",
                base_address,
                length
            );
        }
    }

    pub fn get_size(&self) -> usize {
        self.ram.len()
    }
}
