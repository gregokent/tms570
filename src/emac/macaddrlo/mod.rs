#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACADDRLO {
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
    bits: u16,
}
impl _RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALIDR {
    #[doc = "no description"]
    INVALID,
    #[doc = "no description"]
    VALID,
}
impl VALIDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VALIDR::INVALID => false,
            VALIDR::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VALIDR {
        match value {
            false => VALIDR::INVALID,
            true => VALIDR::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline]
    pub fn is_invalid(&self) -> bool {
        *self == VALIDR::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VALIDR::VALID
    }
}
#[doc = "Possible values of the field `MATCHFILT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCHFILTR {
    #[doc = "no description"]
    FILTER,
    #[doc = "no description"]
    MATCH,
}
impl MATCHFILTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MATCHFILTR::FILTER => false,
            MATCHFILTR::MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MATCHFILTR {
        match value {
            false => MATCHFILTR::FILTER,
            true => MATCHFILTR::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER`"]
    #[inline]
    pub fn is_filter(&self) -> bool {
        *self == MATCHFILTR::FILTER
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == MATCHFILTR::MATCH
    }
}
#[doc = r" Value of the field"]
pub struct CHANNELR {
    bits: u8,
}
impl CHANNELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MACADDR0R {
    bits: u8,
}
impl MACADDR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MACADDR1R {
    bits: u8,
}
impl MACADDR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct __RSVD1W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VALID`"]
pub enum VALIDW {
    #[doc = "no description"]
    INVALID,
    #[doc = "no description"]
    VALID,
}
impl VALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VALIDW::INVALID => false,
            VALIDW::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _VALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn invalid(self) -> &'a mut W {
        self.variant(VALIDW::INVALID)
    }
    #[doc = "no description"]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VALIDW::VALID)
    }
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MATCHFILT`"]
pub enum MATCHFILTW {
    #[doc = "no description"]
    FILTER,
    #[doc = "no description"]
    MATCH,
}
impl MATCHFILTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MATCHFILTW::FILTER => false,
            MATCHFILTW::MATCH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATCHFILTW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCHFILTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATCHFILTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn filter(self) -> &'a mut W {
        self.variant(MATCHFILTW::FILTER)
    }
    #[doc = "no description"]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(MATCHFILTW::MATCH)
    }
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHANNELW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MACADDR0W<'a> {
    w: &'a mut W,
}
impl<'a> _MACADDR0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MACADDR1W<'a> {
    w: &'a mut W,
}
impl<'a> _MACADDR1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 21:31 - Reserved"]
    #[inline]
    pub fn _rsvd1(&self) -> _RSVD1R {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        _RSVD1R { bits }
    }
    #[doc = "Bit 20 - no description"]
    #[inline]
    pub fn valid(&self) -> VALIDR {
        VALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - no description"]
    #[inline]
    pub fn matchfilt(&self) -> MATCHFILTR {
        MATCHFILTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - no description"]
    #[inline]
    pub fn channel(&self) -> CHANNELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHANNELR { bits }
    }
    #[doc = "Bits 8:15 - MAC addres lower 8 bits (byte 0)"]
    #[inline]
    pub fn macaddr0(&self) -> MACADDR0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MACADDR0R { bits }
    }
    #[doc = "Bits 0:7 - MAC addres bits 15:8 (byte 1)"]
    #[inline]
    pub fn macaddr1(&self) -> MACADDR1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MACADDR1R { bits }
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
    #[doc = "Bits 21:31 - Reserved"]
    #[inline]
    pub fn _rsvd1(&mut self) -> __RSVD1W {
        __RSVD1W { w: self }
    }
    #[doc = "Bit 20 - no description"]
    #[inline]
    pub fn valid(&mut self) -> _VALIDW {
        _VALIDW { w: self }
    }
    #[doc = "Bit 19 - no description"]
    #[inline]
    pub fn matchfilt(&mut self) -> _MATCHFILTW {
        _MATCHFILTW { w: self }
    }
    #[doc = "Bits 16:18 - no description"]
    #[inline]
    pub fn channel(&mut self) -> _CHANNELW {
        _CHANNELW { w: self }
    }
    #[doc = "Bits 8:15 - MAC addres lower 8 bits (byte 0)"]
    #[inline]
    pub fn macaddr0(&mut self) -> _MACADDR0W {
        _MACADDR0W { w: self }
    }
    #[doc = "Bits 0:7 - MAC addres bits 15:8 (byte 1)"]
    #[inline]
    pub fn macaddr1(&mut self) -> _MACADDR1W {
        _MACADDR1W { w: self }
    }
}
