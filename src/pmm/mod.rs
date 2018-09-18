#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Logic Power Domain Control Register"]
    pub logic_pdctrl0: LOGICPDCTRL0,
    #[doc = "0x04 - Logic Power Domain Control Register"]
    pub logic_pdctrl1: LOGICPDCTRL1,
    #[doc = "0x08 - Logic Power Domain Control Register"]
    pub logic_pdctrl2: LOGICPDCTRL2,
    #[doc = "0x0c - Logic Power Domain Control Register"]
    pub logic_pdctrl3: LOGICPDCTRL3,
    #[doc = "0x10 - Memory Power Domain Control Register"]
    pub mem_pdctrl0: MEMPDCTRL0,
    #[doc = "0x14 - Memory Power Domain Control Register"]
    pub mem_pdctrl1: MEMPDCTRL1,
    _reserved0: [u8; 8usize],
    #[doc = "0x20 - Power Domain Clock Disable Register"]
    pub pdclk_dis: PDCLKDIS,
    #[doc = "0x24 - Power Domain Clock Disable SET Register"]
    pub pdclk_dis_set: PDCLKDISSET,
    #[doc = "0x28 - Power Domain Clock Disable CLEAR Register"]
    pub pdclk_dis_clr: PDCLKDISCLR,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - Isolation Extension Register"]
    pub iso_extend: ISOEXTEND,
    #[doc = "0x34 - Isolation Extension SET Register"]
    pub iso_extend_set: ISOEXTENDSET,
    #[doc = "0x38 - Isolation Extension CLEAR Register"]
    pub iso_extend_clr: ISOEXTENDCLR,
    _reserved2: [u8; 4usize],
    #[doc = "0x40 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat0: LOGICPDPWRSTAT0,
    #[doc = "0x44 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat1: LOGICPDPWRSTAT1,
    #[doc = "0x48 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat2: LOGICPDPWRSTAT2,
    #[doc = "0x4c - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat3: LOGICPDPWRSTAT3,
    #[doc = "0x50 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat4: LOGICPDPWRSTAT4,
    #[doc = "0x54 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat5: LOGICPDPWRSTAT5,
    #[doc = "0x58 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat6: LOGICPDPWRSTAT6,
    #[doc = "0x5c - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat7: LOGICPDPWRSTAT7,
    #[doc = "0x60 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat8: LOGICPDPWRSTAT8,
    #[doc = "0x64 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat9: LOGICPDPWRSTAT9,
    #[doc = "0x68 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat10: LOGICPDPWRSTAT10,
    #[doc = "0x6c - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat11: LOGICPDPWRSTAT11,
    #[doc = "0x70 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat12: LOGICPDPWRSTAT12,
    #[doc = "0x74 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat13: LOGICPDPWRSTAT13,
    #[doc = "0x78 - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat14: LOGICPDPWRSTAT14,
    #[doc = "0x7c - Logic Power Domain Power Status"]
    pub logic_pdpwr_stat15: LOGICPDPWRSTAT15,
    #[doc = "0x80 - Memory Power Domain Power Status"]
    pub mem_pwr_stat0: MEMPWRSTAT0,
    #[doc = "0x84 - Memory Power Domain Power Status"]
    pub mem_pwr_stat1: MEMPWRSTAT1,
    #[doc = "0x88 - Memory Power Domain Power Status"]
    pub mem_pwr_stat2: MEMPWRSTAT2,
    #[doc = "0x8c - Memory Power Domain Power Status"]
    pub mem_pwr_stat3: MEMPWRSTAT3,
    #[doc = "0x90 - Memory Power Domain Power Status"]
    pub mem_pwr_stat4: MEMPWRSTAT4,
    #[doc = "0x94 - Memory Power Domain Power Status"]
    pub mem_pwr_stat5: MEMPWRSTAT5,
    #[doc = "0x98 - Memory Power Domain Power Status"]
    pub mem_pwr_stat6: MEMPWRSTAT6,
    #[doc = "0x9c - Memory Power Domain Power Status"]
    pub mem_pwr_stat7: MEMPWRSTAT7,
    #[doc = "0xa0 - Global Control Register1"]
    pub global_ctrl1: GLOBALCTRL1,
    #[doc = "0xa4 - Global Control Register2"]
    pub global_ctrl2: GLOBALCTRL2,
    #[doc = "0xa8 - Global Status Register"]
    pub global_status: GLOBALSTATUS,
    #[doc = "0xac - PSCON Diagnostic Compare Key Register"]
    pub pscon_diag_comp_key: PSCONDIAGCOMPKEY,
    #[doc = "0xb0 - LogicPD PSCON Diagnostic Compare Status Register1"]
    pub pscon_diag_comp_stat1: PSCONDIAGCOMPSTAT1,
    #[doc = "0xb4 - LogicPD PSCON Diagnostic Compare Status Register2"]
    pub pscon_diag_comp_stat2: PSCONDIAGCOMPSTAT2,
    #[doc = "0xb8 - Memory PD PSCON Diagnostic Compare Status Register1"]
    pub mpddia_comp_stat1: MPDDIACOMPSTAT1,
    #[doc = "0xbc - Memory PD PSCON Diagnostic Compare Status Register2"]
    pub mpddia_comp_stat2: MPDDIACOMPSTAT2,
    #[doc = "0xc0 - Isolation Diagnostic Status"]
    pub iso_dia_stat: ISODIASTAT,
}
#[doc = "Logic Power Domain Control Register"]
pub struct LOGICPDCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Control Register"]
pub mod logic_pdctrl0;
#[doc = "Logic Power Domain Control Register"]
pub struct LOGICPDCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Control Register"]
pub mod logic_pdctrl1;
#[doc = "Logic Power Domain Control Register"]
pub struct LOGICPDCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Control Register"]
pub mod logic_pdctrl2;
#[doc = "Logic Power Domain Control Register"]
pub struct LOGICPDCTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Control Register"]
pub mod logic_pdctrl3;
#[doc = "Memory Power Domain Control Register"]
pub struct MEMPDCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Power Domain Control Register"]
pub mod mem_pdctrl0;
#[doc = "Memory Power Domain Control Register"]
pub struct MEMPDCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Power Domain Control Register"]
pub mod mem_pdctrl1;
#[doc = "Power Domain Clock Disable Register"]
pub struct PDCLKDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Domain Clock Disable Register"]
pub mod pdclk_dis;
#[doc = "Power Domain Clock Disable SET Register"]
pub struct PDCLKDISSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Domain Clock Disable SET Register"]
pub mod pdclk_dis_set;
#[doc = "Power Domain Clock Disable CLEAR Register"]
pub struct PDCLKDISCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Domain Clock Disable CLEAR Register"]
pub mod pdclk_dis_clr;
#[doc = "Isolation Extension Register"]
pub struct ISOEXTEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Isolation Extension Register"]
pub mod iso_extend;
#[doc = "Isolation Extension SET Register"]
pub struct ISOEXTENDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Isolation Extension SET Register"]
pub mod iso_extend_set;
#[doc = "Isolation Extension CLEAR Register"]
pub struct ISOEXTENDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Isolation Extension CLEAR Register"]
pub mod iso_extend_clr;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat0;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat1;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat2;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat3;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat4;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat5;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat6;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat7;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat8;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat9;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat10;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat11;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat12;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat13;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat14;
#[doc = "Logic Power Domain Power Status"]
pub struct LOGICPDPWRSTAT15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Logic Power Domain Power Status"]
pub mod logic_pdpwr_stat15;
#[doc = "Memory Power Domain Power Status"]
pub struct MEMPWRSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Power Domain Power Status"]
pub mod mem_pwr_stat0;
#[doc = "Memory Power Domain Power Status"]
pub struct MEMPWRSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Power Domain Power Status"]
pub mod mem_pwr_stat1;
#[doc = "Memory Power Domain Power Status"]
pub struct MEMPWRSTAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Power Domain Power Status"]
pub mod mem_pwr_stat2;
#[doc = "Memory Power Domain Power Status"]
pub struct MEMPWRSTAT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Power Domain Power Status"]
pub mod mem_pwr_stat3;
#[doc = "Memory Power Domain Power Status"]
pub struct MEMPWRSTAT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Power Domain Power Status"]
pub mod mem_pwr_stat4;
#[doc = "Memory Power Domain Power Status"]
pub struct MEMPWRSTAT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Power Domain Power Status"]
pub mod mem_pwr_stat5;
#[doc = "Memory Power Domain Power Status"]
pub struct MEMPWRSTAT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Power Domain Power Status"]
pub mod mem_pwr_stat6;
#[doc = "Memory Power Domain Power Status"]
pub struct MEMPWRSTAT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Power Domain Power Status"]
pub mod mem_pwr_stat7;
#[doc = "Global Control Register1"]
pub struct GLOBALCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register1"]
pub mod global_ctrl1;
#[doc = "Global Control Register2"]
pub struct GLOBALCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register2"]
pub mod global_ctrl2;
#[doc = "Global Status Register"]
pub struct GLOBALSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Status Register"]
pub mod global_status;
#[doc = "PSCON Diagnostic Compare Key Register"]
pub struct PSCONDIAGCOMPKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSCON Diagnostic Compare Key Register"]
pub mod pscon_diag_comp_key;
#[doc = "LogicPD PSCON Diagnostic Compare Status Register1"]
pub struct PSCONDIAGCOMPSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LogicPD PSCON Diagnostic Compare Status Register1"]
pub mod pscon_diag_comp_stat1;
#[doc = "LogicPD PSCON Diagnostic Compare Status Register2"]
pub struct PSCONDIAGCOMPSTAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LogicPD PSCON Diagnostic Compare Status Register2"]
pub mod pscon_diag_comp_stat2;
#[doc = "Memory PD PSCON Diagnostic Compare Status Register1"]
pub struct MPDDIACOMPSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory PD PSCON Diagnostic Compare Status Register1"]
pub mod mpddia_comp_stat1;
#[doc = "Memory PD PSCON Diagnostic Compare Status Register2"]
pub struct MPDDIACOMPSTAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory PD PSCON Diagnostic Compare Status Register2"]
pub mod mpddia_comp_stat2;
#[doc = "Isolation Diagnostic Status"]
pub struct ISODIASTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Isolation Diagnostic Status"]
pub mod iso_dia_stat;
