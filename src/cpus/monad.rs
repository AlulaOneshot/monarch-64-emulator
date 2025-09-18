use std::sync::Mutex;

use crate::{cpus::Monarch64CPU, misc::memory_bus::MemoryBus48};

pub struct MonadCPU {
    pub r0: u64,
    pub r1: u64,
    pub r2: u64,
    pub r3: u64,
    pub r4: u64,
    pub r5: u64,
    pub r6: u64,
    pub r7: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
    pub rip: u64,
    pub rsp: u64,
    pub rpt: u64,
    pub rit: u64,
    pub cr0: u64,
    pub cr1: u64,
    imm0: u64,
    imm1: u64,
    imm2: u64,
    imm3: u64,
    imm4: u64,
    imm5: u64,
    imm6: u64,
    imm7: u64,
    running: bool,
}

impl Monarch64CPU for MonadCPU {
    fn execute_cycle(&mut self, memory_bus: &Mutex<MemoryBus48>) {
        let operation: u64;
        operation = u64::from_le_bytes(
            memory_bus
                .lock()
                .unwrap()
                .read_bytes(self.rip, 8)
                .try_into()
                .unwrap(),
        );
        log::info!("Executing Operation 0x{:X} at address 0x{:X}", operation, self.rip);
        let opcode = (operation & 0xFFFF) as u16;

        match opcode {
            0x0001 => {
                self.smemb(operation, memory_bus);
            }
            0x0002 => {
                self.smemw(operation, memory_bus);
            }
            0x0003 => {
                self.smemd(operation, memory_bus);
            }
            0x0004 => {
                self.smemq(operation, memory_bus);
            },
            0x0005 => {
                self.lmemb(operation, memory_bus);
            },
            0x0006 => {
                self.lmemw(operation, memory_bus);
            },
            0x0007 => {
                self.lmemd(operation, memory_bus);
            },
            0x0008 => {
                self.lmemq(operation, memory_bus);
            },
            0x0009 => {
                self.lli(operation);
            },
            0x000A => {
                self.lui(operation);
            },
            0x000B => {
                self.cbw(operation);
            }
            0x000C => {
                self.cbws(operation);
            }
            0x000D => {
                self.cwd(operation); //Chronic Wasting Disease?!? Riproducer!?! Lol
            },
            0x000E => {
                self.cwds(operation);
            },
            0x000F => {
                self.cdq(operation);
            }
            0x0010 => {
                self.cdqs(operation);
            }
            0x030E => {
                log::info!("HALT encountered. Stopping execution.");
                self.running = false;
            }
            _ => {
                log::error!("Unknown opcode encountered: {:#X}", opcode);
                panic!("Unknown opcode");
            }
        }
        self.rip += 8;
    }

    fn run_cpu(&mut self, memory_bus: &Mutex<MemoryBus48>) {
        self.running = true;
        while self.running {
            self.execute_cycle(memory_bus);
        }
    }
}

impl MonadCPU {
    fn smemb(&mut self, operation: u64, memory_bus: &Mutex<MemoryBus48>) {
        let source_reg = ((operation >> 16) & 0xFFFF) as u16;
        let dest_reg = ((operation >> 32) & 0xFFFF) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFF) as u8;
        let dest_address = self.get_register_value_from_code(dest_reg);

