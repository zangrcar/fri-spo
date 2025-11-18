use crate::machine::Error;

pub const MAX_ADDRESS: usize = 0xFFFFF;
pub const MEM_SIZE: usize = MAX_ADDRESS + 1;


pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { 
            data: vec![0u8; MEM_SIZE]  
        }
    }

    #[inline]
    pub fn check_mem(addr: usize) -> Result<(), Error> {
        if addr <= MAX_ADDRESS {
            Ok(())
        } else {
            Err(Error::MemOutOfRange)
        }
    }

    pub fn get_byte(&self, addr: usize) -> Result<u8, Error> {
        Self::check_mem(addr)?;
        Ok(self.data[addr])
    }

    pub fn set_byte(&mut self, addr: usize, val: u8) -> Result<(), Error> {
        Self::check_mem(addr)?;
        self.data[addr] = val;
        Ok(())
    }

    pub fn get_word(&self, addr: usize) -> Result<u32, Error> {
        Self::check_mem(addr+2)?;
        let b0 = self.data[addr] as u32;
        let b1 = self.data[addr+1] as u32;
        let b2 = self.data[addr+2] as u32;
        Ok((b0 << 16) | (b1 << 8) | b2)
    }

    pub fn set_word(&mut self, addr: usize, val: u32) -> Result<(), Error> {
        Self::check_mem(addr+2)?;
        let v = val & 0xFFFFFF;
        self.data[addr+2] = (v & 0xFF) as u8;
        self.data[addr+1] = ((v >> 8) & 0xFF) as u8;
        self.data[addr] = ((v >> 16) & 0xFF) as u8;
        Ok(())
    }
}