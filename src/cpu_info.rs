use crate::{print, println};
use core::fmt;
use core::str::from_utf8;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CPU_INFO: CPUInfo = CPUInfo::new();
}

#[repr(u32)]
enum CpuidEax {
    ManufactorID = 0x0,
    ProcInfo,
    CacheTLB,
    SerialNumber,
    ThreadInfo,
    PowerInfo,
}

pub struct CPUInfo {
    manufactor_id: [u8; 12],
    proc_info: ProcInfo,
}

impl fmt::Display for CPUInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "manufactor_id: {}\n{}",
            self.manufactor_id(),
            self.proc_info()
        )
    }
}

pub struct ProcInfo {
    eax: u32,
}

impl fmt::Display for ProcInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "stepping_id: {}\nfamily_id: {}\nproc_type: {}\nmodel: {}",
            self.stepping_id(),
            self.family_id(),
            self.processor_type(),
            self.model()
        )
    }
}

impl ProcInfo {
    fn new(eax: u32, ebx: u32, edx: u32, ecx: u32) -> Self {
        Self { eax }
    }

    pub fn stepping_id(&self) -> u8 {
        (self.eax & 0xF) as u8
    }

    pub fn family_id(&self) -> u8 {
        let family_id = ((self.eax >> 8) & 0xF) as u8;
        return if family_id == 15 {
            family_id + self.extended_family_id()
        } else {
            family_id
        };
    }

    pub fn extended_family_id(&self) -> u8 {
        (self.eax >> 20) as u8
    }

    pub fn processor_type(&self) -> u8 {
        ((self.eax >> 12) & 0x3) as u8
    }

    pub fn model(&self) -> u8 {
        let model = ((self.eax >> 4) & 0xF) as u8;
        return if self.family_id() == 15 || self.family_id() == 6 {
            (self.extended_model_id() << 4) + model
        } else {
            model
        };
    }

    pub fn extended_model_id(&self) -> u8 {
        ((self.eax >> 16) & 0xF) as u8
    }
}

impl CPUInfo {
    pub fn new() -> Self {
        let manufactor_id = Self::parse_manufactor_id();
        let proc_info = Self::parse_proc_info();
        Self {
            manufactor_id,
            proc_info,
        }
    }

    pub fn proc_info(&self) -> &ProcInfo {
        &self.proc_info
    }

    pub fn manufactor_id(&self) -> &str {
        from_utf8(&self.manufactor_id).unwrap()
    }

    fn parse_manufactor_id() -> [u8; 12] {
        let mut chars: [u8; 12] = [0; 12];
        let registers = Self::get_info(CpuidEax::ManufactorID).1;
        let mut registers_index = 2;
        let mut byte_offset = 32;
        for i in 0..12 {
            if byte_offset == 0 {
                byte_offset = 32;
                registers_index -= 1;
            }
            byte_offset -= 8;
            chars[11 - i] = ((registers[registers_index] >> byte_offset) & 0xFF) as u8;
        }
        return chars;
    }

    fn parse_proc_info() -> ProcInfo {
        let (eax, registers) = Self::get_info(CpuidEax::ProcInfo);
        ProcInfo::new(eax, registers[0], registers[1], registers[2])
    }

    fn get_info(eax: CpuidEax) -> (u32, [u32; 3]) {
        let mut eax: u32 = eax as u32;
        let ebx: u32;
        let edx: u32;
        let ecx: u32;
        unsafe {
            asm!(
                "cpuid",
                inout("eax") eax => eax,
                lateout("edx") edx,
                lateout("ecx") ecx,
            );
            asm!("mov {:e}, ebx", out(reg) ebx);
        }
        return (eax, [ebx, edx, ecx]);
    }
}
