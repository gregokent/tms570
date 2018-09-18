#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RTIDWDCNTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RSV1R {
    bits: u8,
}
impl RSV1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DWDCNTRR {
    bits: u32,
}
impl DWDCNTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 25:31 - Reserved"]
    #[inline]
    pub fn rsv1(&self) -> RSV1R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSV1R { bits }
    }
    #[doc = "Bits 0:24 - DWD down counter"]
    #[inline]
    pub fn dwdcntr(&self) -> DWDCNTRR {
        let bits = {
            const MASK: u32 = 33554431;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DWDCNTRR { bits }
    }
}
