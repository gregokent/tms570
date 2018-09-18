#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::C2MISCEN {
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
#[doc = "Possible values of the field `STATPENDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATPENDENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl STATPENDENR {
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
            STATPENDENR::DISABLE => false,
            STATPENDENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATPENDENR {
        match value {
            false => STATPENDENR::DISABLE,
            true => STATPENDENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == STATPENDENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == STATPENDENR::ENABLE
    }
}
#[doc = "Possible values of the field `HOSTPENDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOSTPENDENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl HOSTPENDENR {
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
            HOSTPENDENR::DISABLE => false,
            HOSTPENDENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HOSTPENDENR {
        match value {
            false => HOSTPENDENR::DISABLE,
            true => HOSTPENDENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HOSTPENDENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == HOSTPENDENR::ENABLE
    }
}
#[doc = "Possible values of the field `LINKINT0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINKINT0ENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl LINKINT0ENR {
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
            LINKINT0ENR::DISABLE => false,
            LINKINT0ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINKINT0ENR {
        match value {
            false => LINKINT0ENR::DISABLE,
            true => LINKINT0ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == LINKINT0ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == LINKINT0ENR::ENABLE
    }
}
#[doc = "Possible values of the field `USERINT0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERINT0ENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl USERINT0ENR {
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
            USERINT0ENR::DISABLE => false,
            USERINT0ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USERINT0ENR {
        match value {
            false => USERINT0ENR::DISABLE,
            true => USERINT0ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USERINT0ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USERINT0ENR::ENABLE
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
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STATPENDEN`"]
pub enum STATPENDENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl STATPENDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STATPENDENW::DISABLE => false,
            STATPENDENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATPENDENW<'a> {
    w: &'a mut W,
}
impl<'a> _STATPENDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATPENDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(STATPENDENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(STATPENDENW::ENABLE)
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
#[doc = "Values that can be written to the field `HOSTPENDEN`"]
pub enum HOSTPENDENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl HOSTPENDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HOSTPENDENW::DISABLE => false,
            HOSTPENDENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HOSTPENDENW<'a> {
    w: &'a mut W,
}
impl<'a> _HOSTPENDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HOSTPENDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HOSTPENDENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(HOSTPENDENW::ENABLE)
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
#[doc = "Values that can be written to the field `LINKINT0EN`"]
pub enum LINKINT0ENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl LINKINT0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINKINT0ENW::DISABLE => false,
            LINKINT0ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINKINT0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LINKINT0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINKINT0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(LINKINT0ENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(LINKINT0ENW::ENABLE)
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
#[doc = "Values that can be written to the field `USERINT0EN`"]
pub enum USERINT0ENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "no description"]
    ENABLE,
}
impl USERINT0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USERINT0ENW::DISABLE => false,
            USERINT0ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USERINT0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USERINT0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USERINT0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USERINT0ENW::DISABLE)
    }
    #[doc = "no description"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USERINT0ENW::ENABLE)
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
    #[doc = "Bits 4:31 - no description"]
    #[inline]
    pub fn _rsvd1(&self) -> _RSVD1R {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        _RSVD1R { bits }
    }
    #[doc = "Bit 3 - Status Pending Interrupt"]
    #[inline]
    pub fn statpenden(&self) -> STATPENDENR {
        STATPENDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Host Pending Interrupt"]
    #[inline]
    pub fn hostpenden(&self) -> HOSTPENDENR {
        HOSTPENDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - MDIO LINKINT[0]"]
    #[inline]
    pub fn linkint0en(&self) -> LINKINT0ENR {
        LINKINT0ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - MDIO USERINT[0]"]
    #[inline]
    pub fn userint0en(&self) -> USERINT0ENR {
        USERINT0ENR::_from({
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
    #[doc = "Bits 4:31 - no description"]
    #[inline]
    pub fn _rsvd1(&mut self) -> __RSVD1W {
        __RSVD1W { w: self }
    }
    #[doc = "Bit 3 - Status Pending Interrupt"]
    #[inline]
    pub fn statpenden(&mut self) -> _STATPENDENW {
        _STATPENDENW { w: self }
    }
    #[doc = "Bit 2 - Host Pending Interrupt"]
    #[inline]
    pub fn hostpenden(&mut self) -> _HOSTPENDENW {
        _HOSTPENDENW { w: self }
    }
    #[doc = "Bit 1 - MDIO LINKINT[0]"]
    #[inline]
    pub fn linkint0en(&mut self) -> _LINKINT0ENW {
        _LINKINT0ENW { w: self }
    }
    #[doc = "Bit 0 - MDIO USERINT[0]"]
    #[inline]
    pub fn userint0en(&mut self) -> _USERINT0ENW {
        _USERINT0ENW { w: self }
    }
}
