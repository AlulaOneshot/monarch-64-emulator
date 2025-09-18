pub struct IoHandler {
    pub read_u8: fn(port: u16) -> u8,
    pub read_u16: fn(port: u16) -> u16,
    pub read_u32: fn(port: u16) -> u32,
    pub read_u64: fn(port: u16) -> u64,
    pub write_u8: fn(port: u16, value: u8),
    pub write_u16: fn(port: u16, value: u16),
    pub write_u32: fn(port: u16, value: u32),
    pub write_u64: fn(port: u16, value: u64),
}

pub struct IoBus {
    pub io_handlers: Vec<IoHandler>,
}

impl IoBus {
    pub fn new() -> Self {
        IoBus {io_handlers: vec![]}
    }

    pub fn read_u8(&self, port: u16) -> u8 {
        let handler = &self.io_handlers[port as usize];
        (handler.read_u8)(port)
    }

    pub fn read_u16(&self, port: u16) -> u16 {
        let handler = &self.io_handlers[port as usize];
        (handler.read_u16)(port)
    }

    pub fn read_u32(&self, port: u16) -> u32 {
        let handler = &self.io_handlers[port as usize];
        (handler.read_u32)(port)
    }

    pub fn read_u64(&self, port: u16) -> u64 {
        let handler = &self.io_handlers[port as usize];
        (handler.read_u64)(port)
    }

    pub fn write_u8(&mut self, port: u16, value: u8) {
        let handler = &self.io_handlers[port as usize];
        (handler.write_u8)(port, value);
    }

    pub fn write_u16(&mut self, port: u16, value: u16) {
        let handler = &self.io_handlers[port as usize];
        (handler.write_u16)(port, value);
    }

    pub fn write_u32(&mut self, port: u16, value: u32) {
        let handler = &self.io_handlers[port as usize];
        (handler.write_u32)(port, value);
    }

    pub fn write_u64(&mut self, port: u16, value: u64) {
        let handler = &self.io_handlers[port as usize];
        (handler.write_u64)(port, value);
    }
}