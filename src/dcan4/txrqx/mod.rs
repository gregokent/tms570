#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TXRQX {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RSV1R {
    bits: u16,
}
impl RSV1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXRQSTREG8R {
    bits: u8,
}
impl TXRQSTREG8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXRQSTREG7R {
    bits: u8,
}
impl TXRQSTREG7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXRQSTREG6R {
    bits: u8,
}
impl TXRQSTREG6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXRQSTREG5R {
    bits: u8,
}
impl TXRQSTREG5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXRQSTREG4R {
    bits: u8,
}
impl TXRQSTREG4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXRQSTREG3R {
    bits: u8,
}
impl TXRQSTREG3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXRQSTREG2R {
    bits: u8,
}
impl TXRQSTREG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXRQSTREG1R {
    bits: u8,
}
impl TXRQSTREG1R {
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
    #[doc = "Bits 16:31 - Reserved"]
    #[inline]
    pub fn rsv1(&self) -> RSV1R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RSV1R { bits }
    }
    #[doc = "Bits 14:15 - Not Zero = One or More Bits of TXRQ8 are Set"]
    #[inline]
    pub fn txrqstreg8(&self) -> TXRQSTREG8R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXRQSTREG8R { bits }
    }
    #[doc = "Bits 12:13 - Not Zero = One or More Bits of TXRQ7 are Set"]
    #[inline]
    pub fn txrqstreg7(&self) -> TXRQSTREG7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXRQSTREG7R { bits }
    }
    #[doc = "Bits 10:11 - Not Zero = One or More Bits of TXRQ6 are Set"]
    #[inline]
    pub fn txrqstreg6(&self) -> TXRQSTREG6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXRQSTREG6R { bits }
    }
    #[doc = "Bits 8:9 - Not Zero = One or More Bits of TXRQ5 are Set"]
    #[inline]
    pub fn txrqstreg5(&self) -> TXRQSTREG5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXRQSTREG5R { bits }
    }
    #[doc = "Bits 6:7 - Not Zero = One or More Bits of TXRQ4 are Set"]
    #[inline]
    pub fn txrqstreg4(&self) -> TXRQSTREG4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXRQSTREG4R { bits }
    }
    #[doc = "Bits 4:5 - Not Zero = One or More Bits of TXRQ3 are Set"]
    #[inline]
    pub fn txrqstreg3(&self) -> TXRQSTREG3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXRQSTREG3R { bits }
    }
    #[doc = "Bits 2:3 - Not Zero = One or More Bits of TXRQ2 are Set"]
    #[inline]
    pub fn txrqstreg2(&self) -> TXRQSTREG2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXRQSTREG2R { bits }
    }
    #[doc = "Bits 0:1 - Not Zero = One or More Bits of TXRQ1 are Set"]
    #[inline]
    pub fn txrqstreg1(&self) -> TXRQSTREG1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXRQSTREG1R { bits }
    }
}
