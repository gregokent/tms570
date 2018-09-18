#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACSTATUS {
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
#[doc = "Possible values of the field `IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLER {
    #[doc = "The EMAC is not idle"]
    NOIDLE,
    #[doc = "The EMAC is in the idle state"]
    IDLE,
}
impl IDLER {
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
            IDLER::NOIDLE => false,
            IDLER::IDLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLER {
        match value {
            false => IDLER::NOIDLE,
            true => IDLER::IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOIDLE`"]
    #[inline]
    pub fn is_noidle(&self) -> bool {
        *self == IDLER::NOIDLE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == IDLER::IDLE
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD1R {
    bits: u8,
}
impl _RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TXERRCODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXERRCODER {
    #[doc = "No error"]
    NOERROR,
    #[doc = "SOP error"]
    SOPERROR,
    #[doc = "Ownership bit not set in SOP buffer"]
    OWNERSHIP,
    #[doc = "Zero next buffer descriptor pointer without EOP"]
    NOEOP,
    #[doc = "Zero buffer pointer"]
    NULLPTR,
    #[doc = "Zero buffer length"]
    NULLEN,
    #[doc = "Packet length error (sum of buffers &lt; packet length)"]
    LENERROR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXERRCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXERRCODER::NOERROR => 0,
            TXERRCODER::SOPERROR => 1,
            TXERRCODER::OWNERSHIP => 2,
            TXERRCODER::NOEOP => 3,
            TXERRCODER::NULLPTR => 4,
            TXERRCODER::NULLEN => 5,
            TXERRCODER::LENERROR => 6,
            TXERRCODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXERRCODER {
        match value {
            0 => TXERRCODER::NOERROR,
            1 => TXERRCODER::SOPERROR,
            2 => TXERRCODER::OWNERSHIP,
            3 => TXERRCODER::NOEOP,
            4 => TXERRCODER::NULLPTR,
            5 => TXERRCODER::NULLEN,
            6 => TXERRCODER::LENERROR,
            i => TXERRCODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_noerror(&self) -> bool {
        *self == TXERRCODER::NOERROR
    }
    #[doc = "Checks if the value of the field is `SOPERROR`"]
    #[inline]
    pub fn is_soperror(&self) -> bool {
        *self == TXERRCODER::SOPERROR
    }
    #[doc = "Checks if the value of the field is `OWNERSHIP`"]
    #[inline]
    pub fn is_ownership(&self) -> bool {
        *self == TXERRCODER::OWNERSHIP
    }
    #[doc = "Checks if the value of the field is `NOEOP`"]
    #[inline]
    pub fn is_noeop(&self) -> bool {
        *self == TXERRCODER::NOEOP
    }
    #[doc = "Checks if the value of the field is `NULLPTR`"]
    #[inline]
    pub fn is_nullptr(&self) -> bool {
        *self == TXERRCODER::NULLPTR
    }
    #[doc = "Checks if the value of the field is `NULLEN`"]
    #[inline]
    pub fn is_nullen(&self) -> bool {
        *self == TXERRCODER::NULLEN
    }
    #[doc = "Checks if the value of the field is `LENERROR`"]
    #[inline]
    pub fn is_lenerror(&self) -> bool {
        *self == TXERRCODER::LENERROR
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD2R {
    bits: bool,
}
impl _RSVD2R {
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
#[doc = "Possible values of the field `TXERRCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXERRCHR {
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
impl TXERRCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXERRCHR::CHA0 => 0,
            TXERRCHR::CHA1 => 1,
            TXERRCHR::CHA2 => 2,
            TXERRCHR::CHA3 => 3,
            TXERRCHR::CHA4 => 4,
            TXERRCHR::CHA5 => 5,
            TXERRCHR::CHA6 => 6,
            TXERRCHR::CHA7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXERRCHR {
        match value {
            0 => TXERRCHR::CHA0,
            1 => TXERRCHR::CHA1,
            2 => TXERRCHR::CHA2,
            3 => TXERRCHR::CHA3,
            4 => TXERRCHR::CHA4,
            5 => TXERRCHR::CHA5,
            6 => TXERRCHR::CHA6,
            7 => TXERRCHR::CHA7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHA0`"]
    #[inline]
    pub fn is_cha0(&self) -> bool {
        *self == TXERRCHR::CHA0
    }
    #[doc = "Checks if the value of the field is `CHA1`"]
    #[inline]
    pub fn is_cha1(&self) -> bool {
        *self == TXERRCHR::CHA1
    }
    #[doc = "Checks if the value of the field is `CHA2`"]
    #[inline]
    pub fn is_cha2(&self) -> bool {
        *self == TXERRCHR::CHA2
    }
    #[doc = "Checks if the value of the field is `CHA3`"]
    #[inline]
    pub fn is_cha3(&self) -> bool {
        *self == TXERRCHR::CHA3
    }
    #[doc = "Checks if the value of the field is `CHA4`"]
    #[inline]
    pub fn is_cha4(&self) -> bool {
        *self == TXERRCHR::CHA4
    }
    #[doc = "Checks if the value of the field is `CHA5`"]
    #[inline]
    pub fn is_cha5(&self) -> bool {
        *self == TXERRCHR::CHA5
    }
    #[doc = "Checks if the value of the field is `CHA6`"]
    #[inline]
    pub fn is_cha6(&self) -> bool {
        *self == TXERRCHR::CHA6
    }
    #[doc = "Checks if the value of the field is `CHA7`"]
    #[inline]
    pub fn is_cha7(&self) -> bool {
        *self == TXERRCHR::CHA7
    }
}
#[doc = "Possible values of the field `RXERRCODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERRCODER {
    #[doc = "No error"]
    NOERROR,
    #[doc = "Ownership bit not set in SOP buffer"]
    OWNERSHIP,
    #[doc = "Zero buffer pointer"]
    NULLPTR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXERRCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXERRCODER::NOERROR => 0,
            RXERRCODER::OWNERSHIP => 2,
            RXERRCODER::NULLPTR => 4,
            RXERRCODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXERRCODER {
        match value {
            0 => RXERRCODER::NOERROR,
            2 => RXERRCODER::OWNERSHIP,
            4 => RXERRCODER::NULLPTR,
            i => RXERRCODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_noerror(&self) -> bool {
        *self == RXERRCODER::NOERROR
    }
    #[doc = "Checks if the value of the field is `OWNERSHIP`"]
    #[inline]
    pub fn is_ownership(&self) -> bool {
        *self == RXERRCODER::OWNERSHIP
    }
    #[doc = "Checks if the value of the field is `NULLPTR`"]
    #[inline]
    pub fn is_nullptr(&self) -> bool {
        *self == RXERRCODER::NULLPTR
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD3R {
    bits: bool,
}
impl _RSVD3R {
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
#[doc = "Possible values of the field `RXERRCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERRCHR {
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
impl RXERRCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXERRCHR::CHA0 => 0,
            RXERRCHR::CHA1 => 1,
            RXERRCHR::CHA2 => 2,
            RXERRCHR::CHA3 => 3,
            RXERRCHR::CHA4 => 4,
            RXERRCHR::CHA5 => 5,
            RXERRCHR::CHA6 => 6,
            RXERRCHR::CHA7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXERRCHR {
        match value {
            0 => RXERRCHR::CHA0,
            1 => RXERRCHR::CHA1,
            2 => RXERRCHR::CHA2,
            3 => RXERRCHR::CHA3,
            4 => RXERRCHR::CHA4,
            5 => RXERRCHR::CHA5,
            6 => RXERRCHR::CHA6,
            7 => RXERRCHR::CHA7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHA0`"]
    #[inline]
    pub fn is_cha0(&self) -> bool {
        *self == RXERRCHR::CHA0
    }
    #[doc = "Checks if the value of the field is `CHA1`"]
    #[inline]
    pub fn is_cha1(&self) -> bool {
        *self == RXERRCHR::CHA1
    }
    #[doc = "Checks if the value of the field is `CHA2`"]
    #[inline]
    pub fn is_cha2(&self) -> bool {
        *self == RXERRCHR::CHA2
    }
    #[doc = "Checks if the value of the field is `CHA3`"]
    #[inline]
    pub fn is_cha3(&self) -> bool {
        *self == RXERRCHR::CHA3
    }
    #[doc = "Checks if the value of the field is `CHA4`"]
    #[inline]
    pub fn is_cha4(&self) -> bool {
        *self == RXERRCHR::CHA4
    }
    #[doc = "Checks if the value of the field is `CHA5`"]
    #[inline]
    pub fn is_cha5(&self) -> bool {
        *self == RXERRCHR::CHA5
    }
    #[doc = "Checks if the value of the field is `CHA6`"]
    #[inline]
    pub fn is_cha6(&self) -> bool {
        *self == RXERRCHR::CHA6
    }
    #[doc = "Checks if the value of the field is `CHA7`"]
    #[inline]
    pub fn is_cha7(&self) -> bool {
        *self == RXERRCHR::CHA7
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD4R {
    bits: u8,
}
impl _RSVD4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD5R {
    bits: bool,
}
impl _RSVD5R {
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
pub struct _RSVD6R {
    bits: bool,
}
impl _RSVD6R {
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
#[doc = "Possible values of the field `RXQOSACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXQOSACTR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXQOSACTR {
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
            RXQOSACTR::DISABLE => false,
            RXQOSACTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXQOSACTR {
        match value {
            false => RXQOSACTR::DISABLE,
            true => RXQOSACTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXQOSACTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXQOSACTR::ENABLE
    }
}
#[doc = "Possible values of the field `RXFLOWACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFLOWACTR {
    #[doc = "no description"]
    INACTIVE,
    #[doc = "no description"]
    ACTIVE,
}
impl RXFLOWACTR {
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
            RXFLOWACTR::INACTIVE => false,
            RXFLOWACTR::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFLOWACTR {
        match value {
            false => RXFLOWACTR::INACTIVE,
            true => RXFLOWACTR::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == RXFLOWACTR::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == RXFLOWACTR::ACTIVE
    }
}
#[doc = "Possible values of the field `TXFLOWACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFLOWACTR {
    #[doc = "no description"]
    INACTIVE,
    #[doc = "no description"]
    ACTIVE,
}
impl TXFLOWACTR {
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
            TXFLOWACTR::INACTIVE => false,
            TXFLOWACTR::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFLOWACTR {
        match value {
            false => TXFLOWACTR::INACTIVE,
            true => TXFLOWACTR::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == TXFLOWACTR::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == TXFLOWACTR::ACTIVE
    }
}
#[doc = r" Proxy"]
pub struct __RSVD1W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 24;
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
pub struct __RSVD3W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD3W<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct __RSVD4W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct __RSVD5W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD5W<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct __RSVD6W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD6W<'a> {
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 31 - CPGMAC idle"]
    #[inline]
    pub fn idle(&self) -> IDLER {
        IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline]
    pub fn _rsvd1(&self) -> _RSVD1R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        _RSVD1R { bits }
    }
    #[doc = "Bits 20:23 - TX host error code"]
    #[inline]
    pub fn txerrcode(&self) -> TXERRCODER {
        TXERRCODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline]
    pub fn _rsvd2(&self) -> _RSVD2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD2R { bits }
    }
    #[doc = "Bits 16:18 - TX host error channel"]
    #[inline]
    pub fn txerrch(&self) -> TXERRCHR {
        TXERRCHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - RX host error code"]
    #[inline]
    pub fn rxerrcode(&self) -> RXERRCODER {
        RXERRCODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline]
    pub fn _rsvd3(&self) -> _RSVD3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD3R { bits }
    }
    #[doc = "Bits 8:10 - RX host error channel"]
    #[inline]
    pub fn rxerrch(&self) -> RXERRCHR {
        RXERRCHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline]
    pub fn _rsvd4(&self) -> _RSVD4R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        _RSVD4R { bits }
    }
    #[doc = "Bit 4 - no description"]
    #[inline]
    pub fn _rsvd5(&self) -> _RSVD5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD5R { bits }
    }
    #[doc = "Bit 3 - no description"]
    #[inline]
    pub fn _rsvd6(&self) -> _RSVD6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD6R { bits }
    }
    #[doc = "Bit 2 - RX Quality of Service active"]
    #[inline]
    pub fn rxqosact(&self) -> RXQOSACTR {
        RXQOSACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RX flow control active"]
    #[inline]
    pub fn rxflowact(&self) -> RXFLOWACTR {
        RXFLOWACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - TX flow control active"]
    #[inline]
    pub fn txflowact(&self) -> TXFLOWACTR {
        TXFLOWACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 24:30 - Reserved"]
    #[inline]
    pub fn _rsvd1(&mut self) -> __RSVD1W {
        __RSVD1W { w: self }
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline]
    pub fn _rsvd2(&mut self) -> __RSVD2W {
        __RSVD2W { w: self }
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline]
    pub fn _rsvd3(&mut self) -> __RSVD3W {
        __RSVD3W { w: self }
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline]
    pub fn _rsvd4(&mut self) -> __RSVD4W {
        __RSVD4W { w: self }
    }
    #[doc = "Bit 4 - no description"]
    #[inline]
    pub fn _rsvd5(&mut self) -> __RSVD5W {
        __RSVD5W { w: self }
    }
    #[doc = "Bit 3 - no description"]
    #[inline]
    pub fn _rsvd6(&mut self) -> __RSVD6W {
        __RSVD6W { w: self }
    }
}
