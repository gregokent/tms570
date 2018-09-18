#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACCONTROL {
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
    bits: u16,
}
impl _RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD2R {
    bits: bool,
}
impl _RSVD2R {
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
pub struct _RSVD3R {
    bits: bool,
}
impl _RSVD3R {
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
pub struct _RSVD4R {
    bits: bool,
}
impl _RSVD4R {
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
#[doc = "Possible values of the field `RMIISPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMIISPEEDR {
    #[doc = "no description"]
    _10MBIT,
    #[doc = "no description"]
    _100MBIT,
}
impl RMIISPEEDR {
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
            RMIISPEEDR::_10MBIT => false,
            RMIISPEEDR::_100MBIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMIISPEEDR {
        match value {
            false => RMIISPEEDR::_10MBIT,
            true => RMIISPEEDR::_100MBIT,
        }
    }
    #[doc = "Checks if the value of the field is `_10MBIT`"]
    #[inline]
    pub fn is_10mbit(&self) -> bool {
        *self == RMIISPEEDR::_10MBIT
    }
    #[doc = "Checks if the value of the field is `_100MBIT`"]
    #[inline]
    pub fn is_100mbit(&self) -> bool {
        *self == RMIISPEEDR::_100MBIT
    }
}
#[doc = "Possible values of the field `RXOFFLENBLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOFFLENBLOCKR {
    #[doc = "Do not block the DMA writes to the receive buffer descriptor offset/buffer length word"]
    NOBLOCK,
    #[doc = "Block all EMAC DMA controller writes to the receive buffer descriptor offset/buffer length words during CPPI packet processing. When this bit is set, the EMAC will never write the third word to any receive buffer descriptor"]
    BLOCK,
}
impl RXOFFLENBLOCKR {
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
            RXOFFLENBLOCKR::NOBLOCK => false,
            RXOFFLENBLOCKR::BLOCK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXOFFLENBLOCKR {
        match value {
            false => RXOFFLENBLOCKR::NOBLOCK,
            true => RXOFFLENBLOCKR::BLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `NOBLOCK`"]
    #[inline]
    pub fn is_noblock(&self) -> bool {
        *self == RXOFFLENBLOCKR::NOBLOCK
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline]
    pub fn is_block(&self) -> bool {
        *self == RXOFFLENBLOCKR::BLOCK
    }
}
#[doc = "Possible values of the field `RXOWNERSHIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOWNERSHIPR {
    #[doc = "The EMAC writes the RX ownership bit to zero at the end of packet processing "]
    ZERO,
    #[doc = "The EMAC writes the RX ownership bit to one at the end of packet processing "]
    ONE,
}
impl RXOWNERSHIPR {
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
            RXOWNERSHIPR::ZERO => false,
            RXOWNERSHIPR::ONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXOWNERSHIPR {
        match value {
            false => RXOWNERSHIPR::ZERO,
            true => RXOWNERSHIPR::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == RXOWNERSHIPR::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == RXOWNERSHIPR::ONE
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD5R {
    bits: bool,
}
impl _RSVD5R {
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
#[doc = "Possible values of the field `CMDIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDIDLER {
    #[doc = "Idle not commanded"]
    DISABLE,
    #[doc = "Idle commanded (read IDLE is MACSTATUS)"]
    ENABLE,
}
impl CMDIDLER {
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
            CMDIDLER::DISABLE => false,
            CMDIDLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMDIDLER {
        match value {
            false => CMDIDLER::DISABLE,
            true => CMDIDLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CMDIDLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CMDIDLER::ENABLE
    }
}
#[doc = "Possible values of the field `TXSHORTGAPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSHORTGAPENR {
    #[doc = "TX with a short IPG is disabled"]
    DISABLE,
    #[doc = "TX with a short IPG (when TX_HSORT_GAP input is asserted) is enabled"]
    ENABLE,
}
impl TXSHORTGAPENR {
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
            TXSHORTGAPENR::DISABLE => false,
            TXSHORTGAPENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSHORTGAPENR {
        match value {
            false => TXSHORTGAPENR::DISABLE,
            true => TXSHORTGAPENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXSHORTGAPENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXSHORTGAPENR::ENABLE
    }
}
#[doc = "Possible values of the field `TXPTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPTYPER {
    #[doc = "The queue uses a round-robin scheme to select the next channel for transmission"]
    RROBIN,
    #[doc = "The queue uses a fixed-priority (channel 7 highest priority) scheme to select the next channel for transmission"]
    CHANNELPRI,
}
impl TXPTYPER {
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
            TXPTYPER::RROBIN => false,
            TXPTYPER::CHANNELPRI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXPTYPER {
        match value {
            false => TXPTYPER::RROBIN,
            true => TXPTYPER::CHANNELPRI,
        }
    }
    #[doc = "Checks if the value of the field is `RROBIN`"]
    #[inline]
    pub fn is_rrobin(&self) -> bool {
        *self == TXPTYPER::RROBIN
    }
    #[doc = "Checks if the value of the field is `CHANNELPRI`"]
    #[inline]
    pub fn is_channelpri(&self) -> bool {
        *self == TXPTYPER::CHANNELPRI
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD6R {
    bits: bool,
}
impl _RSVD6R {
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
pub struct _RSVD7R {
    bits: bool,
}
impl _RSVD7R {
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
#[doc = "Possible values of the field `TXPACE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPACER {
    #[doc = "TX pacing is disabled"]
    DISABLE,
    #[doc = "TX pacing is enabled"]
    ENABLE,
}
impl TXPACER {
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
            TXPACER::DISABLE => false,
            TXPACER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXPACER {
        match value {
            false => TXPACER::DISABLE,
            true => TXPACER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXPACER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXPACER::ENABLE
    }
}
#[doc = "Possible values of the field `GMIIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GMIIENR {
    #[doc = "GMII RX and TX are held in reset"]
    DISABLE,
    #[doc = "GMII RX and TX are released from reset"]
    ENABLE,
}
impl GMIIENR {
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
            GMIIENR::DISABLE => false,
            GMIIENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GMIIENR {
        match value {
            false => GMIIENR::DISABLE,
            true => GMIIENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == GMIIENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == GMIIENR::ENABLE
    }
}
#[doc = "Possible values of the field `TXFLOWEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFLOWENR {
    #[doc = "TX flow control is disabled. Full-duplex mode: incoming pause frames are not acted upon"]
    DISABLE,
    #[doc = "TX flow control is enabled. Full-duplex mode: incoming pause frames are acted upon"]
    ENABLE,
}
impl TXFLOWENR {
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
            TXFLOWENR::DISABLE => false,
            TXFLOWENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFLOWENR {
        match value {
            false => TXFLOWENR::DISABLE,
            true => TXFLOWENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXFLOWENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXFLOWENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXBUFFERFLOWEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBUFFERFLOWENR {
    #[doc = "RX flow control is disabled. Half-duplex mode: no flow control generated collisions are sent. Full-duplex mode: no outgoing pause frames are sent"]
    DISABLE,
    #[doc = "RX flow control is enabled. Half-duplex mode: collisions are initiated when receive flow control is triggered. Full-duplex mode: outgoing pause frames are sent when receive flow control is triggered"]
    ENABLE,
}
impl RXBUFFERFLOWENR {
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
            RXBUFFERFLOWENR::DISABLE => false,
            RXBUFFERFLOWENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXBUFFERFLOWENR {
        match value {
            false => RXBUFFERFLOWENR::DISABLE,
            true => RXBUFFERFLOWENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXBUFFERFLOWENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXBUFFERFLOWENR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD8R {
    bits: bool,
}
impl _RSVD8R {
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
#[doc = "Possible values of the field `LOOPBACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPBACKR {
    #[doc = "Loopback mode is disabled"]
    DISABLE,
    #[doc = "Loopback mode is enabled"]
    ENABLE,
}
impl LOOPBACKR {
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
            LOOPBACKR::DISABLE => false,
            LOOPBACKR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPBACKR {
        match value {
            false => LOOPBACKR::DISABLE,
            true => LOOPBACKR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == LOOPBACKR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == LOOPBACKR::ENABLE
    }
}
#[doc = "Possible values of the field `FULLDUPLEX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULLDUPLEXR {
    #[doc = "Half-duplex mode "]
    HALFDUPLEX,
    #[doc = "Full-duplex mode"]
    FULLDUPLEX,
}
impl FULLDUPLEXR {
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
            FULLDUPLEXR::HALFDUPLEX => false,
            FULLDUPLEXR::FULLDUPLEX => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FULLDUPLEXR {
        match value {
            false => FULLDUPLEXR::HALFDUPLEX,
            true => FULLDUPLEXR::FULLDUPLEX,
        }
    }
    #[doc = "Checks if the value of the field is `HALFDUPLEX`"]
    #[inline]
    pub fn is_halfduplex(&self) -> bool {
        *self == FULLDUPLEXR::HALFDUPLEX
    }
    #[doc = "Checks if the value of the field is `FULLDUPLEX`"]
    #[inline]
    pub fn is_fullduplex(&self) -> bool {
        *self == FULLDUPLEXR::FULLDUPLEX
    }
}
#[doc = r" Proxy"]
pub struct __RSVD1W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct __RSVD2W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD2W<'a> {
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
pub struct __RSVD3W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD3W<'a> {
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
pub struct __RSVD4W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD4W<'a> {
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
#[doc = "Values that can be written to the field `RMIISPEED`"]
pub enum RMIISPEEDW {
    #[doc = "no description"]
    _10MBIT,
    #[doc = "no description"]
    _100MBIT,
}
impl RMIISPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMIISPEEDW::_10MBIT => false,
            RMIISPEEDW::_100MBIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMIISPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _RMIISPEEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMIISPEEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn _10mbit(self) -> &'a mut W {
        self.variant(RMIISPEEDW::_10MBIT)
    }
    #[doc = "no description"]
    #[inline]
    pub fn _100mbit(self) -> &'a mut W {
        self.variant(RMIISPEEDW::_100MBIT)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXOFFLENBLOCK`"]
pub enum RXOFFLENBLOCKW {
    #[doc = "Do not block the DMA writes to the receive buffer descriptor offset/buffer length word"]
    NOBLOCK,
    #[doc = "Block all EMAC DMA controller writes to the receive buffer descriptor offset/buffer length words during CPPI packet processing. When this bit is set, the EMAC will never write the third word to any receive buffer descriptor"]
    BLOCK,
}
impl RXOFFLENBLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXOFFLENBLOCKW::NOBLOCK => false,
            RXOFFLENBLOCKW::BLOCK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXOFFLENBLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOFFLENBLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXOFFLENBLOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not block the DMA writes to the receive buffer descriptor offset/buffer length word"]
    #[inline]
    pub fn noblock(self) -> &'a mut W {
        self.variant(RXOFFLENBLOCKW::NOBLOCK)
    }
    #[doc = "Block all EMAC DMA controller writes to the receive buffer descriptor offset/buffer length words during CPPI packet processing. When this bit is set, the EMAC will never write the third word to any receive buffer descriptor"]
    #[inline]
    pub fn block(self) -> &'a mut W {
        self.variant(RXOFFLENBLOCKW::BLOCK)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXOWNERSHIP`"]
pub enum RXOWNERSHIPW {
    #[doc = "The EMAC writes the RX ownership bit to zero at the end of packet processing "]
    ZERO,
    #[doc = "The EMAC writes the RX ownership bit to one at the end of packet processing "]
    ONE,
}
impl RXOWNERSHIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXOWNERSHIPW::ZERO => false,
            RXOWNERSHIPW::ONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXOWNERSHIPW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOWNERSHIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXOWNERSHIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The EMAC writes the RX ownership bit to zero at the end of packet processing"]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(RXOWNERSHIPW::ZERO)
    }
    #[doc = "The EMAC writes the RX ownership bit to one at the end of packet processing"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(RXOWNERSHIPW::ONE)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct __RSVD5W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD5W<'a> {
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
#[doc = "Values that can be written to the field `CMDIDLE`"]
pub enum CMDIDLEW {
    #[doc = "Idle not commanded"]
    DISABLE,
    #[doc = "Idle commanded (read IDLE is MACSTATUS)"]
    ENABLE,
}
impl CMDIDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMDIDLEW::DISABLE => false,
            CMDIDLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDIDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDIDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Idle not commanded"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CMDIDLEW::DISABLE)
    }
    #[doc = "Idle commanded (read IDLE is MACSTATUS)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CMDIDLEW::ENABLE)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSHORTGAPEN`"]
pub enum TXSHORTGAPENW {
    #[doc = "TX with a short IPG is disabled"]
    DISABLE,
    #[doc = "TX with a short IPG (when TX_HSORT_GAP input is asserted) is enabled"]
    ENABLE,
}
impl TXSHORTGAPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSHORTGAPENW::DISABLE => false,
            TXSHORTGAPENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSHORTGAPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSHORTGAPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSHORTGAPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX with a short IPG is disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXSHORTGAPENW::DISABLE)
    }
    #[doc = "TX with a short IPG (when TX_HSORT_GAP input is asserted) is enabled"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXSHORTGAPENW::ENABLE)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXPTYPE`"]
pub enum TXPTYPEW {
    #[doc = "The queue uses a round-robin scheme to select the next channel for transmission"]
    RROBIN,
    #[doc = "The queue uses a fixed-priority (channel 7 highest priority) scheme to select the next channel for transmission"]
    CHANNELPRI,
}
impl TXPTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXPTYPEW::RROBIN => false,
            TXPTYPEW::CHANNELPRI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPTYPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The queue uses a round-robin scheme to select the next channel for transmission"]
    #[inline]
    pub fn rrobin(self) -> &'a mut W {
        self.variant(TXPTYPEW::RROBIN)
    }
    #[doc = "The queue uses a fixed-priority (channel 7 highest priority) scheme to select the next channel for transmission"]
    #[inline]
    pub fn channelpri(self) -> &'a mut W {
        self.variant(TXPTYPEW::CHANNELPRI)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct __RSVD6W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD6W<'a> {
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
pub struct __RSVD7W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD7W<'a> {
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
#[doc = "Values that can be written to the field `TXPACE`"]
pub enum TXPACEW {
    #[doc = "TX pacing is disabled"]
    DISABLE,
    #[doc = "TX pacing is enabled"]
    ENABLE,
}
impl TXPACEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXPACEW::DISABLE => false,
            TXPACEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPACEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPACEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPACEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX pacing is disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXPACEW::DISABLE)
    }
    #[doc = "TX pacing is enabled"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXPACEW::ENABLE)
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
#[doc = "Values that can be written to the field `GMIIEN`"]
pub enum GMIIENW {
    #[doc = "GMII RX and TX are held in reset"]
    DISABLE,
    #[doc = "GMII RX and TX are released from reset"]
    ENABLE,
}
impl GMIIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GMIIENW::DISABLE => false,
            GMIIENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GMIIENW<'a> {
    w: &'a mut W,
}
impl<'a> _GMIIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GMIIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GMII RX and TX are held in reset"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(GMIIENW::DISABLE)
    }
    #[doc = "GMII RX and TX are released from reset"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(GMIIENW::ENABLE)
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
#[doc = "Values that can be written to the field `TXFLOWEN`"]
pub enum TXFLOWENW {
    #[doc = "TX flow control is disabled. Full-duplex mode: incoming pause frames are not acted upon"]
    DISABLE,
    #[doc = "TX flow control is enabled. Full-duplex mode: incoming pause frames are acted upon"]
    ENABLE,
}
impl TXFLOWENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFLOWENW::DISABLE => false,
            TXFLOWENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFLOWENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFLOWENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFLOWENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX flow control is disabled. Full-duplex mode: incoming pause frames are not acted upon"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXFLOWENW::DISABLE)
    }
    #[doc = "TX flow control is enabled. Full-duplex mode: incoming pause frames are acted upon"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXFLOWENW::ENABLE)
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
#[doc = "Values that can be written to the field `RXBUFFERFLOWEN`"]
pub enum RXBUFFERFLOWENW {
    #[doc = "RX flow control is disabled. Half-duplex mode: no flow control generated collisions are sent. Full-duplex mode: no outgoing pause frames are sent"]
    DISABLE,
    #[doc = "RX flow control is enabled. Half-duplex mode: collisions are initiated when receive flow control is triggered. Full-duplex mode: outgoing pause frames are sent when receive flow control is triggered"]
    ENABLE,
}
impl RXBUFFERFLOWENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXBUFFERFLOWENW::DISABLE => false,
            RXBUFFERFLOWENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXBUFFERFLOWENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBUFFERFLOWENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXBUFFERFLOWENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX flow control is disabled. Half-duplex mode: no flow control generated collisions are sent. Full-duplex mode: no outgoing pause frames are sent"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXBUFFERFLOWENW::DISABLE)
    }
    #[doc = "RX flow control is enabled. Half-duplex mode: collisions are initiated when receive flow control is triggered. Full-duplex mode: outgoing pause frames are sent when receive flow control is triggered"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXBUFFERFLOWENW::ENABLE)
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
#[doc = r" Proxy"]
pub struct __RSVD8W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD8W<'a> {
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
#[doc = "Values that can be written to the field `LOOPBACK`"]
pub enum LOOPBACKW {
    #[doc = "Loopback mode is disabled"]
    DISABLE,
    #[doc = "Loopback mode is enabled"]
    ENABLE,
}
impl LOOPBACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPBACKW::DISABLE => false,
            LOOPBACKW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPBACKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPBACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPBACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loopback mode is disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOOPBACKW::DISABLE)
    }
    #[doc = "Loopback mode is enabled"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOOPBACKW::ENABLE)
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
#[doc = "Values that can be written to the field `FULLDUPLEX`"]
pub enum FULLDUPLEXW {
    #[doc = "Half-duplex mode "]
    HALFDUPLEX,
    #[doc = "Full-duplex mode"]
    FULLDUPLEX,
}
impl FULLDUPLEXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FULLDUPLEXW::HALFDUPLEX => false,
            FULLDUPLEXW::FULLDUPLEX => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FULLDUPLEXW<'a> {
    w: &'a mut W,
}
impl<'a> _FULLDUPLEXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FULLDUPLEXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Half-duplex mode"]
    #[inline]
    pub fn halfduplex(self) -> &'a mut W {
        self.variant(FULLDUPLEXW::HALFDUPLEX)
    }
    #[doc = "Full-duplex mode"]
    #[inline]
    pub fn fullduplex(self) -> &'a mut W {
        self.variant(FULLDUPLEXW::FULLDUPLEX)
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
    #[doc = "Bits 19:31 - Reserved"]
    #[inline]
    pub fn _rsvd1(&self) -> _RSVD1R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        _RSVD1R { bits }
    }
    #[doc = "Bit 18 - Enables external select of Dupex/Gigabit modes"]
    #[inline]
    pub fn _rsvd2(&self) -> _RSVD2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD2R { bits }
    }
    #[doc = "Bit 17 - Gigabit force mode"]
    #[inline]
    pub fn _rsvd3(&self) -> _RSVD3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD3R { bits }
    }
    #[doc = "Bit 16 - Interface Control B"]
    #[inline]
    pub fn _rsvd4(&self) -> _RSVD4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD4R { bits }
    }
    #[doc = "Bit 15 - RMII 10/100 Speed Select (IFCTLA)"]
    #[inline]
    pub fn rmiispeed(&self) -> RMIISPEEDR {
        RMIISPEEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Receive Offset / Length word write block"]
    #[inline]
    pub fn rxofflenblock(&self) -> RXOFFLENBLOCKR {
        RXOFFLENBLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Receive ownership write bit value"]
    #[inline]
    pub fn rxownership(&self) -> RXOWNERSHIPR {
        RXOWNERSHIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline]
    pub fn _rsvd5(&self) -> _RSVD5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD5R { bits }
    }
    #[doc = "Bit 11 - Command Idle"]
    #[inline]
    pub fn cmdidle(&self) -> CMDIDLER {
        CMDIDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transmit short gap enable"]
    #[inline]
    pub fn txshortgapen(&self) -> TXSHORTGAPENR {
        TXSHORTGAPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Transmit queue priority type"]
    #[inline]
    pub fn txptype(&self) -> TXPTYPER {
        TXPTYPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - FIFO RAM processor read/write enable (RAM test mode)"]
    #[inline]
    pub fn _rsvd6(&self) -> _RSVD6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD6R { bits }
    }
    #[doc = "Bit 7 - Gigabit mode"]
    #[inline]
    pub fn _rsvd7(&self) -> _RSVD7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD7R { bits }
    }
    #[doc = "Bit 6 - Transmit pacing enable"]
    #[inline]
    pub fn txpace(&self) -> TXPACER {
        TXPACER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - GMII enable"]
    #[inline]
    pub fn gmiien(&self) -> GMIIENR {
        GMIIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Transmit flow control enable"]
    #[inline]
    pub fn txflowen(&self) -> TXFLOWENR {
        TXFLOWENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Receive buffer flow control enable"]
    #[inline]
    pub fn rxbufferflowen(&self) -> RXBUFFERFLOWENR {
        RXBUFFERFLOWENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Manufacturing test mode"]
    #[inline]
    pub fn _rsvd8(&self) -> _RSVD8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD8R { bits }
    }
    #[doc = "Bit 1 - Loopback mode"]
    #[inline]
    pub fn loopback(&self) -> LOOPBACKR {
        LOOPBACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Full duplex mode; gigabit mode forces full duplex mode regardless of whether the fullduplex bit isset or not"]
    #[inline]
    pub fn fullduplex(&self) -> FULLDUPLEXR {
        FULLDUPLEXR::_from({
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
    #[doc = "Bits 19:31 - Reserved"]
    #[inline]
    pub fn _rsvd1(&mut self) -> __RSVD1W {
        __RSVD1W { w: self }
    }
    #[doc = "Bit 18 - Enables external select of Dupex/Gigabit modes"]
    #[inline]
    pub fn _rsvd2(&mut self) -> __RSVD2W {
        __RSVD2W { w: self }
    }
    #[doc = "Bit 17 - Gigabit force mode"]
    #[inline]
    pub fn _rsvd3(&mut self) -> __RSVD3W {
        __RSVD3W { w: self }
    }
    #[doc = "Bit 16 - Interface Control B"]
    #[inline]
    pub fn _rsvd4(&mut self) -> __RSVD4W {
        __RSVD4W { w: self }
    }
    #[doc = "Bit 15 - RMII 10/100 Speed Select (IFCTLA)"]
    #[inline]
    pub fn rmiispeed(&mut self) -> _RMIISPEEDW {
        _RMIISPEEDW { w: self }
    }
    #[doc = "Bit 14 - Receive Offset / Length word write block"]
    #[inline]
    pub fn rxofflenblock(&mut self) -> _RXOFFLENBLOCKW {
        _RXOFFLENBLOCKW { w: self }
    }
    #[doc = "Bit 13 - Receive ownership write bit value"]
    #[inline]
    pub fn rxownership(&mut self) -> _RXOWNERSHIPW {
        _RXOWNERSHIPW { w: self }
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline]
    pub fn _rsvd5(&mut self) -> __RSVD5W {
        __RSVD5W { w: self }
    }
    #[doc = "Bit 11 - Command Idle"]
    #[inline]
    pub fn cmdidle(&mut self) -> _CMDIDLEW {
        _CMDIDLEW { w: self }
    }
    #[doc = "Bit 10 - Transmit short gap enable"]
    #[inline]
    pub fn txshortgapen(&mut self) -> _TXSHORTGAPENW {
        _TXSHORTGAPENW { w: self }
    }
    #[doc = "Bit 9 - Transmit queue priority type"]
    #[inline]
    pub fn txptype(&mut self) -> _TXPTYPEW {
        _TXPTYPEW { w: self }
    }
    #[doc = "Bit 8 - FIFO RAM processor read/write enable (RAM test mode)"]
    #[inline]
    pub fn _rsvd6(&mut self) -> __RSVD6W {
        __RSVD6W { w: self }
    }
    #[doc = "Bit 7 - Gigabit mode"]
    #[inline]
    pub fn _rsvd7(&mut self) -> __RSVD7W {
        __RSVD7W { w: self }
    }
    #[doc = "Bit 6 - Transmit pacing enable"]
    #[inline]
    pub fn txpace(&mut self) -> _TXPACEW {
        _TXPACEW { w: self }
    }
    #[doc = "Bit 5 - GMII enable"]
    #[inline]
    pub fn gmiien(&mut self) -> _GMIIENW {
        _GMIIENW { w: self }
    }
    #[doc = "Bit 4 - Transmit flow control enable"]
    #[inline]
    pub fn txflowen(&mut self) -> _TXFLOWENW {
        _TXFLOWENW { w: self }
    }
    #[doc = "Bit 3 - Receive buffer flow control enable"]
    #[inline]
    pub fn rxbufferflowen(&mut self) -> _RXBUFFERFLOWENW {
        _RXBUFFERFLOWENW { w: self }
    }
    #[doc = "Bit 2 - Manufacturing test mode"]
    #[inline]
    pub fn _rsvd8(&mut self) -> __RSVD8W {
        __RSVD8W { w: self }
    }
    #[doc = "Bit 1 - Loopback mode"]
    #[inline]
    pub fn loopback(&mut self) -> _LOOPBACKW {
        _LOOPBACKW { w: self }
    }
    #[doc = "Bit 0 - Full duplex mode; gigabit mode forces full duplex mode regardless of whether the fullduplex bit isset or not"]
    #[inline]
    pub fn fullduplex(&mut self) -> _FULLDUPLEXW {
        _FULLDUPLEXW { w: self }
    }
}