        memory_bus
            .lock()
            .unwrap()
            .write_bytes(dest_address, source_value.to_le_bytes().as_ref());
    }

    fn smemw(&mut self, operation: u64, memory_bus: &Mutex<MemoryBus48>) {
        let source_reg = ((operation >> 16) & 0xFFFF) as u16;
        let dest_reg = ((operation >> 32) & 0xFFFF) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFFFF) as u16;
        let dest_address = self.get_register_value_from_code(dest_reg);

        memory_bus
            .lock()
            .unwrap()
            .write_bytes(dest_address, source_value.to_le_bytes().as_ref());
    }

    fn smemd(&mut self, operation: u64, memory_bus: &Mutex<MemoryBus48>) {
        let source_reg = ((operation >> 16) & 0xFFFF) as u16;
        let dest_reg = ((operation >> 32) & 0xFFFF) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFFFFFFFF) as u32;
        let dest_address = self.get_register_value_from_code(dest_reg);

        memory_bus
            .lock()
            .unwrap()
            .write_bytes(dest_address, source_value.to_le_bytes().as_ref());
    }

    fn smemq(&mut self, operation: u64, memory_bus: &Mutex<MemoryBus48>) {
        let source_reg = ((operation >> 16) & 0xFFFF) as u16;
        let dest_reg = ((operation >> 32) & 0xFFFF) as u16;

        let source_value = self.get_register_value_from_code(source_reg);
        let dest_address = self.get_register_value_from_code(dest_reg);

        memory_bus
            .lock()
            .unwrap()
            .write_bytes(dest_address, source_value.to_le_bytes().as_ref());
    }

    fn lmemb(&mut self, operation: u64, memory_bus: &Mutex<MemoryBus48>) {
        let source_reg = ((operation >> 16) & 0xFFFF) as u16;
        let dest_reg = ((operation >> 32) & 0xFFFF) as u16;

        let source_address = self.get_register_value_from_code(source_reg);

        let dest_value = self.get_register_value_from_code(dest_reg);

        self.set_register_value_from_code(
            dest_reg,
            (dest_value & 0xFFFFFFFFFFFFFF00) | ((u8::from_le_bytes(memory_bus.lock().unwrap().read_bytes(source_address, 1).try_into().unwrap()))) as u64
        );
    }

    fn lmemw(&mut self, operation: u64, memory_bus: &Mutex<MemoryBus48>) {
        let source_reg = ((operation >> 16) & 0xFFFF) as u16;
        let dest_reg = ((operation >> 32) & 0xFFFF) as u16;

        let source_address = self.get_register_value_from_code(source_reg);

        let dest_value = self.get_register_value_from_code(dest_reg);

        self.set_register_value_from_code(
            dest_reg,
            (dest_value & 0xFFFFFFFFFFFF0000) | ((u16::from_le_bytes(memory_bus.lock().unwrap().read_bytes(source_address, 2).try_into().unwrap()))) as u64
        );
    }

    fn lmemd(&mut self, operation: u64, memory_bus: &Mutex<MemoryBus48>) {
        let source_reg = ((operation >> 16) & 0xFFFF) as u16;
        let dest_reg = ((operation >> 32) & 0xFFFF) as u16;

        let source_address = self.get_register_value_from_code(source_reg);

        let dest_value = self.get_register_value_from_code(dest_reg);

        self.set_register_value_from_code(
            dest_reg,
            (dest_value & 0xFFFFFFFF00000000) | ((u32::from_le_bytes(memory_bus.lock().unwrap().read_bytes(source_address, 4).try_into().unwrap()))) as u64
        );
    }

    fn lmemq(&mut self, operation: u64, memory_bus: &Mutex<MemoryBus48>) {
        let source_reg = ((operation >> 16) & 0xFFFF) as u16;
        let dest_reg = ((operation >> 32) & 0xFFFF) as u16;

        let source_address = self.get_register_value_from_code(source_reg);

        self.set_register_value_from_code(
            dest_reg,
            u64::from_le_bytes(memory_bus.lock().unwrap().read_bytes(source_address, 8).try_into().unwrap())
        );
    }

    fn lli(&mut self, operation: u64) {
        let imm_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        if (imm_reg < 0xF000) | (imm_reg > 0xF007) {
            log::error!("Tried to load immediate into a non imm register.");
            panic!("Invalid immediate register");
        }

        let value = ((operation & 0xFFFFFFFF00000000) >> 32) as u32;
        let initial_value = self.get_register_value_from_code(imm_reg);
        let new_value = (initial_value & 0xFFFFFFFF00000000) | value as u64;
        self.set_register_value_from_code(imm_reg, new_value);
    }

    fn lui(&mut self, operation: u64) {
        let imm_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        if (imm_reg < 0xF000) || (imm_reg > 0xF007) {
            log::error!("Tried to load immediate into a non imm register.");
            panic!("Invalid immediate register");
        }

        let value = ((operation & 0xFFFFFFFF00000000) >> 32) as u32;
        let initial_value = self.get_register_value_from_code(imm_reg);
        let new_value = (initial_value & 0x00000000FFFFFFFF) | ((value as u64) << 32); // Adjust to upper of register
        self.set_register_value_from_code(imm_reg, new_value);
    }

    fn cbw(&mut self, operation: u64) {
        let source_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let converted_value = (dest_value & 0xFFFFFFFFFFFF0000) | source_value as u64;
        self.set_register_value_from_code(dest_reg, converted_value);
    }

    fn cbws(&mut self, operation: u64) {
        let source_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let sign_bit = (source_value & 0b10000000) >> 7;
        let converted_value = if sign_bit == 1 {
            (dest_value & 0xFFFFFFFFFFFF0000) | (((source_value as u16) | 0xFF00) as u64)
        } else {
            (dest_value & 0xFFFFFFFFFFFF0000) | source_value as u64
        };

        self.set_register_value_from_code(dest_reg, converted_value);
    }

    fn cwd(&mut self, operation: u64) {
        let source_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let converted_value = (dest_value & 0xFFFFFFFF00000000) | source_value as u64;
        self.set_register_value_from_code(dest_reg, converted_value);
    }

    fn cwds(&mut self, operation: u64) {
        let source_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let sign_bit = (source_value & 0b1000000000000000) >> 15;
        let converted_value = if sign_bit == 1 {
            (dest_value & 0xFFFFFFFF00000000) | (((source_value as u32) | 0xFFFF0000) as u64)
        } else {
            (dest_value & 0xFFFFFFFF00000000) | source_value as u64
        };

        self.set_register_value_from_code(dest_reg, converted_value);
    }

    fn cdq(&mut self, operation: u64) {
        let source_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFFFFFFFF) as u16;

        let converted_value = source_value as u64;
        self.set_register_value_from_code(dest_reg, converted_value);
    }

    fn cdqs(&mut self, operation: u64) {
        let source_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFFFFFFFF) as u32;

        let sign_bit = (source_value & 0b10000000000000000000000000000000) >> 31;
        let converted_value = if sign_bit == 1 {
            (source_value as u64) | 0xFFFFFFFF00000000
        } else {
            source_value as u64
        };

        self.set_register_value_from_code(dest_reg, converted_value);
    }

    fn get_register_value_from_code(&mut self, code: u16) -> u64 {
        match code {
            0x0000 => self.r0,
            0x0001 => self.r1,
            0x0002 => self.r2,
            0x0003 => self.r3,
            0x0004 => self.r4,
            0x0005 => self.r5,
            0x0006 => self.r6,
            0x0007 => self.r7,
            0x0008 => self.r8,
            0x0009 => self.r9,
            0x000A => self.r10,
            0x000B => self.r11,
            0x000C => self.r12,
            0x000D => self.r13,
            0x000E => self.r14,
            0x000F => self.r15,
            0x0010 => self.rflags,
            0x0011 => self.rip,
            0x0012 => self.rsp,
            0x0013 => self.rpt,
            0x0014 => self.rit,
            0x0015 => self.cr0,
            0x0016 => self.cr1,
            0xF000 => self.imm0,
            0xF001 => self.imm1,
            0xF002 => self.imm2,
            0xF003 => self.imm3,
            0xF004 => self.imm4,
            0xF005 => self.imm5,
            0xF006 => self.imm6,
            0xF007 => self.imm7,
            _ => {
                log::error!("Invalid register code: {:#X}", code);
                panic!("Invalid register code");
            }
        }
    }

    fn set_register_value_from_code(&mut self, code: u16, value: u64) {
        let register: &mut u64 = match code {
            0x0000 => &mut self.r0,
            0x0001 => &mut self.r1,
            0x0002 => &mut self.r2,
            0x0003 => &mut self.r3,
            0x0004 => &mut self.r4,
            0x0005 => &mut self.r5,
            0x0006 => &mut self.r6,
            0x0007 => &mut self.r7,
            0x0008 => &mut self.r8,
            0x0009 => &mut self.r9,
            0x000A => &mut self.r10,
            0x000B => &mut self.r11,
            0x000C => &mut self.r12,
            0x000D => &mut self.r13,
            0x000E => &mut self.r14,
            0x000F => &mut self.r15,
            0x0010 => &mut self.rflags,
            0x0011 => &mut self.rip,
            0x0012 => &mut self.rsp,
            0x0013 => &mut self.rpt,
            0x0014 => &mut self.rit,
            0x0015 => &mut self.cr0,
            0x0016 => &mut self.cr1,
            0xF000 => &mut self.imm0,
            0xF001 => &mut self.imm1,
            0xF002 => &mut self.imm2,
            0xF003 => &mut self.imm3,
            0xF004 => &mut self.imm4,
            0xF005 => &mut self.imm5,
            0xF006 => &mut self.imm6,
            0xF007 => &mut self.imm7,
            _ => {
                log::error!("Invalid register code: {:#X}", code);
                panic!("Invalid register code");
            }
        };

        *register = value;
    }

    pub fn new() -> Self {
        Self {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rflags: 0,
            rip: 0,
            rsp: 0,
            rpt: 0,
            rit: 0,
            cr0: 0,
            cr1: 0,
            imm0: 0,
            imm1: 0,
            imm2: 0,
            imm3: 0,
            imm4: 0,
            imm5: 0,
            imm6: 0,
            imm7: 0,
            running: false,
        }
    }
}
