#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF3UPD34 {
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
pub struct IF3UPDEN64R {
    bits: bool,
}
impl IF3UPDEN64R {
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
pub struct IF3UPDEN63R {
    bits: bool,
}
impl IF3UPDEN63R {
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
pub struct IF3UPDEN62R {
    bits: bool,
}
impl IF3UPDEN62R {
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
pub struct IF3UPDEN61R {
    bits: bool,
}
impl IF3UPDEN61R {
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
pub struct IF3UPDEN60R {
    bits: bool,
}
impl IF3UPDEN60R {
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
pub struct IF3UPDEN59R {
    bits: bool,
}
impl IF3UPDEN59R {
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
pub struct IF3UPDEN58R {
    bits: bool,
}
impl IF3UPDEN58R {
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
pub struct IF3UPDEN57R {
    bits: bool,
}
impl IF3UPDEN57R {
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
pub struct IF3UPDEN56R {
    bits: bool,
}
impl IF3UPDEN56R {
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
pub struct IF3UPDEN55R {
    bits: bool,
}
impl IF3UPDEN55R {
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
pub struct IF3UPDEN54R {
    bits: bool,
}
impl IF3UPDEN54R {
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
pub struct IF3UPDEN53R {
    bits: bool,
}
impl IF3UPDEN53R {
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
pub struct IF3UPDEN52R {
    bits: bool,
}
impl IF3UPDEN52R {
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
pub struct IF3UPDEN51R {
    bits: bool,
}
impl IF3UPDEN51R {
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
pub struct IF3UPDEN50R {
    bits: bool,
}
impl IF3UPDEN50R {
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
pub struct IF3UPDEN49R {
    bits: bool,
}
impl IF3UPDEN49R {
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
pub struct IF3UPDEN48R {
    bits: bool,
}
impl IF3UPDEN48R {
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
pub struct IF3UPDEN47R {
    bits: bool,
}
impl IF3UPDEN47R {
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
pub struct IF3UPDEN46R {
    bits: bool,
}
impl IF3UPDEN46R {
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
pub struct IF3UPDEN45R {
    bits: bool,
}
impl IF3UPDEN45R {
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
pub struct IF3UPDEN44R {
    bits: bool,
}
impl IF3UPDEN44R {
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
pub struct IF3UPDEN43R {
    bits: bool,
}
impl IF3UPDEN43R {
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
pub struct IF3UPDEN42R {
    bits: bool,
}
impl IF3UPDEN42R {
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
pub struct IF3UPDEN41R {
    bits: bool,
}
impl IF3UPDEN41R {
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
pub struct IF3UPDEN40R {
    bits: bool,
}
impl IF3UPDEN40R {
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
pub struct IF3UPDEN39R {
    bits: bool,
}
impl IF3UPDEN39R {
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
pub struct IF3UPDEN38R {
    bits: bool,
}
impl IF3UPDEN38R {
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
pub struct IF3UPDEN37R {
    bits: bool,
}
impl IF3UPDEN37R {
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
pub struct IF3UPDEN36R {
    bits: bool,
}
impl IF3UPDEN36R {
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
pub struct IF3UPDEN35R {
    bits: bool,
}
impl IF3UPDEN35R {
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
pub struct IF3UPDEN34R {
    bits: bool,
}
impl IF3UPDEN34R {
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
pub struct IF3UPDEN33R {
    bits: bool,
}
impl IF3UPDEN33R {
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
pub struct _IF3UPDEN64W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN64W<'a> {
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
pub struct _IF3UPDEN63W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN63W<'a> {
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
pub struct _IF3UPDEN62W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN62W<'a> {
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
pub struct _IF3UPDEN61W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN61W<'a> {
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
pub struct _IF3UPDEN60W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN60W<'a> {
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
pub struct _IF3UPDEN59W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN59W<'a> {
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
pub struct _IF3UPDEN58W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN58W<'a> {
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
pub struct _IF3UPDEN57W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN57W<'a> {
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
pub struct _IF3UPDEN56W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN56W<'a> {
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
pub struct _IF3UPDEN55W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN55W<'a> {
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
pub struct _IF3UPDEN54W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN54W<'a> {
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
pub struct _IF3UPDEN53W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN53W<'a> {
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
pub struct _IF3UPDEN52W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN52W<'a> {
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
pub struct _IF3UPDEN51W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN51W<'a> {
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
pub struct _IF3UPDEN50W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN50W<'a> {
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
pub struct _IF3UPDEN49W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN49W<'a> {
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
pub struct _IF3UPDEN48W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN48W<'a> {
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
pub struct _IF3UPDEN47W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN47W<'a> {
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
pub struct _IF3UPDEN46W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN46W<'a> {
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
pub struct _IF3UPDEN45W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN45W<'a> {
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
pub struct _IF3UPDEN44W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN44W<'a> {
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
pub struct _IF3UPDEN43W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN43W<'a> {
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
pub struct _IF3UPDEN42W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN42W<'a> {
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
pub struct _IF3UPDEN41W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN41W<'a> {
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
pub struct _IF3UPDEN40W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN40W<'a> {
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
pub struct _IF3UPDEN39W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN39W<'a> {
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
pub struct _IF3UPDEN38W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN38W<'a> {
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
pub struct _IF3UPDEN37W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN37W<'a> {
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
pub struct _IF3UPDEN36W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN36W<'a> {
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
pub struct _IF3UPDEN35W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN35W<'a> {
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
pub struct _IF3UPDEN34W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN34W<'a> {
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
pub struct _IF3UPDEN33W<'a> {
    w: &'a mut W,
}
impl<'a> _IF3UPDEN33W<'a> {
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
    #[doc = "Bit 31 - Automatic IF3 Update Enabled for Message Object 64"]
    #[inline]
    pub fn if3upden64(&self) -> IF3UPDEN64R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN64R { bits }
    }
    #[doc = "Bit 30 - Automatic IF3 Update Enabled for Message Object 63"]
    #[inline]
    pub fn if3upden63(&self) -> IF3UPDEN63R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN63R { bits }
    }
    #[doc = "Bit 29 - Automatic IF3 Update Enabled for Message Object 62"]
    #[inline]
    pub fn if3upden62(&self) -> IF3UPDEN62R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN62R { bits }
    }
    #[doc = "Bit 28 - Automatic IF3 Update Enabled for Message Object 61"]
    #[inline]
    pub fn if3upden61(&self) -> IF3UPDEN61R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN61R { bits }
    }
    #[doc = "Bit 27 - Automatic IF3 Update Enabled for Message Object 60"]
    #[inline]
    pub fn if3upden60(&self) -> IF3UPDEN60R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN60R { bits }
    }
    #[doc = "Bit 26 - Automatic IF3 Update Enabled for Message Object 59"]
    #[inline]
    pub fn if3upden59(&self) -> IF3UPDEN59R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN59R { bits }
    }
    #[doc = "Bit 25 - Automatic IF3 Update Enabled for Message Object 58"]
    #[inline]
    pub fn if3upden58(&self) -> IF3UPDEN58R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN58R { bits }
    }
    #[doc = "Bit 24 - Automatic IF3 Update Enabled for Message Object 57"]
    #[inline]
    pub fn if3upden57(&self) -> IF3UPDEN57R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN57R { bits }
    }
    #[doc = "Bit 23 - Automatic IF3 Update Enabled for Message Object 56"]
    #[inline]
    pub fn if3upden56(&self) -> IF3UPDEN56R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN56R { bits }
    }
    #[doc = "Bit 22 - Automatic IF3 Update Enabled for Message Object 55"]
    #[inline]
    pub fn if3upden55(&self) -> IF3UPDEN55R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN55R { bits }
    }
    #[doc = "Bit 21 - Automatic IF3 Update Enabled for Message Object 54"]
    #[inline]
    pub fn if3upden54(&self) -> IF3UPDEN54R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN54R { bits }
    }
    #[doc = "Bit 20 - Automatic IF3 Update Enabled for Message Object 53"]
    #[inline]
    pub fn if3upden53(&self) -> IF3UPDEN53R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN53R { bits }
    }
    #[doc = "Bit 19 - Automatic IF3 Update Enabled for Message Object 52"]
    #[inline]
    pub fn if3upden52(&self) -> IF3UPDEN52R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN52R { bits }
    }
    #[doc = "Bit 18 - Automatic IF3 Update Enabled for Message Object 51"]
    #[inline]
    pub fn if3upden51(&self) -> IF3UPDEN51R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN51R { bits }
    }
    #[doc = "Bit 17 - Automatic IF3 Update Enabled for Message Object 50"]
    #[inline]
    pub fn if3upden50(&self) -> IF3UPDEN50R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN50R { bits }
    }
    #[doc = "Bit 16 - Automatic IF3 Update Enabled for Message Object 49"]
    #[inline]
    pub fn if3upden49(&self) -> IF3UPDEN49R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN49R { bits }
    }
    #[doc = "Bit 15 - Automatic IF3 Update Enabled for Message Object 48"]
    #[inline]
    pub fn if3upden48(&self) -> IF3UPDEN48R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN48R { bits }
    }
    #[doc = "Bit 14 - Automatic IF3 Update Enabled for Message Object 47"]
    #[inline]
    pub fn if3upden47(&self) -> IF3UPDEN47R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN47R { bits }
    }
    #[doc = "Bit 13 - Automatic IF3 Update Enabled for Message Object 46"]
    #[inline]
    pub fn if3upden46(&self) -> IF3UPDEN46R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN46R { bits }
    }
    #[doc = "Bit 12 - Automatic IF3 Update Enabled for Message Object 45"]
    #[inline]
    pub fn if3upden45(&self) -> IF3UPDEN45R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN45R { bits }
    }
    #[doc = "Bit 11 - Automatic IF3 Update Enabled for Message Object 44"]
    #[inline]
    pub fn if3upden44(&self) -> IF3UPDEN44R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN44R { bits }
    }
    #[doc = "Bit 10 - Automatic IF3 Update Enabled for Message Object 43"]
    #[inline]
    pub fn if3upden43(&self) -> IF3UPDEN43R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN43R { bits }
    }
    #[doc = "Bit 9 - Automatic IF3 Update Enabled for Message Object 42"]
    #[inline]
    pub fn if3upden42(&self) -> IF3UPDEN42R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN42R { bits }
    }
    #[doc = "Bit 8 - Automatic IF3 Update Enabled for Message Object 41"]
    #[inline]
    pub fn if3upden41(&self) -> IF3UPDEN41R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN41R { bits }
    }
    #[doc = "Bit 7 - Automatic IF3 Update Enabled for Message Object 40"]
    #[inline]
    pub fn if3upden40(&self) -> IF3UPDEN40R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN40R { bits }
    }
    #[doc = "Bit 6 - Automatic IF3 Update Enabled for Message Object 39"]
    #[inline]
    pub fn if3upden39(&self) -> IF3UPDEN39R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN39R { bits }
    }
    #[doc = "Bit 5 - Automatic IF3 Update Enabled for Message Object 38"]
    #[inline]
    pub fn if3upden38(&self) -> IF3UPDEN38R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN38R { bits }
    }
    #[doc = "Bit 4 - Automatic IF3 Update Enabled for Message Object 37"]
    #[inline]
    pub fn if3upden37(&self) -> IF3UPDEN37R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN37R { bits }
    }
    #[doc = "Bit 3 - Automatic IF3 Update Enabled for Message Object 36"]
    #[inline]
    pub fn if3upden36(&self) -> IF3UPDEN36R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN36R { bits }
    }
    #[doc = "Bit 2 - Automatic IF3 Update Enabled for Message Object 35"]
    #[inline]
    pub fn if3upden35(&self) -> IF3UPDEN35R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN35R { bits }
    }
    #[doc = "Bit 1 - Automatic IF3 Update Enabled for Message Object 34"]
    #[inline]
    pub fn if3upden34(&self) -> IF3UPDEN34R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN34R { bits }
    }
    #[doc = "Bit 0 - Automatic IF3 Update Enabled for Message Object 33"]
    #[inline]
    pub fn if3upden33(&self) -> IF3UPDEN33R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IF3UPDEN33R { bits }
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
    #[doc = "Bit 31 - Automatic IF3 Update Enabled for Message Object 64"]
    #[inline]
    pub fn if3upden64(&mut self) -> _IF3UPDEN64W {
        _IF3UPDEN64W { w: self }
    }
    #[doc = "Bit 30 - Automatic IF3 Update Enabled for Message Object 63"]
    #[inline]
    pub fn if3upden63(&mut self) -> _IF3UPDEN63W {
        _IF3UPDEN63W { w: self }
    }
    #[doc = "Bit 29 - Automatic IF3 Update Enabled for Message Object 62"]
    #[inline]
    pub fn if3upden62(&mut self) -> _IF3UPDEN62W {
        _IF3UPDEN62W { w: self }
    }
    #[doc = "Bit 28 - Automatic IF3 Update Enabled for Message Object 61"]
    #[inline]
    pub fn if3upden61(&mut self) -> _IF3UPDEN61W {
        _IF3UPDEN61W { w: self }
    }
    #[doc = "Bit 27 - Automatic IF3 Update Enabled for Message Object 60"]
    #[inline]
    pub fn if3upden60(&mut self) -> _IF3UPDEN60W {
        _IF3UPDEN60W { w: self }
    }
    #[doc = "Bit 26 - Automatic IF3 Update Enabled for Message Object 59"]
    #[inline]
    pub fn if3upden59(&mut self) -> _IF3UPDEN59W {
        _IF3UPDEN59W { w: self }
    }
    #[doc = "Bit 25 - Automatic IF3 Update Enabled for Message Object 58"]
    #[inline]
    pub fn if3upden58(&mut self) -> _IF3UPDEN58W {
        _IF3UPDEN58W { w: self }
    }
    #[doc = "Bit 24 - Automatic IF3 Update Enabled for Message Object 57"]
    #[inline]
    pub fn if3upden57(&mut self) -> _IF3UPDEN57W {
        _IF3UPDEN57W { w: self }
    }
    #[doc = "Bit 23 - Automatic IF3 Update Enabled for Message Object 56"]
    #[inline]
    pub fn if3upden56(&mut self) -> _IF3UPDEN56W {
        _IF3UPDEN56W { w: self }
    }
    #[doc = "Bit 22 - Automatic IF3 Update Enabled for Message Object 55"]
    #[inline]
    pub fn if3upden55(&mut self) -> _IF3UPDEN55W {
        _IF3UPDEN55W { w: self }
    }
    #[doc = "Bit 21 - Automatic IF3 Update Enabled for Message Object 54"]
    #[inline]
    pub fn if3upden54(&mut self) -> _IF3UPDEN54W {
        _IF3UPDEN54W { w: self }
    }
    #[doc = "Bit 20 - Automatic IF3 Update Enabled for Message Object 53"]
    #[inline]
    pub fn if3upden53(&mut self) -> _IF3UPDEN53W {
        _IF3UPDEN53W { w: self }
    }
    #[doc = "Bit 19 - Automatic IF3 Update Enabled for Message Object 52"]
    #[inline]
    pub fn if3upden52(&mut self) -> _IF3UPDEN52W {
        _IF3UPDEN52W { w: self }
    }
    #[doc = "Bit 18 - Automatic IF3 Update Enabled for Message Object 51"]
    #[inline]
    pub fn if3upden51(&mut self) -> _IF3UPDEN51W {
        _IF3UPDEN51W { w: self }
    }
    #[doc = "Bit 17 - Automatic IF3 Update Enabled for Message Object 50"]
    #[inline]
    pub fn if3upden50(&mut self) -> _IF3UPDEN50W {
        _IF3UPDEN50W { w: self }
    }
    #[doc = "Bit 16 - Automatic IF3 Update Enabled for Message Object 49"]
    #[inline]
    pub fn if3upden49(&mut self) -> _IF3UPDEN49W {
        _IF3UPDEN49W { w: self }
    }
    #[doc = "Bit 15 - Automatic IF3 Update Enabled for Message Object 48"]
    #[inline]
    pub fn if3upden48(&mut self) -> _IF3UPDEN48W {
        _IF3UPDEN48W { w: self }
    }
    #[doc = "Bit 14 - Automatic IF3 Update Enabled for Message Object 47"]
    #[inline]
    pub fn if3upden47(&mut self) -> _IF3UPDEN47W {
        _IF3UPDEN47W { w: self }
    }
    #[doc = "Bit 13 - Automatic IF3 Update Enabled for Message Object 46"]
    #[inline]
    pub fn if3upden46(&mut self) -> _IF3UPDEN46W {
        _IF3UPDEN46W { w: self }
    }
    #[doc = "Bit 12 - Automatic IF3 Update Enabled for Message Object 45"]
    #[inline]
    pub fn if3upden45(&mut self) -> _IF3UPDEN45W {
        _IF3UPDEN45W { w: self }
    }
    #[doc = "Bit 11 - Automatic IF3 Update Enabled for Message Object 44"]
    #[inline]
    pub fn if3upden44(&mut self) -> _IF3UPDEN44W {
        _IF3UPDEN44W { w: self }
    }
    #[doc = "Bit 10 - Automatic IF3 Update Enabled for Message Object 43"]
    #[inline]
    pub fn if3upden43(&mut self) -> _IF3UPDEN43W {
        _IF3UPDEN43W { w: self }
    }
    #[doc = "Bit 9 - Automatic IF3 Update Enabled for Message Object 42"]
    #[inline]
    pub fn if3upden42(&mut self) -> _IF3UPDEN42W {
        _IF3UPDEN42W { w: self }
    }
    #[doc = "Bit 8 - Automatic IF3 Update Enabled for Message Object 41"]
    #[inline]
    pub fn if3upden41(&mut self) -> _IF3UPDEN41W {
        _IF3UPDEN41W { w: self }
    }
    #[doc = "Bit 7 - Automatic IF3 Update Enabled for Message Object 40"]
    #[inline]
    pub fn if3upden40(&mut self) -> _IF3UPDEN40W {
        _IF3UPDEN40W { w: self }
    }
    #[doc = "Bit 6 - Automatic IF3 Update Enabled for Message Object 39"]
    #[inline]
    pub fn if3upden39(&mut self) -> _IF3UPDEN39W {
        _IF3UPDEN39W { w: self }
    }
    #[doc = "Bit 5 - Automatic IF3 Update Enabled for Message Object 38"]
    #[inline]
    pub fn if3upden38(&mut self) -> _IF3UPDEN38W {
        _IF3UPDEN38W { w: self }
    }
    #[doc = "Bit 4 - Automatic IF3 Update Enabled for Message Object 37"]
    #[inline]
    pub fn if3upden37(&mut self) -> _IF3UPDEN37W {
        _IF3UPDEN37W { w: self }
    }
    #[doc = "Bit 3 - Automatic IF3 Update Enabled for Message Object 36"]
    #[inline]
    pub fn if3upden36(&mut self) -> _IF3UPDEN36W {
        _IF3UPDEN36W { w: self }
    }
    #[doc = "Bit 2 - Automatic IF3 Update Enabled for Message Object 35"]
    #[inline]
    pub fn if3upden35(&mut self) -> _IF3UPDEN35W {
        _IF3UPDEN35W { w: self }
    }
    #[doc = "Bit 1 - Automatic IF3 Update Enabled for Message Object 34"]
    #[inline]
    pub fn if3upden34(&mut self) -> _IF3UPDEN34W {
        _IF3UPDEN34W { w: self }
    }
    #[doc = "Bit 0 - Automatic IF3 Update Enabled for Message Object 33"]
    #[inline]
    pub fn if3upden33(&mut self) -> _IF3UPDEN33W {
        _IF3UPDEN33W { w: self }
    }
}
