#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF3UPD78 {
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
pub struct IF3UPDEN128R {
    bits: bool,
}
impl IF3UPDEN128R {
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
pub struct IF3UPDEN127R {
    bits: bool,
}
impl IF3UPDEN127R {
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
pub struct IF3UPDEN126R {
    bits: bool,
}
impl IF3UPDEN126R {
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
pub struct IF3UPDEN125R {
    bits: bool,
}
impl IF3UPDEN125R {
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
pub struct IF3UPDEN124R {
    bits: bool,
}
impl IF3UPDEN124R {
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
pub struct IF3UPDEN123R {
    bits: bool,
}
impl IF3UPDEN123R {
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
pub struct IF3UPDEN122R {
    bits: bool,
}
impl IF3UPDEN122R {
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
pub struct IF3UPDEN121R {
    bits: bool,
}
impl IF3UPDEN121R {
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
pub struct IF3UPDEN120R {
    bits: bool,
}
impl IF3UPDEN120R {
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
pub struct IF3UPDEN119R {
    bits: bool,
}
impl IF3UPDEN119R {
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
pub struct IF3UPDEN118R {
    bits: bool,
}
impl IF3UPDEN118R {
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
pub struct IF3UPDEN117R {
    bits: bool,
}
impl IF3UPDEN117R {
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
pub struct IF3UPDEN116R {
    bits: bool,
}
impl IF3UPDEN116R {
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
pub struct IF3UPDEN115R {
    bits: bool,
}
impl IF3UPDEN115R {
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
pub struct IF3UPDEN114R {
    bits: bool,
}
impl IF3UPDEN114R {
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
pub struct IF3UPDEN113R {
    bits: bool,
}
impl IF3UPDEN113R {
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
pub struct IF3UPDEN112R {
    bits: bool,
}
impl IF3UPDEN112R {
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
pub struct IF3UPDEN111R {
    bits: bool,
}
impl IF3UPDEN111R {
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
pub struct IF3UPDEN110R {
    bits: bool,
}
impl IF3UPDEN110R {
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
pub struct IF3UPDEN109R {
    bits: bool,
}
impl IF3UPDEN109R {
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
pub struct IF3UPDEN108R {
    bits: bool,
}
impl IF3UPDEN108R {
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
pub struct IF3UPDEN107R {
    bits: bool,
}
impl IF3UPDEN107R {
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
pub struct IF3UPDEN106R {
    bits: bool,
}
impl IF3UPDEN106R {
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
pub struct IF3UPDEN105R {
    bits: bool,
}
impl IF3UPDEN105R {
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
pub struct IF3UPDEN104R {
    bits: bool,
}
impl IF3UPDEN104R {
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
pub struct IF3UPDEN103R {
    bits: bool,
}
impl IF3UPDEN103R {
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
pub struct IF3UPDEN102R {
    bits: bool,
}
impl IF3UPDEN102R {
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
pub struct IF3UPDEN101R {
    bits: bool,
}
impl IF3UPDEN101R {
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
pub struct IF3UPDEN100R {
    bits: bool,
}
impl IF3UPDEN100R {
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
pub struct IF3UPDEN99R {
    bits: bool,
}
impl IF3UPDEN99R {
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
pub struct IF3UPDEN98R {
    bits: bool,
}
impl IF3UPDEN98R {
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
pub struct IF3UPDEN97R {
    bits: bool,
}
impl IF3UPDEN97R {
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
pub struct _IF3UPDEN128W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN128W<'a> {
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
pub struct _IF3UPDEN127W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN127W<'a> {
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
pub struct _IF3UPDEN126W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN126W<'a> {
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
pub struct _IF3UPDEN125W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN125W<'a> {
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
pub struct _IF3UPDEN124W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN124W<'a> {
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
pub struct _IF3UPDEN123W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN123W<'a> {
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
pub struct _IF3UPDEN122W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN122W<'a> {
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
pub struct _IF3UPDEN121W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN121W<'a> {
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
pub struct _IF3UPDEN120W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN120W<'a> {
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
pub struct _IF3UPDEN119W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN119W<'a> {
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
pub struct _IF3UPDEN118W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN118W<'a> {
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
pub struct _IF3UPDEN117W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN117W<'a> {
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
pub struct _IF3UPDEN116W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN116W<'a> {
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
pub struct _IF3UPDEN115W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN115W<'a> {
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
pub struct _IF3UPDEN114W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN114W<'a> {
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
pub struct _IF3UPDEN113W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN113W<'a> {
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
pub struct _IF3UPDEN112W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN112W<'a> {
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
pub struct _IF3UPDEN111W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN111W<'a> {
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
pub struct _IF3UPDEN110W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN110W<'a> {
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
pub struct _IF3UPDEN109W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN109W<'a> {
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
pub struct _IF3UPDEN108W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN108W<'a> {
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
pub struct _IF3UPDEN107W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN107W<'a> {
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
pub struct _IF3UPDEN106W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN106W<'a> {
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
pub struct _IF3UPDEN105W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN105W<'a> {
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
pub struct _IF3UPDEN104W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN104W<'a> {
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
pub struct _IF3UPDEN103W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN103W<'a> {
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
pub struct _IF3UPDEN102W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN102W<'a> {
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
pub struct _IF3UPDEN101W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN101W<'a> {
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
pub struct _IF3UPDEN100W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN100W<'a> {
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
pub struct _IF3UPDEN99W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN99W<'a> {
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
pub struct _IF3UPDEN98W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN98W<'a> {
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
pub struct _IF3UPDEN97W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN97W<'a> {
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
    #[doc = "Bit 31 - Automatic IF3 Update Enabled for Message Object 128"]
    #[inline]
    pub fn if3upden128(&self) -> IF3UPDEN128R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN128R { bits }
    }
    #[doc = "Bit 30 - Automatic IF3 Update Enabled for Message Object 127"]
    #[inline]
    pub fn if3upden127(&self) -> IF3UPDEN127R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN127R { bits }
    }
    #[doc = "Bit 29 - Automatic IF3 Update Enabled for Message Object 126"]
    #[inline]
    pub fn if3upden126(&self) -> IF3UPDEN126R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN126R { bits }
    }
    #[doc = "Bit 28 - Automatic IF3 Update Enabled for Message Object 125"]
    #[inline]
    pub fn if3upden125(&self) -> IF3UPDEN125R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN125R { bits }
    }
    #[doc = "Bit 27 - Automatic IF3 Update Enabled for Message Object 124"]
    #[inline]
    pub fn if3upden124(&self) -> IF3UPDEN124R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN124R { bits }
    }
    #[doc = "Bit 26 - Automatic IF3 Update Enabled for Message Object 123"]
    #[inline]
    pub fn if3upden123(&self) -> IF3UPDEN123R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN123R { bits }
    }
    #[doc = "Bit 25 - Automatic IF3 Update Enabled for Message Object 122"]
    #[inline]
    pub fn if3upden122(&self) -> IF3UPDEN122R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN122R { bits }
    }
    #[doc = "Bit 24 - Automatic IF3 Update Enabled for Message Object 121"]
    #[inline]
    pub fn if3upden121(&self) -> IF3UPDEN121R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN121R { bits }
    }
    #[doc = "Bit 23 - Automatic IF3 Update Enabled for Message Object 120"]
    #[inline]
    pub fn if3upden120(&self) -> IF3UPDEN120R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN120R { bits }
    }
    #[doc = "Bit 22 - Automatic IF3 Update Enabled for Message Object 119"]
    #[inline]
    pub fn if3upden119(&self) -> IF3UPDEN119R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN119R { bits }
    }
    #[doc = "Bit 21 - Automatic IF3 Update Enabled for Message Object 118"]
    #[inline]
    pub fn if3upden118(&self) -> IF3UPDEN118R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN118R { bits }
    }
    #[doc = "Bit 20 - Automatic IF3 Update Enabled for Message Object 117"]
    #[inline]
    pub fn if3upden117(&self) -> IF3UPDEN117R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN117R { bits }
    }
    #[doc = "Bit 19 - Automatic IF3 Update Enabled for Message Object 116"]
    #[inline]
    pub fn if3upden116(&self) -> IF3UPDEN116R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN116R { bits }
    }
    #[doc = "Bit 18 - Automatic IF3 Update Enabled for Message Object 115"]
    #[inline]
    pub fn if3upden115(&self) -> IF3UPDEN115R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN115R { bits }
    }
    #[doc = "Bit 17 - Automatic IF3 Update Enabled for Message Object 114"]
    #[inline]
    pub fn if3upden114(&self) -> IF3UPDEN114R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN114R { bits }
    }
    #[doc = "Bit 16 - Automatic IF3 Update Enabled for Message Object 112"]
    #[inline]
    pub fn if3upden113(&self) -> IF3UPDEN113R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN113R { bits }
    }
    #[doc = "Bit 15 - Automatic IF3 Update Enabled for Message Object 112"]
    #[inline]
    pub fn if3upden112(&self) -> IF3UPDEN112R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN112R { bits }
    }
    #[doc = "Bit 14 - Automatic IF3 Update Enabled for Message Object 111"]
    #[inline]
    pub fn if3upden111(&self) -> IF3UPDEN111R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN111R { bits }
    }
    #[doc = "Bit 13 - Automatic IF3 Update Enabled for Message Object 110"]
    #[inline]
    pub fn if3upden110(&self) -> IF3UPDEN110R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN110R { bits }
    }
    #[doc = "Bit 12 - Automatic IF3 Update Enabled for Message Object 109"]
    #[inline]
    pub fn if3upden109(&self) -> IF3UPDEN109R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN109R { bits }
    }
    #[doc = "Bit 11 - Automatic IF3 Update Enabled for Message Object 108"]
    #[inline]
    pub fn if3upden108(&self) -> IF3UPDEN108R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN108R { bits }
    }
    #[doc = "Bit 10 - Automatic IF3 Update Enabled for Message Object 107"]
    #[inline]
    pub fn if3upden107(&self) -> IF3UPDEN107R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN107R { bits }
    }
    #[doc = "Bit 9 - Automatic IF3 Update Enabled for Message Object 106"]
    #[inline]
    pub fn if3upden106(&self) -> IF3UPDEN106R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN106R { bits }
    }
    #[doc = "Bit 8 - Automatic IF3 Update Enabled for Message Object 105"]
    #[inline]
    pub fn if3upden105(&self) -> IF3UPDEN105R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN105R { bits }
    }
    #[doc = "Bit 7 - Automatic IF3 Update Enabled for Message Object 104"]
    #[inline]
    pub fn if3upden104(&self) -> IF3UPDEN104R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN104R { bits }
    }
    #[doc = "Bit 6 - Automatic IF3 Update Enabled for Message Object 103"]
    #[inline]
    pub fn if3upden103(&self) -> IF3UPDEN103R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN103R { bits }
    }
    #[doc = "Bit 5 - Automatic IF3 Update Enabled for Message Object 102"]
    #[inline]
    pub fn if3upden102(&self) -> IF3UPDEN102R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN102R { bits }
    }
    #[doc = "Bit 4 - Automatic IF3 Update Enabled for Message Object 101"]
    #[inline]
    pub fn if3upden101(&self) -> IF3UPDEN101R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN101R { bits }
    }
    #[doc = "Bit 3 - Automatic IF3 Update Enabled for Message Object 100"]
    #[inline]
    pub fn if3upden100(&self) -> IF3UPDEN100R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN100R { bits }
    }
    #[doc = "Bit 2 - Automatic IF3 Update Enabled for Message Object 99"]
    #[inline]
    pub fn if3upden99(&self) -> IF3UPDEN99R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN99R { bits }
    }
    #[doc = "Bit 1 - Automatic IF3 Update Enabled for Message Object 98"]
    #[inline]
    pub fn if3upden98(&self) -> IF3UPDEN98R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN98R { bits }
    }
    #[doc = "Bit 0 - Automatic IF3 Update Enabled for Message Object 97"]
    #[inline]
    pub fn if3upden97(&self) -> IF3UPDEN97R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN97R { bits }
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
    #[doc = "Bit 31 - Automatic IF3 Update Enabled for Message Object 128"]
    #[inline]
    pub fn if3upden128(&mut self) -> _IF3UPDEN128W {
        _IF3UPDEN128W { w: self }
    }
    #[doc = "Bit 30 - Automatic IF3 Update Enabled for Message Object 127"]
    #[inline]
    pub fn if3upden127(&mut self) -> _IF3UPDEN127W {
        _IF3UPDEN127W { w: self }
    }
    #[doc = "Bit 29 - Automatic IF3 Update Enabled for Message Object 126"]
    #[inline]
    pub fn if3upden126(&mut self) -> _IF3UPDEN126W {
        _IF3UPDEN126W { w: self }
    }
    #[doc = "Bit 28 - Automatic IF3 Update Enabled for Message Object 125"]
    #[inline]
    pub fn if3upden125(&mut self) -> _IF3UPDEN125W {
        _IF3UPDEN125W { w: self }
    }
    #[doc = "Bit 27 - Automatic IF3 Update Enabled for Message Object 124"]
    #[inline]
    pub fn if3upden124(&mut self) -> _IF3UPDEN124W {
        _IF3UPDEN124W { w: self }
    }
    #[doc = "Bit 26 - Automatic IF3 Update Enabled for Message Object 123"]
    #[inline]
    pub fn if3upden123(&mut self) -> _IF3UPDEN123W {
        _IF3UPDEN123W { w: self }
    }
    #[doc = "Bit 25 - Automatic IF3 Update Enabled for Message Object 122"]
    #[inline]
    pub fn if3upden122(&mut self) -> _IF3UPDEN122W {
        _IF3UPDEN122W { w: self }
    }
    #[doc = "Bit 24 - Automatic IF3 Update Enabled for Message Object 121"]
    #[inline]
    pub fn if3upden121(&mut self) -> _IF3UPDEN121W {
        _IF3UPDEN121W { w: self }
    }
    #[doc = "Bit 23 - Automatic IF3 Update Enabled for Message Object 120"]
    #[inline]
    pub fn if3upden120(&mut self) -> _IF3UPDEN120W {
        _IF3UPDEN120W { w: self }
    }
    #[doc = "Bit 22 - Automatic IF3 Update Enabled for Message Object 119"]
    #[inline]
    pub fn if3upden119(&mut self) -> _IF3UPDEN119W {
        _IF3UPDEN119W { w: self }
    }
    #[doc = "Bit 21 - Automatic IF3 Update Enabled for Message Object 118"]
    #[inline]
    pub fn if3upden118(&mut self) -> _IF3UPDEN118W {
        _IF3UPDEN118W { w: self }
    }
    #[doc = "Bit 20 - Automatic IF3 Update Enabled for Message Object 117"]
    #[inline]
    pub fn if3upden117(&mut self) -> _IF3UPDEN117W {
        _IF3UPDEN117W { w: self }
    }
    #[doc = "Bit 19 - Automatic IF3 Update Enabled for Message Object 116"]
    #[inline]
    pub fn if3upden116(&mut self) -> _IF3UPDEN116W {
        _IF3UPDEN116W { w: self }
    }
    #[doc = "Bit 18 - Automatic IF3 Update Enabled for Message Object 115"]
    #[inline]
    pub fn if3upden115(&mut self) -> _IF3UPDEN115W {
        _IF3UPDEN115W { w: self }
    }
    #[doc = "Bit 17 - Automatic IF3 Update Enabled for Message Object 114"]
    #[inline]
    pub fn if3upden114(&mut self) -> _IF3UPDEN114W {
        _IF3UPDEN114W { w: self }
    }
    #[doc = "Bit 16 - Automatic IF3 Update Enabled for Message Object 112"]
    #[inline]
    pub fn if3upden113(&mut self) -> _IF3UPDEN113W {
        _IF3UPDEN113W { w: self }
    }
    #[doc = "Bit 15 - Automatic IF3 Update Enabled for Message Object 112"]
    #[inline]
    pub fn if3upden112(&mut self) -> _IF3UPDEN112W {
        _IF3UPDEN112W { w: self }
    }
    #[doc = "Bit 14 - Automatic IF3 Update Enabled for Message Object 111"]
    #[inline]
    pub fn if3upden111(&mut self) -> _IF3UPDEN111W {
        _IF3UPDEN111W { w: self }
    }
    #[doc = "Bit 13 - Automatic IF3 Update Enabled for Message Object 110"]
    #[inline]
    pub fn if3upden110(&mut self) -> _IF3UPDEN110W {
        _IF3UPDEN110W { w: self }
    }
    #[doc = "Bit 12 - Automatic IF3 Update Enabled for Message Object 109"]
    #[inline]
    pub fn if3upden109(&mut self) -> _IF3UPDEN109W {
        _IF3UPDEN109W { w: self }
    }
    #[doc = "Bit 11 - Automatic IF3 Update Enabled for Message Object 108"]
    #[inline]
    pub fn if3upden108(&mut self) -> _IF3UPDEN108W {
        _IF3UPDEN108W { w: self }
    }
    #[doc = "Bit 10 - Automatic IF3 Update Enabled for Message Object 107"]
    #[inline]
    pub fn if3upden107(&mut self) -> _IF3UPDEN107W {
        _IF3UPDEN107W { w: self }
    }
    #[doc = "Bit 9 - Automatic IF3 Update Enabled for Message Object 106"]
    #[inline]
    pub fn if3upden106(&mut self) -> _IF3UPDEN106W {
        _IF3UPDEN106W { w: self }
    }
    #[doc = "Bit 8 - Automatic IF3 Update Enabled for Message Object 105"]
    #[inline]
    pub fn if3upden105(&mut self) -> _IF3UPDEN105W {
        _IF3UPDEN105W { w: self }
    }
    #[doc = "Bit 7 - Automatic IF3 Update Enabled for Message Object 104"]
    #[inline]
    pub fn if3upden104(&mut self) -> _IF3UPDEN104W {
        _IF3UPDEN104W { w: self }
    }
    #[doc = "Bit 6 - Automatic IF3 Update Enabled for Message Object 103"]
    #[inline]
    pub fn if3upden103(&mut self) -> _IF3UPDEN103W {
        _IF3UPDEN103W { w: self }
    }
    #[doc = "Bit 5 - Automatic IF3 Update Enabled for Message Object 102"]
    #[inline]
    pub fn if3upden102(&mut self) -> _IF3UPDEN102W {
        _IF3UPDEN102W { w: self }
    }
    #[doc = "Bit 4 - Automatic IF3 Update Enabled for Message Object 101"]
    #[inline]
    pub fn if3upden101(&mut self) -> _IF3UPDEN101W {
        _IF3UPDEN101W { w: self }
    }
    #[doc = "Bit 3 - Automatic IF3 Update Enabled for Message Object 100"]
    #[inline]
    pub fn if3upden100(&mut self) -> _IF3UPDEN100W {
        _IF3UPDEN100W { w: self }
    }
    #[doc = "Bit 2 - Automatic IF3 Update Enabled for Message Object 99"]
    #[inline]
    pub fn if3upden99(&mut self) -> _IF3UPDEN99W {
        _IF3UPDEN99W { w: self }
    }
    #[doc = "Bit 1 - Automatic IF3 Update Enabled for Message Object 98"]
    #[inline]
    pub fn if3upden98(&mut self) -> _IF3UPDEN98W {
        _IF3UPDEN98W { w: self }
    }
    #[doc = "Bit 0 - Automatic IF3 Update Enabled for Message Object 97"]
    #[inline]
    pub fn if3upden97(&mut self) -> _IF3UPDEN97W {
        _IF3UPDEN97W { w: self }
    }
}
