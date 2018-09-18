#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTCONTROL {
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
    bits: u16,
}
impl _RSVD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `C2TXPACEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C2TXPACEENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C2TXPACEENR {
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
            C2TXPACEENR::DISABLE => false,
            C2TXPACEENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C2TXPACEENR {
        match value {
            false => C2TXPACEENR::DISABLE,
            true => C2TXPACEENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == C2TXPACEENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == C2TXPACEENR::ENABLE
    }
}
#[doc = "Possible values of the field `C2RXPACEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C2RXPACEENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C2RXPACEENR {
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
            C2RXPACEENR::DISABLE => false,
            C2RXPACEENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C2RXPACEENR {
        match value {
            false => C2RXPACEENR::DISABLE,
            true => C2RXPACEENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == C2RXPACEENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == C2RXPACEENR::ENABLE
    }
}
#[doc = "Possible values of the field `C1TXPACEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1TXPACEENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C1TXPACEENR {
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
            C1TXPACEENR::DISABLE => false,
            C1TXPACEENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C1TXPACEENR {
        match value {
            false => C1TXPACEENR::DISABLE,
            true => C1TXPACEENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == C1TXPACEENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == C1TXPACEENR::ENABLE
    }
}
#[doc = "Possible values of the field `C1RXPACEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1RXPACEENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C1RXPACEENR {
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
            C1RXPACEENR::DISABLE => false,
            C1RXPACEENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C1RXPACEENR {
        match value {
            false => C1RXPACEENR::DISABLE,
            true => C1RXPACEENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == C1RXPACEENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == C1RXPACEENR::ENABLE
    }
}
#[doc = "Possible values of the field `C0TXPACEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0TXPACEENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C0TXPACEENR {
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
            C0TXPACEENR::DISABLE => false,
            C0TXPACEENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C0TXPACEENR {
        match value {
            false => C0TXPACEENR::DISABLE,
            true => C0TXPACEENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == C0TXPACEENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == C0TXPACEENR::ENABLE
    }
}
#[doc = "Possible values of the field `C0RXPACEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0RXPACEENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C0RXPACEENR {
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
            C0RXPACEENR::DISABLE => false,
            C0RXPACEENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> C0RXPACEENR {
        match value {
            false => C0RXPACEENR::DISABLE,
            true => C0RXPACEENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == C0RXPACEENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == C0RXPACEENR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD3R {
    bits: u8,
}
impl _RSVD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INTPRESCALER {
    bits: u16,
}
impl INTPRESCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
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
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C2TXPACEEN`"]
pub enum C2TXPACEENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C2TXPACEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            C2TXPACEENW::DISABLE => false,
            C2TXPACEENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C2TXPACEENW<'a> {
    w: &'a mut W,
}
impl<'a> _C2TXPACEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C2TXPACEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(C2TXPACEENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(C2TXPACEENW::ENABLE)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C2RXPACEEN`"]
pub enum C2RXPACEENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C2RXPACEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            C2RXPACEENW::DISABLE => false,
            C2RXPACEENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C2RXPACEENW<'a> {
    w: &'a mut W,
}
impl<'a> _C2RXPACEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C2RXPACEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(C2RXPACEENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(C2RXPACEENW::ENABLE)
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
#[doc = "Values that can be written to the field `C1TXPACEEN`"]
pub enum C1TXPACEENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C1TXPACEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            C1TXPACEENW::DISABLE => false,
            C1TXPACEENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C1TXPACEENW<'a> {
    w: &'a mut W,
}
impl<'a> _C1TXPACEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C1TXPACEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(C1TXPACEENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(C1TXPACEENW::ENABLE)
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
#[doc = "Values that can be written to the field `C1RXPACEEN`"]
pub enum C1RXPACEENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C1RXPACEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            C1RXPACEENW::DISABLE => false,
            C1RXPACEENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C1RXPACEENW<'a> {
    w: &'a mut W,
}
impl<'a> _C1RXPACEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C1RXPACEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(C1RXPACEENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(C1RXPACEENW::ENABLE)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C0TXPACEEN`"]
pub enum C0TXPACEENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C0TXPACEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            C0TXPACEENW::DISABLE => false,
            C0TXPACEENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C0TXPACEENW<'a> {
    w: &'a mut W,
}
impl<'a> _C0TXPACEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C0TXPACEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(C0TXPACEENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(C0TXPACEENW::ENABLE)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C0RXPACEEN`"]
pub enum C0RXPACEENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl C0RXPACEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            C0RXPACEENW::DISABLE => false,
            C0RXPACEENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C0RXPACEENW<'a> {
    w: &'a mut W,
}
impl<'a> _C0RXPACEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C0RXPACEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(C0RXPACEENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(C0RXPACEENW::ENABLE)
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
        const OFFSET: u8 = 16;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTPRESCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _INTPRESCALEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
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
    #[doc = "Bit 31 - Interrupt Test"]
    #[inline]
    pub fn _rsvd1(&self) -> _RSVD1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD1R { bits }
    }
    #[doc = "Bits 22:30 - no description"]
    #[inline]
    pub fn _rsvd2(&self) -> _RSVD2R {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        _RSVD2R { bits }
    }
    #[doc = "Bit 21 - C2_TX_PULSE Pacing"]
    #[inline]
    pub fn c2txpaceen(&self) -> C2TXPACEENR {
        C2TXPACEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - C2_RX_PULSE Pacing"]
    #[inline]
    pub fn c2rxpaceen(&self) -> C2RXPACEENR {
        C2RXPACEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - C1_TX_PULSE Pacing"]
    #[inline]
    pub fn c1txpaceen(&self) -> C1TXPACEENR {
        C1TXPACEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - C1_RX_PULSE Pacing"]
    #[inline]
    pub fn c1rxpaceen(&self) -> C1RXPACEENR {
        C1RXPACEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - C0_TX_PULSE Pacing"]
    #[inline]
    pub fn c0txpaceen(&self) -> C0TXPACEENR {
        C0TXPACEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - C0_RX_PULSE Pacing"]
    #[inline]
    pub fn c0rxpaceen(&self) -> C0RXPACEENR {
        C0RXPACEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:15 - no description"]
    #[inline]
    pub fn _rsvd3(&self) -> _RSVD3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        _RSVD3R { bits }
    }
    #[doc = "Bits 0:11 - Interrupt Counter Prescaler (Number of 4us VBUSP_CLKs)"]
    #[inline]
    pub fn intprescale(&self) -> INTPRESCALER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INTPRESCALER { bits }
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
    #[doc = "Bit 31 - Interrupt Test"]
    #[inline]
    pub fn _rsvd1(&mut self) -> __RSVD1W {
        __RSVD1W { w: self }
    }
    #[doc = "Bits 22:30 - no description"]
    #[inline]
    pub fn _rsvd2(&mut self) -> __RSVD2W {
        __RSVD2W { w: self }
    }
    #[doc = "Bit 21 - C2_TX_PULSE Pacing"]
    #[inline]
    pub fn c2txpaceen(&mut self) -> _C2TXPACEENW {
        _C2TXPACEENW { w: self }
    }
    #[doc = "Bit 20 - C2_RX_PULSE Pacing"]
    #[inline]
    pub fn c2rxpaceen(&mut self) -> _C2RXPACEENW {
        _C2RXPACEENW { w: self }
    }
    #[doc = "Bit 19 - C1_TX_PULSE Pacing"]
    #[inline]
    pub fn c1txpaceen(&mut self) -> _C1TXPACEENW {
        _C1TXPACEENW { w: self }
    }
    #[doc = "Bit 18 - C1_RX_PULSE Pacing"]
    #[inline]
    pub fn c1rxpaceen(&mut self) -> _C1RXPACEENW {
        _C1RXPACEENW { w: self }
    }
    #[doc = "Bit 17 - C0_TX_PULSE Pacing"]
    #[inline]
    pub fn c0txpaceen(&mut self) -> _C0TXPACEENW {
        _C0TXPACEENW { w: self }
    }
    #[doc = "Bit 16 - C0_RX_PULSE Pacing"]
    #[inline]
    pub fn c0rxpaceen(&mut self) -> _C0RXPACEENW {
        _C0RXPACEENW { w: self }
    }
    #[doc = "Bits 12:15 - no description"]
    #[inline]
    pub fn _rsvd3(&mut self) -> __RSVD3W {
        __RSVD3W { w: self }
    }
    #[doc = "Bits 0:11 - Interrupt Counter Prescaler (Number of 4us VBUSP_CLKs)"]
    #[inline]
    pub fn intprescale(&mut self) -> _INTPRESCALEW {
        _INTPRESCALEW { w: self }
    }
}
