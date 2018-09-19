#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RTIINTCLRENABLE {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
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
pub struct INTCLRENABLE3R {
    bits: u8,
}
impl INTCLRENABLE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSV2R {
    bits: u8,
}
impl RSV2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTCLRENABLE2R {
    bits: u8,
}
impl INTCLRENABLE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSV3R {
    bits: u8,
}
impl RSV3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTCLRENABLE1R {
    bits: u8,
}
impl INTCLRENABLE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSV4R {
    bits: u8,
}
impl RSV4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTCLRENABLE0R {
    bits: u8,
}
impl INTCLRENABLE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _INTCLRENABLE3W<'a> {
    w: &'a mut W,
}
impl<'a> _INTCLRENABLE3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTCLRENABLE2W<'a> {
    w: &'a mut W,
}
impl<'a> _INTCLRENABLE2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTCLRENABLE1W<'a> {
    w: &'a mut W,
}
impl<'a> _INTCLRENABLE1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTCLRENABLE0W<'a> {
    w: &'a mut W,
}
impl<'a> _INTCLRENABLE0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 28:31 - Reserved"]
    #[inline]
    pub fn rsv1(&self) -> RSV1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSV1R { bits }
    }
    #[doc = "Bits 24:27 - 0x5 = Auto-Clear for RTICOMP3 Interrupt Disabled, Others=Enabled"]
    #[inline]
    pub fn intclrenable3(&self) -> INTCLRENABLE3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTCLRENABLE3R { bits }
    }
    #[doc = "Bits 20:23 - Reserved"]
    #[inline]
    pub fn rsv2(&self) -> RSV2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSV2R { bits }
    }
    #[doc = "Bits 16:19 - 0x5 = Auto-Clear for RTICOMP2 Interrupt Disabled, Others=Enabled"]
    #[inline]
    pub fn intclrenable2(&self) -> INTCLRENABLE2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTCLRENABLE2R { bits }
    }
    #[doc = "Bits 12:15 - Reserved"]
    #[inline]
    pub fn rsv3(&self) -> RSV3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSV3R { bits }
    }
    #[doc = "Bits 8:11 - 0x5 = Auto-Clear for RTICOMP1 Interrupt Disabled, Others=Enabled"]
    #[inline]
    pub fn intclrenable1(&self) -> INTCLRENABLE1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTCLRENABLE1R { bits }
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline]
    pub fn rsv4(&self) -> RSV4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSV4R { bits }
    }
    #[doc = "Bits 0:3 - 0x5 = Auto-Clear for RTICOMP0 Interrupt Disabled, Others=Enabled"]
    #[inline]
    pub fn intclrenable0(&self) -> INTCLRENABLE0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTCLRENABLE0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 84215045 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:27 - 0x5 = Auto-Clear for RTICOMP3 Interrupt Disabled, Others=Enabled"]
    #[inline]
    pub fn intclrenable3(&mut self) -> _INTCLRENABLE3W {
        _INTCLRENABLE3W { w: self }
    }
    #[doc = "Bits 16:19 - 0x5 = Auto-Clear for RTICOMP2 Interrupt Disabled, Others=Enabled"]
    #[inline]
    pub fn intclrenable2(&mut self) -> _INTCLRENABLE2W {
        _INTCLRENABLE2W { w: self }
    }
    #[doc = "Bits 8:11 - 0x5 = Auto-Clear for RTICOMP1 Interrupt Disabled, Others=Enabled"]
    #[inline]
    pub fn intclrenable1(&mut self) -> _INTCLRENABLE1W {
        _INTCLRENABLE1W { w: self }
    }
    #[doc = "Bits 0:3 - 0x5 = Auto-Clear for RTICOMP0 Interrupt Disabled, Others=Enabled"]
    #[inline]
    pub fn intclrenable0(&mut self) -> _INTCLRENABLE0W {
        _INTCLRENABLE0W { w: self }
    }
}
