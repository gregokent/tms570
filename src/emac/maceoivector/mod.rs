#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACEOIVECTOR {
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
pub struct _RSVD1R {
    bits: u32,
}
impl _RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `INTVECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTVECTR {
    #[doc = "no description"]
    C0RXTHRESH,
    #[doc = "no description"]
    C0RX,
    #[doc = "no description"]
    C0TX,
    #[doc = "no description"]
    C0MISC,
    #[doc = "no description"]
    C1RXTHRESH,
    #[doc = "no description"]
    C1RX,
    #[doc = "no description"]
    C1TX,
    #[doc = "no description"]
    C1MISC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INTVECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INTVECTR::C0RXTHRESH => 0,
            INTVECTR::C0RX => 1,
            INTVECTR::C0TX => 2,
            INTVECTR::C0MISC => 3,
            INTVECTR::C1RXTHRESH => 4,
            INTVECTR::C1RX => 5,
            INTVECTR::C1TX => 6,
            INTVECTR::C1MISC => 7,
            INTVECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INTVECTR {
        match value {
            0 => INTVECTR::C0RXTHRESH,
            1 => INTVECTR::C0RX,
            2 => INTVECTR::C0TX,
            3 => INTVECTR::C0MISC,
            4 => INTVECTR::C1RXTHRESH,
            5 => INTVECTR::C1RX,
            6 => INTVECTR::C1TX,
            7 => INTVECTR::C1MISC,
            i => INTVECTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `C0RXTHRESH`"]
    #[inline]
    pub fn is_c0rxthresh(&self) -> bool {
        *self == INTVECTR::C0RXTHRESH
    }
    #[doc = "Checks if the value of the field is `C0RX`"]
    #[inline]
    pub fn is_c0rx(&self) -> bool {
        *self == INTVECTR::C0RX
    }
    #[doc = "Checks if the value of the field is `C0TX`"]
    #[inline]
    pub fn is_c0tx(&self) -> bool {
        *self == INTVECTR::C0TX
    }
    #[doc = "Checks if the value of the field is `C0MISC`"]
    #[inline]
    pub fn is_c0misc(&self) -> bool {
        *self == INTVECTR::C0MISC
    }
    #[doc = "Checks if the value of the field is `C1RXTHRESH`"]
    #[inline]
    pub fn is_c1rxthresh(&self) -> bool {
        *self == INTVECTR::C1RXTHRESH
    }
    #[doc = "Checks if the value of the field is `C1RX`"]
    #[inline]
    pub fn is_c1rx(&self) -> bool {
        *self == INTVECTR::C1RX
    }
    #[doc = "Checks if the value of the field is `C1TX`"]
    #[inline]
    pub fn is_c1tx(&self) -> bool {
        *self == INTVECTR::C1TX
    }
    #[doc = "Checks if the value of the field is `C1MISC`"]
    #[inline]
    pub fn is_c1misc(&self) -> bool {
        *self == INTVECTR::C1MISC
    }
}
#[doc = r" Proxy"]
pub struct __RSVD1W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 134217727;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTVECT`"]
pub enum INTVECTW {
    #[doc = "no description"]
    C0RXTHRESH,
    #[doc = "no description"]
    C0RX,
    #[doc = "no description"]
    C0TX,
    #[doc = "no description"]
    C0MISC,
    #[doc = "no description"]
    C1RXTHRESH,
    #[doc = "no description"]
    C1RX,
    #[doc = "no description"]
    C1TX,
    #[doc = "no description"]
    C1MISC,
}
impl INTVECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INTVECTW::C0RXTHRESH => 0,
            INTVECTW::C0RX => 1,
            INTVECTW::C0TX => 2,
            INTVECTW::C0MISC => 3,
            INTVECTW::C1RXTHRESH => 4,
            INTVECTW::C1RX => 5,
            INTVECTW::C1TX => 6,
            INTVECTW::C1MISC => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTVECTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTVECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTVECTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "no description"]
    #[inline]
    pub fn c0rxthresh(self) -> &'a mut W {
        self.variant(INTVECTW::C0RXTHRESH)
    }
    #[doc = "no description"]
    #[inline]
    pub fn c0rx(self) -> &'a mut W {
        self.variant(INTVECTW::C0RX)
    }
    #[doc = "no description"]
    #[inline]
    pub fn c0tx(self) -> &'a mut W {
        self.variant(INTVECTW::C0TX)
    }
    #[doc = "no description"]
    #[inline]
    pub fn c0misc(self) -> &'a mut W {
        self.variant(INTVECTW::C0MISC)
    }
    #[doc = "no description"]
    #[inline]
    pub fn c1rxthresh(self) -> &'a mut W {
        self.variant(INTVECTW::C1RXTHRESH)
    }
    #[doc = "no description"]
    #[inline]
    pub fn c1rx(self) -> &'a mut W {
        self.variant(INTVECTW::C1RX)
    }
    #[doc = "no description"]
    #[inline]
    pub fn c1tx(self) -> &'a mut W {
        self.variant(INTVECTW::C1TX)
    }
    #[doc = "no description"]
    #[inline]
    pub fn c1misc(self) -> &'a mut W {
        self.variant(INTVECTW::C1MISC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 5:31 - Reserved"]
    #[inline]
    pub fn _rsvd1(&self) -> _RSVD1R {
        let bits = {
            const MASK: u32 = 134217727;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        _RSVD1R { bits }
    }
    #[doc = "Bits 0:4 - MAC End of Interrupt Vector; reflects the value written to this location one VBUSP_CLK cycle after a write to this location."]
    #[inline]
    pub fn intvect(&self) -> INTVECTR {
        INTVECTR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 5:31 - Reserved"]
    #[inline]
    pub fn _rsvd1(&mut self) -> __RSVD1W {
        __RSVD1W { w: self }
    }
    #[doc = "Bits 0:4 - MAC End of Interrupt Vector; reflects the value written to this location one VBUSP_CLK cycle after a write to this location."]
    #[inline]
    pub fn intvect(&mut self) -> _INTVECTW {
        _INTVECTW { w: self }
    }
}
