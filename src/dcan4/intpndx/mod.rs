#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTPNDX {
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
pub struct INTPNDREG8R {
    bits: u8,
}
impl INTPNDREG8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTPNDREG7R {
    bits: u8,
}
impl INTPNDREG7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTPNDREG6R {
    bits: u8,
}
impl INTPNDREG6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTPNDREG5R {
    bits: u8,
}
impl INTPNDREG5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTPNDREG4R {
    bits: u8,
}
impl INTPNDREG4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTPNDREG3R {
    bits: u8,
}
impl INTPNDREG3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTPNDREG2R {
    bits: u8,
}
impl INTPNDREG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTPNDREG1R {
    bits: u8,
}
impl INTPNDREG1R {
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
    #[doc = "Bits 14:15 - Not Zero = One or More Bits of INTPND8 are Set"]
    #[inline]
    pub fn intpndreg8(&self) -> INTPNDREG8R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPNDREG8R { bits }
    }
    #[doc = "Bits 12:13 - Not Zero = One or More Bits of INTPND7 are Set"]
    #[inline]
    pub fn intpndreg7(&self) -> INTPNDREG7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPNDREG7R { bits }
    }
    #[doc = "Bits 10:11 - Not Zero = One or More Bits of INTPND6 are Set"]
    #[inline]
    pub fn intpndreg6(&self) -> INTPNDREG6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPNDREG6R { bits }
    }
    #[doc = "Bits 8:9 - Not Zero = One or More Bits of INTPND5 are Set"]
    #[inline]
    pub fn intpndreg5(&self) -> INTPNDREG5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPNDREG5R { bits }
    }
    #[doc = "Bits 6:7 - Not Zero = One or More Bits of INTPND4 are Set"]
    #[inline]
    pub fn intpndreg4(&self) -> INTPNDREG4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPNDREG4R { bits }
    }
    #[doc = "Bits 4:5 - Not Zero = One or More Bits of INTPND3 are Set"]
    #[inline]
    pub fn intpndreg3(&self) -> INTPNDREG3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPNDREG3R { bits }
    }
    #[doc = "Bits 2:3 - Not Zero = One or More Bits of INTPND2 are Set"]
    #[inline]
    pub fn intpndreg2(&self) -> INTPNDREG2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPNDREG2R { bits }
    }
    #[doc = "Bits 0:1 - Not Zero = One or More Bits of INTPND1 are Set"]
    #[inline]
    pub fn intpndreg1(&self) -> INTPNDREG1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPNDREG1R { bits }
    }
}
