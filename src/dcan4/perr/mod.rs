#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PERR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RSV1R {
    bits: u32,
}
impl RSV1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WORDNUMBERR {
    bits: u8,
}
impl WORDNUMBERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MESSAGENUMBERR {
    bits: u8,
}
impl MESSAGENUMBERR {
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
    #[doc = "Bits 11:31 - Reserved"]
    #[inline]
    pub fn rsv1(&self) -> RSV1R {
        let bits = {
            const MASK: u32 = 2097151;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RSV1R { bits }
    }
    #[doc = "Bits 8:10 - Reserved - Always Reads 0"]
    #[inline]
    pub fn word_number(&self) -> WORDNUMBERR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WORDNUMBERR { bits }
    }
    #[doc = "Bits 0:7 - 0x1-0x80 Message Number where Double Bit Error Detected"]
    #[inline]
    pub fn message_number(&self) -> MESSAGENUMBERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MESSAGENUMBERR { bits }
    }
}
