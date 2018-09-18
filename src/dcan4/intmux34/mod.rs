#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTMUX34 {
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
pub struct INTMUX64R {
    bits: bool,
}
impl INTMUX64R {
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
pub struct INTMUX63R {
    bits: bool,
}
impl INTMUX63R {
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
pub struct INTMUX62R {
    bits: bool,
}
impl INTMUX62R {
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
pub struct INTMUX61R {
    bits: bool,
}
impl INTMUX61R {
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
pub struct INTMUX60R {
    bits: bool,
}
impl INTMUX60R {
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
pub struct INTMUX59R {
    bits: bool,
}
impl INTMUX59R {
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
pub struct INTMUX58R {
    bits: bool,
}
impl INTMUX58R {
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
pub struct INTMUX57R {
    bits: bool,
}
impl INTMUX57R {
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
pub struct INTMUX56R {
    bits: bool,
}
impl INTMUX56R {
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
pub struct INTMUX55R {
    bits: bool,
}
impl INTMUX55R {
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
pub struct INTMUX54R {
    bits: bool,
}
impl INTMUX54R {
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
pub struct INTMUX53R {
    bits: bool,
}
impl INTMUX53R {
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
pub struct INTMUX52R {
    bits: bool,
}
impl INTMUX52R {
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
pub struct INTMUX51R {
    bits: bool,
}
impl INTMUX51R {
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
pub struct INTMUX50R {
    bits: bool,
}
impl INTMUX50R {
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
pub struct INTMUX49R {
    bits: bool,
}
impl INTMUX49R {
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
pub struct INTMUX48R {
    bits: bool,
}
impl INTMUX48R {
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
pub struct INTMUX47R {
    bits: bool,
}
impl INTMUX47R {
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
pub struct INTMUX46R {
    bits: bool,
}
impl INTMUX46R {
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
pub struct INTMUX45R {
    bits: bool,
}
impl INTMUX45R {
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
pub struct INTMUX44R {
    bits: bool,
}
impl INTMUX44R {
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
pub struct INTMUX43R {
    bits: bool,
}
impl INTMUX43R {
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
pub struct INTMUX42R {
    bits: bool,
}
impl INTMUX42R {
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
pub struct INTMUX41R {
    bits: bool,
}
impl INTMUX41R {
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
pub struct INTMUX40R {
    bits: bool,
}
impl INTMUX40R {
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
pub struct INTMUX39R {
    bits: bool,
}
impl INTMUX39R {
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
pub struct INTMUX38R {
    bits: bool,
}
impl INTMUX38R {
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
pub struct INTMUX37R {
    bits: bool,
}
impl INTMUX37R {
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
pub struct INTMUX36R {
    bits: bool,
}
impl INTMUX36R {
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
pub struct INTMUX35R {
    bits: bool,
}
impl INTMUX35R {
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
pub struct INTMUX34R {
    bits: bool,
}
impl INTMUX34R {
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
pub struct INTMUX33R {
    bits: bool,
}
impl INTMUX33R {
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
pub struct _INTMUX64W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX64W<'a> {
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
pub struct _INTMUX63W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX63W<'a> {
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
pub struct _INTMUX62W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX62W<'a> {
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
pub struct _INTMUX61W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX61W<'a> {
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
pub struct _INTMUX60W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX60W<'a> {
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
pub struct _INTMUX59W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX59W<'a> {
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
pub struct _INTMUX58W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX58W<'a> {
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
pub struct _INTMUX57W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX57W<'a> {
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
pub struct _INTMUX56W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX56W<'a> {
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
pub struct _INTMUX55W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX55W<'a> {
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
pub struct _INTMUX54W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX54W<'a> {
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
pub struct _INTMUX53W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX53W<'a> {
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
pub struct _INTMUX52W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX52W<'a> {
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
pub struct _INTMUX51W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX51W<'a> {
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
pub struct _INTMUX50W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX50W<'a> {
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
pub struct _INTMUX49W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX49W<'a> {
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
pub struct _INTMUX48W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX48W<'a> {
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
pub struct _INTMUX47W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX47W<'a> {
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
pub struct _INTMUX46W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX46W<'a> {
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
pub struct _INTMUX45W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX45W<'a> {
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
pub struct _INTMUX44W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX44W<'a> {
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
pub struct _INTMUX43W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX43W<'a> {
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
pub struct _INTMUX42W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX42W<'a> {
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
pub struct _INTMUX41W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX41W<'a> {
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
pub struct _INTMUX40W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX40W<'a> {
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
pub struct _INTMUX39W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX39W<'a> {
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
pub struct _INTMUX38W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX38W<'a> {
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
pub struct _INTMUX37W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX37W<'a> {
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
pub struct _INTMUX36W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX36W<'a> {
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
pub struct _INTMUX35W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX35W<'a> {
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
pub struct _INTMUX34W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX34W<'a> {
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
pub struct _INTMUX33W<'a> {
    w: &'a mut W,
}
impl<'a> _INTMUX33W<'a> {
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
    #[doc = "Bit 31 - Message Object 64 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux64(&self) -> INTMUX64R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX64R { bits }
    }
    #[doc = "Bit 30 - Message Object 63 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux63(&self) -> INTMUX63R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX63R { bits }
    }
    #[doc = "Bit 29 - Message Object 62 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux62(&self) -> INTMUX62R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX62R { bits }
    }
    #[doc = "Bit 28 - Message Object 61 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux61(&self) -> INTMUX61R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX61R { bits }
    }
    #[doc = "Bit 27 - Message Object 60 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux60(&self) -> INTMUX60R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX60R { bits }
    }
    #[doc = "Bit 26 - Message Object 59 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux59(&self) -> INTMUX59R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX59R { bits }
    }
    #[doc = "Bit 25 - Message Object 58 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux58(&self) -> INTMUX58R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX58R { bits }
    }
    #[doc = "Bit 24 - Message Object 57 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux57(&self) -> INTMUX57R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX57R { bits }
    }
    #[doc = "Bit 23 - Message Object 56 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux56(&self) -> INTMUX56R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX56R { bits }
    }
    #[doc = "Bit 22 - Message Object 55 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux55(&self) -> INTMUX55R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX55R { bits }
    }
    #[doc = "Bit 21 - Message Object 54 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux54(&self) -> INTMUX54R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX54R { bits }
    }
    #[doc = "Bit 20 - Message Object 53 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux53(&self) -> INTMUX53R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX53R { bits }
    }
    #[doc = "Bit 19 - Message Object 52 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux52(&self) -> INTMUX52R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX52R { bits }
    }
    #[doc = "Bit 18 - Message Object 51 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux51(&self) -> INTMUX51R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX51R { bits }
    }
    #[doc = "Bit 17 - Message Object 50 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux50(&self) -> INTMUX50R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX50R { bits }
    }
    #[doc = "Bit 16 - Message Object 49 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux49(&self) -> INTMUX49R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX49R { bits }
    }
    #[doc = "Bit 15 - Message Object 48 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux48(&self) -> INTMUX48R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX48R { bits }
    }
    #[doc = "Bit 14 - Message Object 47 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux47(&self) -> INTMUX47R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX47R { bits }
    }
    #[doc = "Bit 13 - Message Object 46 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux46(&self) -> INTMUX46R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX46R { bits }
    }
    #[doc = "Bit 12 - Message Object 45 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux45(&self) -> INTMUX45R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX45R { bits }
    }
    #[doc = "Bit 11 - Message Object 44 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux44(&self) -> INTMUX44R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX44R { bits }
    }
    #[doc = "Bit 10 - Message Object 43 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux43(&self) -> INTMUX43R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX43R { bits }
    }
    #[doc = "Bit 9 - Message Object 42 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux42(&self) -> INTMUX42R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX42R { bits }
    }
    #[doc = "Bit 8 - Message Object 41 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux41(&self) -> INTMUX41R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX41R { bits }
    }
    #[doc = "Bit 7 - Message Object 40 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux40(&self) -> INTMUX40R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX40R { bits }
    }
    #[doc = "Bit 6 - Message Object 39 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux39(&self) -> INTMUX39R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX39R { bits }
    }
    #[doc = "Bit 5 - Message Object 38 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux38(&self) -> INTMUX38R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX38R { bits }
    }
    #[doc = "Bit 4 - Message Object 37 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux37(&self) -> INTMUX37R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX37R { bits }
    }
    #[doc = "Bit 3 - Message Object 36 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux36(&self) -> INTMUX36R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX36R { bits }
    }
    #[doc = "Bit 2 - Message Object 35 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux35(&self) -> INTMUX35R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX35R { bits }
    }
    #[doc = "Bit 1 - Message Object 34 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux34(&self) -> INTMUX34R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX34R { bits }
    }
    #[doc = "Bit 0 - Message Object 33 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux33(&self) -> INTMUX33R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTMUX33R { bits }
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
    #[doc = "Bit 31 - Message Object 64 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux64(&mut self) -> _INTMUX64W {
        _INTMUX64W { w: self }
    }
    #[doc = "Bit 30 - Message Object 63 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux63(&mut self) -> _INTMUX63W {
        _INTMUX63W { w: self }
    }
    #[doc = "Bit 29 - Message Object 62 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux62(&mut self) -> _INTMUX62W {
        _INTMUX62W { w: self }
    }
    #[doc = "Bit 28 - Message Object 61 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux61(&mut self) -> _INTMUX61W {
        _INTMUX61W { w: self }
    }
    #[doc = "Bit 27 - Message Object 60 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux60(&mut self) -> _INTMUX60W {
        _INTMUX60W { w: self }
    }
    #[doc = "Bit 26 - Message Object 59 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux59(&mut self) -> _INTMUX59W {
        _INTMUX59W { w: self }
    }
    #[doc = "Bit 25 - Message Object 58 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux58(&mut self) -> _INTMUX58W {
        _INTMUX58W { w: self }
    }
    #[doc = "Bit 24 - Message Object 57 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux57(&mut self) -> _INTMUX57W {
        _INTMUX57W { w: self }
    }
    #[doc = "Bit 23 - Message Object 56 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux56(&mut self) -> _INTMUX56W {
        _INTMUX56W { w: self }
    }
    #[doc = "Bit 22 - Message Object 55 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux55(&mut self) -> _INTMUX55W {
        _INTMUX55W { w: self }
    }
    #[doc = "Bit 21 - Message Object 54 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux54(&mut self) -> _INTMUX54W {
        _INTMUX54W { w: self }
    }
    #[doc = "Bit 20 - Message Object 53 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux53(&mut self) -> _INTMUX53W {
        _INTMUX53W { w: self }
    }
    #[doc = "Bit 19 - Message Object 52 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux52(&mut self) -> _INTMUX52W {
        _INTMUX52W { w: self }
    }
    #[doc = "Bit 18 - Message Object 51 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux51(&mut self) -> _INTMUX51W {
        _INTMUX51W { w: self }
    }
    #[doc = "Bit 17 - Message Object 50 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux50(&mut self) -> _INTMUX50W {
        _INTMUX50W { w: self }
    }
    #[doc = "Bit 16 - Message Object 49 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux49(&mut self) -> _INTMUX49W {
        _INTMUX49W { w: self }
    }
    #[doc = "Bit 15 - Message Object 48 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux48(&mut self) -> _INTMUX48W {
        _INTMUX48W { w: self }
    }
    #[doc = "Bit 14 - Message Object 47 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux47(&mut self) -> _INTMUX47W {
        _INTMUX47W { w: self }
    }
    #[doc = "Bit 13 - Message Object 46 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux46(&mut self) -> _INTMUX46W {
        _INTMUX46W { w: self }
    }
    #[doc = "Bit 12 - Message Object 45 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux45(&mut self) -> _INTMUX45W {
        _INTMUX45W { w: self }
    }
    #[doc = "Bit 11 - Message Object 44 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux44(&mut self) -> _INTMUX44W {
        _INTMUX44W { w: self }
    }
    #[doc = "Bit 10 - Message Object 43 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux43(&mut self) -> _INTMUX43W {
        _INTMUX43W { w: self }
    }
    #[doc = "Bit 9 - Message Object 42 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux42(&mut self) -> _INTMUX42W {
        _INTMUX42W { w: self }
    }
    #[doc = "Bit 8 - Message Object 41 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux41(&mut self) -> _INTMUX41W {
        _INTMUX41W { w: self }
    }
    #[doc = "Bit 7 - Message Object 40 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux40(&mut self) -> _INTMUX40W {
        _INTMUX40W { w: self }
    }
    #[doc = "Bit 6 - Message Object 39 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux39(&mut self) -> _INTMUX39W {
        _INTMUX39W { w: self }
    }
    #[doc = "Bit 5 - Message Object 38 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux38(&mut self) -> _INTMUX38W {
        _INTMUX38W { w: self }
    }
    #[doc = "Bit 4 - Message Object 37 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux37(&mut self) -> _INTMUX37W {
        _INTMUX37W { w: self }
    }
    #[doc = "Bit 3 - Message Object 36 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux36(&mut self) -> _INTMUX36W {
        _INTMUX36W { w: self }
    }
    #[doc = "Bit 2 - Message Object 35 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux35(&mut self) -> _INTMUX35W {
        _INTMUX35W { w: self }
    }
    #[doc = "Bit 1 - Message Object 34 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux34(&mut self) -> _INTMUX34W {
        _INTMUX34W { w: self }
    }
    #[doc = "Bit 0 - Message Object 33 Interrupt Mux: 1=DCAN1INT, 0=DCAN0INT"]
    #[inline]
    pub fn intmux33(&mut self) -> _INTMUX33W {
        _INTMUX33W { w: self }
    }
}
