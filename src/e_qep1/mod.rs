#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eQEP Position Counter"]
    pub qposcnt: QPOSCNT,
    #[doc = "0x04 - eQEP Initialization Position Count"]
    pub qposinit: QPOSINIT,
    #[doc = "0x08 - eQEP Maximum Position Count"]
    pub qposmax: QPOSMAX,
    #[doc = "0x0c - eQEP Position-compare"]
    pub qposcmp: QPOSCMP,
    #[doc = "0x10 - eQEP Index Position Latch"]
    pub qposilat: QPOSILAT,
    #[doc = "0x14 - eQEP Strobe Position Latch"]
    pub qposslat: QPOSSLAT,
    #[doc = "0x18 - eQEP Position Latch"]
    pub qposlat: QPOSLAT,
    #[doc = "0x1c - eQEP Unit Timer"]
    pub qutmr: QUTMR,
    #[doc = "0x20 - eQEP Unit Period Register"]
    pub quprd: QUPRD,
    #[doc = "0x24 - eQEP Watchdog Period Register"]
    pub qwdprd: QWDPRD,
    #[doc = "0x26 - eQEP Watchdog Timer"]
    pub qwdtmr: QWDTMR,
    #[doc = "0x28 - eQEP Control Register"]
    pub qepctl: QEPCTL,
    #[doc = "0x2a - eQEP Decoder Control Register"]
    pub qdecctl: QDECCTL,
    #[doc = "0x2c - eQEP Position-compare Control Register"]
    pub qposctl: QPOSCTL,
    #[doc = "0x2e - eQEP Capture Control Register"]
    pub qcapctl: QCAPCTL,
    #[doc = "0x30 - eQEP Interrupt Flag Register"]
    pub qflg: QFLG,
    #[doc = "0x32 - eQEP Interrupt Enable Register"]
    pub qeint: QEINT,
    #[doc = "0x34 - eQEP Interrupt Force Register"]
    pub qfrc: QFRC,
    #[doc = "0x36 - eQEP Interrupt Clear Register"]
    pub qclr: QCLR,
    #[doc = "0x38 - eQEP Capture Timer"]
    pub qctmr: QCTMR,
    #[doc = "0x3a - eQEP Status Register"]
    pub qepsts: QEPSTS,
    #[doc = "0x3c - eQEP Capture Timer Latch"]
    pub qctmrlat: QCTMRLAT,
    #[doc = "0x3e - eQEP Capture Period Register"]
    pub qcprd: QCPRD,
    #[doc = "0x40 - RESERVED"]
    pub reserved: RESERVED,
    #[doc = "0x42 - eQEP Capture Period Latch"]
    pub qcprdlat: QCPRDLAT,
}
#[doc = "eQEP Position Counter"]
pub struct QPOSCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eQEP Position Counter"]
pub mod qposcnt;
#[doc = "eQEP Initialization Position Count"]
pub struct QPOSINIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eQEP Initialization Position Count"]
pub mod qposinit;
#[doc = "eQEP Maximum Position Count"]
pub struct QPOSMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eQEP Maximum Position Count"]
pub mod qposmax;
#[doc = "eQEP Position-compare"]
pub struct QPOSCMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eQEP Position-compare"]
pub mod qposcmp;
#[doc = "eQEP Index Position Latch"]
pub struct QPOSILAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eQEP Index Position Latch"]
pub mod qposilat;
#[doc = "eQEP Strobe Position Latch"]
pub struct QPOSSLAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eQEP Strobe Position Latch"]
pub mod qposslat;
#[doc = "eQEP Position Latch"]
pub struct QPOSLAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eQEP Position Latch"]
pub mod qposlat;
#[doc = "eQEP Unit Timer"]
pub struct QUTMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eQEP Unit Timer"]
pub mod qutmr;
#[doc = "eQEP Unit Period Register"]
pub struct QUPRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eQEP Unit Period Register"]
pub mod quprd;
#[doc = "eQEP Watchdog Period Register"]
pub struct QWDPRD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Watchdog Period Register"]
pub mod qwdprd;
#[doc = "eQEP Watchdog Timer"]
pub struct QWDTMR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Watchdog Timer"]
pub mod qwdtmr;
#[doc = "eQEP Control Register"]
pub struct QEPCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Control Register"]
pub mod qepctl;
#[doc = "eQEP Decoder Control Register"]
pub struct QDECCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Decoder Control Register"]
pub mod qdecctl;
#[doc = "eQEP Position-compare Control Register"]
pub struct QPOSCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Position-compare Control Register"]
pub mod qposctl;
#[doc = "eQEP Capture Control Register"]
pub struct QCAPCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Capture Control Register"]
pub mod qcapctl;
#[doc = "eQEP Interrupt Flag Register"]
pub struct QFLG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Interrupt Flag Register"]
pub mod qflg;
#[doc = "eQEP Interrupt Enable Register"]
pub struct QEINT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Interrupt Enable Register"]
pub mod qeint;
#[doc = "eQEP Interrupt Force Register"]
pub struct QFRC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Interrupt Force Register"]
pub mod qfrc;
#[doc = "eQEP Interrupt Clear Register"]
pub struct QCLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Interrupt Clear Register"]
pub mod qclr;
#[doc = "eQEP Capture Timer"]
pub struct QCTMR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Capture Timer"]
pub mod qctmr;
#[doc = "eQEP Status Register"]
pub struct QEPSTS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Status Register"]
pub mod qepsts;
#[doc = "eQEP Capture Timer Latch"]
pub struct QCTMRLAT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Capture Timer Latch"]
pub mod qctmrlat;
#[doc = "eQEP Capture Period Register"]
pub struct QCPRD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Capture Period Register"]
pub mod qcprd;
#[doc = "RESERVED"]
pub struct RESERVED {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RESERVED"]
pub mod reserved;
#[doc = "eQEP Capture Period Latch"]
pub struct QCPRDLAT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "eQEP Capture Period Latch"]
pub mod qcprdlat;
