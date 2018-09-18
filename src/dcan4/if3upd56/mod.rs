#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF3UPD56 {
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
pub struct IF3UPDEN96R {
    bits: bool,
}
impl IF3UPDEN96R {
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
pub struct IF3UPDEN95R {
    bits: bool,
}
impl IF3UPDEN95R {
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
pub struct IF3UPDEN94R {
    bits: bool,
}
impl IF3UPDEN94R {
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
pub struct IF3UPDEN93R {
    bits: bool,
}
impl IF3UPDEN93R {
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
pub struct IF3UPDEN92R {
    bits: bool,
}
impl IF3UPDEN92R {
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
pub struct IF3UPDEN91R {
    bits: bool,
}
impl IF3UPDEN91R {
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
pub struct IF3UPDEN90R {
    bits: bool,
}
impl IF3UPDEN90R {
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
pub struct IF3UPDEN89R {
    bits: bool,
}
impl IF3UPDEN89R {
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
pub struct IF3UPDEN88R {
    bits: bool,
}
impl IF3UPDEN88R {
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
pub struct IF3UPDEN87R {
    bits: bool,
}
impl IF3UPDEN87R {
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
pub struct IF3UPDEN86R {
    bits: bool,
}
impl IF3UPDEN86R {
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
pub struct IF3UPDEN85R {
    bits: bool,
}
impl IF3UPDEN85R {
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
pub struct IF3UPDEN84R {
    bits: bool,
}
impl IF3UPDEN84R {
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
pub struct IF3UPDEN83R {
    bits: bool,
}
impl IF3UPDEN83R {
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
pub struct IF3UPDEN82R {
    bits: bool,
}
impl IF3UPDEN82R {
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
pub struct IF3UPDEN81R {
    bits: bool,
}
impl IF3UPDEN81R {
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
pub struct IF3UPDEN80R {
    bits: bool,
}
impl IF3UPDEN80R {
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
pub struct IF3UPDEN79R {
    bits: bool,
}
impl IF3UPDEN79R {
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
pub struct IF3UPDEN78R {
    bits: bool,
}
impl IF3UPDEN78R {
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
pub struct IF3UPDEN77R {
    bits: bool,
}
impl IF3UPDEN77R {
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
pub struct IF3UPDEN76R {
    bits: bool,
}
impl IF3UPDEN76R {
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
pub struct IF3UPDEN75R {
    bits: bool,
}
impl IF3UPDEN75R {
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
pub struct IF3UPDEN74R {
    bits: bool,
}
impl IF3UPDEN74R {
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
pub struct IF3UPDEN73R {
    bits: bool,
}
impl IF3UPDEN73R {
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
pub struct IF3UPDEN72R {
    bits: bool,
}
impl IF3UPDEN72R {
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
pub struct IF3UPDEN71R {
    bits: bool,
}
impl IF3UPDEN71R {
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
pub struct IF3UPDEN70R {
    bits: bool,
}
impl IF3UPDEN70R {
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
pub struct IF3UPDEN69R {
    bits: bool,
}
impl IF3UPDEN69R {
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
pub struct IF3UPDEN68R {
    bits: bool,
}
impl IF3UPDEN68R {
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
pub struct IF3UPDEN67R {
    bits: bool,
}
impl IF3UPDEN67R {
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
pub struct IF3UPDEN66R {
    bits: bool,
}
impl IF3UPDEN66R {
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
pub struct IF3UPDEN65R {
    bits: bool,
}
impl IF3UPDEN65R {
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
pub struct _IF3UPDEN96W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN96W<'a> {
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
pub struct _IF3UPDEN95W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN95W<'a> {
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
pub struct _IF3UPDEN94W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN94W<'a> {
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
pub struct _IF3UPDEN93W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN93W<'a> {
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
pub struct _IF3UPDEN92W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN92W<'a> {
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
pub struct _IF3UPDEN91W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN91W<'a> {
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
pub struct _IF3UPDEN90W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN90W<'a> {
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
pub struct _IF3UPDEN89W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN89W<'a> {
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
pub struct _IF3UPDEN88W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN88W<'a> {
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
pub struct _IF3UPDEN87W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN87W<'a> {
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
pub struct _IF3UPDEN86W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN86W<'a> {
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
pub struct _IF3UPDEN85W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN85W<'a> {
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
pub struct _IF3UPDEN84W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN84W<'a> {
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
pub struct _IF3UPDEN83W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN83W<'a> {
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
pub struct _IF3UPDEN82W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN82W<'a> {
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
pub struct _IF3UPDEN81W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN81W<'a> {
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
pub struct _IF3UPDEN80W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN80W<'a> {
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
pub struct _IF3UPDEN79W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN79W<'a> {
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
pub struct _IF3UPDEN78W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN78W<'a> {
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
pub struct _IF3UPDEN77W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN77W<'a> {
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
pub struct _IF3UPDEN76W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN76W<'a> {
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
pub struct _IF3UPDEN75W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN75W<'a> {
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
pub struct _IF3UPDEN74W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN74W<'a> {
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
pub struct _IF3UPDEN73W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN73W<'a> {
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
pub struct _IF3UPDEN72W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN72W<'a> {
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
pub struct _IF3UPDEN71W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN71W<'a> {
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
pub struct _IF3UPDEN70W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN70W<'a> {
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
pub struct _IF3UPDEN69W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN69W<'a> {
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
pub struct _IF3UPDEN68W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN68W<'a> {
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
pub struct _IF3UPDEN67W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN67W<'a> {
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
pub struct _IF3UPDEN66W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN66W<'a> {
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
pub struct _IF3UPDEN65W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN65W<'a> {
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
    #[doc = "Bit 31 - Automatic IF3 Update Enabled for Message Object 96"]
    #[inline]
    pub fn if3upden96(&self) -> IF3UPDEN96R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN96R { bits }
    }
    #[doc = "Bit 30 - Automatic IF3 Update Enabled for Message Object 95"]
    #[inline]
    pub fn if3upden95(&self) -> IF3UPDEN95R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN95R { bits }
    }
    #[doc = "Bit 29 - Automatic IF3 Update Enabled for Message Object 94"]
    #[inline]
    pub fn if3upden94(&self) -> IF3UPDEN94R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN94R { bits }
    }
    #[doc = "Bit 28 - Automatic IF3 Update Enabled for Message Object 93"]
    #[inline]
    pub fn if3upden93(&self) -> IF3UPDEN93R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN93R { bits }
    }
    #[doc = "Bit 27 - Automatic IF3 Update Enabled for Message Object 92"]
    #[inline]
    pub fn if3upden92(&self) -> IF3UPDEN92R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN92R { bits }
    }
    #[doc = "Bit 26 - Automatic IF3 Update Enabled for Message Object 91"]
    #[inline]
    pub fn if3upden91(&self) -> IF3UPDEN91R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN91R { bits }
    }
    #[doc = "Bit 25 - Automatic IF3 Update Enabled for Message Object 90"]
    #[inline]
    pub fn if3upden90(&self) -> IF3UPDEN90R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN90R { bits }
    }
    #[doc = "Bit 24 - Automatic IF3 Update Enabled for Message Object 89"]
    #[inline]
    pub fn if3upden89(&self) -> IF3UPDEN89R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN89R { bits }
    }
    #[doc = "Bit 23 - Automatic IF3 Update Enabled for Message Object 88"]
    #[inline]
    pub fn if3upden88(&self) -> IF3UPDEN88R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN88R { bits }
    }
    #[doc = "Bit 22 - Automatic IF3 Update Enabled for Message Object 87"]
    #[inline]
    pub fn if3upden87(&self) -> IF3UPDEN87R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN87R { bits }
    }
    #[doc = "Bit 21 - Automatic IF3 Update Enabled for Message Object 86"]
    #[inline]
    pub fn if3upden86(&self) -> IF3UPDEN86R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN86R { bits }
    }
    #[doc = "Bit 20 - Automatic IF3 Update Enabled for Message Object 85"]
    #[inline]
    pub fn if3upden85(&self) -> IF3UPDEN85R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN85R { bits }
    }
    #[doc = "Bit 19 - Automatic IF3 Update Enabled for Message Object 84"]
    #[inline]
    pub fn if3upden84(&self) -> IF3UPDEN84R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN84R { bits }
    }
    #[doc = "Bit 18 - Automatic IF3 Update Enabled for Message Object 83"]
    #[inline]
    pub fn if3upden83(&self) -> IF3UPDEN83R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN83R { bits }
    }
    #[doc = "Bit 17 - Automatic IF3 Update Enabled for Message Object 82"]
    #[inline]
    pub fn if3upden82(&self) -> IF3UPDEN82R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN82R { bits }
    }
    #[doc = "Bit 16 - Automatic IF3 Update Enabled for Message Object 81"]
    #[inline]
    pub fn if3upden81(&self) -> IF3UPDEN81R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN81R { bits }
    }
    #[doc = "Bit 15 - Automatic IF3 Update Enabled for Message Object 80"]
    #[inline]
    pub fn if3upden80(&self) -> IF3UPDEN80R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN80R { bits }
    }
    #[doc = "Bit 14 - Automatic IF3 Update Enabled for Message Object 79"]
    #[inline]
    pub fn if3upden79(&self) -> IF3UPDEN79R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN79R { bits }
    }
    #[doc = "Bit 13 - Automatic IF3 Update Enabled for Message Object 78"]
    #[inline]
    pub fn if3upden78(&self) -> IF3UPDEN78R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN78R { bits }
    }
    #[doc = "Bit 12 - Automatic IF3 Update Enabled for Message Object 77"]
    #[inline]
    pub fn if3upden77(&self) -> IF3UPDEN77R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN77R { bits }
    }
    #[doc = "Bit 11 - Automatic IF3 Update Enabled for Message Object 76"]
    #[inline]
    pub fn if3upden76(&self) -> IF3UPDEN76R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN76R { bits }
    }
    #[doc = "Bit 10 - Automatic IF3 Update Enabled for Message Object 75"]
    #[inline]
    pub fn if3upden75(&self) -> IF3UPDEN75R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN75R { bits }
    }
    #[doc = "Bit 9 - Automatic IF3 Update Enabled for Message Object 74"]
    #[inline]
    pub fn if3upden74(&self) -> IF3UPDEN74R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN74R { bits }
    }
    #[doc = "Bit 8 - Automatic IF3 Update Enabled for Message Object 73"]
    #[inline]
    pub fn if3upden73(&self) -> IF3UPDEN73R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN73R { bits }
    }
    #[doc = "Bit 7 - Automatic IF3 Update Enabled for Message Object 72"]
    #[inline]
    pub fn if3upden72(&self) -> IF3UPDEN72R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN72R { bits }
    }
    #[doc = "Bit 6 - Automatic IF3 Update Enabled for Message Object 71"]
    #[inline]
    pub fn if3upden71(&self) -> IF3UPDEN71R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN71R { bits }
    }
    #[doc = "Bit 5 - Automatic IF3 Update Enabled for Message Object 70"]
    #[inline]
    pub fn if3upden70(&self) -> IF3UPDEN70R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN70R { bits }
    }
    #[doc = "Bit 4 - Automatic IF3 Update Enabled for Message Object 69"]
    #[inline]
    pub fn if3upden69(&self) -> IF3UPDEN69R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN69R { bits }
    }
    #[doc = "Bit 3 - Automatic IF3 Update Enabled for Message Object 68"]
    #[inline]
    pub fn if3upden68(&self) -> IF3UPDEN68R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN68R { bits }
    }
    #[doc = "Bit 2 - Automatic IF3 Update Enabled for Message Object 67"]
    #[inline]
    pub fn if3upden67(&self) -> IF3UPDEN67R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN67R { bits }
    }
    #[doc = "Bit 1 - Automatic IF3 Update Enabled for Message Object 66"]
    #[inline]
    pub fn if3upden66(&self) -> IF3UPDEN66R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN66R { bits }
    }
    #[doc = "Bit 0 - Automatic IF3 Update Enabled for Message Object 65"]
    #[inline]
    pub fn if3upden65(&self) -> IF3UPDEN65R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN65R { bits }
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
    #[doc = "Bit 31 - Automatic IF3 Update Enabled for Message Object 96"]
    #[inline]
    pub fn if3upden96(&mut self) -> _IF3UPDEN96W {
        _IF3UPDEN96W { w: self }
    }
    #[doc = "Bit 30 - Automatic IF3 Update Enabled for Message Object 95"]
    #[inline]
    pub fn if3upden95(&mut self) -> _IF3UPDEN95W {
        _IF3UPDEN95W { w: self }
    }
    #[doc = "Bit 29 - Automatic IF3 Update Enabled for Message Object 94"]
    #[inline]
    pub fn if3upden94(&mut self) -> _IF3UPDEN94W {
        _IF3UPDEN94W { w: self }
    }
    #[doc = "Bit 28 - Automatic IF3 Update Enabled for Message Object 93"]
    #[inline]
    pub fn if3upden93(&mut self) -> _IF3UPDEN93W {
        _IF3UPDEN93W { w: self }
    }
    #[doc = "Bit 27 - Automatic IF3 Update Enabled for Message Object 92"]
    #[inline]
    pub fn if3upden92(&mut self) -> _IF3UPDEN92W {
        _IF3UPDEN92W { w: self }
    }
    #[doc = "Bit 26 - Automatic IF3 Update Enabled for Message Object 91"]
    #[inline]
    pub fn if3upden91(&mut self) -> _IF3UPDEN91W {
        _IF3UPDEN91W { w: self }
    }
    #[doc = "Bit 25 - Automatic IF3 Update Enabled for Message Object 90"]
    #[inline]
    pub fn if3upden90(&mut self) -> _IF3UPDEN90W {
        _IF3UPDEN90W { w: self }
    }
    #[doc = "Bit 24 - Automatic IF3 Update Enabled for Message Object 89"]
    #[inline]
    pub fn if3upden89(&mut self) -> _IF3UPDEN89W {
        _IF3UPDEN89W { w: self }
    }
    #[doc = "Bit 23 - Automatic IF3 Update Enabled for Message Object 88"]
    #[inline]
    pub fn if3upden88(&mut self) -> _IF3UPDEN88W {
        _IF3UPDEN88W { w: self }
    }
    #[doc = "Bit 22 - Automatic IF3 Update Enabled for Message Object 87"]
    #[inline]
    pub fn if3upden87(&mut self) -> _IF3UPDEN87W {
        _IF3UPDEN87W { w: self }
    }
    #[doc = "Bit 21 - Automatic IF3 Update Enabled for Message Object 86"]
    #[inline]
    pub fn if3upden86(&mut self) -> _IF3UPDEN86W {
        _IF3UPDEN86W { w: self }
    }
    #[doc = "Bit 20 - Automatic IF3 Update Enabled for Message Object 85"]
    #[inline]
    pub fn if3upden85(&mut self) -> _IF3UPDEN85W {
        _IF3UPDEN85W { w: self }
    }
    #[doc = "Bit 19 - Automatic IF3 Update Enabled for Message Object 84"]
    #[inline]
    pub fn if3upden84(&mut self) -> _IF3UPDEN84W {
        _IF3UPDEN84W { w: self }
    }
    #[doc = "Bit 18 - Automatic IF3 Update Enabled for Message Object 83"]
    #[inline]
    pub fn if3upden83(&mut self) -> _IF3UPDEN83W {
        _IF3UPDEN83W { w: self }
    }
    #[doc = "Bit 17 - Automatic IF3 Update Enabled for Message Object 82"]
    #[inline]
    pub fn if3upden82(&mut self) -> _IF3UPDEN82W {
        _IF3UPDEN82W { w: self }
    }
    #[doc = "Bit 16 - Automatic IF3 Update Enabled for Message Object 81"]
    #[inline]
    pub fn if3upden81(&mut self) -> _IF3UPDEN81W {
        _IF3UPDEN81W { w: self }
    }
    #[doc = "Bit 15 - Automatic IF3 Update Enabled for Message Object 80"]
    #[inline]
    pub fn if3upden80(&mut self) -> _IF3UPDEN80W {
        _IF3UPDEN80W { w: self }
    }
    #[doc = "Bit 14 - Automatic IF3 Update Enabled for Message Object 79"]
    #[inline]
    pub fn if3upden79(&mut self) -> _IF3UPDEN79W {
        _IF3UPDEN79W { w: self }
    }
    #[doc = "Bit 13 - Automatic IF3 Update Enabled for Message Object 78"]
    #[inline]
    pub fn if3upden78(&mut self) -> _IF3UPDEN78W {
        _IF3UPDEN78W { w: self }
    }
    #[doc = "Bit 12 - Automatic IF3 Update Enabled for Message Object 77"]
    #[inline]
    pub fn if3upden77(&mut self) -> _IF3UPDEN77W {
        _IF3UPDEN77W { w: self }
    }
    #[doc = "Bit 11 - Automatic IF3 Update Enabled for Message Object 76"]
    #[inline]
    pub fn if3upden76(&mut self) -> _IF3UPDEN76W {
        _IF3UPDEN76W { w: self }
    }
    #[doc = "Bit 10 - Automatic IF3 Update Enabled for Message Object 75"]
    #[inline]
    pub fn if3upden75(&mut self) -> _IF3UPDEN75W {
        _IF3UPDEN75W { w: self }
    }
    #[doc = "Bit 9 - Automatic IF3 Update Enabled for Message Object 74"]
    #[inline]
    pub fn if3upden74(&mut self) -> _IF3UPDEN74W {
        _IF3UPDEN74W { w: self }
    }
    #[doc = "Bit 8 - Automatic IF3 Update Enabled for Message Object 73"]
    #[inline]
    pub fn if3upden73(&mut self) -> _IF3UPDEN73W {
        _IF3UPDEN73W { w: self }
    }
    #[doc = "Bit 7 - Automatic IF3 Update Enabled for Message Object 72"]
    #[inline]
    pub fn if3upden72(&mut self) -> _IF3UPDEN72W {
        _IF3UPDEN72W { w: self }
    }
    #[doc = "Bit 6 - Automatic IF3 Update Enabled for Message Object 71"]
    #[inline]
    pub fn if3upden71(&mut self) -> _IF3UPDEN71W {
        _IF3UPDEN71W { w: self }
    }
    #[doc = "Bit 5 - Automatic IF3 Update Enabled for Message Object 70"]
    #[inline]
    pub fn if3upden70(&mut self) -> _IF3UPDEN70W {
        _IF3UPDEN70W { w: self }
    }
    #[doc = "Bit 4 - Automatic IF3 Update Enabled for Message Object 69"]
    #[inline]
    pub fn if3upden69(&mut self) -> _IF3UPDEN69W {
        _IF3UPDEN69W { w: self }
    }
    #[doc = "Bit 3 - Automatic IF3 Update Enabled for Message Object 68"]
    #[inline]
    pub fn if3upden68(&mut self) -> _IF3UPDEN68W {
        _IF3UPDEN68W { w: self }
    }
    #[doc = "Bit 2 - Automatic IF3 Update Enabled for Message Object 67"]
    #[inline]
    pub fn if3upden67(&mut self) -> _IF3UPDEN67W {
        _IF3UPDEN67W { w: self }
    }
    #[doc = "Bit 1 - Automatic IF3 Update Enabled for Message Object 66"]
    #[inline]
    pub fn if3upden66(&mut self) -> _IF3UPDEN66W {
        _IF3UPDEN66W { w: self }
    }
    #[doc = "Bit 0 - Automatic IF3 Update Enabled for Message Object 65"]
    #[inline]
    pub fn if3upden65(&mut self) -> _IF3UPDEN65W {
        _IF3UPDEN65W { w: self }
    }
}
