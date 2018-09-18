#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::C2RXTHRESHEN {
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
#[doc = "Possible values of the field `RXCH7THRESHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH7THRESHENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH7THRESHENR {
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
            RXCH7THRESHENR::DISABLE => false,
            RXCH7THRESHENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH7THRESHENR {
        match value {
            false => RXCH7THRESHENR::DISABLE,
            true => RXCH7THRESHENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCH7THRESHENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCH7THRESHENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXCH6THRESHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH6THRESHENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH6THRESHENR {
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
            RXCH6THRESHENR::DISABLE => false,
            RXCH6THRESHENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH6THRESHENR {
        match value {
            false => RXCH6THRESHENR::DISABLE,
            true => RXCH6THRESHENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCH6THRESHENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCH6THRESHENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXCH5THRESHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH5THRESHENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH5THRESHENR {
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
            RXCH5THRESHENR::DISABLE => false,
            RXCH5THRESHENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH5THRESHENR {
        match value {
            false => RXCH5THRESHENR::DISABLE,
            true => RXCH5THRESHENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCH5THRESHENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCH5THRESHENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXCH4THRESHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH4THRESHENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH4THRESHENR {
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
            RXCH4THRESHENR::DISABLE => false,
            RXCH4THRESHENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH4THRESHENR {
        match value {
            false => RXCH4THRESHENR::DISABLE,
            true => RXCH4THRESHENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCH4THRESHENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCH4THRESHENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXCH3THRESHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH3THRESHENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH3THRESHENR {
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
            RXCH3THRESHENR::DISABLE => false,
            RXCH3THRESHENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH3THRESHENR {
        match value {
            false => RXCH3THRESHENR::DISABLE,
            true => RXCH3THRESHENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCH3THRESHENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCH3THRESHENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXCH2THRESHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH2THRESHENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH2THRESHENR {
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
            RXCH2THRESHENR::DISABLE => false,
            RXCH2THRESHENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH2THRESHENR {
        match value {
            false => RXCH2THRESHENR::DISABLE,
            true => RXCH2THRESHENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCH2THRESHENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCH2THRESHENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXCH1THRESHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH1THRESHENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH1THRESHENR {
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
            RXCH1THRESHENR::DISABLE => false,
            RXCH1THRESHENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH1THRESHENR {
        match value {
            false => RXCH1THRESHENR::DISABLE,
            true => RXCH1THRESHENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCH1THRESHENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCH1THRESHENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXCH0THRESHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH0THRESHENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH0THRESHENR {
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
            RXCH0THRESHENR::DISABLE => false,
            RXCH0THRESHENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH0THRESHENR {
        match value {
            false => RXCH0THRESHENR::DISABLE,
            true => RXCH0THRESHENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCH0THRESHENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCH0THRESHENR::ENABLE
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
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH7THRESHEN`"]
pub enum RXCH7THRESHENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH7THRESHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH7THRESHENW::DISABLE => false,
            RXCH7THRESHENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH7THRESHENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH7THRESHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH7THRESHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCH7THRESHENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCH7THRESHENW::ENABLE)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH6THRESHEN`"]
pub enum RXCH6THRESHENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH6THRESHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH6THRESHENW::DISABLE => false,
            RXCH6THRESHENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH6THRESHENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH6THRESHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH6THRESHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCH6THRESHENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCH6THRESHENW::ENABLE)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH5THRESHEN`"]
pub enum RXCH5THRESHENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH5THRESHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH5THRESHENW::DISABLE => false,
            RXCH5THRESHENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH5THRESHENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH5THRESHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH5THRESHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCH5THRESHENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCH5THRESHENW::ENABLE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH4THRESHEN`"]
pub enum RXCH4THRESHENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH4THRESHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH4THRESHENW::DISABLE => false,
            RXCH4THRESHENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH4THRESHENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH4THRESHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH4THRESHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCH4THRESHENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCH4THRESHENW::ENABLE)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH3THRESHEN`"]
pub enum RXCH3THRESHENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH3THRESHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH3THRESHENW::DISABLE => false,
            RXCH3THRESHENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH3THRESHENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH3THRESHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH3THRESHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCH3THRESHENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCH3THRESHENW::ENABLE)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH2THRESHEN`"]
pub enum RXCH2THRESHENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH2THRESHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH2THRESHENW::DISABLE => false,
            RXCH2THRESHENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH2THRESHENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH2THRESHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH2THRESHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCH2THRESHENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCH2THRESHENW::ENABLE)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH1THRESHEN`"]
pub enum RXCH1THRESHENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH1THRESHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH1THRESHENW::DISABLE => false,
            RXCH1THRESHENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH1THRESHENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH1THRESHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH1THRESHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCH1THRESHENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCH1THRESHENW::ENABLE)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH0THRESHEN`"]
pub enum RXCH0THRESHENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl RXCH0THRESHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH0THRESHENW::DISABLE => false,
            RXCH0THRESHENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH0THRESHENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH0THRESHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH0THRESHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCH0THRESHENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCH0THRESHENW::ENABLE)
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
    #[doc = "Bits 8:31 - no description"]
    #[inline]
    pub fn _rsvd1(&self) -> _RSVD1R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        _RSVD1R { bits }
    }
    #[doc = "Bit 7 - Corresponds to the interrupt for RXCH7THRESH"]
    #[inline]
    pub fn rxch7threshen(&self) -> RXCH7THRESHENR {
        RXCH7THRESHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Corresponds to the interrupt for RXCH6THRESH"]
    #[inline]
    pub fn rxch6threshen(&self) -> RXCH6THRESHENR {
        RXCH6THRESHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Corresponds to the interrupt for RXCH5THRESH"]
    #[inline]
    pub fn rxch5threshen(&self) -> RXCH5THRESHENR {
        RXCH5THRESHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Corresponds to the interrupt for RXCH4THRESH"]
    #[inline]
    pub fn rxch4threshen(&self) -> RXCH4THRESHENR {
        RXCH4THRESHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Corresponds to the interrupt for RXCH3THRESH"]
    #[inline]
    pub fn rxch3threshen(&self) -> RXCH3THRESHENR {
        RXCH3THRESHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Corresponds to the interrupt for RXCH2THRESH"]
    #[inline]
    pub fn rxch2threshen(&self) -> RXCH2THRESHENR {
        RXCH2THRESHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Corresponds to the interrupt for RXCH1THRESH"]
    #[inline]
    pub fn rxch1threshen(&self) -> RXCH1THRESHENR {
        RXCH1THRESHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Corresponds to the interrupt for RXCH0THRESH"]
    #[inline]
    pub fn rxch0threshen(&self) -> RXCH0THRESHENR {
        RXCH0THRESHENR::_from({
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
    #[doc = "Bits 8:31 - no description"]
    #[inline]
    pub fn _rsvd1(&mut self) -> __RSVD1W {
        __RSVD1W { w: self }
    }
    #[doc = "Bit 7 - Corresponds to the interrupt for RXCH7THRESH"]
    #[inline]
    pub fn rxch7threshen(&mut self) -> _RXCH7THRESHENW {
        _RXCH7THRESHENW { w: self }
    }
    #[doc = "Bit 6 - Corresponds to the interrupt for RXCH6THRESH"]
    #[inline]
    pub fn rxch6threshen(&mut self) -> _RXCH6THRESHENW {
        _RXCH6THRESHENW { w: self }
    }
    #[doc = "Bit 5 - Corresponds to the interrupt for RXCH5THRESH"]
    #[inline]
    pub fn rxch5threshen(&mut self) -> _RXCH5THRESHENW {
        _RXCH5THRESHENW { w: self }
    }
    #[doc = "Bit 4 - Corresponds to the interrupt for RXCH4THRESH"]
    #[inline]
    pub fn rxch4threshen(&mut self) -> _RXCH4THRESHENW {
        _RXCH4THRESHENW { w: self }
    }
    #[doc = "Bit 3 - Corresponds to the interrupt for RXCH3THRESH"]
    #[inline]
    pub fn rxch3threshen(&mut self) -> _RXCH3THRESHENW {
        _RXCH3THRESHENW { w: self }
    }
    #[doc = "Bit 2 - Corresponds to the interrupt for RXCH2THRESH"]
    #[inline]
    pub fn rxch2threshen(&mut self) -> _RXCH2THRESHENW {
        _RXCH2THRESHENW { w: self }
    }
    #[doc = "Bit 1 - Corresponds to the interrupt for RXCH1THRESH"]
    #[inline]
    pub fn rxch1threshen(&mut self) -> _RXCH1THRESHENW {
        _RXCH1THRESHENW { w: self }
    }
    #[doc = "Bit 0 - Corresponds to the interrupt for RXCH0THRESH"]
    #[inline]
    pub fn rxch0threshen(&mut self) -> _RXCH0THRESHENW {
        _RXCH0THRESHENW { w: self }
    }
}
