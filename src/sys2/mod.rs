#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PLL Control Register 3"]
    pub pllctl3: PLLCTL3,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - CPU Logic BIST Clock Divider"]
    pub stcclkdiv: STCCLKDIV,
    #[doc = "0x0c - Clock Hibernate Mode Global Enable Register"]
    pub clkhb_glbreg: CLKHB_GLBREG,
    #[doc = "0x10 - Clocked Hibernate RTI Domain Control Register"]
    pub clkhb_rtidreg: CLKHB_RTIDREG,
    #[doc = "0x14 - Hibernate Clock Domain Status Register"]
    pub hbcd_stat: HBCD_STAT,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - Clock Trim 1 Register"]
    pub clktrmi1: CLKTRMI1,
    _reserved2: [u8; 24usize],
    #[doc = "0x3c - Clock 2 Control Register"]
    pub clk2cntrl: CLK2CNTRL,
    #[doc = "0x40 - Peripheral Asynchronous Clock Configuration 1 Register"]
    pub vclkacon1: VCLKACON1,
    #[doc = "0x44 - Clock 3 Control Register"]
    pub clk3cntrl: CLK3CNTRL,
    #[doc = "0x48 - Peripheral Asynchronous Clock Configuration 2 Register"]
    pub vclkacon2: VCLKACON2,
    _reserved3: [u8; 4usize],
    #[doc = "0x50 - Peripheral Asynchronous Clock Configuration 3 Register"]
    pub vclkacon3: VCLKACON3,
    #[doc = "0x54 - HCLK1 Control Register"]
    pub hclk1ctrl: HCLK1CTRL,
    _reserved4: [u8; 24usize],
    #[doc = "0x70 - Clock Slip Register"]
    pub clkslip: CLKSLIP,
    _reserved5: [u8; 120usize],
    #[doc = "0xec - EFUSE Controller Control Register"]
    pub efc_ctlreg: EFC_CTLREG,
    #[doc = "0xf0 - Die Identification Register Lower Word"]
    pub dieidl_reg0: DIEIDL_REG0,
    #[doc = "0xf4 - Die Identification Register Upper Word"]
    pub dieidh_reg1: DIEIDH_REG1,
    #[doc = "0xf8 - Die Identification Register Lower Word"]
    pub dieidh_reg2: DIEIDH_REG2,
    #[doc = "0xfc - Die Identification Register Upper Word"]
    pub dieidh_reg3: DIEIDH_REG3,
}
#[doc = "PLL Control Register 3"]
pub struct PLLCTL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Control Register 3"]
pub mod pllctl3;
#[doc = "CPU Logic BIST Clock Divider"]
pub struct STCCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Logic BIST Clock Divider"]
pub mod stcclkdiv;
#[doc = "Clock Hibernate Mode Global Enable Register"]
pub struct CLKHB_GLBREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Hibernate Mode Global Enable Register"]
pub mod clkhb_glbreg;
#[doc = "Clocked Hibernate RTI Domain Control Register"]
pub struct CLKHB_RTIDREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clocked Hibernate RTI Domain Control Register"]
pub mod clkhb_rtidreg;
#[doc = "Hibernate Clock Domain Status Register"]
pub struct HBCD_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Clock Domain Status Register"]
pub mod hbcd_stat;
#[doc = "Clock Trim 1 Register"]
pub struct CLKTRMI1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Trim 1 Register"]
pub mod clktrmi1;
#[doc = "Clock 2 Control Register"]
pub struct CLK2CNTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock 2 Control Register"]
pub mod clk2cntrl;
#[doc = "Peripheral Asynchronous Clock Configuration 1 Register"]
pub struct VCLKACON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Asynchronous Clock Configuration 1 Register"]
pub mod vclkacon1;
#[doc = "Clock 3 Control Register"]
pub struct CLK3CNTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock 3 Control Register"]
pub mod clk3cntrl;
#[doc = "Peripheral Asynchronous Clock Configuration 2 Register"]
pub struct VCLKACON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Asynchronous Clock Configuration 2 Register"]
pub mod vclkacon2;
#[doc = "Peripheral Asynchronous Clock Configuration 3 Register"]
pub struct VCLKACON3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Asynchronous Clock Configuration 3 Register"]
pub mod vclkacon3;
#[doc = "HCLK1 Control Register"]
pub struct HCLK1CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HCLK1 Control Register"]
pub mod hclk1ctrl;
#[doc = "Clock Slip Register"]
pub struct CLKSLIP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Slip Register"]
pub mod clkslip;
#[doc = "Clock2 Slip Register"]
pub struct CLK2SLIP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock2 Slip Register"]
pub mod clk2slip;
#[doc = "EFUSE Controller Control Register"]
pub struct EFC_CTLREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EFUSE Controller Control Register"]
pub mod efc_ctlreg;
#[doc = "Die Identification Register Lower Word"]
pub struct DIEIDL_REG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Identification Register Lower Word"]
pub mod dieidl_reg0;
#[doc = "Die Identification Register Upper Word"]
pub struct DIEIDH_REG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Identification Register Upper Word"]
pub mod dieidh_reg1;
#[doc = "Die Identification Register Lower Word"]
pub struct DIEIDH_REG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Identification Register Lower Word"]
pub mod dieidh_reg2;
#[doc = "Die Identification Register Upper Word"]
pub struct DIEIDH_REG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Identification Register Upper Word"]
pub mod dieidh_reg3;
