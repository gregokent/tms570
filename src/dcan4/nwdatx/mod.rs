#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::NWDATX {
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
pub struct NEWDATREG8R {
    bits: u8,
}
impl NEWDATREG8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NEWDATREG7R {
    bits: u8,
}
impl NEWDATREG7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NEWDATREG6R {
    bits: u8,
}
impl NEWDATREG6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NEWDATREG5R {
    bits: u8,
}
impl NEWDATREG5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NEWDATREG4R {
    bits: u8,
}
impl NEWDATREG4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NEWDATREG3R {
    bits: u8,
}
impl NEWDATREG3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NEWDATREG2R {
    bits: u8,
}
impl NEWDATREG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NEWDATREG1R {
    bits: u8,
}
impl NEWDATREG1R {
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
    #[doc = "Bits 14:15 - Not Zero = One or More Bits of NWDAT8 are Set"]
    #[inline]
    pub fn newdatreg8(&self) -> NEWDATREG8R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NEWDATREG8R { bits }
    }
    #[doc = "Bits 12:13 - Not Zero = One or More Bits of NWDAT7 are Set"]
    #[inline]
    pub fn newdatreg7(&self) -> NEWDATREG7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NEWDATREG7R { bits }
    }
    #[doc = "Bits 10:11 - Not Zero = One or More Bits of NWDAT6 are Set"]
    #[inline]
    pub fn newdatreg6(&self) -> NEWDATREG6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NEWDATREG6R { bits }
    }
    #[doc = "Bits 8:9 - Not Zero = One or More Bits of NWDAT5 are Set"]
    #[inline]
    pub fn newdatreg5(&self) -> NEWDATREG5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NEWDATREG5R { bits }
    }
    #[doc = "Bits 6:7 - Not Zero = One or More Bits of NWDAT4 are Set"]
    #[inline]
    pub fn newdatreg4(&self) -> NEWDATREG4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NEWDATREG4R { bits }
    }
    #[doc = "Bits 4:5 - Not Zero = One or More Bits of NWDAT3 are Set"]
    #[inline]
    pub fn newdatreg3(&self) -> NEWDATREG3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NEWDATREG3R { bits }
    }
    #[doc = "Bits 2:3 - Not Zero = One or More Bits of NWDAT2 are Set"]
    #[inline]
    pub fn newdatreg2(&self) -> NEWDATREG2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NEWDATREG2R { bits }
    }
    #[doc = "Bits 0:1 - Not Zero = One or More Bits of NWDAT1 are Set"]
    #[inline]
    pub fn newdatreg1(&self) -> NEWDATREG1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NEWDATREG1R { bits }
    }
}
