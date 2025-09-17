use crate::cpus::Monarch64CPU;

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
}

impl Monarch64CPU for MonadCPU {
    
}