use std::{ops::Neg, sync::Mutex};

use crate::{cpus::Monarch64CPU, misc::{io_bus::IoBus, memory_bus::MemoryBus48}};

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
    fn execute_cycle(&mut self, memory_bus: &Mutex<MemoryBus48>, io_bus: &Mutex<IoBus>) {
        let operation: u64;
        operation = u64::from_le_bytes(
            memory_bus
                .lock()
                .unwrap()
                .read_bytes(self.rip, 8)
                .try_into()
                .unwrap(),
        );
        self.rip += 8;
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
            },
            0x0011 => {
                self.movb(operation);
            },
            0x0012 => {
                self.movw(operation);
            },
            0x0013 => {
                self.movd(operation);
            },
            0x0014 => {
                self.movq(operation);
            },
            // Begin ALU Operations
            0x0100 => {
                self.addb(operation);
            },
            0x0101 => {
                self.addw(operation);
            },
            0x0102 => {
                self.addd(operation);
            },
            0x0103 => {
                self.addq(operation);
            },
            0x0104 => {
                self.addbs(operation);
            },
            0x0105 => {
                self.addws(operation);
            },
            0x0106 => {
                self.addds(operation);
            },
            0x0107 => {
                self.addqs(operation);
            },
            0x0108 => {
                self.subb(operation);
            },
            0x0109 => {
                self.subw(operation);
            },
            0x010A => {
                self.subd(operation);
            },
            0x010B => {
                self.subq(operation);
            },
            0x010C => {
                self.subbs(operation);
            },
            0x010D => {
                self.subws(operation);
            },
            0x010E => {
                self.subds(operation);
            },
            0x010F => {
                self.subqs(operation);
            },
            0x0110 => {
                self.mulb(operation);
            },
            0x0111 => {
                self.mulw(operation);
            },
            0x0112 => {
                self.muld(operation);
            },
            0x0113 => {
                self.mulq(operation);
            },
            0x0114 => {
                self.mulbs(operation);
            },
            0x0115 => {
                self.mulws(operation);
            },
            0x0116 => {
                self.mulds(operation);
            },
            0x0117 => {
                self.mulqs(operation);
            },
            0x0118 => {
                self.divb(operation);
            },
            0x0119 => {
                self.divw(operation);
            },
            0x011A => {
                self.divd(operation);
            },
            0x011B => {
                self.divq(operation);
            },
            0x011C => {
                self.divbs(operation);
            },
            0x011D => {
                self.divws(operation);
            },
            0x011E => {
                self.divds(operation);
            },
            0x011F => {
                self.divqs(operation);
            },
            0x0120 => {
                self.incb(operation);
            },
            0x0121 => {
                self.incw(operation);
            },
            0x0122 => {
                self.incd(operation);
            },
            0x0123 => {
                self.incq(operation);
            },
            0x0124 => {
                self.incbs(operation);
            },
            0x0125 => {
                self.incws(operation);
            },
            0x0126 => {
                self.incds(operation);
            },
            0x0127 => {
                self.incqs(operation);
            },
            0x0128 => {
                self.decb(operation);
            },
            0x0129 => {
                self.decw(operation);
            },
            0x012A => {
                self.decd(operation);
            },
            0x012B => {
                self.decq(operation);
            },
            0x012C => {
                self.decbs(operation);
            },
            0x012D => {
                self.decws(operation);
            },
            0x012E => {
                self.decds(operation);
            },
            0x012F => {
                self.decqs(operation);
            },
            0x0130 => {
                self.negb(operation);
            },
            0x0131 => {
                self.negw(operation);
            },
            0x0132 => {
                self.negd(operation);
            },
            0x0133 => {
                self.negq(operation);
            },
            0x0134 => {
                self.cmpb(operation);
            },
            0x0135 => {
                self.cmpw(operation);
            },
            0x0136 => {
                self.cmpd(operation);
            },
            0x0137 => {
                self.cmpq(operation);
            },
            0x0138 => {
                self.cmpbs(operation);
            },
            0x0139 => {
                self.cmpws(operation);
            },
            0x013A => {
                self.cmpds(operation);
            },
            0x013B => {
                self.cmpqs(operation);
            },
            0x013C => {
                self.andb(operation);
            },
            0x013D => {
                self.andw(operation);
            },
            0x013E => {
                self.andd(operation);
            },
            0x013F => {
                self.andq(operation);
            },
            0x0140 => {
                self.orb(operation);
            },
            0x0141 => {
                self.orw(operation);
            },
            0x0142 => {
                self.ord(operation);
            },
            0x0143 => {
                self.orq(operation);
            },
            0x0144 => {
                self.xorb(operation);
            },
            0x0145 => {
                self.xorw(operation);
            },
            0x0146 => {
                self.xord(operation);
            },
            0x0147 => {
                self.xorq(operation);
            },
            0x0148 => {
                self.notb(operation);
            },
            0x0149 => {
                self.notw(operation);
            },
            0x014A => {
                self.notd(operation);
            },
            0x014B => {
                self.notq(operation);
            },
            0x014C => {
                self.norb(operation);
            },
            0x014D => {
                self.norw(operation);
            },
            0x014E => {
                self.nord(operation);
            },
            0x014F => {
                self.norq(operation);
            },
            0x0150 => {
                self.nandb(operation);
            },
            0x0151 => {
                self.nandw(operation);
            },
            0x0152 => {
                self.nandd(operation);
            },
            0x0153 => {
                self.nandq(operation);
            },
            0x154 => {
                self.shlb(operation);
            },
            0x0155 => {
                self.shlw(operation);
            },
            0x0156 => {
                self.shld(operation);
            },
            0x0157 => {
                self.shlq(operation);
            },
            0x0158 => {
                self.shrb(operation);
            },
            0x0159 => {
                self.shrw(operation);
            },
            0x015A => {
                self.shrd(operation);
            },
            0x015B => {
                self.shrq(operation);
            },
            0x015c => {
                self.rolb(operation);
            },
            0x015d => {
                self.rolw(operation);
            },
            0x015e => {
                self.rold(operation);
            },
            0x015f => {
                self.rolq(operation);
            },
            0x0160 => {
                self.rorb(operation);
            },
            0x0161 => {
                self.rorw(operation);
            },
            0x0162 => {
                self.rord(operation);
            },
            0x0163 => {
                self.rorq(operation);
            },
            // Begin Bitwise Instructions
            0x0200 => {
                self.bitt(operation);
            },
            0x0201 => {
                self.bits(operation);
            },
            0x0202 => {
                self.bitc(operation);
            },
            // Begin Control Transfer Instructions
            0x0300 => {
                self.jmp(operation);
            },
            0x0301 => {
                self.jmpeq(operation);
            },
            0x0302 => {
                self.jmpz(operation);
            },
            0x0303 => {
                self.jmpneq(operation);
            },
            0x0304 => {
                self.jmpnz(operation);
            },
            0x305 => {
                self.jmpgt(operation);
            },
            0x0306 => {
                self.jmpge(operation);
            },
            0x0307 => {
                self.jmplt(operation);
            },
            0x0308 => {
                self.jmple(operation);
            },
            0x0309 => {
                self.jmpo(operation);
            },
            0x030A => {
                self.jmpn(operation);
            },
            0x030B => {
                self.jmpp(operation);
            },
            0x030C => {
                self.int(operation);
            },
            0x030D => {
                self.wfi(operation);
            }
            0x030E => {
                self.rst(operation);
            },
            // Begin I/O Instructions
            0x0400 => {
                self.inb(operation, io_bus);
            },
            0x0401 => {
                self.inw(operation, io_bus);
            },
            0x0402 => {
                self.ind(operation, io_bus);
            },
            0x0403 => {
                self.inq(operation, io_bus);
            },
            0x0404 => {
                self.outb(operation, io_bus);
            },
            0x0405 => {
                self.outw(operation, io_bus);
            },
            0x0406 => {
                self.outd(operation, io_bus);
            },
            0x0407 => {
                self.outq(operation, io_bus);
            },
            0x0000 => {
                // NOP
            },
            0x0FFF => {
                self.cpuid(operation);
            },
            _ => {
                log::error!("Unknown opcode encountered: {:#X}", opcode);
                panic!("Unknown opcode");
            }
        }
    }

    fn run_cpu(&mut self, memory_bus: &Mutex<MemoryBus48>, io_bus: &Mutex<IoBus>) {
        self.running = true;
        while self.running {
            self.execute_cycle(memory_bus, io_bus);
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

    fn movb(&mut self, operation: u64) {
        let source_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let new_value = (dest_value & 0xFFFFFFFFFFFFFF00) | source_value as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn movw(&mut self, operation: u64) {
        let source_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let new_value = (dest_value & 0xFFFFFFFFFFFF0000) | source_value as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn movd(&mut self, operation: u64) {
        let source_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let source_value = (self.get_register_value_from_code(source_reg) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let new_value = (dest_value & 0xFFFFFFFF00000000) | source_value as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn movq(&mut self, operation: u64) {
        let source_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let source_value = self.get_register_value_from_code(source_reg);
        self.set_register_value_from_code(dest_reg, source_value);
    }

    fn addb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_add(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn addw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_add(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn addd(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_add(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn addq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let input_value_2 = self.get_register_value_from_code(input_reg_2);

        let (result, overflow) = input_value_1.overflowing_add(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        self.set_register_value_from_code(dest_reg, result);
    }

    fn addbs(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as i8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as i8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_add(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn addws(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as i16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as i16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_add(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn addds(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as i32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as i32;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_add(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn addqs(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1) as i64;
        let input_value_2 = self.get_register_value_from_code(input_reg_2) as i64;

        let (result, overflow) = input_value_1.overflowing_add(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn subb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_sub(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned subtraction, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn subw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_sub(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn subd(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_sub(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn subq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let input_value_2 = self.get_register_value_from_code(input_reg_2);

        let (result, overflow) = input_value_1.overflowing_sub(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        self.set_register_value_from_code(dest_reg, result);
    }

    fn subbs(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as i8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as i8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_sub(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn subws(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as i16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as i16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_sub(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn subds(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as i32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as i32;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_sub(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn subqs(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1) as i64;
        let input_value_2 = self.get_register_value_from_code(input_reg_2) as i64;

        let (result, overflow) = input_value_1.overflowing_sub(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn mulb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_mul(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn mulw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_mul(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn muld(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_mul(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn mulq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let input_value_2 = self.get_register_value_from_code(input_reg_2);

        let (result, overflow) = input_value_1.overflowing_mul(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        self.set_register_value_from_code(dest_reg, result);
    }

    fn mulbs(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as i8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as i8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_mul(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn mulws(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as i16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as i16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_mul(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn mulds(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as i32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as i32;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_mul(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn mulqs(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1) as i64;
        let input_value_2 = self.get_register_value_from_code(input_reg_2) as i64;

        let (result, overflow) = input_value_1.overflowing_mul(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn divb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_div(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned subtraction, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn divw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_div(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn divd(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_div(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (dest_value & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn divq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let input_value_2 = self.get_register_value_from_code(input_reg_2);

        let (result, overflow) = input_value_1.overflowing_div(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        // We cannot get a negative value with unsigned addition, so the sign flag is always cleared.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        self.set_register_value_from_code(dest_reg, result);
    }

    fn divbs(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as i8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as i8;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_div(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn divws(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as i16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as i16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_div(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn divds(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as i32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as i32;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = input_value_1.overflowing_div(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (dest_value & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn divqs(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1) as i64;
        let input_value_2 = self.get_register_value_from_code(input_reg_2) as i64;

        let (result, overflow) = input_value_1.overflowing_div(input_value_2);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn incb(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFF) as u8;

        let (result, overflow) = dest_value.overflowing_add(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        // Sign flag is never set for unsigned addition.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn incw(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFFFF) as u16;

        let (result, overflow) = dest_value.overflowing_add(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        // Sign flag is never set for unsigned addition.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn incd(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFF) as u32;

        let (result, overflow) = dest_value.overflowing_add(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        // Sign flag is never set for unsigned addition.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn incq(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = dest_value.overflowing_add(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        // Sign flag is never set for unsigned addition.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn incbs(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFF) as i8;

        let (result, overflow) = dest_value.overflowing_add(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn incws(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFFFF) as i16;

        let (result, overflow) = dest_value.overflowing_add(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn incds(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFF) as i32;

        let (result, overflow) = dest_value.overflowing_add(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn incqs(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg) as i64;

        let (result, overflow) = dest_value.overflowing_add(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn decb(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFF) as u8;

        let (result, overflow) = dest_value.overflowing_sub(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        // Sign flag is never set for unsigned addition.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn decw(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFFFF) as u16;

        let (result, overflow) = dest_value.overflowing_sub(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        // Sign flag is never set for unsigned addition.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn decd(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFF) as u32;

        let (result, overflow) = dest_value.overflowing_sub(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        // Sign flag is never set for unsigned addition.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn decq(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);

        let (result, overflow) = dest_value.overflowing_sub(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        // Sign flag is never set for unsigned addition.
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn decbs(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFF) as i8;

        let (result, overflow) = dest_value.overflowing_sub(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn decws(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFFFF) as i16;

        let (result, overflow) = dest_value.overflowing_sub(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn decds(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFF) as i32;

        let (result, overflow) = dest_value.overflowing_sub(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn decqs(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg) as i64;

        let (result, overflow) = dest_value.overflowing_sub(1);
        if overflow {
            self.rflags |= 0b10000;
        }
        else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn negb(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFF) as i8;

        let result = dest_value.neg();
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFFFFFFFF00) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn negw(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFFFF) as i16;

        let result = dest_value.neg();
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFFFFFF0000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn negd(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFF) as i32;

        let result = dest_value.neg();
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        let new_value = (self.get_register_value_from_code(dest_reg) & 0xFFFFFFFF00000000) | result as u64;
        self.set_register_value_from_code(dest_reg, new_value);
    }

    fn negq(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg) as i64;

        let result = dest_value.neg();
        self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if result < 0 {
            self.rflags |= 0b100000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn cmpb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as u8;

        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
    }

    fn cmpw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as u16;

        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
    }

    fn cmpd(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as u32;

        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
    }

    fn cmpq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let input_value_2 = self.get_register_value_from_code(input_reg_2);

        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
    }

    fn cmpbs(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as i8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as i8;

        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
    }

    fn cmpws(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as i16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as i16;

        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
    }

    fn cmpds(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as i32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as i32;

        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
    }

    fn cmpqs(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1) as i64;
        let input_value_2 = self.get_register_value_from_code(input_reg_2) as i64;

        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1000;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
    }

    fn andb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value_1 & input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | result as u64);
    }

    fn andw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value_1 & input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | result as u64);
    }

    fn andd(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value_1 & input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | result as u64);
    }

    fn andq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let input_value_2 = self.get_register_value_from_code(input_reg_2);
        let result = input_value_1 & input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn orb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value_1 | input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | result as u64);
    }

    fn orw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value_1 | input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | result as u64);
    }

    fn ord(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value_1 | input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | result as u64);
    }

    fn orq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let input_value_2 = self.get_register_value_from_code(input_reg_2);
        let result = input_value_1 | input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn xorb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value_1 ^ input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | result as u64);
    }

    fn xorw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value_1 ^ input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | result as u64);
    }

    fn xord(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value_1 ^ input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | result as u64);
    }

    fn xorq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let input_value_2 = self.get_register_value_from_code(input_reg_2);
        let result = input_value_1 ^ input_value_2;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn notb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = !input_value_1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | result as u64);
    }

    fn notw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = !input_value_1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | result as u64);
    }

    fn notd(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = !input_value_1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | result as u64);
    }

    fn notq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let result = !input_value_1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn norb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = !(input_value_1 | input_value_2);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | result as u64);
    }

    fn norw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = !(input_value_1 | input_value_2);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | result as u64);
    }

    fn nord(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = !(input_value_1 | input_value_2);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | result as u64);
    }

    fn norq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let input_value_2 = self.get_register_value_from_code(input_reg_2);
        let result = !(input_value_1 | input_value_2);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn nandb(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFF) as u8;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = !(input_value_1 & input_value_2);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | result as u64);
    }

    fn nandw(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFF) as u16;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = !(input_value_1 & input_value_2);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }

        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | result as u64);
    }

    fn nandd(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = (self.get_register_value_from_code(input_reg_1) & 0xFFFFFFFF) as u32;
        let input_value_2 = (self.get_register_value_from_code(input_reg_2) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = !(input_value_1 & input_value_2);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | result as u64);
    }

    fn nandq(&mut self, operation: u64) {
        let input_reg_1 = ((operation & 0xFFFF0000) >> 16) as u16;
        let input_reg_2 = ((operation & 0xFFFF00000000) >> 32) as u16;
        let dest_reg = ((operation & 0xFFFF000000000000) >> 48) as u16;

        let input_value_1 = self.get_register_value_from_code(input_reg_1);
        let input_value_2 = self.get_register_value_from_code(input_reg_2);
        let result = !(input_value_1 & input_value_2);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        if input_value_1 == input_value_2 {
            self.rflags |= 0b10;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10;
        }
        if input_value_1 > input_value_2 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
        if input_value_1 < input_value_2 {
            self.rflags |= 0b1000;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b10000;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }
    
    fn shlb(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value << 1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | result as u64);
    }

    fn shlw(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value << 1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | result as u64);
    }

    fn shld(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value << 1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | result as u64);
    }

    fn shlq(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = self.get_register_value_from_code(input_reg);

        let result = input_value << 1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn shrb(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value >> 1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | result as u64);
    }

    fn shrw(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value >> 1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | result as u64);
    }

    fn shrd(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value >> 1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | result as u64);
    }

    fn shrq(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = self.get_register_value_from_code(input_reg);

        let result = input_value >> 1;
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn rolb(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value.rotate_left(1);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | result as u64);
    }

    fn rolw(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value.rotate_left(1);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | result as u64);
    }

    fn rold(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value.rotate_left(1);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | result as u64);
    }

    fn rolq(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = self.get_register_value_from_code(input_reg);

        let result = input_value.rotate_left(1);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn rorb(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFF) as u8;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value.rotate_right(1);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | result as u64);
    }

    fn rorw(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFFFF) as u16;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value.rotate_right(1);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | result as u64);
    }

    fn rord(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = (self.get_register_value_from_code(input_reg) & 0xFFFFFFFF) as u32;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = input_value.rotate_right(1);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | result as u64);
    }

    fn rorq(&mut self, operation: u64) {
        let input_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let input_value = self.get_register_value_from_code(input_reg);

        let result = input_value.rotate_right(1);
        if result == 0 {
            self.rflags |= 0b1;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b1;
        }
        self.set_register_value_from_code(dest_reg, result as u64);
    }

    fn bitt(&mut self, operation: u64) {
        let test_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let index_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let index_value = self.get_register_value_from_code(index_reg) & 0b111111;
        let test_value = self.get_register_value_from_code(test_reg);
        if (test_value & (1 << index_value)) != 0 {
            self.rflags |= 0b100;
        } else {
            self.rflags &= 0xFFFFFFFFFFFFFFFF ^ 0b100;
        }
    }

    fn bits(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let index_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let index_value = self.get_register_value_from_code(index_reg) & 0b111111;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = dest_value | (1 << index_value);
        self.set_register_value_from_code(dest_reg, result);
    }

    fn bitc(&mut self, operation: u64) {
        let dest_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let index_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let index_value = self.get_register_value_from_code(index_reg) & 0b111111;
        let dest_value = self.get_register_value_from_code(dest_reg);
        let result = dest_value & !(1 << index_value);
        self.set_register_value_from_code(dest_reg, result);
    }

    fn jmp(&mut self, operation: u64) {
        let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let target_value = self.get_register_value_from_code(target_reg);
        self.rip = target_value;
    }

    fn jmpeq(&mut self, operation: u64) {
        if (self.rflags & 0b10) != 0 {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn jmpz(&mut self, operation: u64) {
        if (self.rflags & 0b1) != 0 {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn jmpneq(&mut self, operation: u64) {
        if (self.rflags & 0b10) == 0 {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn jmpnz(&mut self, operation: u64) {
        if (self.rflags & 0b1) != 0 {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn jmpgt(&mut self, operation: u64) {
        if (self.rflags & 0b1000) != 0 {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn jmpge(&mut self, operation: u64) {
        if ((self.rflags & 0b1000) != 0) | ((self.rflags & 0b10) != 0) {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn jmplt(&mut self, operation: u64) {
        if (self.rflags & 0b10000) != 0 {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn jmple(&mut self, operation: u64) {
        if ((self.rflags & 0b10000) != 0) | ((self.rflags & 0b10) != 0) {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn jmpo(&mut self, operation: u64) {
        if (self.rflags & 0b100000) != 0 {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn jmpn(&mut self, operation: u64) {
        if (self.rflags & 0b1000000) != 0 {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn jmpp(&mut self, operation: u64) {
        if (self.rflags & 0b1000000) == 0 {
            let target_reg = ((operation & 0xFFFF0000) >> 16) as u16;
            let target_value = self.get_register_value_from_code(target_reg);
            self.rip = target_value;
        }
    }

    fn int(&mut self, operation: u64) {
        todo!("Interrupts are not implemented yet");
    }

    fn wfi(&mut self, operation: u64) {
        self.running = false;
    }

    fn rst(&mut self, operation: u64) {
        self.r0 = 0;
        self.r1 = 0;
        self.r2 = 0;
        self.r3 = 0;
        self.r4 = 0;
        self.r5 = 0;
        self.r6 = 0;
        self.r7 = 0;
        self.r8 = 0;
        self.r9 = 0;
        self.r10 = 0;
        self.r11 = 0;
        self.r12 = 0;
        self.r13 = 0;
        self.r14 = 0;
        self.r15 = 0;
        self.rflags = 0;
        self.rip = 0;
        self.rsp = 0;
        self.rpt = 0;
        self.rit = 0;
        self.cr0 = 0;
        self.cr1 = 0;
        self.imm0 = 0;
        self.imm1 = 0;
        self.imm2 = 0;
        self.imm3 = 0;
        self.imm4 = 0;
        self.imm5 = 0;
        self.imm6 = 0;
        self.imm7 = 0;
        self.running = true;
    }

    fn inb(&mut self, operation: u64, io_bus: &Mutex<IoBus>) {
        let port_index_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;
        
        let port_index = (self.get_register_value_from_code(port_index_reg) & 0xFFFF) as u16;
        let port_value = io_bus.lock().unwrap().read_u8(port_index);
        let dest_value = self.get_register_value_from_code(dest_reg);
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFFFF00 | port_value as u64);
    }

    fn inw(&mut self, operation: u64, io_bus: &Mutex<IoBus>) {
        let port_index_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;
        
        let port_index = (self.get_register_value_from_code(port_index_reg) & 0xFFFF) as u16;
        let port_value = io_bus.lock().unwrap().read_u16(port_index);
        let dest_value = self.get_register_value_from_code(dest_reg);
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFFFFFF0000 | port_value as u64);
    }

    fn ind(&mut self, operation: u64, io_bus: &Mutex<IoBus>) {
        let port_index_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;
        
        let port_index = (self.get_register_value_from_code(port_index_reg) & 0xFFFF) as u16;
        let port_value = io_bus.lock().unwrap().read_u32(port_index);
        let dest_value = self.get_register_value_from_code(dest_reg);
        self.set_register_value_from_code(dest_reg, dest_value & 0xFFFFFFFF00000000 | port_value as u64);
    }

    fn inq(&mut self, operation: u64, io_bus: &Mutex<IoBus>) {
        let port_index_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let dest_reg = ((operation & 0xFFFF00000000) >> 32) as u16;
        
        let port_index = (self.get_register_value_from_code(port_index_reg) & 0xFFFF) as u16;
        let port_value = io_bus.lock().unwrap().read_u64(port_index);

        self.set_register_value_from_code(dest_reg, port_value);
    }

    fn outb(&mut self, operation: u64, io_bus: &Mutex<IoBus>) {
        let port_index_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let value_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let port_index = (self.get_register_value_from_code(port_index_reg) & 0xFFFF) as u16;
        let input_value = (self.get_register_value_from_code(value_reg) & 0xFF) as u8;

        io_bus.lock().unwrap().write_u8(port_index, input_value);
    }

    fn outw(&mut self, operation: u64, io_bus: &Mutex<IoBus>) {
        let port_index_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let value_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let port_index = (self.get_register_value_from_code(port_index_reg) & 0xFFFF) as u16;
        let input_value = (self.get_register_value_from_code(value_reg) & 0xFFFF) as u16;

        io_bus.lock().unwrap().write_u16(port_index, input_value);
    }

    fn outd(&mut self, operation: u64, io_bus: &Mutex<IoBus>) {
        let port_index_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let value_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let port_index = (self.get_register_value_from_code(port_index_reg) & 0xFFFF) as u16;
        let input_value = (self.get_register_value_from_code(value_reg) & 0xFFFFFFFF) as u32;

        io_bus.lock().unwrap().write_u32(port_index, input_value);
    }

    fn outq(&mut self, operation: u64, io_bus: &Mutex<IoBus>) {
        let port_index_reg = ((operation & 0xFFFF0000) >> 16) as u16;
        let value_reg = ((operation & 0xFFFF00000000) >> 32) as u16;

        let port_index = (self.get_register_value_from_code(port_index_reg) & 0xFFFF) as u16;
        let input_value = self.get_register_value_from_code(value_reg);

        io_bus.lock().unwrap().write_u64(port_index, input_value);
    }

    fn cpuid(&mut self, operation: u64) {
        todo!("CPUID instruction is not implemented yet");
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
