#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDIO Version Register"]
    pub rev: REV,
    #[doc = "0x04 - MDIO Control Register"]
    pub control: CONTROL,
    #[doc = "0x08 - MDIO Alive Register"]
    pub alive: ALIVE,
    #[doc = "0x0c - MDIO Link Register"]
    pub link: LINK,
    #[doc = "0x10 - MDIO Link Interrupt Raw Register"]
    pub linkintraw: LINKINTRAW,
    #[doc = "0x14 - MDIO Link Interrupt Masked Register"]
    pub linkintmasked: LINKINTMASKED,
    _reserved0: [u8; 8usize],
    #[doc = "0x20 - MDIO User Interrupt Raw Register"]
    pub userintraw: USERINTRAW,
    #[doc = "0x24 - MDIO User Interrupt Masked Register"]
    pub userintmasked: USERINTMASKED,
    #[doc = "0x28 - MDIO User Interrupt Mask Set Register"]
    pub userintmaskset: USERINTMASKSET,
    #[doc = "0x2c - MDIO User Interrupt Mask Clear Register"]
    pub userintmaskclear: USERINTMASKCLEAR,
    _reserved1: [u8; 80usize],
    #[doc = "0x80 - MDIO User Access Register 0"]
    pub useraccess0: USERACCESS0,
    #[doc = "0x84 - MDIO User PHY Select REG0"]
    pub userphysel0: USERPHYSEL0,
    #[doc = "0x88 - MDIO User Access Register 1"]
    pub useraccess1: USERACCESS1,
    #[doc = "0x8c - MDIO User PHY Select REG1"]
    pub userphysel1: USERPHYSEL1,
}
#[doc = "MDIO Version Register"]
pub struct REV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO Version Register"]
pub mod rev;
#[doc = "MDIO Control Register"]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO Control Register"]
pub mod control;
#[doc = "MDIO Alive Register"]
pub struct ALIVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO Alive Register"]
pub mod alive;
#[doc = "MDIO Link Register"]
pub struct LINK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO Link Register"]
pub mod link;
#[doc = "MDIO Link Interrupt Raw Register"]
pub struct LINKINTRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO Link Interrupt Raw Register"]
pub mod linkintraw;
#[doc = "MDIO Link Interrupt Masked Register"]
pub struct LINKINTMASKED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO Link Interrupt Masked Register"]
pub mod linkintmasked;
#[doc = "MDIO User Interrupt Raw Register"]
pub struct USERINTRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO User Interrupt Raw Register"]
pub mod userintraw;
#[doc = "MDIO User Interrupt Masked Register"]
pub struct USERINTMASKED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO User Interrupt Masked Register"]
pub mod userintmasked;
#[doc = "MDIO User Interrupt Mask Set Register"]
pub struct USERINTMASKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO User Interrupt Mask Set Register"]
pub mod userintmaskset;
#[doc = "MDIO User Interrupt Mask Clear Register"]
pub struct USERINTMASKCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO User Interrupt Mask Clear Register"]
pub mod userintmaskclear;
#[doc = "MDIO User Access Register 0"]
pub struct USERACCESS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO User Access Register 0"]
pub mod useraccess0;
#[doc = "MDIO User PHY Select REG0"]
pub struct USERPHYSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO User PHY Select REG0"]
pub mod userphysel0;
#[doc = "MDIO User Access Register 1"]
pub struct USERACCESS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO User Access Register 1"]
pub mod useraccess1;
#[doc = "MDIO User PHY Select REG1"]
pub struct USERPHYSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIO User PHY Select REG1"]
pub mod userphysel1;
