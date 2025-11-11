use crate::devices::{Device, NullDevice, StderrDevice, StdinDevice, StdoutDevice};
use crate::memory::Memory;

pub const MAX_DEVICES: usize = 256;

pub const REG_A: u32 = 0;
pub const REG_X: u32 = 1;
pub const REG_L: u32 = 2;
pub const REG_B: u32 = 3;
pub const REG_S: u32 = 4;
pub const REG_T: u32 = 5;
pub const REG_F: u32 = 6;

pub const REG_PC: u32 = 8;
pub const REG_SW: u32 = 9;

pub const CC_LT: u32 = 0x00;
pub const CC_EQ: u32 = 0x40;
pub const CC_GT: u32 = 0x80;

#[inline]
fn mask24(v: u32) -> u32 {
    v & 0x00FFFFFF
}

#[derive(Debug, Clone, Copy)]
pub enum RegValue {
    Int(u32),
    Float(f64),
}

impl RegValue {
    fn as_u32(&self) -> u32 {
        match *self {
            RegValue::Int(v) => v,
            _ => 0,
        }
    }
    fn as_f64(&self) -> f64 {
        match *self {
            RegValue::Float(v) => v,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegError {
    InvalidRegister,
}

pub struct Machine {
    a: u32,
    x: u32,
    l: u32,
    b: u32,
    s: u32,
    t: u32,
    f: f64,
    pc: u32,
    sw: u32,

    pub memory: Memory,
    pub devices: [Box<dyn Device>; MAX_DEVICES],
}

impl Machine {
    pub fn new() -> Self {
        let mut devs: [Box<dyn Device>; MAX_DEVICES] = std::array::from_fn(|_| -> Box<dyn Device> { 
            Box::new(NullDevice::default()) 
        });

        devs[0] = Box::new(StdinDevice::default());
        devs[1] = Box::new(StdoutDevice::default());
        devs[2] = Box::new(StderrDevice::default());

        Self {
            a: 0, 
            x: 0, 
            l: 0, 
            b: 0, 
            s: 0, 
            t: 0,
            f: 0.0,
            pc: 0,
            sw: CC_EQ,
            memory: Memory::new(),
            devices: devs
        }
    }

    pub fn get_a(&self) -> u32 { self.a }
    pub fn set_a(&mut self, v: u32) { self.a = mask24(v); }

    pub fn get_x(&self) -> u32 { self.x }
    pub fn set_x(&mut self, v: u32) { self.x = mask24(v); }

    pub fn get_l(&self) -> u32 { self.l }
    pub fn set_l(&mut self, v: u32) { self.l = mask24(v); }

    pub fn get_b(&self) -> u32 { self.b }
    pub fn set_b(&mut self, v: u32) { self.b = mask24(v); }

    pub fn get_s(&self) -> u32 { self.s }
    pub fn set_s(&mut self, v: u32) { self.s = mask24(v); }

    pub fn get_t(&self) -> u32 { self.t }
    pub fn set_t(&mut self, v: u32) { self.t = mask24(v); }

    pub fn get_f(&self) -> f64 { self.f }
    pub fn set_f(&mut self, v: f64) { self.f = v; }

    pub fn get_pc(&self) -> u32 { self.pc }
    pub fn set_pc(&mut self, v: u32) { self.pc = mask24(v); }

    pub fn get_sw(&self) -> u32 { self.sw }
    pub fn set_sw(&mut self, v: u32) { self.sw = v & 0xFF; }

    pub fn get_reg(&self, reg: u32) -> Result<RegValue, RegError> {
        match reg {
            REG_A => Ok(RegValue::Int(self.a)),
            REG_X => Ok(RegValue::Int(self.x)),
            REG_L => Ok(RegValue::Int(self.l)),
            REG_B => Ok(RegValue::Int(self.b)),
            REG_S => Ok(RegValue::Int(self.s)),
            REG_T => Ok(RegValue::Int(self.t)),
            REG_F => Ok(RegValue::Float(self.f)),
            REG_PC => Ok(RegValue::Int(self.pc)),
            REG_SW => Ok(RegValue::Int(self.sw)),
            _ => Err(RegError::InvalidRegister),
        }
    }

    pub fn set_reg(&mut self, reg: u32, v: RegValue) -> Result<(), RegError> {
        match reg {
            REG_A => { self.a = mask24(v.as_u32()); Ok(()) },
            REG_X => { self.x = mask24(v.as_u32()); Ok(()) },
            REG_L => { self.l = mask24(v.as_u32()); Ok(()) },
            REG_B => { self.b = mask24(v.as_u32()); Ok(()) },
            REG_S => { self.s = mask24(v.as_u32()); Ok(()) },
            REG_T => { self.t = mask24(v.as_u32()); Ok(()) },
            REG_F => { self.f = v.as_f64(); Ok(()) },
            REG_PC => { self.pc = mask24(v.as_u32()); Ok(()) },
            REG_SW => { self.sw = mask24(v.as_u32()); Ok(()) },
            _ => Err(RegError::InvalidRegister),
        }
    }

    pub fn get_device(&mut self, num: usize) -> Option<&mut (dyn Device + 'static)> {
        self.devices.get_mut(num).map(|b| b.as_mut())
    }

    pub fn set_device(&mut self, num: usize, dev: Box<dyn Device>) -> Result<(), &'static str> {
        if num >= MAX_DEVICES { return Err("device index out of range"); }
        self.devices[num] = dev;
        Ok(())
    }
}