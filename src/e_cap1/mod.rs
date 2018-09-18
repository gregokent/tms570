#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eCAP Time-Stamp Counter"]
    pub tsctr: TSCTR,
    #[doc = "0x04 - eCAP Counter Phase Offset Value Register"]
    pub ctrphs: CTRPHS,
    #[doc = "0x08 - eCAP Capture 1 Register"]
    pub cap1: CAP1,
    #[doc = "0x0c - eCAP Capture 2 Register"]
    pub cap2: CAP2,
    #[doc = "0x10 - eCAP Capture 3 Register"]
    pub cap3: CAP3,
    #[doc = "0x14 - eCAP Capture 4 Register"]
    pub cap4: CAP4,
    _reserved0: [u8; 16usize],
    #[doc = "0x28 - eCAP Capture Control Register 2"]
    pub ecctl2: ECCTL2,
    #[doc = "0x2a - eCAP Capture Control Register 1"]
    pub ecctl1: ECCTL1,
    #[doc = "0x2c - eCAP Capture Interrupt Flag Register"]
    pub ecflg: ECFLG,
    #[doc = "0x2e - eCAP Capture Interrupt Enable Register"]
    pub eceint: ECEINT,
    #[doc = "0x30 - eCAP Capture Interrupt Force Register"]
    pub ecfrc: ECFRC,
    #[doc = "0x32 - eCAP Capture Interrupt Clear Register"]
    pub ecclr: ECCLR,
}
#[doc = "eCAP Time-Stamp Counter"]
pub struct TSCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eCAP Time-Stamp Counter"]
pub mod tsctr;
#[doc = "eCAP Counter Phase Offset Value Register"]
pub struct CTRPHS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eCAP Counter Phase Offset Value Register"]
pub mod ctrphs;
#[doc = "eCAP Capture 1 Register"]
pub struct CAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eCAP Capture 1 Register"]
pub mod cap1;
#[doc = "eCAP Capture 2 Register"]
pub struct CAP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eCAP Capture 2 Register"]
pub mod cap2;
#[doc = "eCAP Capture 3 Register"]
pub struct CAP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eCAP Capture 3 Register"]
pub mod cap3;
#[doc = "eCAP Capture 4 Register"]
pub struct CAP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eCAP Capture 4 Register"]
pub mod cap4;
#[doc = "eCAP Capture Control Register 2"]
pub struct ECCTL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eCAP Capture Control Register 2"]
pub mod ecctl2;
#[doc = "eCAP Capture Control Register 1"]
pub struct ECCTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eCAP Capture Control Register 1"]
pub mod ecctl1;
#[doc = "eCAP Capture Interrupt Flag Register"]
pub struct ECFLG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eCAP Capture Interrupt Flag Register"]
pub mod ecflg;
#[doc = "eCAP Capture Interrupt Enable Register"]
pub struct ECEINT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eCAP Capture Interrupt Enable Register"]
pub mod eceint;
#[doc = "eCAP Capture Interrupt Force Register"]
pub struct ECFRC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eCAP Capture Interrupt Force Register"]
pub mod ecfrc;
#[doc = "eCAP Capture Interrupt Clear Register"]
pub struct ECCLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eCAP Capture Interrupt Clear Register"]
pub mod ecclr;
