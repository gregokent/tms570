#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXMBPENABLE {
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
    bits: bool,
}
impl _RSVD1R {
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
#[doc = "Possible values of the field `RXPASSCRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPASSCRCR {
    #[doc = "Received CRC is discarded for all channels and is not included in the buffer descriptor packet length field"]
    DISCARD,
    #[doc = "Received CRC is transferred to memory for all channels and is included in the buffer descriptor packet length"]
    INCLUDE,
}
impl RXPASSCRCR {
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
            RXPASSCRCR::DISCARD => false,
            RXPASSCRCR::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPASSCRCR {
        match value {
            false => RXPASSCRCR::DISCARD,
            true => RXPASSCRCR::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `DISCARD`"]
    #[inline]
    pub fn is_discard(&self) -> bool {
        *self == RXPASSCRCR::DISCARD
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == RXPASSCRCR::INCLUDE
    }
}
#[doc = "Possible values of the field `RXQOSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXQOSENR {
    #[doc = "Receive QOS is disabled"]
    DISABLE,
    #[doc = "Receive QOS is enabled"]
    ENABLE,
}
impl RXQOSENR {
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
            RXQOSENR::DISABLE => false,
            RXQOSENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXQOSENR {
        match value {
            false => RXQOSENR::DISABLE,
            true => RXQOSENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXQOSENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXQOSENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXNOCHAIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNOCHAINR {
    #[doc = "Received frames can span multiple buffers"]
    DISABLE,
    #[doc = "RX DMA controller transfers each frame into a single buffer regardless of the frame or buffer size. All remaining frame data after the first buffer is discarded"]
    ENABLE,
}
impl RXNOCHAINR {
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
            RXNOCHAINR::DISABLE => false,
            RXNOCHAINR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXNOCHAINR {
        match value {
            false => RXNOCHAINR::DISABLE,
            true => RXNOCHAINR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXNOCHAINR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXNOCHAINR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD2R {
    bits: u8,
}
impl _RSVD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RXCMFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCMFENR {
    #[doc = "MAC control frames are filtered (but acted upon if enabled)"]
    DISABLE,
    #[doc = "MAC control frames are transferred to memory"]
    ENABLE,
}
impl RXCMFENR {
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
            RXCMFENR::DISABLE => false,
            RXCMFENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCMFENR {
        match value {
            false => RXCMFENR::DISABLE,
            true => RXCMFENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCMFENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCMFENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXCSFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCSFENR {
    #[doc = "Short frames are filtered"]
    DISABLE,
    #[doc = "Short frames are transferred to memory"]
    ENABLE,
}
impl RXCSFENR {
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
            RXCSFENR::DISABLE => false,
            RXCSFENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCSFENR {
        match value {
            false => RXCSFENR::DISABLE,
            true => RXCSFENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCSFENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCSFENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXCEFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCEFENR {
    #[doc = "Frames containing errors are filtered"]
    DISABLE,
    #[doc = "Frames containing errors are transferred to memory"]
    ENABLE,
}
impl RXCEFENR {
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
            RXCEFENR::DISABLE => false,
            RXCEFENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCEFENR {
        match value {
            false => RXCEFENR::DISABLE,
            true => RXCEFENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCEFENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCEFENR::ENABLE
    }
}
#[doc = "Possible values of the field `RXCAFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCAFENR {
    #[doc = "no description"]
    DISABLE,
    #[doc = "Frames that do not address match are transferred to the promiscuous channel selected by RXPROMCH bits"]
    ENABLE,
}
impl RXCAFENR {
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
            RXCAFENR::DISABLE => false,
            RXCAFENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCAFENR {
        match value {
            false => RXCAFENR::DISABLE,
            true => RXCAFENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXCAFENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXCAFENR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD3R {
    bits: u8,
}
impl _RSVD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RXPROMCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPROMCHR {
    #[doc = "no description"]
    CHA0,
    #[doc = "no description"]
    CHA1,
    #[doc = "no description"]
    CHA2,
    #[doc = "no description"]
    CHA3,
    #[doc = "no description"]
    CHA4,
    #[doc = "no description"]
    CHA5,
    #[doc = "no description"]
    CHA6,
    #[doc = "no description"]
    CHA7,
}
impl RXPROMCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXPROMCHR::CHA0 => 0,
            RXPROMCHR::CHA1 => 1,
            RXPROMCHR::CHA2 => 2,
            RXPROMCHR::CHA3 => 3,
            RXPROMCHR::CHA4 => 4,
            RXPROMCHR::CHA5 => 5,
            RXPROMCHR::CHA6 => 6,
            RXPROMCHR::CHA7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXPROMCHR {
        match value {
            0 => RXPROMCHR::CHA0,
            1 => RXPROMCHR::CHA1,
            2 => RXPROMCHR::CHA2,
            3 => RXPROMCHR::CHA3,
            4 => RXPROMCHR::CHA4,
            5 => RXPROMCHR::CHA5,
            6 => RXPROMCHR::CHA6,
            7 => RXPROMCHR::CHA7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHA0`"]
    #[inline]
    pub fn is_cha0(&self) -> bool {
        *self == RXPROMCHR::CHA0
    }
    #[doc = "Checks if the value of the field is `CHA1`"]
    #[inline]
    pub fn is_cha1(&self) -> bool {
        *self == RXPROMCHR::CHA1
    }
    #[doc = "Checks if the value of the field is `CHA2`"]
    #[inline]
    pub fn is_cha2(&self) -> bool {
        *self == RXPROMCHR::CHA2
    }
    #[doc = "Checks if the value of the field is `CHA3`"]
    #[inline]
    pub fn is_cha3(&self) -> bool {
        *self == RXPROMCHR::CHA3
    }
    #[doc = "Checks if the value of the field is `CHA4`"]
    #[inline]
    pub fn is_cha4(&self) -> bool {
        *self == RXPROMCHR::CHA4
    }
    #[doc = "Checks if the value of the field is `CHA5`"]
    #[inline]
    pub fn is_cha5(&self) -> bool {
        *self == RXPROMCHR::CHA5
    }
    #[doc = "Checks if the value of the field is `CHA6`"]
    #[inline]
    pub fn is_cha6(&self) -> bool {
        *self == RXPROMCHR::CHA6
    }
    #[doc = "Checks if the value of the field is `CHA7`"]
    #[inline]
    pub fn is_cha7(&self) -> bool {
        *self == RXPROMCHR::CHA7
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD4R {
    bits: u8,
}
impl _RSVD4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RXBROADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBROADENR {
    #[doc = "Broadcast frames are filtered"]
    DISABLE,
    #[doc = "Broadcast frames are copied to the channel selected by RXBROADCH bits"]
    ENABLE,
}
impl RXBROADENR {
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
            RXBROADENR::DISABLE => false,
            RXBROADENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXBROADENR {
        match value {
            false => RXBROADENR::DISABLE,
            true => RXBROADENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXBROADENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXBROADENR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD5R {
    bits: u8,
}
impl _RSVD5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RXBROADCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBROADCHR {
    #[doc = "no description"]
    CHA0,
    #[doc = "no description"]
    CHA1,
    #[doc = "no description"]
    CHA2,
    #[doc = "no description"]
    CHA3,
    #[doc = "no description"]
    CHA4,
    #[doc = "no description"]
    CHA5,
    #[doc = "no description"]
    CHA6,
    #[doc = "no description"]
    CHA7,
}
impl RXBROADCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXBROADCHR::CHA0 => 0,
            RXBROADCHR::CHA1 => 1,
            RXBROADCHR::CHA2 => 2,
            RXBROADCHR::CHA3 => 3,
            RXBROADCHR::CHA4 => 4,
            RXBROADCHR::CHA5 => 5,
            RXBROADCHR::CHA6 => 6,
            RXBROADCHR::CHA7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXBROADCHR {
        match value {
            0 => RXBROADCHR::CHA0,
            1 => RXBROADCHR::CHA1,
            2 => RXBROADCHR::CHA2,
            3 => RXBROADCHR::CHA3,
            4 => RXBROADCHR::CHA4,
            5 => RXBROADCHR::CHA5,
            6 => RXBROADCHR::CHA6,
            7 => RXBROADCHR::CHA7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHA0`"]
    #[inline]
    pub fn is_cha0(&self) -> bool {
        *self == RXBROADCHR::CHA0
    }
    #[doc = "Checks if the value of the field is `CHA1`"]
    #[inline]
    pub fn is_cha1(&self) -> bool {
        *self == RXBROADCHR::CHA1
    }
    #[doc = "Checks if the value of the field is `CHA2`"]
    #[inline]
    pub fn is_cha2(&self) -> bool {
        *self == RXBROADCHR::CHA2
    }
    #[doc = "Checks if the value of the field is `CHA3`"]
    #[inline]
    pub fn is_cha3(&self) -> bool {
        *self == RXBROADCHR::CHA3
    }
    #[doc = "Checks if the value of the field is `CHA4`"]
    #[inline]
    pub fn is_cha4(&self) -> bool {
        *self == RXBROADCHR::CHA4
    }
    #[doc = "Checks if the value of the field is `CHA5`"]
    #[inline]
    pub fn is_cha5(&self) -> bool {
        *self == RXBROADCHR::CHA5
    }
    #[doc = "Checks if the value of the field is `CHA6`"]
    #[inline]
    pub fn is_cha6(&self) -> bool {
        *self == RXBROADCHR::CHA6
    }
    #[doc = "Checks if the value of the field is `CHA7`"]
    #[inline]
    pub fn is_cha7(&self) -> bool {
        *self == RXBROADCHR::CHA7
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD6R {
    bits: u8,
}
impl _RSVD6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RXMULTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMULTENR {
    #[doc = "Multicast frames are filtered"]
    DISABLE,
    #[doc = "Multicast frames are copied to the channel selected by RXMULTCH bits"]
    ENABLE,
}
impl RXMULTENR {
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
            RXMULTENR::DISABLE => false,
            RXMULTENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXMULTENR {
        match value {
            false => RXMULTENR::DISABLE,
            true => RXMULTENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXMULTENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXMULTENR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct _RSVD7R {
    bits: u8,
}
impl _RSVD7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RXMULTCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMULTCHR {
    #[doc = "no description"]
    CHA0,
    #[doc = "no description"]
    CHA1,
    #[doc = "no description"]
    CHA2,
    #[doc = "no description"]
    CHA3,
    #[doc = "no description"]
    CHA4,
    #[doc = "no description"]
    CHA5,
    #[doc = "no description"]
    CHA6,
    #[doc = "no description"]
    CHA7,
}
impl RXMULTCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXMULTCHR::CHA0 => 0,
            RXMULTCHR::CHA1 => 1,
            RXMULTCHR::CHA2 => 2,
            RXMULTCHR::CHA3 => 3,
            RXMULTCHR::CHA4 => 4,
            RXMULTCHR::CHA5 => 5,
            RXMULTCHR::CHA6 => 6,
            RXMULTCHR::CHA7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXMULTCHR {
        match value {
            0 => RXMULTCHR::CHA0,
            1 => RXMULTCHR::CHA1,
            2 => RXMULTCHR::CHA2,
            3 => RXMULTCHR::CHA3,
            4 => RXMULTCHR::CHA4,
            5 => RXMULTCHR::CHA5,
            6 => RXMULTCHR::CHA6,
            7 => RXMULTCHR::CHA7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHA0`"]
    #[inline]
    pub fn is_cha0(&self) -> bool {
        *self == RXMULTCHR::CHA0
    }
    #[doc = "Checks if the value of the field is `CHA1`"]
    #[inline]
    pub fn is_cha1(&self) -> bool {
        *self == RXMULTCHR::CHA1
    }
    #[doc = "Checks if the value of the field is `CHA2`"]
    #[inline]
    pub fn is_cha2(&self) -> bool {
        *self == RXMULTCHR::CHA2
    }
    #[doc = "Checks if the value of the field is `CHA3`"]
    #[inline]
    pub fn is_cha3(&self) -> bool {
        *self == RXMULTCHR::CHA3
    }
    #[doc = "Checks if the value of the field is `CHA4`"]
    #[inline]
    pub fn is_cha4(&self) -> bool {
        *self == RXMULTCHR::CHA4
    }
    #[doc = "Checks if the value of the field is `CHA5`"]
    #[inline]
    pub fn is_cha5(&self) -> bool {
        *self == RXMULTCHR::CHA5
    }
    #[doc = "Checks if the value of the field is `CHA6`"]
    #[inline]
    pub fn is_cha6(&self) -> bool {
        *self == RXMULTCHR::CHA6
    }
    #[doc = "Checks if the value of the field is `CHA7`"]
    #[inline]
    pub fn is_cha7(&self) -> bool {
        *self == RXMULTCHR::CHA7
    }
}
#[doc = r" Proxy"]
pub struct __RSVD1W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD1W<'a> {
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
#[doc = "Values that can be written to the field `RXPASSCRC`"]
pub enum RXPASSCRCW {
    #[doc = "Received CRC is discarded for all channels and is not included in the buffer descriptor packet length field"]
    DISCARD,
    #[doc = "Received CRC is transferred to memory for all channels and is included in the buffer descriptor packet length"]
    INCLUDE,
}
impl RXPASSCRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXPASSCRCW::DISCARD => false,
            RXPASSCRCW::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPASSCRCW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPASSCRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPASSCRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received CRC is discarded for all channels and is not included in the buffer descriptor packet length field"]
    #[inline]
    pub fn discard(self) -> &'a mut W {
        self.variant(RXPASSCRCW::DISCARD)
    }
    #[doc = "Received CRC is transferred to memory for all channels and is included in the buffer descriptor packet length"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(RXPASSCRCW::INCLUDE)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXQOSEN`"]
pub enum RXQOSENW {
    #[doc = "Receive QOS is disabled"]
    DISABLE,
    #[doc = "Receive QOS is enabled"]
    ENABLE,
}
impl RXQOSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXQOSENW::DISABLE => false,
            RXQOSENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXQOSENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXQOSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXQOSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive QOS is disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXQOSENW::DISABLE)
    }
    #[doc = "Receive QOS is enabled"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXQOSENW::ENABLE)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXNOCHAIN`"]
pub enum RXNOCHAINW {
    #[doc = "Received frames can span multiple buffers"]
    DISABLE,
    #[doc = "RX DMA controller transfers each frame into a single buffer regardless of the frame or buffer size. All remaining frame data after the first buffer is discarded"]
    ENABLE,
}
impl RXNOCHAINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXNOCHAINW::DISABLE => false,
            RXNOCHAINW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXNOCHAINW<'a> {
    w: &'a mut W,
}
impl<'a> _RXNOCHAINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXNOCHAINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received frames can span multiple buffers"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXNOCHAINW::DISABLE)
    }
    #[doc = "RX DMA controller transfers each frame into a single buffer regardless of the frame or buffer size. All remaining frame data after the first buffer is discarded"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXNOCHAINW::ENABLE)
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
        const OFFSET: u8 = 28;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCMFEN`"]
pub enum RXCMFENW {
    #[doc = "MAC control frames are filtered (but acted upon if enabled)"]
    DISABLE,
    #[doc = "MAC control frames are transferred to memory"]
    ENABLE,
}
impl RXCMFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCMFENW::DISABLE => false,
            RXCMFENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCMFENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCMFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCMFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC control frames are filtered (but acted upon if enabled)"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCMFENW::DISABLE)
    }
    #[doc = "MAC control frames are transferred to memory"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCMFENW::ENABLE)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCSFEN`"]
pub enum RXCSFENW {
    #[doc = "Short frames are filtered"]
    DISABLE,
    #[doc = "Short frames are transferred to memory"]
    ENABLE,
}
impl RXCSFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCSFENW::DISABLE => false,
            RXCSFENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCSFENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCSFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCSFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Short frames are filtered"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCSFENW::DISABLE)
    }
    #[doc = "Short frames are transferred to memory"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCSFENW::ENABLE)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCEFEN`"]
pub enum RXCEFENW {
    #[doc = "Frames containing errors are filtered"]
    DISABLE,
    #[doc = "Frames containing errors are transferred to memory"]
    ENABLE,
}
impl RXCEFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCEFENW::DISABLE => false,
            RXCEFENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCEFENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCEFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCEFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frames containing errors are filtered"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCEFENW::DISABLE)
    }
    #[doc = "Frames containing errors are transferred to memory"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCEFENW::ENABLE)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCAFEN`"]
pub enum RXCAFENW {
    #[doc = "no description"]
    DISABLE,
    #[doc = "Frames that do not address match are transferred to the promiscuous channel selected by RXPROMCH bits"]
    ENABLE,
}
impl RXCAFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCAFENW::DISABLE => false,
            RXCAFENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCAFENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCAFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCAFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXCAFENW::DISABLE)
    }
    #[doc = "Frames that do not address match are transferred to the promiscuous channel selected by RXPROMCH bits"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXCAFENW::ENABLE)
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
        const OFFSET: u8 = 21;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXPROMCH`"]
pub enum RXPROMCHW {
    #[doc = "no description"]
    CHA0,
    #[doc = "no description"]
    CHA1,
    #[doc = "no description"]
    CHA2,
    #[doc = "no description"]
    CHA3,
    #[doc = "no description"]
    CHA4,
    #[doc = "no description"]
    CHA5,
    #[doc = "no description"]
    CHA6,
    #[doc = "no description"]
    CHA7,
}
impl RXPROMCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXPROMCHW::CHA0 => 0,
            RXPROMCHW::CHA1 => 1,
            RXPROMCHW::CHA2 => 2,
            RXPROMCHW::CHA3 => 3,
            RXPROMCHW::CHA4 => 4,
            RXPROMCHW::CHA5 => 5,
            RXPROMCHW::CHA6 => 6,
            RXPROMCHW::CHA7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPROMCHW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPROMCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPROMCHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha0(self) -> &'a mut W {
        self.variant(RXPROMCHW::CHA0)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha1(self) -> &'a mut W {
        self.variant(RXPROMCHW::CHA1)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha2(self) -> &'a mut W {
        self.variant(RXPROMCHW::CHA2)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha3(self) -> &'a mut W {
        self.variant(RXPROMCHW::CHA3)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha4(self) -> &'a mut W {
        self.variant(RXPROMCHW::CHA4)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha5(self) -> &'a mut W {
        self.variant(RXPROMCHW::CHA5)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha6(self) -> &'a mut W {
        self.variant(RXPROMCHW::CHA6)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha7(self) -> &'a mut W {
        self.variant(RXPROMCHW::CHA7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXBROADEN`"]
pub enum RXBROADENW {
    #[doc = "Broadcast frames are filtered"]
    DISABLE,
    #[doc = "Broadcast frames are copied to the channel selected by RXBROADCH bits"]
    ENABLE,
}
impl RXBROADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXBROADENW::DISABLE => false,
            RXBROADENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXBROADENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBROADENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXBROADENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Broadcast frames are filtered"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXBROADENW::DISABLE)
    }
    #[doc = "Broadcast frames are copied to the channel selected by RXBROADCH bits"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXBROADENW::ENABLE)
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXBROADCH`"]
pub enum RXBROADCHW {
    #[doc = "no description"]
    CHA0,
    #[doc = "no description"]
    CHA1,
    #[doc = "no description"]
    CHA2,
    #[doc = "no description"]
    CHA3,
    #[doc = "no description"]
    CHA4,
    #[doc = "no description"]
    CHA5,
    #[doc = "no description"]
    CHA6,
    #[doc = "no description"]
    CHA7,
}
impl RXBROADCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXBROADCHW::CHA0 => 0,
            RXBROADCHW::CHA1 => 1,
            RXBROADCHW::CHA2 => 2,
            RXBROADCHW::CHA3 => 3,
            RXBROADCHW::CHA4 => 4,
            RXBROADCHW::CHA5 => 5,
            RXBROADCHW::CHA6 => 6,
            RXBROADCHW::CHA7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXBROADCHW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBROADCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXBROADCHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha0(self) -> &'a mut W {
        self.variant(RXBROADCHW::CHA0)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha1(self) -> &'a mut W {
        self.variant(RXBROADCHW::CHA1)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha2(self) -> &'a mut W {
        self.variant(RXBROADCHW::CHA2)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha3(self) -> &'a mut W {
        self.variant(RXBROADCHW::CHA3)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha4(self) -> &'a mut W {
        self.variant(RXBROADCHW::CHA4)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha5(self) -> &'a mut W {
        self.variant(RXBROADCHW::CHA5)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha6(self) -> &'a mut W {
        self.variant(RXBROADCHW::CHA6)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha7(self) -> &'a mut W {
        self.variant(RXBROADCHW::CHA7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXMULTEN`"]
pub enum RXMULTENW {
    #[doc = "Multicast frames are filtered"]
    DISABLE,
    #[doc = "Multicast frames are copied to the channel selected by RXMULTCH bits"]
    ENABLE,
}
impl RXMULTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXMULTENW::DISABLE => false,
            RXMULTENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXMULTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXMULTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXMULTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Multicast frames are filtered"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXMULTENW::DISABLE)
    }
    #[doc = "Multicast frames are copied to the channel selected by RXMULTCH bits"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXMULTENW::ENABLE)
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
#[doc = r" Proxy"]
pub struct __RSVD7W<'a> {
    w: &'a mut W,
}
impl<'a> __RSVD7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXMULTCH`"]
pub enum RXMULTCHW {
    #[doc = "no description"]
    CHA0,
    #[doc = "no description"]
    CHA1,
    #[doc = "no description"]
    CHA2,
    #[doc = "no description"]
    CHA3,
    #[doc = "no description"]
    CHA4,
    #[doc = "no description"]
    CHA5,
    #[doc = "no description"]
    CHA6,
    #[doc = "no description"]
    CHA7,
}
impl RXMULTCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXMULTCHW::CHA0 => 0,
            RXMULTCHW::CHA1 => 1,
            RXMULTCHW::CHA2 => 2,
            RXMULTCHW::CHA3 => 3,
            RXMULTCHW::CHA4 => 4,
            RXMULTCHW::CHA5 => 5,
            RXMULTCHW::CHA6 => 6,
            RXMULTCHW::CHA7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXMULTCHW<'a> {
    w: &'a mut W,
}
impl<'a> _RXMULTCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXMULTCHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha0(self) -> &'a mut W {
        self.variant(RXMULTCHW::CHA0)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha1(self) -> &'a mut W {
        self.variant(RXMULTCHW::CHA1)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha2(self) -> &'a mut W {
        self.variant(RXMULTCHW::CHA2)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha3(self) -> &'a mut W {
        self.variant(RXMULTCHW::CHA3)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha4(self) -> &'a mut W {
        self.variant(RXMULTCHW::CHA4)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha5(self) -> &'a mut W {
        self.variant(RXMULTCHW::CHA5)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha6(self) -> &'a mut W {
        self.variant(RXMULTCHW::CHA6)
    }
    #[doc = "no description"]
    #[inline]
    pub fn cha7(self) -> &'a mut W {
        self.variant(RXMULTCHW::CHA7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 31 - Reserved"]
    #[inline]
    pub fn _rsvd1(&self) -> _RSVD1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        _RSVD1R { bits }
    }
    #[doc = "Bit 30 - Pass receive CRC enable"]
    #[inline]
    pub fn rxpasscrc(&self) -> RXPASSCRCR {
        RXPASSCRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Receive quality of service enable"]
    #[inline]
    pub fn rxqosen(&self) -> RXQOSENR {
        RXQOSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Receive no buffer chaining"]
    #[inline]
    pub fn rxnochain(&self) -> RXNOCHAINR {
        RXNOCHAINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 25:27 - Reserved"]
    #[inline]
    pub fn _rsvd2(&self) -> _RSVD2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        _RSVD2R { bits }
    }
    #[doc = "Bit 24 - Receive copy MAC control frames enable"]
    #[inline]
    pub fn rxcmfen(&self) -> RXCMFENR {
        RXCMFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Receive copy short frames enable"]
    #[inline]
    pub fn rxcsfen(&self) -> RXCSFENR {
        RXCSFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Receive copy error frames enable"]
    #[inline]
    pub fn rxcefen(&self) -> RXCEFENR {
        RXCEFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Receive copy all frames enable"]
    #[inline]
    pub fn rxcafen(&self) -> RXCAFENR {
        RXCAFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:20 - Reserved"]
    #[inline]
    pub fn _rsvd3(&self) -> _RSVD3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        _RSVD3R { bits }
    }
    #[doc = "Bits 16:18 - Receive promiscuous channel select"]
    #[inline]
    pub fn rxpromch(&self) -> RXPROMCHR {
        RXPROMCHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Reserved"]
    #[inline]
    pub fn _rsvd4(&self) -> _RSVD4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        _RSVD4R { bits }
    }
    #[doc = "Bit 13 - Receive broadcast enable"]
    #[inline]
    pub fn rxbroaden(&self) -> RXBROADENR {
        RXBROADENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline]
    pub fn _rsvd5(&self) -> _RSVD5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        _RSVD5R { bits }
    }
    #[doc = "Bits 8:10 - Receive broadcast channel select"]
    #[inline]
    pub fn rxbroadch(&self) -> RXBROADCHR {
        RXBROADCHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline]
    pub fn _rsvd6(&self) -> _RSVD6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        _RSVD6R { bits }
    }
    #[doc = "Bit 5 - RX multicast enable"]
    #[inline]
    pub fn rxmulten(&self) -> RXMULTENR {
        RXMULTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - Reserved"]
    #[inline]
    pub fn _rsvd7(&self) -> _RSVD7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        _RSVD7R { bits }
    }
    #[doc = "Bits 0:2 - RX multicast channel select"]
    #[inline]
    pub fn rxmultch(&self) -> RXMULTCHR {
        RXMULTCHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 31 - Reserved"]
    #[inline]
    pub fn _rsvd1(&mut self) -> __RSVD1W {
        __RSVD1W { w: self }
    }
    #[doc = "Bit 30 - Pass receive CRC enable"]
    #[inline]
    pub fn rxpasscrc(&mut self) -> _RXPASSCRCW {
        _RXPASSCRCW { w: self }
    }
    #[doc = "Bit 29 - Receive quality of service enable"]
    #[inline]
    pub fn rxqosen(&mut self) -> _RXQOSENW {
        _RXQOSENW { w: self }
    }
    #[doc = "Bit 28 - Receive no buffer chaining"]
    #[inline]
    pub fn rxnochain(&mut self) -> _RXNOCHAINW {
        _RXNOCHAINW { w: self }
    }
    #[doc = "Bits 25:27 - Reserved"]
    #[inline]
    pub fn _rsvd2(&mut self) -> __RSVD2W {
        __RSVD2W { w: self }
    }
    #[doc = "Bit 24 - Receive copy MAC control frames enable"]
    #[inline]
    pub fn rxcmfen(&mut self) -> _RXCMFENW {
        _RXCMFENW { w: self }
    }
    #[doc = "Bit 23 - Receive copy short frames enable"]
    #[inline]
    pub fn rxcsfen(&mut self) -> _RXCSFENW {
        _RXCSFENW { w: self }
    }
    #[doc = "Bit 22 - Receive copy error frames enable"]
    #[inline]
    pub fn rxcefen(&mut self) -> _RXCEFENW {
        _RXCEFENW { w: self }
    }
    #[doc = "Bit 21 - Receive copy all frames enable"]
    #[inline]
    pub fn rxcafen(&mut self) -> _RXCAFENW {
        _RXCAFENW { w: self }
    }
    #[doc = "Bits 19:20 - Reserved"]
    #[inline]
    pub fn _rsvd3(&mut self) -> __RSVD3W {
        __RSVD3W { w: self }
    }
    #[doc = "Bits 16:18 - Receive promiscuous channel select"]
    #[inline]
    pub fn rxpromch(&mut self) -> _RXPROMCHW {
        _RXPROMCHW { w: self }
    }
    #[doc = "Bits 14:15 - Reserved"]
    #[inline]
    pub fn _rsvd4(&mut self) -> __RSVD4W {
        __RSVD4W { w: self }
    }
    #[doc = "Bit 13 - Receive broadcast enable"]
    #[inline]
    pub fn rxbroaden(&mut self) -> _RXBROADENW {
        _RXBROADENW { w: self }
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline]
    pub fn _rsvd5(&mut self) -> __RSVD5W {
        __RSVD5W { w: self }
    }
    #[doc = "Bits 8:10 - Receive broadcast channel select"]
    #[inline]
    pub fn rxbroadch(&mut self) -> _RXBROADCHW {
        _RXBROADCHW { w: self }
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline]
    pub fn _rsvd6(&mut self) -> __RSVD6W {
        __RSVD6W { w: self }
    }
    #[doc = "Bit 5 - RX multicast enable"]
    #[inline]
    pub fn rxmulten(&mut self) -> _RXMULTENW {
        _RXMULTENW { w: self }
    }
    #[doc = "Bits 3:4 - Reserved"]
    #[inline]
    pub fn _rsvd7(&mut self) -> __RSVD7W {
        __RSVD7W { w: self }
    }
    #[doc = "Bits 0:2 - RX multicast channel select"]
    #[inline]
    pub fn rxmultch(&mut self) -> _RXMULTCHW {
        _RXMULTCHW { w: self }
    }
}
