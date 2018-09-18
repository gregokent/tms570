#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXUNICASTSET {
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
#[doc = "Possible values of the field `RXCH7EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH7ENR {
    #[doc = "no description"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXCH7ENR {
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
            RXCH7ENR::SET => true,
            RXCH7ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH7ENR {
        match value {
            true => RXCH7ENR::SET,
            i => RXCH7ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RXCH7ENR::SET
    }
}
#[doc = "Possible values of the field `RXCH6EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH6ENR {
    #[doc = "no description"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXCH6ENR {
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
            RXCH6ENR::SET => true,
            RXCH6ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH6ENR {
        match value {
            true => RXCH6ENR::SET,
            i => RXCH6ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RXCH6ENR::SET
    }
}
#[doc = "Possible values of the field `RXCH5EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH5ENR {
    #[doc = "no description"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXCH5ENR {
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
            RXCH5ENR::SET => true,
            RXCH5ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH5ENR {
        match value {
            true => RXCH5ENR::SET,
            i => RXCH5ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RXCH5ENR::SET
    }
}
#[doc = "Possible values of the field `RXCH4EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH4ENR {
    #[doc = "no description"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXCH4ENR {
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
            RXCH4ENR::SET => true,
            RXCH4ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH4ENR {
        match value {
            true => RXCH4ENR::SET,
            i => RXCH4ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RXCH4ENR::SET
    }
}
#[doc = "Possible values of the field `RXCH3EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH3ENR {
    #[doc = "no description"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXCH3ENR {
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
            RXCH3ENR::SET => true,
            RXCH3ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH3ENR {
        match value {
            true => RXCH3ENR::SET,
            i => RXCH3ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RXCH3ENR::SET
    }
}
#[doc = "Possible values of the field `RXCH2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH2ENR {
    #[doc = "no description"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXCH2ENR {
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
            RXCH2ENR::SET => true,
            RXCH2ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH2ENR {
        match value {
            true => RXCH2ENR::SET,
            i => RXCH2ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RXCH2ENR::SET
    }
}
#[doc = "Possible values of the field `RXCH1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH1ENR {
    #[doc = "no description"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXCH1ENR {
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
            RXCH1ENR::SET => true,
            RXCH1ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH1ENR {
        match value {
            true => RXCH1ENR::SET,
            i => RXCH1ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RXCH1ENR::SET
    }
}
#[doc = "Possible values of the field `RXCH0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCH0ENR {
    #[doc = "no description"]
    SET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXCH0ENR {
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
            RXCH0ENR::SET => true,
            RXCH0ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCH0ENR {
        match value {
            true => RXCH0ENR::SET,
            i => RXCH0ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RXCH0ENR::SET
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
#[doc = "Values that can be written to the field `RXCH7EN`"]
pub enum RXCH7ENW {
    #[doc = "no description"]
    SET,
}
impl RXCH7ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH7ENW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH7ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH7ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH7ENW) -> &'a mut W {
        unsafe { self.bit(variant._bits()) }
    }
    #[doc = "no description"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RXCH7ENW::SET)
    }
    #[doc = r" Sets the field bit"]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH6EN`"]
pub enum RXCH6ENW {
    #[doc = "no description"]
    SET,
}
impl RXCH6ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH6ENW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH6ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH6ENW) -> &'a mut W {
        unsafe { self.bit(variant._bits()) }
    }
    #[doc = "no description"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RXCH6ENW::SET)
    }
    #[doc = r" Sets the field bit"]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH5EN`"]
pub enum RXCH5ENW {
    #[doc = "no description"]
    SET,
}
impl RXCH5ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH5ENW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH5ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH5ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH5ENW) -> &'a mut W {
        unsafe { self.bit(variant._bits()) }
    }
    #[doc = "no description"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RXCH5ENW::SET)
    }
    #[doc = r" Sets the field bit"]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH4EN`"]
pub enum RXCH4ENW {
    #[doc = "no description"]
    SET,
}
impl RXCH4ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH4ENW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH4ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH4ENW) -> &'a mut W {
        unsafe { self.bit(variant._bits()) }
    }
    #[doc = "no description"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RXCH4ENW::SET)
    }
    #[doc = r" Sets the field bit"]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH3EN`"]
pub enum RXCH3ENW {
    #[doc = "no description"]
    SET,
}
impl RXCH3ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH3ENW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH3ENW) -> &'a mut W {
        unsafe { self.bit(variant._bits()) }
    }
    #[doc = "no description"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RXCH3ENW::SET)
    }
    #[doc = r" Sets the field bit"]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH2EN`"]
