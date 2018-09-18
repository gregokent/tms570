#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTMUX12 {
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
pub struct INTMUX32R {
    bits: bool,
}
impl INTMUX32R {
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
pub struct INTMUX31R {
    bits: bool,
}
impl INTMUX31R {
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
pub struct INTMUX30R {
    bits: bool,
}
impl INTMUX30R {
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
pub struct INTMUX29R {
    bits: bool,
}
impl INTMUX29R {
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
pub struct INTMUX28R {
    bits: bool,
}
impl INTMUX28R {
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
pub struct INTMUX27R {
    bits: bool,
}
impl INTMUX27R {
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
pub struct INTMUX26R {
    bits: bool,
}
impl INTMUX26R {
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
pub struct INTMUX25R {
    bits: bool,
}
impl INTMUX25R {
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
pub struct INTMUX24R {
    bits: bool,
}
impl INTMUX24R {
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
pub struct INTMUX23R {
    bits: bool,
}
impl INTMUX23R {
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
pub struct INTMUX22R {
    bits: bool,
}
impl INTMUX22R {
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
pub struct INTMUX21R {
    bits: bool,
}
impl INTMUX21R {
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
pub struct INTMUX20R {
    bits: bool,
}
impl INTMUX20R {
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
pub struct INTMUX19R {
    bits: bool,
}
impl INTMUX19R {
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
pub struct INTMUX18R {
    bits: bool,
}
impl INTMUX18R {
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
pub struct INTMUX17R {
    bits: bool,
}
impl INTMUX17R {
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
pub struct INTMUX16R {
    bits: bool,
}
impl INTMUX16R {
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
pub struct INTMUX15R {
    bits: bool,
}
impl INTMUX15R {
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
pub struct INTMUX14R {
    bits: bool,
}
impl INTMUX14R {
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
pub struct INTMUX13R {
    bits: bool,
}
impl INTMUX13R {
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
pub struct INTMUX12R {
    bits: bool,
}
impl INTMUX12R {
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
pub struct INTMUX11R {
    bits: bool,
}
impl INTMUX11R {
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
pub struct INTMUX10R {
    bits: bool,
}
impl INTMUX10R {
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
pub struct INTMUX9R {
    bits: bool,
}
impl INTMUX9R {
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
pub struct INTMUX8R {
    bits: bool,
}
impl INTMUX8R {
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
pub struct INTMUX7R {
    bits: bool,
}
impl INTMUX7R {
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
pub struct INTMUX6R {
    bits: bool,
}
impl INTMUX6R {
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
pub struct INTMUX5R {
    bits: bool,
}
impl INTMUX5R {
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
pub struct INTMUX4R {
    bits: bool,
}
impl INTMUX4R {
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
pub struct INTMUX3R {
    bits: bool,
}
impl INTMUX3R {
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
pub struct INTMUX2R {
    bits: bool,
}
impl INTMUX2R {
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
pub struct INTMUX1R {
    bits: bool,
}
impl INTMUX1R {
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
pub struct _INTMUX32W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX32W<'a> {
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
pub struct _INTMUX31W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX31W<'a> {
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
pub struct _INTMUX30W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX30W<'a> {
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
pub struct _INTMUX29W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX29W<'a> {
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
pub struct _INTMUX28W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX28W<'a> {
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
pub struct _INTMUX27W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX27W<'a> {
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
pub struct _INTMUX26W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX26W<'a> {
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
pub struct _INTMUX25W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX25W<'a> {
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
pub struct _INTMUX24W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX24W<'a> {
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
pub struct _INTMUX23W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX23W<'a> {
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
pub struct _INTMUX22W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX22W<'a> {
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
pub struct _INTMUX21W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX21W<'a> {
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
pub struct _INTMUX20W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX20W<'a> {
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
pub struct _INTMUX19W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX19W<'a> {
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
pub struct _INTMUX18W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX18W<'a> {
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
pub struct _INTMUX17W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX17W<'a> {
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
pub struct _INTMUX16W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX16W<'a> {
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
pub struct _INTMUX15W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX15W<'a> {
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
pub struct _INTMUX14W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX14W<'a> {
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
pub struct _INTMUX13W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX13W<'a> {
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
pub struct _INTMUX12W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX12W<'a> {
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
pub struct _INTMUX11W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX11W<'a> {
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
pub struct _INTMUX10W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX10W<'a> {
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
pub struct _INTMUX9W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX9W<'a> {
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
pub struct _INTMUX8W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX8W<'a> {
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
pub struct _INTMUX7W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX7W<'a> {
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
pub struct _INTMUX6W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX6W<'a> {
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
pub struct _INTMUX5W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX5W<'a> {
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
pub struct _INTMUX4W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX4W<'a> {
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
pub struct _INTMUX3W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX3W<'a> {
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
pub struct _INTMUX2W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX2W<'a> {
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
pub struct _INTMUX1W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX1W<'a> {
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
    #[doc = "Bit 31 - Message Object 32 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux32(&self) -> INTMUX32R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX32R { bits }
    }
    #[doc = "Bit 30 - Message Object 31 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux31(&self) -> INTMUX31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX31R { bits }
    }
    #[doc = "Bit 29 - Message Object 30 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux30(&self) -> INTMUX30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX30R { bits }
    }
    #[doc = "Bit 28 - Message Object 29 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux29(&self) -> INTMUX29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX29R { bits }
    }
    #[doc = "Bit 27 - Message Object 28 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux28(&self) -> INTMUX28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX28R { bits }
    }
    #[doc = "Bit 26 - Message Object 27 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux27(&self) -> INTMUX27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX27R { bits }
    }
    #[doc = "Bit 25 - Message Object 26 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux26(&self) -> INTMUX26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX26R { bits }
    }
    #[doc = "Bit 24 - Message Object 25 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux25(&self) -> INTMUX25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX25R { bits }
    }
    #[doc = "Bit 23 - Message Object 24 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux24(&self) -> INTMUX24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX24R { bits }
    }
    #[doc = "Bit 22 - Message Object 23 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux23(&self) -> INTMUX23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX23R { bits }
    }
    #[doc = "Bit 21 - Message Object 22 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux22(&self) -> INTMUX22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX22R { bits }
    }
    #[doc = "Bit 20 - Message Object 21 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux21(&self) -> INTMUX21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX21R { bits }
    }
    #[doc = "Bit 19 - Message Object 20 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux20(&self) -> INTMUX20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX20R { bits }
    }
    #[doc = "Bit 18 - Message Object 19 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux19(&self) -> INTMUX19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX19R { bits }
    }
    #[doc = "Bit 17 - Message Object 18 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux18(&self) -> INTMUX18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX18R { bits }
    }
    #[doc = "Bit 16 - Message Object 17 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux17(&self) -> INTMUX17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX17R { bits }
    }
    #[doc = "Bit 15 - Message Object 16 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux16(&self) -> INTMUX16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX16R { bits }
    }
    #[doc = "Bit 14 - Message Object 15 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux15(&self) -> INTMUX15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX15R { bits }
    }
    #[doc = "Bit 13 - Message Object 14 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux14(&self) -> INTMUX14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX14R { bits }
    }
    #[doc = "Bit 12 - Message Object 13 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux13(&self) -> INTMUX13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX13R { bits }
    }
    #[doc = "Bit 11 - Message Object 12 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux12(&self) -> INTMUX12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX12R { bits }
    }
    #[doc = "Bit 10 - Message Object 11 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux11(&self) -> INTMUX11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX11R { bits }
    }
    #[doc = "Bit 9 - Message Object 10 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux10(&self) -> INTMUX10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX10R { bits }
    }
    #[doc = "Bit 8 - Message Object 9 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux9(&self) -> INTMUX9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX9R { bits }
    }
    #[doc = "Bit 7 - Message Object 8 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux8(&self) -> INTMUX8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX8R { bits }
    }
    #[doc = "Bit 6 - Message Object 7 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux7(&self) -> INTMUX7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX7R { bits }
    }
    #[doc = "Bit 5 - Message Object 6 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux6(&self) -> INTMUX6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX6R { bits }
    }
    #[doc = "Bit 4 - Message Object 5 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux5(&self) -> INTMUX5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX5R { bits }
    }
    #[doc = "Bit 3 - Message Object 4 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux4(&self) -> INTMUX4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX4R { bits }
    }
    #[doc = "Bit 2 - Message Object 3 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux3(&self) -> INTMUX3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX3R { bits }
    }
    #[doc = "Bit 1 - Message Object 2 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux2(&self) -> INTMUX2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX2R { bits }
    }
    #[doc = "Bit 0 - Message Object 1 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux1(&self) -> INTMUX1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX1R { bits }
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
    #[doc = "Bit 31 - Message Object 32 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux32(&mut self) -> _INTMUX32W {
        _INTMUX32W { w: self }
    }
    #[doc = "Bit 30 - Message Object 31 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux31(&mut self) -> _INTMUX31W {
        _INTMUX31W { w: self }
    }
    #[doc = "Bit 29 - Message Object 30 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux30(&mut self) -> _INTMUX30W {
        _INTMUX30W { w: self }
    }
    #[doc = "Bit 28 - Message Object 29 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux29(&mut self) -> _INTMUX29W {
        _INTMUX29W { w: self }
    }
    #[doc = "Bit 27 - Message Object 28 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux28(&mut self) -> _INTMUX28W {
        _INTMUX28W { w: self }
    }
    #[doc = "Bit 26 - Message Object 27 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux27(&mut self) -> _INTMUX27W {
        _INTMUX27W { w: self }
    }
    #[doc = "Bit 25 - Message Object 26 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux26(&mut self) -> _INTMUX26W {
        _INTMUX26W { w: self }
    }
    #[doc = "Bit 24 - Message Object 25 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux25(&mut self) -> _INTMUX25W {
        _INTMUX25W { w: self }
    }
    #[doc = "Bit 23 - Message Object 24 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux24(&mut self) -> _INTMUX24W {
        _INTMUX24W { w: self }
    }
    #[doc = "Bit 22 - Message Object 23 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux23(&mut self) -> _INTMUX23W {
        _INTMUX23W { w: self }
    }
    #[doc = "Bit 21 - Message Object 22 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux22(&mut self) -> _INTMUX22W {
        _INTMUX22W { w: self }
    }
    #[doc = "Bit 20 - Message Object 21 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux21(&mut self) -> _INTMUX21W {
        _INTMUX21W { w: self }
    }
    #[doc = "Bit 19 - Message Object 20 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux20(&mut self) -> _INTMUX20W {
        _INTMUX20W { w: self }
    }
    #[doc = "Bit 18 - Message Object 19 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux19(&mut self) -> _INTMUX19W {
        _INTMUX19W { w: self }
    }
    #[doc = "Bit 17 - Message Object 18 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux18(&mut self) -> _INTMUX18W {
        _INTMUX18W { w: self }
    }
    #[doc = "Bit 16 - Message Object 17 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux17(&mut self) -> _INTMUX17W {
        _INTMUX17W { w: self }
    }
    #[doc = "Bit 15 - Message Object 16 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux16(&mut self) -> _INTMUX16W {
        _INTMUX16W { w: self }
    }
    #[doc = "Bit 14 - Message Object 15 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux15(&mut self) -> _INTMUX15W {
        _INTMUX15W { w: self }
    }
    #[doc = "Bit 13 - Message Object 14 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux14(&mut self) -> _INTMUX14W {
        _INTMUX14W { w: self }
    }
    #[doc = "Bit 12 - Message Object 13 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux13(&mut self) -> _INTMUX13W {
        _INTMUX13W { w: self }
    }
    #[doc = "Bit 11 - Message Object 12 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux12(&mut self) -> _INTMUX12W {
        _INTMUX12W { w: self }
    }
    #[doc = "Bit 10 - Message Object 11 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux11(&mut self) -> _INTMUX11W {
        _INTMUX11W { w: self }
    }
    #[doc = "Bit 9 - Message Object 10 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux10(&mut self) -> _INTMUX10W {
        _INTMUX10W { w: self }
    }
    #[doc = "Bit 8 - Message Object 9 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux9(&mut self) -> _INTMUX9W {
        _INTMUX9W { w: self }
    }
    #[doc = "Bit 7 - Message Object 8 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux8(&mut self) -> _INTMUX8W {
        _INTMUX8W { w: self }
    }
    #[doc = "Bit 6 - Message Object 7 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux7(&mut self) -> _INTMUX7W {
        _INTMUX7W { w: self }
    }
    #[doc = "Bit 5 - Message Object 6 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux6(&mut self) -> _INTMUX6W {
        _INTMUX6W { w: self }
    }
    #[doc = "Bit 4 - Message Object 5 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux5(&mut self) -> _INTMUX5W {
        _INTMUX5W { w: self }
    }
    #[doc = "Bit 3 - Message Object 4 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux4(&mut self) -> _INTMUX4W {
        _INTMUX4W { w: self }
    }
    #[doc = "Bit 2 - Message Object 3 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux3(&mut self) -> _INTMUX3W {
        _INTMUX3W { w: self }
    }
    #[doc = "Bit 1 - Message Object 2 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux2(&mut self) -> _INTMUX2W {
        _INTMUX2W { w: self }
    }
    #[doc = "Bit 0 - Message Object 1 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux1(&mut self) -> _INTMUX1W {
        _INTMUX1W { w: self }
    }
}
