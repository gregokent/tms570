#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTMUX78 {
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
pub struct INTMUX128R {
    bits: bool,
}
impl INTMUX128R {
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
pub struct INTMUX127R {
    bits: bool,
}
impl INTMUX127R {
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
pub struct INTMUX126R {
    bits: bool,
}
impl INTMUX126R {
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
pub struct INTMUX125R {
    bits: bool,
}
impl INTMUX125R {
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
pub struct INTMUX124R {
    bits: bool,
}
impl INTMUX124R {
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
pub struct INTMUX123R {
    bits: bool,
}
impl INTMUX123R {
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
pub struct INTMUX122R {
    bits: bool,
}
impl INTMUX122R {
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
pub struct INTMUX121R {
    bits: bool,
}
impl INTMUX121R {
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
pub struct INTMUX120R {
    bits: bool,
}
impl INTMUX120R {
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
pub struct INTMUX119R {
    bits: bool,
}
impl INTMUX119R {
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
pub struct INTMUX118R {
    bits: bool,
}
impl INTMUX118R {
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
pub struct INTMUX117R {
    bits: bool,
}
impl INTMUX117R {
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
pub struct INTMUX116R {
    bits: bool,
}
impl INTMUX116R {
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
pub struct INTMUX115R {
    bits: bool,
}
impl INTMUX115R {
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
pub struct INTMUX114R {
    bits: bool,
}
impl INTMUX114R {
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
pub struct INTMUX113R {
    bits: bool,
}
impl INTMUX113R {
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
pub struct INTMUX112R {
    bits: bool,
}
impl INTMUX112R {
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
pub struct INTMUX111R {
    bits: bool,
}
impl INTMUX111R {
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
pub struct INTMUX110R {
    bits: bool,
}
impl INTMUX110R {
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
pub struct INTMUX109R {
    bits: bool,
}
impl INTMUX109R {
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
pub struct INTMUX108R {
    bits: bool,
}
impl INTMUX108R {
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
pub struct INTMUX107R {
    bits: bool,
}
impl INTMUX107R {
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
pub struct INTMUX106R {
    bits: bool,
}
impl INTMUX106R {
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
pub struct INTMUX105R {
    bits: bool,
}
impl INTMUX105R {
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
pub struct INTMUX104R {
    bits: bool,
}
impl INTMUX104R {
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
pub struct INTMUX103R {
    bits: bool,
}
impl INTMUX103R {
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
pub struct INTMUX102R {
    bits: bool,
}
impl INTMUX102R {
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
pub struct INTMUX101R {
    bits: bool,
}
impl INTMUX101R {
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
pub struct INTMUX100R {
    bits: bool,
}
impl INTMUX100R {
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
pub struct INTMUX99R {
    bits: bool,
}
impl INTMUX99R {
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
pub struct INTMUX98R {
    bits: bool,
}
impl INTMUX98R {
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
pub struct INTMUX97R {
    bits: bool,
}
impl INTMUX97R {
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
#[doc = r" Proxy"]
pub struct _INTMUX128W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX128W<'a> {
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
pub struct _INTMUX127W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX127W<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX126W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX126W<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX125W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX125W<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX124W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX124W<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX123W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX123W<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX122W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX122W<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX121W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX121W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX120W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX120W<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX119W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX119W<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX118W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX118W<'a> {
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
#[doc = r" Proxy"]
pub struct _INTMUX117W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX117W<'a> {
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
#[doc = r" Proxy"]
pub struct _INTMUX116W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX116W<'a> {
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
pub struct _INTMUX115W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX115W<'a> {
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
#[doc = r" Proxy"]
pub struct _INTMUX114W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX114W<'a> {
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
#[doc = r" Proxy"]
pub struct _INTMUX113W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX113W<'a> {
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
pub struct _INTMUX112W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX112W<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX111W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX111W<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX110W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX110W<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX109W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX109W<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX108W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX108W<'a> {
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
pub struct _INTMUX107W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX107W<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX106W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX106W<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX105W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX105W<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTMUX104W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX104W<'a> {
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
#[doc = r" Proxy"]
pub struct _INTMUX103W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX103W<'a> {
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
#[doc = r" Proxy"]
pub struct _INTMUX102W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX102W<'a> {
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
#[doc = r" Proxy"]
pub struct _INTMUX101W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX101W<'a> {
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
pub struct _INTMUX100W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX100W<'a> {
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
#[doc = r" Proxy"]
pub struct _INTMUX99W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX99W<'a> {
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
#[doc = r" Proxy"]
pub struct _INTMUX98W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX98W<'a> {
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
#[doc = r" Proxy"]
pub struct _INTMUX97W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX97W<'a> {
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
    #[doc = "Bit 31 - Message Object 128 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux128(&self) -> INTMUX128R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX128R { bits }
    }
    #[doc = "Bit 30 - Message Object 127 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux127(&self) -> INTMUX127R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX127R { bits }
    }
    #[doc = "Bit 29 - Message Object 126 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux126(&self) -> INTMUX126R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX126R { bits }
    }
    #[doc = "Bit 28 - Message Object 125 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux125(&self) -> INTMUX125R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX125R { bits }
    }
    #[doc = "Bit 27 - Message Object 124 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux124(&self) -> INTMUX124R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX124R { bits }
    }
    #[doc = "Bit 26 - Message Object 123 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux123(&self) -> INTMUX123R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX123R { bits }
    }
    #[doc = "Bit 25 - Message Object 122 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux122(&self) -> INTMUX122R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX122R { bits }
    }
    #[doc = "Bit 24 - Message Object 121 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux121(&self) -> INTMUX121R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX121R { bits }
    }
    #[doc = "Bit 23 - Message Object 120 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux120(&self) -> INTMUX120R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX120R { bits }
    }
    #[doc = "Bit 22 - Message Object 119 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux119(&self) -> INTMUX119R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX119R { bits }
    }
    #[doc = "Bit 21 - Message Object 118 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux118(&self) -> INTMUX118R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX118R { bits }
    }
    #[doc = "Bit 20 - Message Object 117 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux117(&self) -> INTMUX117R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX117R { bits }
    }
    #[doc = "Bit 19 - Message Object 116 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux116(&self) -> INTMUX116R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX116R { bits }
    }
    #[doc = "Bit 18 - Message Object 115 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux115(&self) -> INTMUX115R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX115R { bits }
    }
    #[doc = "Bit 17 - Message Object 114 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux114(&self) -> INTMUX114R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX114R { bits }
    }
    #[doc = "Bit 16 - Message Object 112 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux113(&self) -> INTMUX113R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX113R { bits }
    }
    #[doc = "Bit 15 - Message Object 112 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux112(&self) -> INTMUX112R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX112R { bits }
    }
    #[doc = "Bit 14 - Message Object 111 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux111(&self) -> INTMUX111R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX111R { bits }
    }
    #[doc = "Bit 13 - Message Object 110 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux110(&self) -> INTMUX110R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX110R { bits }
    }
    #[doc = "Bit 12 - Message Object 109 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux109(&self) -> INTMUX109R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX109R { bits }
    }
    #[doc = "Bit 11 - Message Object 108 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux108(&self) -> INTMUX108R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX108R { bits }
    }
    #[doc = "Bit 10 - Message Object 107 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux107(&self) -> INTMUX107R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX107R { bits }
    }
    #[doc = "Bit 9 - Message Object 106 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux106(&self) -> INTMUX106R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX106R { bits }
    }
    #[doc = "Bit 8 - Message Object 105 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux105(&self) -> INTMUX105R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX105R { bits }
    }
    #[doc = "Bit 7 - Message Object 104 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux104(&self) -> INTMUX104R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX104R { bits }
    }
    #[doc = "Bit 6 - Message Object 103 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux103(&self) -> INTMUX103R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX103R { bits }
    }
    #[doc = "Bit 5 - Message Object 102 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux102(&self) -> INTMUX102R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX102R { bits }
    }
    #[doc = "Bit 4 - Message Object 101 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux101(&self) -> INTMUX101R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX101R { bits }
    }
    #[doc = "Bit 3 - Message Object 100 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux100(&self) -> INTMUX100R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX100R { bits }
    }
    #[doc = "Bit 2 - Message Object 99 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux99(&self) -> INTMUX99R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX99R { bits }
    }
    #[doc = "Bit 1 - Message Object 98 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux98(&self) -> INTMUX98R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX98R { bits }
    }
    #[doc = "Bit 0 - Message Object 97 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux97(&self) -> INTMUX97R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX97R { bits }
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
    #[doc = "Bit 31 - Message Object 128 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux128(&mut self) -> _INTMUX128W {
        _INTMUX128W { w: self }
    }
    #[doc = "Bit 30 - Message Object 127 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux127(&mut self) -> _INTMUX127W {
        _INTMUX127W { w: self }
    }
    #[doc = "Bit 29 - Message Object 126 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux126(&mut self) -> _INTMUX126W {
        _INTMUX126W { w: self }
    }
    #[doc = "Bit 28 - Message Object 125 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux125(&mut self) -> _INTMUX125W {
        _INTMUX125W { w: self }
    }
    #[doc = "Bit 27 - Message Object 124 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux124(&mut self) -> _INTMUX124W {
        _INTMUX124W { w: self }
    }
    #[doc = "Bit 26 - Message Object 123 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux123(&mut self) -> _INTMUX123W {
        _INTMUX123W { w: self }
    }
    #[doc = "Bit 25 - Message Object 122 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux122(&mut self) -> _INTMUX122W {
        _INTMUX122W { w: self }
    }
    #[doc = "Bit 24 - Message Object 121 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux121(&mut self) -> _INTMUX121W {
        _INTMUX121W { w: self }
    }
    #[doc = "Bit 23 - Message Object 120 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux120(&mut self) -> _INTMUX120W {
        _INTMUX120W { w: self }
    }
    #[doc = "Bit 22 - Message Object 119 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux119(&mut self) -> _INTMUX119W {
        _INTMUX119W { w: self }
    }
    #[doc = "Bit 21 - Message Object 118 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux118(&mut self) -> _INTMUX118W {
        _INTMUX118W { w: self }
    }
    #[doc = "Bit 20 - Message Object 117 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux117(&mut self) -> _INTMUX117W {
        _INTMUX117W { w: self }
    }
    #[doc = "Bit 19 - Message Object 116 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux116(&mut self) -> _INTMUX116W {
        _INTMUX116W { w: self }
    }
    #[doc = "Bit 18 - Message Object 115 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux115(&mut self) -> _INTMUX115W {
        _INTMUX115W { w: self }
    }
    #[doc = "Bit 17 - Message Object 114 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux114(&mut self) -> _INTMUX114W {
        _INTMUX114W { w: self }
    }
    #[doc = "Bit 16 - Message Object 112 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux113(&mut self) -> _INTMUX113W {
        _INTMUX113W { w: self }
    }
    #[doc = "Bit 15 - Message Object 112 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux112(&mut self) -> _INTMUX112W {
        _INTMUX112W { w: self }
    }
    #[doc = "Bit 14 - Message Object 111 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux111(&mut self) -> _INTMUX111W {
        _INTMUX111W { w: self }
    }
    #[doc = "Bit 13 - Message Object 110 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux110(&mut self) -> _INTMUX110W {
        _INTMUX110W { w: self }
    }
    #[doc = "Bit 12 - Message Object 109 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux109(&mut self) -> _INTMUX109W {
        _INTMUX109W { w: self }
    }
    #[doc = "Bit 11 - Message Object 108 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux108(&mut self) -> _INTMUX108W {
        _INTMUX108W { w: self }
    }
    #[doc = "Bit 10 - Message Object 107 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux107(&mut self) -> _INTMUX107W {
        _INTMUX107W { w: self }
    }
    #[doc = "Bit 9 - Message Object 106 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux106(&mut self) -> _INTMUX106W {
        _INTMUX106W { w: self }
    }
    #[doc = "Bit 8 - Message Object 105 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux105(&mut self) -> _INTMUX105W {
        _INTMUX105W { w: self }
    }
    #[doc = "Bit 7 - Message Object 104 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux104(&mut self) -> _INTMUX104W {
        _INTMUX104W { w: self }
    }
    #[doc = "Bit 6 - Message Object 103 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux103(&mut self) -> _INTMUX103W {
        _INTMUX103W { w: self }
    }
    #[doc = "Bit 5 - Message Object 102 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux102(&mut self) -> _INTMUX102W {
        _INTMUX102W { w: self }
    }
    #[doc = "Bit 4 - Message Object 101 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux101(&mut self) -> _INTMUX101W {
        _INTMUX101W { w: self }
    }
    #[doc = "Bit 3 - Message Object 100 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux100(&mut self) -> _INTMUX100W {
        _INTMUX100W { w: self }
    }
    #[doc = "Bit 2 - Message Object 99 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux99(&mut self) -> _INTMUX99W {
        _INTMUX99W { w: self }
    }
    #[doc = "Bit 1 - Message Object 98 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux98(&mut self) -> _INTMUX98W {
        _INTMUX98W { w: self }
    }
    #[doc = "Bit 0 - Message Object 97 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux97(&mut self) -> _INTMUX97W {
        _INTMUX97W { w: self }
    }
}
