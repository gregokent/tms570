#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    pub glb_ctrl: GLBCTRL,
    #[doc = "0x04 - Power Down"]
    pub pw_dn: PWDN,
    #[doc = "0x08 - Interrupt Detect"]
    pub int_det: INTDET,
    #[doc = "0x0c - Interrupt Polarity"]
    pub int_pol: INTPOL,
    #[doc = "0x10 - Interrupt Enable Set"]
    pub int_ena_set: INTENASET,
    #[doc = "0x14 - Interrupt Enable Clear"]
    pub int_ena_clr: INTENACLR,
    #[doc = "0x18 - Interrupt Priority Set"]
    pub int_lvl_set: INTLVLSET,
    #[doc = "0x1c - Interrupt Priority Clear"]
    pub int_lvl_clr: INTLVLCLR,
    #[doc = "0x20 - Interrupt Flag"]
    pub int_flg: INTFLG,
    #[doc = "0x24 - Offset A"]
    pub offst_a: OFFSTA,
    #[doc = "0x28 - Offset B"]
    pub offst_b: OFFSTB,
    #[doc = "0x2c - Emulation A"]
    pub emu_a: EMUA,
    #[doc = "0x30 - Emulation B"]
    pub emu_b: EMUB,
}
#[doc = "Global Control Register"]
pub struct GLBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod glb_ctrl;
#[doc = "Power Down"]
pub struct PWDN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Down"]
pub mod pw_dn;
#[doc = "Interrupt Detect"]
pub struct INTDET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Detect"]
pub mod int_det;
#[doc = "Interrupt Polarity"]
pub struct INTPOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Polarity"]
pub mod int_pol;
#[doc = "Interrupt Enable Set"]
pub struct INTENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set"]
pub mod int_ena_set;
#[doc = "Interrupt Enable Clear"]
pub struct INTENACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear"]
pub mod int_ena_clr;
#[doc = "Interrupt Priority Set"]
pub struct INTLVLSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Set"]
pub mod int_lvl_set;
#[doc = "Interrupt Priority Clear"]
pub struct INTLVLCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Clear"]
pub mod int_lvl_clr;
#[doc = "Interrupt Flag"]
pub struct INTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag"]
pub mod int_flg;
#[doc = "Offset A"]
pub struct OFFSTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Offset A"]
pub mod offst_a;
#[doc = "Offset B"]
pub struct OFFSTB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Offset B"]
pub mod offst_b;
#[doc = "Emulation A"]
pub struct EMUA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Emulation A"]
pub mod emu_a;
#[doc = "Emulation B"]
pub struct EMUB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Emulation B"]
pub mod emu_b;
