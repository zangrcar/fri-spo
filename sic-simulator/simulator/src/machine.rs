use crate::devices::{Device, NullDevice, StderrDevice, StdinDevice, StdoutDevice};
use crate::memory::{Memory};
use crate::opcodes::{ADD, ADDF, ADDR, AND, CLEAR, COMP, COMPF, COMPR, DIV, DIVF, DIVR, FIX, FLOAT, HIO, J, JEQ, JGT, JLT, JSUB, LDA, LDB, LDCH, LDF, LDL, LDS, LDT, LDX, LPS, MUL, MULF, MULR, NORM, OR, RD, RMO, RSUB, SHIFTL, SHIFTR, SIO, SSK, STA, STB, STCH, STF, STI, STL, STS, STSW, STT, STX, SUB, SUBF, SUBR, SVC, TD, TIO, TIX, TIXR, WD};

pub const MAX_DEVICES: usize = 256;

pub const REG_A: usize = 0;
pub const REG_X: usize = 1;
pub const REG_L: usize = 2;
pub const REG_B: usize = 3;
pub const REG_S: usize = 4;
pub const REG_T: usize = 5;
pub const REG_F: usize = 6;

pub const REG_PC: usize = 8;
pub const REG_SW: usize = 9;

pub const CC_LT: usize = 0x00;
pub const CC_EQ: usize = 0x40;
pub const CC_GT: usize = 0x80;

#[inline]
fn mask24(v: usize) -> usize {
    v & 0x00FFFFFF
}

#[derive(Debug, Clone, Copy)]
pub enum RegValue {
    Int(usize),
    Float(f64),
}

#[derive(Debug, Clone, Copy)]
pub enum InstructuionType {
    F1,
    F2,
    OTHER,
    F3,
    F4,
    SIC
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidOpcode,
    InvalidRegister,
    InvalidAddressing,
    NotImplemented,
    InvalidInstruction,
    MemOutOfRange,
    Interrupted,
    DivideByZero
}


