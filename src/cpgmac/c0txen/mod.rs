#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::C0TXEN {
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
#[doc = "Possible values of the field `TXCH7EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCH7ENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH7ENR {
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
            TXCH7ENR::DISABLE => false,
            TXCH7ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCH7ENR {
        match value {
            false => TXCH7ENR::DISABLE,
            true => TXCH7ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXCH7ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXCH7ENR::ENABLE
    }
}
#[doc = "Possible values of the field `TXCH6EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCH6ENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH6ENR {
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
            TXCH6ENR::DISABLE => false,
            TXCH6ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCH6ENR {
        match value {
            false => TXCH6ENR::DISABLE,
            true => TXCH6ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXCH6ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXCH6ENR::ENABLE
    }
}
#[doc = "Possible values of the field `TXCH5EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCH5ENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH5ENR {
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
            TXCH5ENR::DISABLE => false,
            TXCH5ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCH5ENR {
        match value {
            false => TXCH5ENR::DISABLE,
            true => TXCH5ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXCH5ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXCH5ENR::ENABLE
    }
}
#[doc = "Possible values of the field `TXCH4EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCH4ENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH4ENR {
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
            TXCH4ENR::DISABLE => false,
            TXCH4ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCH4ENR {
        match value {
            false => TXCH4ENR::DISABLE,
            true => TXCH4ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXCH4ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXCH4ENR::ENABLE
    }
}
#[doc = "Possible values of the field `TXCH3EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCH3ENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH3ENR {
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
            TXCH3ENR::DISABLE => false,
            TXCH3ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCH3ENR {
        match value {
            false => TXCH3ENR::DISABLE,
            true => TXCH3ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXCH3ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXCH3ENR::ENABLE
    }
}
#[doc = "Possible values of the field `TXCH2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCH2ENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH2ENR {
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
            TXCH2ENR::DISABLE => false,
            TXCH2ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCH2ENR {
        match value {
            false => TXCH2ENR::DISABLE,
            true => TXCH2ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXCH2ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXCH2ENR::ENABLE
    }
}
#[doc = "Possible values of the field `TXCH1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCH1ENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH1ENR {
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
            TXCH1ENR::DISABLE => false,
            TXCH1ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCH1ENR {
        match value {
            false => TXCH1ENR::DISABLE,
            true => TXCH1ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXCH1ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXCH1ENR::ENABLE
    }
}
#[doc = "Possible values of the field `TXCH0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCH0ENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH0ENR {
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
            TXCH0ENR::DISABLE => false,
            TXCH0ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCH0ENR {
        match value {
            false => TXCH0ENR::DISABLE,
            true => TXCH0ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXCH0ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXCH0ENR::ENABLE
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
#[doc = "Values that can be written to the field `TXCH7EN`"]
pub enum TXCH7ENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH7ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCH7ENW::DISABLE => false,
            TXCH7ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCH7ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCH7ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCH7ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXCH7ENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXCH7ENW::ENABLE)
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
#[doc = "Values that can be written to the field `TXCH6EN`"]
pub enum TXCH6ENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH6ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCH6ENW::DISABLE => false,
            TXCH6ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCH6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCH6ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCH6ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXCH6ENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXCH6ENW::ENABLE)
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
#[doc = "Values that can be written to the field `TXCH5EN`"]
pub enum TXCH5ENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH5ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCH5ENW::DISABLE => false,
            TXCH5ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCH5ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCH5ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCH5ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXCH5ENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXCH5ENW::ENABLE)
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
#[doc = "Values that can be written to the field `TXCH4EN`"]
pub enum TXCH4ENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH4ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCH4ENW::DISABLE => false,
            TXCH4ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCH4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCH4ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCH4ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXCH4ENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXCH4ENW::ENABLE)
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
#[doc = "Values that can be written to the field `TXCH3EN`"]
pub enum TXCH3ENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH3ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCH3ENW::DISABLE => false,
            TXCH3ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCH3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCH3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCH3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXCH3ENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXCH3ENW::ENABLE)
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
#[doc = "Values that can be written to the field `TXCH2EN`"]
pub enum TXCH2ENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCH2ENW::DISABLE => false,
            TXCH2ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCH2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCH2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCH2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXCH2ENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXCH2ENW::ENABLE)
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
#[doc = "Values that can be written to the field `TXCH1EN`"]
pub enum TXCH1ENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCH1ENW::DISABLE => false,
            TXCH1ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCH1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCH1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCH1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXCH1ENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXCH1ENW::ENABLE)
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
#[doc = "Values that can be written to the field `TXCH0EN`"]
pub enum TXCH0ENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl TXCH0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCH0ENW::DISABLE => false,
            TXCH0ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCH0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCH0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCH0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXCH0ENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXCH0ENW::ENABLE)
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
    #[doc = "Bit 7 - Corresponds to the interrupt for TXCH7EN"]
    #[inline]
    pub fn txch7en(&self) -> TXCH7ENR {
        TXCH7ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Corresponds to the interrupt for TXCH6EN"]
    #[inline]
    pub fn txch6en(&self) -> TXCH6ENR {
        TXCH6ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Corresponds to the interrupt for TXCH5EN"]
    #[inline]
    pub fn txch5en(&self) -> TXCH5ENR {
        TXCH5ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Corresponds to the interrupt for TXCH4EN"]
    #[inline]
    pub fn txch4en(&self) -> TXCH4ENR {
        TXCH4ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Corresponds to the interrupt for TXCH3EN"]
    #[inline]
    pub fn txch3en(&self) -> TXCH3ENR {
        TXCH3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Corresponds to the interrupt for TXCH2EN"]
    #[inline]
    pub fn txch2en(&self) -> TXCH2ENR {
        TXCH2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Corresponds to the interrupt for TXCH1EN"]
    #[inline]
    pub fn txch1en(&self) -> TXCH1ENR {
        TXCH1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Corresponds to the interrupt for TXCH0EN"]
    #[inline]
    pub fn txch0en(&self) -> TXCH0ENR {
        TXCH0ENR::_from({
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
    #[doc = "Bit 7 - Corresponds to the interrupt for TXCH7EN"]
    #[inline]
    pub fn txch7en(&mut self) -> _TXCH7ENW {
        _TXCH7ENW { w: self }
    }
    #[doc = "Bit 6 - Corresponds to the interrupt for TXCH6EN"]
    #[inline]
    pub fn txch6en(&mut self) -> _TXCH6ENW {
        _TXCH6ENW { w: self }
    }
    #[doc = "Bit 5 - Corresponds to the interrupt for TXCH5EN"]
    #[inline]
    pub fn txch5en(&mut self) -> _TXCH5ENW {
        _TXCH5ENW { w: self }
    }
    #[doc = "Bit 4 - Corresponds to the interrupt for TXCH4EN"]
    #[inline]
    pub fn txch4en(&mut self) -> _TXCH4ENW {
        _TXCH4ENW { w: self }
    }
    #[doc = "Bit 3 - Corresponds to the interrupt for TXCH3EN"]
    #[inline]
    pub fn txch3en(&mut self) -> _TXCH3ENW {
        _TXCH3ENW { w: self }
    }
    #[doc = "Bit 2 - Corresponds to the interrupt for TXCH2EN"]
    #[inline]
    pub fn txch2en(&mut self) -> _TXCH2ENW {
        _TXCH2ENW { w: self }
    }
    #[doc = "Bit 1 - Corresponds to the interrupt for TXCH1EN"]
    #[inline]
    pub fn txch1en(&mut self) -> _TXCH1ENW {
        _TXCH1ENW { w: self }
    }
    #[doc = "Bit 0 - Corresponds to the interrupt for TXCH0EN"]
    #[inline]
    pub fn txch0en(&mut self) -> _TXCH0ENW {
        _TXCH0ENW { w: self }
    }
}
