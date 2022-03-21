use core::fmt;

const HELLO: u32 = u32::from_be_bytes([0x0D, 0x00, 0x00, 0x00]);
const IDENTIFY: u32 = u32::from_be_bytes([0x13, 0x00, 0x00, 0x00]);

const UNKNOWN_01: u32 = u32::from_be_bytes([0x01, 0x00, 0x00, 0x00]);
const UNKNOWN_02: u32 = u32::from_be_bytes([0x02, 0x00, 0x00, 0x00]);
const UNKNOWN_05: u32 = u32::from_be_bytes([0x05, 0x00, 0x00, 0x00]);
const UNKNOWN_0C: u32 = u32::from_be_bytes([0x0C, 0x00, 0x00, 0x00]);
const UNKNOWN_0E: u32 = u32::from_be_bytes([0x0E, 0x00, 0x00, 0x00]);
const UNKNOWN_1F: u32 = u32::from_be_bytes([0x1F, 0x00, 0x00, 0x00]);

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Op(u32);

impl Op {
    pub const HELLO: Self = Self(HELLO);
    pub const IDENTIFY: Self = Self(IDENTIFY);

    pub const UNKNOWN_01: Self = Self(UNKNOWN_01);
    pub const UNKNOWN_02: Self = Self(UNKNOWN_02);
    pub const UNKNOWN_05: Self = Self(UNKNOWN_05);
    pub const UNKNOWN_0C: Self = Self(UNKNOWN_0C);
    pub const UNKNOWN_0E: Self = Self(UNKNOWN_0E);
    pub const UNKNOWN_1F: Self = Self(UNKNOWN_1F);

    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        #[derive(Debug)]
        enum Op {
            Hello,
            Identify,

            Unknown01,
            Unknown02,
            Unknown05,
            Unknown0C,
            Unknown0E,
            Unknown1F,
        }

        match self.0 {
            HELLO => fmt::Debug::fmt(&Op::Hello, fmt),
            IDENTIFY => fmt::Debug::fmt(&Op::Identify, fmt),

            UNKNOWN_01 => fmt::Debug::fmt(&Op::Unknown01, fmt),
            UNKNOWN_02 => fmt::Debug::fmt(&Op::Unknown02, fmt),
            UNKNOWN_05 => fmt::Debug::fmt(&Op::Unknown05, fmt),
            UNKNOWN_0C => fmt::Debug::fmt(&Op::Unknown0C, fmt),
            UNKNOWN_0E => fmt::Debug::fmt(&Op::Unknown0E, fmt),
            UNKNOWN_1F => fmt::Debug::fmt(&Op::Unknown1F, fmt),

            op => panic!("cannot display op: {:02X?}", op.to_be_bytes()),
        }
    }
}