pub enum RXCH2ENW {
    #[doc = "no description"]
    SET,
}
impl RXCH2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH2ENW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH2ENW) -> &'a mut W {
        unsafe { self.bit(variant._bits()) }
    }
    #[doc = "no description"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RXCH2ENW::SET)
    }
    #[doc = r" Sets the field bit"]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH1EN`"]
pub enum RXCH1ENW {
    #[doc = "no description"]
    SET,
}
impl RXCH1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH1ENW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH1ENW) -> &'a mut W {
        unsafe { self.bit(variant._bits()) }
    }
    #[doc = "no description"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RXCH1ENW::SET)
    }
    #[doc = r" Sets the field bit"]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCH0EN`"]
pub enum RXCH0ENW {
    #[doc = "no description"]
    SET,
}
impl RXCH0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCH0ENW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCH0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCH0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCH0ENW) -> &'a mut W {
        unsafe { self.bit(variant._bits()) }
    }
    #[doc = "no description"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RXCH0ENW::SET)
    }
    #[doc = r" Sets the field bit"]
    pub unsafe fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub unsafe fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bit(self, value: bool) -> &'a mut W {
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
    #[doc = "Bits 8:31 - Reserved"]
    #[inline]
    pub fn _rsvd1(&self) -> _RSVD1R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        _RSVD1R { bits }
    }
    #[doc = "Bit 7 - RX channel 7 unicast enable set"]
    #[inline]
    pub fn rxch7en(&self) -> RXCH7ENR {
        RXCH7ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RX channel 6 unicast enable set"]
    #[inline]
    pub fn rxch6en(&self) -> RXCH6ENR {
        RXCH6ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RX channel 5 unicast enable set"]
    #[inline]
    pub fn rxch5en(&self) -> RXCH5ENR {
        RXCH5ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RX channel 4 unicast enable set"]
    #[inline]
    pub fn rxch4en(&self) -> RXCH4ENR {
        RXCH4ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RX channel 3 unicast enable set"]
    #[inline]
    pub fn rxch3en(&self) -> RXCH3ENR {
        RXCH3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RX channel 2 unicast enable set"]
    #[inline]
    pub fn rxch2en(&self) -> RXCH2ENR {
        RXCH2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RX channel 1 unicast enable set"]
    #[inline]
    pub fn rxch1en(&self) -> RXCH1ENR {
        RXCH1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - RX channel 0 unicast enable set"]
    #[inline]
    pub fn rxch0en(&self) -> RXCH0ENR {
        RXCH0ENR::_from({
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
    #[doc = "Bits 8:31 - Reserved"]
    #[inline]
    pub fn _rsvd1(&mut self) -> __RSVD1W {
        __RSVD1W { w: self }
    }
    #[doc = "Bit 7 - RX channel 7 unicast enable set"]
    #[inline]
    pub fn rxch7en(&mut self) -> _RXCH7ENW {
        _RXCH7ENW { w: self }
    }
    #[doc = "Bit 6 - RX channel 6 unicast enable set"]
    #[inline]
    pub fn rxch6en(&mut self) -> _RXCH6ENW {
        _RXCH6ENW { w: self }
    }
    #[doc = "Bit 5 - RX channel 5 unicast enable set"]
    #[inline]
    pub fn rxch5en(&mut self) -> _RXCH5ENW {
        _RXCH5ENW { w: self }
    }
    #[doc = "Bit 4 - RX channel 4 unicast enable set"]
    #[inline]
    pub fn rxch4en(&mut self) -> _RXCH4ENW {
        _RXCH4ENW { w: self }
    }
    #[doc = "Bit 3 - RX channel 3 unicast enable set"]
    #[inline]
    pub fn rxch3en(&mut self) -> _RXCH3ENW {
        _RXCH3ENW { w: self }
    }
    #[doc = "Bit 2 - RX channel 2 unicast enable set"]
    #[inline]
    pub fn rxch2en(&mut self) -> _RXCH2ENW {
        _RXCH2ENW { w: self }
    }
    #[doc = "Bit 1 - RX channel 1 unicast enable set"]
    #[inline]
    pub fn rxch1en(&mut self) -> _RXCH1ENW {
        _RXCH1ENW { w: self }
    }
    #[doc = "Bit 0 - RX channel 0 unicast enable set"]
    #[inline]
    pub fn rxch0en(&mut self) -> _RXCH0ENW {
        _RXCH0ENW { w: self }
    }
}
