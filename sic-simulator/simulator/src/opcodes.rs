pub const ADD:   u8 = 0x18; // ADD m
pub const ADDF:  u8 = 0x58; // ADDF m
pub const ADDR:  u8 = 0x90; // ADDR r1, r2

pub const AND:   u8 = 0x40; // AND m

pub const CLEAR: u8 = 0xB4; // CLEAR r1

pub const COMP:  u8 = 0x28; // COMP m
pub const COMPF: u8 = 0x88; // COMPF m
pub const COMPR: u8 = 0xA0; // COMPR r1, r2

pub const DIV:   u8 = 0x24; // DIV m
pub const DIVF:  u8 = 0x64; // DIVF m
pub const DIVR:  u8 = 0x9C; // DIVR r1, r2

pub const FIX:   u8 = 0xC4; // FIX
pub const FLOAT: u8 = 0xC0; // FLOAT

pub const HIO:   u8 = 0xF4; // HIO

pub const J:     u8 = 0x3C; // J m
pub const JEQ:   u8 = 0x30; // JEQ m
pub const JGT:   u8 = 0x34; // JGT m
pub const JLT:   u8 = 0x38; // JLT m
pub const JSUB:  u8 = 0x48; // JSUB m

pub const LDA:   u8 = 0x00; // LDA m
pub const LDB:   u8 = 0x68; // LDB m
pub const LDCH:  u8 = 0x50; // LDCH m
pub const LDF:   u8 = 0x70; // LDF m
pub const LDL:   u8 = 0x08; // LDL m
pub const LDS:   u8 = 0x6C; // LDS m
pub const LDT:   u8 = 0x74; // LDT m
pub const LDX:   u8 = 0x04; // LDX m
pub const LPS:   u8 = 0xD0; // LPS m

pub const MUL:   u8 = 0x20; // MUL m
pub const MULF:  u8 = 0x60; // MULF m
pub const MULR:  u8 = 0x98; // MULR r1, r2

pub const NORM:  u8 = 0xC8; // NORM

pub const OR:    u8 = 0x44; // OR m

pub const RD:    u8 = 0xD8; // RD m
pub const RMO:   u8 = 0xAC; // RMO r1, r2
pub const RSUB:  u8 = 0x4C; // RSUB

pub const SHIFTL: u8 = 0xA4; // SHIFTL r1, n
pub const SHIFTR: u8 = 0xA8; // SHIFTR r2, n

pub const SIO:   u8 = 0xF0; // SIO
pub const SSK:   u8 = 0xEC; // SSK m

pub const STA:   u8 = 0x0C; // STA m
pub const STB:   u8 = 0x78; // STB m
pub const STCH:  u8 = 0x54; // STCH m
pub const STF:   u8 = 0x80; // STF m
pub const STI:   u8 = 0xD4; // STI
pub const STL:   u8 = 0x14; // STL m
pub const STS:   u8 = 0x7C; // STS m
pub const STSW:  u8 = 0xE8; // STSW m
pub const STT:   u8 = 0x84; // STT m
pub const STX:   u8 = 0x10; // STX m

pub const SUB:   u8 = 0x1C; // SUB m
pub const SUBF:  u8 = 0x5C; // SUBF m
pub const SUBR:  u8 = 0x94; // SUBR r1, r2

pub const SVC:   u8 = 0xB0; // SVC n

pub const TD:    u8 = 0xE0; // TD m
pub const TIO:   u8 = 0xF8; // TIO

pub const TIX:   u8 = 0x2C; // TIX m
pub const TIXR:  u8 = 0xB8; // TIXR r1

pub const WD:    u8 = 0xDC; // WD m