impl RegValue {
    fn as_usize(&self) -> usize {
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

pub struct Machine {
    a: usize,
    x: usize,
    l: usize,
    b: usize,
    s: usize,
    t: usize,
    f: f64,
    pc: usize,
    sw: usize,

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

    pub fn get_a(&self) -> usize { self.a }
    pub fn set_a(&mut self, v: usize) { self.a = mask24(v); }

    pub fn get_x(&self) -> usize { self.x }
    pub fn set_x(&mut self, v: usize) { self.x = mask24(v); }

    pub fn get_l(&self) -> usize { self.l }
    pub fn set_l(&mut self, v: usize) { self.l = mask24(v); }

    pub fn get_b(&self) -> usize { self.b }
    pub fn set_b(&mut self, v: usize) { self.b = mask24(v); }

    pub fn get_s(&self) -> usize { self.s }
    pub fn set_s(&mut self, v: usize) { self.s = mask24(v); }

    pub fn get_t(&self) -> usize { self.t }
    pub fn set_t(&mut self, v: usize) { self.t = mask24(v); }

    pub fn get_f(&self) -> f64 { self.f }
    pub fn set_f(&mut self, v: f64) { self.f = v; }

    pub fn get_pc(&self) -> usize { self.pc }
    pub fn set_pc(&mut self, v: usize) { self.pc = mask24(v); }

    pub fn get_sw(&self) -> usize { self.sw }
    pub fn set_sw(&mut self, v: usize) { self.sw = v & 0xFF; }

    pub fn get_reg(&self, reg: usize) -> Result<RegValue, Error> {
        match reg {
            REG_A => Ok(RegValue::Int(mask24(self.a))),
            REG_X => Ok(RegValue::Int(mask24(self.x))),
            REG_L => Ok(RegValue::Int(mask24(self.l))),
            REG_B => Ok(RegValue::Int(mask24(self.b))),
            REG_S => Ok(RegValue::Int(mask24(self.s))),
            REG_T => Ok(RegValue::Int(mask24(self.t))),
            REG_F => Ok(RegValue::Float(self.f)),
            REG_PC => Ok(RegValue::Int(mask24(self.pc))),
            REG_SW => Ok(RegValue::Int(mask24(self.sw))),
            _ => Err(Error::InvalidRegister),
        }
    }

    pub fn set_reg(&mut self, reg: usize, v: RegValue) -> Result<(), Error> {
        match reg {
            REG_A => { self.a = mask24(v.as_usize()); Ok(()) },
            REG_X => { self.x = mask24(v.as_usize()); Ok(()) },
            REG_L => { self.l = mask24(v.as_usize()); Ok(()) },
            REG_B => { self.b = mask24(v.as_usize()); Ok(()) },
            REG_S => { self.s = mask24(v.as_usize()); Ok(()) },
            REG_T => { self.t = mask24(v.as_usize()); Ok(()) },
            REG_F => { self.f = v.as_f64(); Ok(()) },
            REG_PC => { self.pc = mask24(v.as_usize()); Ok(()) },
            REG_SW => { self.sw = mask24(v.as_usize()); Ok(()) },
            _ => Err(Error::InvalidRegister),
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

    pub fn not_implemented(&self, mnemonic: String) {
        println!("{} is not implemented!", mnemonic);
    }

    pub fn invalid_opcode(&self, opcode: u8) -> Error {
        println!("opcode {} is invalid!", opcode);
        Error::InvalidOpcode
    }

    pub fn invalid_addressing(&self) {
        println!("Invalid addressing was used!");
    }

    pub fn fetch(&mut self) -> Result<u8, Error> {
        let fetched_byte: u8 = self.memory.get_byte(self.pc)?;
        Self::set_pc(self,self.pc+1);
        Ok(fetched_byte)
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        let first_byte = self.fetch()?;
        let instruction_type = self.get_instrution_type1(first_byte)?;
        
        match instruction_type {
            InstructuionType::F1 => self.exec_f1(first_byte),
            InstructuionType::F2 => {
                let second_byte = self.fetch()?;
                let final_instruction = ((first_byte as u32) << 8) | (second_byte as u32);
                self.exec_f2(final_instruction)
            },
            InstructuionType::SIC => {
                let second_byte = self.fetch()?;
                let third_byte = self.fetch()?;
                let final_instruction = ((first_byte as u32) << 16) | ((second_byte as u32) << 8) | (third_byte as u32);
                self.exec_sic(final_instruction)
            },
            InstructuionType::OTHER => {
                let second_byte = self.fetch()?;
                let final_instruction_type = self.get_instrution_type2(second_byte)?;
                match final_instruction_type {
                    InstructuionType::F3 => {
                        let third_byte = self.fetch()?;
                        let final_instruction = 
                            ((first_byte as u32) << 16) | 
                            ((second_byte as u32) << 8) | 
                            (third_byte as u32);
                        self.exec_f3(final_instruction)
                    },
                    InstructuionType::F4 => {
                        let third_byte = self.fetch()?;
                        let fourth_byte = self.fetch()?;
                        let final_instruction = 
                            ((first_byte as u32) << 24) | 
                            ((second_byte as u32) << 16) | 
                            ((third_byte as u32) << 8) |
                            (fourth_byte as u32);
                        self.exec_f4(final_instruction)
                    }
                    _ => Err(Error::InvalidInstruction)
                }
            }
            _ => Err(Error::InvalidInstruction)
        }
    }

    fn exec_f1(&self, code: u8) -> Result<(), Error> {
        match code {
            FIX    |
            FLOAT  |
            HIO    |
            NORM   |
            SIO    |
            TIO => Err(Error::NotImplemented),
            _ => Err(Error::InvalidInstruction)
        }
    }

    fn exec_f2(&mut self, code: u32) -> Result<(), Error> {
        let opcode = ((code >> 8) & 0xFF) as u8;
        let low = (code & 0xFF) as u8;
        let reg1 = (low >> 4) as usize;
        let reg2 = (low & 0x0F) as usize;
        match opcode {
            ADDR => {
                let reg_val1 = self.get_reg(reg1)?.as_usize();
                let reg_val2 = self.get_reg(reg2)?.as_usize();
                self.set_reg(reg2, RegValue::Int(reg_val1 + reg_val2))?;
                Ok(())
            }
            CLEAR => {
                self.set_reg(reg1, RegValue::Int(0))?;
                Ok(())
            }
            COMPR => {
                let reg_val1 = self.get_reg(reg1)?.as_usize();
                let reg_val2 = self.get_reg(reg2)?.as_usize();
                self.set_cc_from_24(reg_val1 as u32, reg_val2 as u32);
                Ok(())
            }
            DIVR => {
                let reg_val1 = self.get_reg(reg1)?.as_usize();
                let reg_val2 = self.get_reg(reg2)?.as_usize();
                if reg_val1 == 0 {
                    return Err(Error::DivideByZero);
                }
                self.set_reg(reg2, RegValue::Int(reg_val2/reg_val1))?;
                Ok(())
            }
            MULR => {
                let reg_val1 = self.get_reg(reg1)?.as_usize();
                let reg_val2 = self.get_reg(reg2)?.as_usize();
                self.set_reg(reg2, RegValue::Int(reg_val2*reg_val1))?;
                Ok(())
            }
            RMO => {
                let reg_val1 = self.get_reg(reg1)?.as_usize();
                self.set_reg(reg2, RegValue::Int(reg_val1))?;
                Ok(())
            }
            SHIFTL => {
                let reg_val1 = self.get_reg(reg1)?.as_usize();
                let n = reg2 % 24;  // rotating by full width gives the same number
                let mut value: usize = 0;
                if n == 0 {
                    value = reg_val1 & 0x00FF_FFFF;
                } else {
                    let left  = (reg_val1 << n) & 0x00FF_FFFF;
                    let right = (reg_val1 >> (24 - n)) & ((1 << n) - 1);
                    value = (left | right) & 0x00FF_FFFF;
                }
                // reg2 is in this case n (number of bits to be shifted)
                self.set_reg(reg1, RegValue::Int(value))?;
                Ok(())
            }
            SHIFTR => {
                let reg_val1 = self.get_reg(reg1)?.as_usize();
                // mimic arithmetic shift (signed)
                let signed = self.sign_extend_24(reg_val1 as u32);
                let shifted = signed >> reg2;
                // reg2 is in this case n (number of bits to be shifted)
                self.set_reg(reg1, RegValue::Int((shifted as usize) & 0x00FF_FFFF))?;
                Ok(())
            }
            SUBR => {
                let reg_val1 = self.get_reg(reg1)?.as_usize();
                let reg_val2 = self.get_reg(reg2)?.as_usize();
                self.set_reg(reg2, RegValue::Int(reg_val2-reg_val1))?;
                Ok(())
            }
            SVC  => Err(Error::Interrupted),
            TIXR => {
                let reg_val = self.get_reg(reg1)?.as_usize();
                self.set_x(self.get_x() + 1);
                let x_val = self.get_x();
                self.set_cc_from_24(x_val as u32, reg_val as u32);
                Ok(())
            }
            _ => Err(Error::InvalidInstruction)
        }
    }

    fn exec_f3(&self, code: u32) -> Result<(), Error> {
        let opcode = ((code >> 16) as u8) & 0xFC; 
        let n = ((code >> 17) & 0x1) as u8;
        let i = ((code >> 16) & 0x1) as u8;
        let x = ((code >> 15) & 0x1) as u8;
        let b = ((code >> 14) & 0x1) as u8;
        let p = ((code >> 13) & 0x1) as u8;
        let mut address = (code & 0xFFF) as i32;

        // pc relative addressing is signed
        if p == 1 {
            if (address & 0x800) != 0 {
                address |= !0xFFF;
            }
        }

        // only add x reg if it is not direct addressing
        if x == 1 && !(n == 0 && i == 1) {
            address += self.get_x() as i32;
        }

        if b + p == 2 {
            return Err(Error::InvalidAddressing)
        } else if b == 1 {
            address += self.get_b() as i32;
        } else if p == 1 {
            address += self.get_pc() as i32;
        }

        let value = if n + i == 2 {
            self.memory.get_word(address as usize)?
        } else if n == 1 {
            let indirect = self.memory.get_word(address as usize)? as usize;
            self.memory.get_word(indirect)?
        } else {
            address as u32
        };
        self.exec_f3_f4_sic(opcode, address as u32, value)
    }

    fn exec_f4(&self, code: u32) -> Result<(), Error> {
        let opcode = ((code >> 24) as u8) & 0xFC; 
        let n = ((code >> 25) & 0x1) as u8;
        let i = ((code >> 24) & 0x1) as u8;
        let x = ((code >> 23) & 0x1) as u8;
        let mut address = code & 0xFFFFF;

        // do not add x for immidiate addressing
        if x == 1 && !(n == 0 && i == 1) {
            address += self.get_x() as u32;
        }

        let value = if n + i == 2 {
            self.memory.get_word(address as usize)?
        } else if n == 1 {
            let indirect = self.memory.get_word(address as usize)? as usize;
            self.memory.get_word(indirect)?
        } else {
            address
        };
        self.exec_f3_f4_sic(opcode, address, value)
    }

    fn exec_sic(&mut self, code: u32) -> Result<(), Error> {
        let opcode = ((code >> 16) & 0xFF) as u8;
        let x = ((code >> 15) & 0x1) as u8;
        let address = (code & 0x7FFF) + (if x == 1 { self.get_x() as u32} else {0});
        let value = self.memory.get_word(address as usize)?;
        match opcode {
            ADD => {
                self.set_a(self.get_a() + value as usize);
                Ok(())
            }
            AND => {
                self.set_a(self.get_a() & value as usize);
                Ok(())
            }
            COMP => {
                self.set_cc_from_24(self.get_a() as u32, value);
                Ok(())
            }
            DIV => {
                if value == 0 {
                    return Err(Error::DivideByZero);
                }
                self.set_a(self.get_a()/(value as usize));
                Ok(())
            }
            J      |
            JEQ    |
            JGT    |
            JLT    |
            JSUB   |
            LDA    |
            LDCH   |
            LDL    |
            LDS    |
            LDT    |
            LDX    |
            MUL    |
            OR     |
            RD     |
            RSUB   |
            STA    |
            STCH   |
            STL    |
            STSW   |
            STX    |
            SUB    |
            TD     |
            TIX    |
            WD => self.exec_f3_f4_sic(opcode, address, value),
            _ => Err(Error::InvalidInstruction)
        }
    }

    fn exec_f3_f4_sic(&self, opcode: u8, address: u32, value: u32) -> Result<(), Error> {
        match opcode {
            ADD    |
            ADDF   |
            AND    |
            COMP   |
            COMPF  |
            DIV    |
            DIVF   |
            J      |
            JEQ    |
            JGT    |
            JLT    |
            JSUB   |
            LDA    |
            LDB    |
            LDCH   |
            LDF    |
            LDL    |
            LDS    |
            LDT    |
            LDX    |
            LPS    |
            MUL    |
            MULF   |
            OR     |
            RD     |
            RSUB   |
            SSK    |
            STA    |
            STB    |
            STCH   |
            STF    |
            STI    |
            STL    |
            STS    |
            STSW   |
            STT    |
            STX    |
            SUB    |
            SUBF   |
            TD     |
            TIX    |
            WD => Err(Error::NotImplemented),
            _ => Err(Error::InvalidInstruction)
        }
    }

    fn get_instrution_type1(&self, opcode: u8) -> Result<InstructuionType, Error> {
        match opcode {
            FIX    |
            FLOAT  |
            HIO    |
            NORM   |
            SIO    |
            TIO => Ok(InstructuionType::F1),
            ADDR   |
            CLEAR  |
            COMPR  |
            DIVR   |
            MULR   |
            RMO    |
            SHIFTL |
            SHIFTR |
            SUBR   |
            SVC    |
            TIXR => Ok(InstructuionType::F2),
            ADD    |
            ADDF   |
            AND    |
            COMP   |
            COMPF  |
            DIV    |
            DIVF   |
            J      |
            JEQ    |
            JGT    |
            JLT    |
            JSUB   |
            LDA    |
            LDB    |
            LDCH   |
            LDF    |
            LDL    |
            LDS    |
            LDT    |
            LDX    |
            LPS    |
            MUL    |
            MULF   |
            OR     |
            RD     |
            RSUB   |
            SSK    |
            STA    |
            STB    |
            STCH   |
            STF    |
            STI    |
            STL    |
            STS    |
            STSW   |
            STT    |
            STX    |
            SUB    |
            SUBF   |
            TD     |
            TIX    |
            WD => {
                if opcode & 0x3 == 0 {
                    Ok(InstructuionType::SIC)
                } else {
                    Ok(InstructuionType::OTHER)
                }
            },
            _ => Err(self.invalid_opcode(opcode)),
        }
    }

    fn get_instrution_type2(&self, second_byte: u8) -> Result<InstructuionType, Error> {
        if second_byte & 0x10 == 0 {
            Ok(InstructuionType::F3)
        } else {
            Ok(InstructuionType::F4)
        }
    }

    fn set_cc_from_24(&mut self, a: u32, b: u32) {
        let a_signed = self.sign_extend_24(a);
        let b_signed = self.sign_extend_24(b);

        if a_signed < b_signed {
            self.set_sw(CC_LT);
        } else if a_signed == b_signed {
            self.set_sw(CC_EQ);
        } else {
            self.set_sw(CC_GT);
        }
    }

    fn sign_extend_24(&self, v: u32) -> i32 {
        ((v << 8) as i32) >> 8
    }
}