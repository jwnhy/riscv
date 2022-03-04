//! misa builder

use crate::register::misa::*;
use bit_field::BitField;

/// mstatus register builder
pub struct MisaBuilder {
    pub(self) bits: usize,
}

/// mstatus register value
#[derive(Clone, Copy, Debug)]
pub struct MisaValue {
    pub(self) bits: usize,
}

impl MisaBuilder {
    pub fn bits(&self) -> usize {
        self.bits
    }

    pub fn mxl(&self) -> MXL {
        let value = match () {
            #[cfg(target_pointer_width = "32")]
            () => (self.bits() >> 30) as u8,
            #[cfg(target_pointer_width = "64")]
            () => (self.bits() >> 62) as u8,
        };
        match value {
            1 => MXL::XLEN32,
            2 => MXL::XLEN64,
            3 => MXL::XLEN128,
            _ => unreachable!(),
        }
    }

    pub fn set_mxl(&mut self, mxl: MXL) {
        let value = match mxl {
            MXL::XLEN32 => 1,
            MXL::XLEN64 => 2,
            MXL::XLEN128 => 3,
            _ => unreachable!(),
        };
        match () {
            #[cfg(target_pointer_width = "32")]
            () => self.bits.set_bits(30..32, value),
            #[cfg(target_pointer_width = "64")]
            () => self.bits.set_bits(62..64, value),
        };
    }

    pub fn build(&self) -> MisaValue {
        MisaValue { bits: self.bits }
    }
}

impl MisaValue {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

impl From<MisaValue> for MisaBuilder {
    #[inline]
    fn from(value: MisaValue) -> Self {
        MisaBuilder { bits: value.bits() }
    }
}

impl From<MisaBuilder> for MisaValue {
    #[inline]
    fn from(value: MisaBuilder) -> Self {
        MisaValue { bits: value.bits() }
    }
}
