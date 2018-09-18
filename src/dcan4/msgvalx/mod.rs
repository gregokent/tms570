#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MSGVALX {
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
pub struct MSGVALREG8R {
    bits: u8,
}
impl MSGVALREG8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MSGVALREG7R {
    bits: u8,
}
impl MSGVALREG7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MSGVALREG6R {
    bits: u8,
}
impl MSGVALREG6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MSGVALREG5R {
    bits: u8,
}
impl MSGVALREG5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MSGVALREG4R {
    bits: u8,
}
impl MSGVALREG4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MSGVALREG3R {
    bits: u8,
}
impl MSGVALREG3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MSGVALREG2R {
    bits: u8,
}
impl MSGVALREG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MSGVALREG1R {
    bits: u8,
}
impl MSGVALREG1R {
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
    #[doc = "Bits 14:15 - Not Zero = One or More Bits of MSGVAL8 are Set"]
    #[inline]
    pub fn msgvalreg8(&self) -> MSGVALREG8R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSGVALREG8R { bits }
    }
    #[doc = "Bits 12:13 - Not Zero = One or More Bits of MSGVAL7 are Set"]
    #[inline]
    pub fn msgvalreg7(&self) -> MSGVALREG7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSGVALREG7R { bits }
    }
    #[doc = "Bits 10:11 - Not Zero = One or More Bits of MSGVAL6 are Set"]
    #[inline]
    pub fn msgvalreg6(&self) -> MSGVALREG6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSGVALREG6R { bits }
    }
    #[doc = "Bits 8:9 - Not Zero = One or More Bits of MSGVAL5 are Set"]
    #[inline]
    pub fn msgvalreg5(&self) -> MSGVALREG5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSGVALREG5R { bits }
    }
    #[doc = "Bits 6:7 - Not Zero = One or More Bits of MSGVAL4 are Set"]
    #[inline]
    pub fn msgvalreg4(&self) -> MSGVALREG4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSGVALREG4R { bits }
    }
    #[doc = "Bits 4:5 - Not Zero = One or More Bits of MSGVAL3 are Set"]
    #[inline]
    pub fn msgvalreg3(&self) -> MSGVALREG3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSGVALREG3R { bits }
    }
    #[doc = "Bits 2:3 - Not Zero = One or More Bits of MSGVAL2 are Set"]
    #[inline]
    pub fn msgvalreg2(&self) -> MSGVALREG2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSGVALREG2R { bits }
    }
    #[doc = "Bits 0:1 - Not Zero = One or More Bits of MSGVAL1 are Set"]
    #[inline]
    pub fn msgvalreg1(&self) -> MSGVALREG1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSGVALREG1R { bits }
    }
}
