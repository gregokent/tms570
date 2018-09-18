#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTMUX56 {
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
pub struct INTMUX96R {
    bits: bool,
}
impl INTMUX96R {
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
pub struct INTMUX95R {
    bits: bool,
}
impl INTMUX95R {
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
pub struct INTMUX94R {
    bits: bool,
}
impl INTMUX94R {
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
pub struct INTMUX93R {
    bits: bool,
}
impl INTMUX93R {
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
pub struct INTMUX92R {
    bits: bool,
}
impl INTMUX92R {
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
pub struct INTMUX91R {
    bits: bool,
}
impl INTMUX91R {
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
pub struct INTMUX90R {
    bits: bool,
}
impl INTMUX90R {
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
pub struct INTMUX89R {
    bits: bool,
}
impl INTMUX89R {
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
pub struct INTMUX88R {
    bits: bool,
}
impl INTMUX88R {
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
pub struct INTMUX87R {
    bits: bool,
}
impl INTMUX87R {
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
pub struct INTMUX86R {
    bits: bool,
}
impl INTMUX86R {
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
pub struct INTMUX85R {
    bits: bool,
}
impl INTMUX85R {
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
pub struct INTMUX84R {
    bits: bool,
}
impl INTMUX84R {
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
pub struct INTMUX83R {
    bits: bool,
}
impl INTMUX83R {
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
pub struct INTMUX82R {
    bits: bool,
}
impl INTMUX82R {
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
pub struct INTMUX81R {
    bits: bool,
}
impl INTMUX81R {
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
pub struct INTMUX80R {
    bits: bool,
}
impl INTMUX80R {
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
pub struct INTMUX79R {
    bits: bool,
}
impl INTMUX79R {
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
pub struct INTMUX78R {
    bits: bool,
}
impl INTMUX78R {
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
pub struct INTMUX77R {
    bits: bool,
}
impl INTMUX77R {
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
pub struct INTMUX76R {
    bits: bool,
}
impl INTMUX76R {
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
pub struct INTMUX75R {
    bits: bool,
}
impl INTMUX75R {
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
pub struct INTMUX74R {
    bits: bool,
}
impl INTMUX74R {
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
pub struct INTMUX73R {
    bits: bool,
}
impl INTMUX73R {
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
pub struct INTMUX72R {
    bits: bool,
}
impl INTMUX72R {
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
pub struct INTMUX71R {
    bits: bool,
}
impl INTMUX71R {
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
pub struct INTMUX70R {
    bits: bool,
}
impl INTMUX70R {
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
pub struct INTMUX69R {
    bits: bool,
}
impl INTMUX69R {
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
pub struct INTMUX68R {
    bits: bool,
}
impl INTMUX68R {
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
pub struct INTMUX67R {
    bits: bool,
}
impl INTMUX67R {
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
pub struct INTMUX66R {
    bits: bool,
}
impl INTMUX66R {
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
pub struct INTMUX65R {
    bits: bool,
}
impl INTMUX65R {
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
pub struct _INTMUX96W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX96W<'a> {
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
pub struct _INTMUX95W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX95W<'a> {
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
pub struct _INTMUX94W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX94W<'a> {
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
pub struct _INTMUX93W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX93W<'a> {
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
pub struct _INTMUX92W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX92W<'a> {
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
pub struct _INTMUX91W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX91W<'a> {
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
pub struct _INTMUX90W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX90W<'a> {
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
pub struct _INTMUX89W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX89W<'a> {
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
pub struct _INTMUX88W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX88W<'a> {
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
pub struct _INTMUX87W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX87W<'a> {
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
pub struct _INTMUX86W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX86W<'a> {
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
pub struct _INTMUX85W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX85W<'a> {
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
pub struct _INTMUX84W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX84W<'a> {
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
pub struct _INTMUX83W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX83W<'a> {
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
pub struct _INTMUX82W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX82W<'a> {
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
pub struct _INTMUX81W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX81W<'a> {
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
pub struct _INTMUX80W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX80W<'a> {
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
pub struct _INTMUX79W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX79W<'a> {
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
pub struct _INTMUX78W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX78W<'a> {
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
pub struct _INTMUX77W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX77W<'a> {
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
pub struct _INTMUX76W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX76W<'a> {
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
pub struct _INTMUX75W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX75W<'a> {
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
pub struct _INTMUX74W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX74W<'a> {
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
pub struct _INTMUX73W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX73W<'a> {
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
pub struct _INTMUX72W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX72W<'a> {
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
pub struct _INTMUX71W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX71W<'a> {
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
pub struct _INTMUX70W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX70W<'a> {
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
pub struct _INTMUX69W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX69W<'a> {
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
pub struct _INTMUX68W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX68W<'a> {
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
pub struct _INTMUX67W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX67W<'a> {
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
pub struct _INTMUX66W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX66W<'a> {
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
pub struct _INTMUX65W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX65W<'a> {
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
    #[doc = "Bit 31 - Message Object 96 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux96(&self) -> INTMUX96R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX96R { bits }
    }
    #[doc = "Bit 30 - Message Object 95 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux95(&self) -> INTMUX95R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX95R { bits }
    }
    #[doc = "Bit 29 - Message Object 94 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux94(&self) -> INTMUX94R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX94R { bits }
    }
    #[doc = "Bit 28 - Message Object 93 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux93(&self) -> INTMUX93R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX93R { bits }
    }
    #[doc = "Bit 27 - Message Object 92 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux92(&self) -> INTMUX92R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX92R { bits }
    }
    #[doc = "Bit 26 - Message Object 91 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux91(&self) -> INTMUX91R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX91R { bits }
    }
    #[doc = "Bit 25 - Message Object 90 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux90(&self) -> INTMUX90R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX90R { bits }
    }
    #[doc = "Bit 24 - Message Object 89 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux89(&self) -> INTMUX89R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX89R { bits }
    }
    #[doc = "Bit 23 - Message Object 88 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux88(&self) -> INTMUX88R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX88R { bits }
    }
    #[doc = "Bit 22 - Message Object 87 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux87(&self) -> INTMUX87R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX87R { bits }
    }
    #[doc = "Bit 21 - Message Object 86 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux86(&self) -> INTMUX86R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX86R { bits }
    }
    #[doc = "Bit 20 - Message Object 85 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux85(&self) -> INTMUX85R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX85R { bits }
    }
    #[doc = "Bit 19 - Message Object 84 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux84(&self) -> INTMUX84R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX84R { bits }
    }
    #[doc = "Bit 18 - Message Object 83 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux83(&self) -> INTMUX83R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX83R { bits }
    }
    #[doc = "Bit 17 - Message Object 82 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux82(&self) -> INTMUX82R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX82R { bits }
    }
    #[doc = "Bit 16 - Message Object 81 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux81(&self) -> INTMUX81R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX81R { bits }
    }
    #[doc = "Bit 15 - Message Object 80 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux80(&self) -> INTMUX80R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX80R { bits }
    }
    #[doc = "Bit 14 - Message Object 79 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux79(&self) -> INTMUX79R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX79R { bits }
    }
    #[doc = "Bit 13 - Message Object 78 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux78(&self) -> INTMUX78R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX78R { bits }
    }
    #[doc = "Bit 12 - Message Object 77 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux77(&self) -> INTMUX77R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX77R { bits }
    }
    #[doc = "Bit 11 - Message Object 76 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux76(&self) -> INTMUX76R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX76R { bits }
    }
    #[doc = "Bit 10 - Message Object 75 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux75(&self) -> INTMUX75R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX75R { bits }
    }
    #[doc = "Bit 9 - Message Object 74 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux74(&self) -> INTMUX74R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX74R { bits }
    }
    #[doc = "Bit 8 - Message Object 73 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux73(&self) -> INTMUX73R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX73R { bits }
    }
    #[doc = "Bit 7 - Message Object 72 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux72(&self) -> INTMUX72R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX72R { bits }
    }
    #[doc = "Bit 6 - Message Object 71 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux71(&self) -> INTMUX71R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX71R { bits }
    }
    #[doc = "Bit 5 - Message Object 70 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux70(&self) -> INTMUX70R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX70R { bits }
    }
    #[doc = "Bit 4 - Message Object 69 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux69(&self) -> INTMUX69R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX69R { bits }
    }
    #[doc = "Bit 3 - Message Object 68 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux68(&self) -> INTMUX68R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX68R { bits }
    }
    #[doc = "Bit 2 - Message Object 67 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux67(&self) -> INTMUX67R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX67R { bits }
    }
    #[doc = "Bit 1 - Message Object 66 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux66(&self) -> INTMUX66R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX66R { bits }
    }
    #[doc = "Bit 0 - Message Object 65 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux65(&self) -> INTMUX65R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX65R { bits }
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
    #[doc = "Bit 31 - Message Object 96 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux96(&mut self) -> _INTMUX96W {
        _INTMUX96W { w: self }
    }
    #[doc = "Bit 30 - Message Object 95 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux95(&mut self) -> _INTMUX95W {
        _INTMUX95W { w: self }
    }
    #[doc = "Bit 29 - Message Object 94 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux94(&mut self) -> _INTMUX94W {
        _INTMUX94W { w: self }
    }
    #[doc = "Bit 28 - Message Object 93 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux93(&mut self) -> _INTMUX93W {
        _INTMUX93W { w: self }
    }
    #[doc = "Bit 27 - Message Object 92 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux92(&mut self) -> _INTMUX92W {
        _INTMUX92W { w: self }
    }
    #[doc = "Bit 26 - Message Object 91 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux91(&mut self) -> _INTMUX91W {
        _INTMUX91W { w: self }
    }
    #[doc = "Bit 25 - Message Object 90 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux90(&mut self) -> _INTMUX90W {
        _INTMUX90W { w: self }
    }
    #[doc = "Bit 24 - Message Object 89 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux89(&mut self) -> _INTMUX89W {
        _INTMUX89W { w: self }
    }
    #[doc = "Bit 23 - Message Object 88 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux88(&mut self) -> _INTMUX88W {
        _INTMUX88W { w: self }
    }
    #[doc = "Bit 22 - Message Object 87 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux87(&mut self) -> _INTMUX87W {
        _INTMUX87W { w: self }
    }
    #[doc = "Bit 21 - Message Object 86 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux86(&mut self) -> _INTMUX86W {
        _INTMUX86W { w: self }
    }
    #[doc = "Bit 20 - Message Object 85 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux85(&mut self) -> _INTMUX85W {
        _INTMUX85W { w: self }
    }
    #[doc = "Bit 19 - Message Object 84 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux84(&mut self) -> _INTMUX84W {
        _INTMUX84W { w: self }
    }
    #[doc = "Bit 18 - Message Object 83 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux83(&mut self) -> _INTMUX83W {
        _INTMUX83W { w: self }
    }
    #[doc = "Bit 17 - Message Object 82 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux82(&mut self) -> _INTMUX82W {
        _INTMUX82W { w: self }
    }
    #[doc = "Bit 16 - Message Object 81 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux81(&mut self) -> _INTMUX81W {
        _INTMUX81W { w: self }
    }
    #[doc = "Bit 15 - Message Object 80 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux80(&mut self) -> _INTMUX80W {
        _INTMUX80W { w: self }
    }
    #[doc = "Bit 14 - Message Object 79 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux79(&mut self) -> _INTMUX79W {
        _INTMUX79W { w: self }
    }
    #[doc = "Bit 13 - Message Object 78 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux78(&mut self) -> _INTMUX78W {
        _INTMUX78W { w: self }
    }
    #[doc = "Bit 12 - Message Object 77 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux77(&mut self) -> _INTMUX77W {
        _INTMUX77W { w: self }
    }
    #[doc = "Bit 11 - Message Object 76 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux76(&mut self) -> _INTMUX76W {
        _INTMUX76W { w: self }
    }
    #[doc = "Bit 10 - Message Object 75 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux75(&mut self) -> _INTMUX75W {
        _INTMUX75W { w: self }
    }
    #[doc = "Bit 9 - Message Object 74 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux74(&mut self) -> _INTMUX74W {
        _INTMUX74W { w: self }
    }
    #[doc = "Bit 8 - Message Object 73 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux73(&mut self) -> _INTMUX73W {
        _INTMUX73W { w: self }
    }
    #[doc = "Bit 7 - Message Object 72 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux72(&mut self) -> _INTMUX72W {
        _INTMUX72W { w: self }
    }
    #[doc = "Bit 6 - Message Object 71 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux71(&mut self) -> _INTMUX71W {
        _INTMUX71W { w: self }
    }
    #[doc = "Bit 5 - Message Object 70 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux70(&mut self) -> _INTMUX70W {
        _INTMUX70W { w: self }
    }
    #[doc = "Bit 4 - Message Object 69 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux69(&mut self) -> _INTMUX69W {
        _INTMUX69W { w: self }
    }
    #[doc = "Bit 3 - Message Object 68 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux68(&mut self) -> _INTMUX68W {
        _INTMUX68W { w: self }
    }
    #[doc = "Bit 2 - Message Object 67 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux67(&mut self) -> _INTMUX67W {
        _INTMUX67W { w: self }
    }
    #[doc = "Bit 1 - Message Object 66 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux66(&mut self) -> _INTMUX66W {
        _INTMUX66W { w: self }
    }
    #[doc = "Bit 0 - Message Object 65 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux65(&mut self) -> _INTMUX65W {
        _INTMUX65W { w: self }
    }
}
