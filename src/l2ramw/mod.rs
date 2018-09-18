#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ram Control Register"]
    pub ram_ctrl: RAMCTRL,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - Ram Error Status Register"]
    pub ram_err_stat: RAMERRSTAT,
    _reserved1: [u8; 16usize],
    #[doc = "0x24 - Diagnostic Data Vector Upper 32bits"]
    pub diag_data_vect_h: DIAGDATAVECTH,
    #[doc = "0x28 - Diagnostic Data Vector Lower 32bits"]
    pub diag_data_vect_l: DIAGDATAVECTL,
    #[doc = "0x2c - Diagnostic ECC Vector"]
    pub diag_ecc_vect: DIAGECCVECT,
    #[doc = "0x30 - RAM Test Control register"]
    pub ram_test: RAMTEST,
    _reserved2: [u8; 4usize],
    #[doc = "0x38 - RAM Address Decode Vector Test Register"]
    pub addr_vect_tst_reg: ADDRVECTTSTREG,
    #[doc = "0x3c - Memory Initialization Domain Register"]
    pub mem_init_domain: MEMINITDOMAIN,
    _reserved3: [u8; 4usize],
    #[doc = "0x44 - Bank to Domain Mapping Register0"]
    pub bnk_dmn_map0: BNKDMNMAP0,
    #[doc = "0x48 - Bank to Domain Mapping Register1"]
    pub bnk_dmn_map1: BNKDMNMAP1,
}
#[doc = "Ram Control Register"]
pub struct RAMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram Control Register"]
pub mod ram_ctrl;
#[doc = "Ram Error Status Register"]
pub struct RAMERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram Error Status Register"]
pub mod ram_err_stat;
#[doc = "Diagnostic Data Vector Upper 32bits"]
pub struct DIAGDATAVECTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Diagnostic Data Vector Upper 32bits"]
pub mod diag_data_vect_h;
#[doc = "Diagnostic Data Vector Lower 32bits"]
pub struct DIAGDATAVECTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Diagnostic Data Vector Lower 32bits"]
pub mod diag_data_vect_l;
#[doc = "Diagnostic ECC Vector"]
pub struct DIAGECCVECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Diagnostic ECC Vector"]
pub mod diag_ecc_vect;
#[doc = "RAM Test Control register"]
pub struct RAMTEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RAM Test Control register"]
pub mod ram_test;
#[doc = "RAM Address Decode Vector Test Register"]
pub struct ADDRVECTTSTREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RAM Address Decode Vector Test Register"]
pub mod addr_vect_tst_reg;
#[doc = "Memory Initialization Domain Register"]
pub struct MEMINITDOMAIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Initialization Domain Register"]
pub mod mem_init_domain;
#[doc = "Bank to Domain Mapping Register0"]
pub struct BNKDMNMAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bank to Domain Mapping Register0"]
pub mod bnk_dmn_map0;
#[doc = "Bank to Domain Mapping Register1"]
pub struct BNKDMNMAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bank to Domain Mapping Register1"]
pub mod bnk_dmn_map1;
