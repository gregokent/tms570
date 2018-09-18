#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXTEARDOWN {
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
    bits: bool,
}
impl _RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD2R {
    bits: u32,
}
impl _RSVD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `TXTDNCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTDNCHR {
    #[doc = "no description"]
    CHA0,
    #[doc = "no description"]
    CHA1,
    #[doc = "no description"]
    CHA2,
    #[doc = "no description"]
    CHA3,
    #[doc = "no description"]
    CHA4,
    #[doc = "no description"]
    CHA5,
    #[doc = "no description"]
    CHA6,
    #[doc = "no description"]
    CHA7,
}
impl TXTDNCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXTDNCHR::CHA0 => 0,
            TXTDNCHR::CHA1 => 1,
            TXTDNCHR::CHA2 => 2,
            TXTDNCHR::CHA3 => 3,
            TXTDNCHR::CHA4 => 4,
            TXTDNCHR::CHA5 => 5,
            TXTDNCHR::CHA6 => 6,
            TXTDNCHR::CHA7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXTDNCHR {
        match value {
            0 => TXTDNCHR::CHA0,
            1 => TXTDNCHR::CHA1,
            2 => TXTDNCHR::CHA2,
            3 => TXTDNCHR::CHA3,
            4 => TXTDNCHR::CHA4,
            5 => TXTDNCHR::CHA5,
            6 => TXTDNCHR::CHA6,
            7 => TXTDNCHR::CHA7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHA0`"]
    #[inline]
    pub fn is_cha0(&self) -> bool {
        *self == TXTDNCHR::CHA0
    }
    #[doc = "Checks if the value of the field is `CHA1`"]
    #[inline]
    pub fn is_cha1(&self) -> bool {
        *self == TXTDNCHR::CHA1
    }
    #[doc = "Checks if the value of the field is `CHA2`"]
    #[inline]
    pub fn is_cha2(&self) -> bool {
        *self == TXTDNCHR::CHA2
    }
    #[doc = "Checks if the value of the field is `CHA3`"]
    #[inline]
    pub fn is_cha3(&self) -> bool {
        *self == TXTDNCHR::CHA3
    }
    #[doc = "Checks if the value of the field is `CHA4`"]
    #[inline]
    pub fn is_cha4(&self) -> bool {
        *self == TXTDNCHR::CHA4
    }
    #[doc = "Checks if the value of the field is `CHA5`"]
    #[inline]
    pub fn is_cha5(&self) -> bool {
        *self == TXTDNCHR::CHA5
    }
    #[doc = "Checks if the value of the field is `CHA6`"]
    #[inline]
    pub fn is_cha6(&self) -> bool {
        *self == TXTDNCHR::CHA6
    }
    #[doc = "Checks if the value of the field is `CHA7`"]
    #[inline]
    pub fn is_cha7(&self) -> bool {
        *self == TXTDNCHR::CHA7
    }
}
#[doc = r" Proxy"]
pub struct __RSVD1W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD1W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct __RSVD2W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXTDNCH`"]
pub enum TXTDNCHW {
    #[doc = "no description"]
    CHA0,
    #[doc = "no description"]
    CHA1,
    #[doc = "no description"]
    CHA2,
    #[doc = "no description"]
    CHA3,
    #[doc = "no description"]
    CHA4,
    #[doc = "no description"]
    CHA5,
    #[doc = "no description"]
    CHA6,
    #[doc = "no description"]
    CHA7,
}
impl TXTDNCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXTDNCHW::CHA0 => 0,
            TXTDNCHW::CHA1 => 1,
            TXTDNCHW::CHA2 => 2,
            TXTDNCHW::CHA3 => 3,
            TXTDNCHW::CHA4 => 4,
            TXTDNCHW::CHA5 => 5,
            TXTDNCHW::CHA6 => 6,
            TXTDNCHW::CHA7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXTDNCHW<'a> {
    w: &'a mut W,
}
impl<'a> _TXTDNCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXTDNCHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha0(self) -> &'a mut W {
        self.variant(TXTDNCHW::CHA0)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha1(self) -> &'a mut W {
        self.variant(TXTDNCHW::CHA1)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha2(self) -> &'a mut W {
        self.variant(TXTDNCHW::CHA2)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha3(self) -> &'a mut W {
        self.variant(TXTDNCHW::CHA3)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha4(self) -> &'a mut W {
        self.variant(TXTDNCHW::CHA4)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha5(self) -> &'a mut W {
        self.variant(TXTDNCHW::CHA5)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha6(self) -> &'a mut W {
        self.variant(TXTDNCHW::CHA6)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha7(self) -> &'a mut W {
        self.variant(TXTDNCHW::CHA7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 31 - TX teardown ready; read as zero, but always assumed to be one - unused"]
    #[inline]
    pub fn _rsvd1(&self) -> _RSVD1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD1R { bits }
    }
    #[doc = "Bits 3:30 - Reserved"]
    #[inline]
    pub fn _rsvd2(&self) -> _RSVD2R {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        _RSVD2R { bits }
    }
    #[doc = "Bits 0:2 - TX teardown channed"]
    #[inline]
    pub fn txtdnch(&self) -> TXTDNCHR {
        TXTDNCHR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bit 31 - TX teardown ready; read as zero, but always assumed to be one - unused"]
    #[inline]
    pub fn _rsvd1(&mut self) -> __RSVD1W {
        __RSVD1W { w: self }
    }
    #[doc = "Bits 3:30 - Reserved"]
    #[inline]
    pub fn _rsvd2(&mut self) -> __RSVD2W {
        __RSVD2W { w: self }
    }
    #[doc = "Bits 0:2 - TX teardown channed"]
    #[inline]
    pub fn txtdnch(&mut self) -> _TXTDNCHW {
        _TXTDNCHW { w: self }
    }
}
