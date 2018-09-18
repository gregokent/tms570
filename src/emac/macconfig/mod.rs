#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MACCONFIG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TXCELLDEPTHR {
    bits: u8,
}
impl TXCELLDEPTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXCELLDEPTHR {
    bits: u8,
}
impl RXCELLDEPTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADDRESSTYPER {
    bits: u8,
}
impl ADDRESSTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MACCFIGR {
    bits: u8,
}
impl MACCFIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 24:31 - TX cell depth - the number of cells in the transmit FIFO"]
    #[inline]
    pub fn txcelldepth(&self) -> TXCELLDEPTHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCELLDEPTHR { bits }
    }
    #[doc = "Bits 16:23 - RX cell depth - the number of cells in the receive FIFO"]
    #[inline]
    pub fn rxcelldepth(&self) -> RXCELLDEPTHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXCELLDEPTHR { bits }
    }
    #[doc = "Bits 8:15 - Address type in the design"]
    #[inline]
    pub fn addresstype(&self) -> ADDRESSTYPER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDRESSTYPER { bits }
    }
    #[doc = "Bits 0:7 - MAC configuration value"]
    #[inline]
    pub fn maccfig(&self) -> MACCFIGR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MACCFIGR { bits }
    }
}
