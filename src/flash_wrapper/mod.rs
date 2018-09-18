#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Read Control Register"]
    pub frd_cntl: FRDCNTL,
    #[doc = "0x04 - Special Read Control Register"]
    pub fsp_rd: FSPRD,
    #[doc = "0x08 - Error Correction Control Register1"]
    pub fedac_ctrl1: FEDACCTRL1,
    _reserved0: [u8; 8usize],
    #[doc = "0x14 - PortA Error Status register"]
    pub port_aerr_stat: PORTAERRSTAT,
    #[doc = "0x18 - PortB Error Status register"]
    pub port_berr_stat: PORTBERRSTAT,
    #[doc = "0x1c - Error Status Register"]
    pub fedac_stat: FEDACSTAT,
    _reserved1: [u8; 4usize],
    #[doc = "0x24 - Error Detection Sector Disable"]
    pub fedac_sdis: FEDACSDIS,
    #[doc = "0x28 - Primary Address Tag Register"]
    pub fpprim_addr_tag: FPPRIMADDRTAG,
    #[doc = "0x2c - Redundant Address Tag Register"]
    pub fredu_addr_tag: FREDUADDRTAG,
    #[doc = "0x30 - Bank Protection Register"]
    pub fbnk_prot: FBNKPROT,
    #[doc = "0x34 - Bank Sector Enable Register"]
    pub fbnk_sec: FBNKSEC,
    #[doc = "0x38 - Bank Busy Register"]
    pub fbusy: FBUSY,
    #[doc = "0x3c - Bank Access Control Register"]
    pub fbnk_acc: FBNKACC,
    #[doc = "0x40 - Bank Fallback Power Register"]
    pub fbnk_fallback: FBNKFALLBACK,
    #[doc = "0x44 - Bank/Pump Ready Register"]
    pub fbnk_pmp_rdy: FBNKPMPRDY,
    #[doc = "0x48 - Pump Access Control Register 1"]
    pub fpmp_acc1: FPMPACC1,
    #[doc = "0x4c - Pump Access Control Register 2"]
    pub fpmp_acc2: FPMPACC2,
    #[doc = "0x50 - Module Access Control Register"]
    pub fmdl_acc: FMDLACC,
    #[doc = "0x54 - Module Status Register"]
    pub fmdl_stat: FMDLSTAT,
    #[doc = "0x58 - EEPROM Emulation Data MSW Register"]
    pub femu_dat_msw: FEMUDATMSW,
    #[doc = "0x5c - EEPROM Emulation Data LSW Register"]
    pub femu_dat_lsw: FEMUDATLSW,
    #[doc = "0x60 - EEPROM Emulation ECC Register"]
    pub femu_ecc: FEMUECC,
    #[doc = "0x64 - Flash Lock Register"]
    pub flock: FLOCK,
    #[doc = "0x68 - EEPROM Emulation Address"]
    pub femu_addr: FEMUADDR,
    #[doc = "0x6c - Diagnostic Control Register"]
    pub fdiag_ctrl: FDIAGCTRL,
    _reserved2: [u8; 4usize],
    #[doc = "0x74 - Raw Address register during Diagnostic mode"]
    pub fraw_addr: FRAWADDR,
    _reserved3: [u8; 4usize],
    #[doc = "0x7c - Parity Override"]
    pub fpar_ovr: FPAROVR,
    _reserved4: [u8; 52usize],
    #[doc = "0xb4 - Reset Config and JSM Key Valid"]
    pub rst_conf_jsm_val: RSTCONFJSMVAL,
    #[doc = "0xb8 - Crossbar access timeout threshold register"]
    pub xbar_threshold: XBARTHRESHOLD,
    _reserved5: [u8; 20usize],
    #[doc = "0xd0 - Reset Config Value lower 32bit"]
    pub rst_conf_val0: RSTCONFVAL0,
    #[doc = "0xd4 - Reset Config Value Upper 32bit"]
    pub rst_conf_val1: RSTCONFVAL1,
    _reserved6: [u8; 8usize],
    #[doc = "0xe0 - JSM Key Bits 31:0"]
    pub jsm_key0: JSMKEY0,
    #[doc = "0xe4 - JSM Key Bits 63:32"]
    pub jsm_key1: JSMKEY1,
    #[doc = "0xe8 - JSM Key Bits 95:64"]
    pub jsm_key2: JSMKEY2,
    #[doc = "0xec - JSM Key Bits 127:96"]
    pub jsm_key3: JSMKEY3,
}
#[doc = "Read Control Register"]
pub struct FRDCNTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read Control Register"]
pub mod frd_cntl;
#[doc = "Special Read Control Register"]
pub struct FSPRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Special Read Control Register"]
pub mod fsp_rd;
#[doc = "Error Correction Control Register1"]
pub struct FEDACCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Correction Control Register1"]
pub mod fedac_ctrl1;
#[doc = "PortA Error Status register"]
pub struct PORTAERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PortA Error Status register"]
pub mod port_aerr_stat;
#[doc = "PortB Error Status register"]
pub struct PORTBERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PortB Error Status register"]
pub mod port_berr_stat;
#[doc = "Error Status Register"]
pub struct FEDACSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Status Register"]
pub mod fedac_stat;
#[doc = "Error Detection Sector Disable"]
pub struct FEDACSDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Detection Sector Disable"]
pub mod fedac_sdis;
#[doc = "Primary Address Tag Register"]
pub struct FPPRIMADDRTAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Primary Address Tag Register"]
pub mod fpprim_addr_tag;
#[doc = "Redundant Address Tag Register"]
pub struct FREDUADDRTAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Redundant Address Tag Register"]
pub mod fredu_addr_tag;
#[doc = "Bank Protection Register"]
pub struct FBNKPROT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bank Protection Register"]
pub mod fbnk_prot;
#[doc = "Bank Sector Enable Register"]
pub struct FBNKSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bank Sector Enable Register"]
pub mod fbnk_sec;
#[doc = "Bank Busy Register"]
pub struct FBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bank Busy Register"]
pub mod fbusy;
#[doc = "Bank Access Control Register"]
pub struct FBNKACC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bank Access Control Register"]
pub mod fbnk_acc;
#[doc = "Bank Fallback Power Register"]
pub struct FBNKFALLBACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bank Fallback Power Register"]
pub mod fbnk_fallback;
#[doc = "Bank/Pump Ready Register"]
pub struct FBNKPMPRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bank/Pump Ready Register"]
pub mod fbnk_pmp_rdy;
#[doc = "Pump Access Control Register 1"]
pub struct FPMPACC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pump Access Control Register 1"]
pub mod fpmp_acc1;
#[doc = "Pump Access Control Register 2"]
pub struct FPMPACC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pump Access Control Register 2"]
pub mod fpmp_acc2;
#[doc = "Module Access Control Register"]
pub struct FMDLACC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Access Control Register"]
pub mod fmdl_acc;
#[doc = "Module Status Register"]
pub struct FMDLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Status Register"]
pub mod fmdl_stat;
#[doc = "EEPROM Emulation Data MSW Register"]
pub struct FEMUDATMSW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Emulation Data MSW Register"]
pub mod femu_dat_msw;
#[doc = "EEPROM Emulation Data LSW Register"]
pub struct FEMUDATLSW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Emulation Data LSW Register"]
pub mod femu_dat_lsw;
#[doc = "EEPROM Emulation ECC Register"]
pub struct FEMUECC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Emulation ECC Register"]
pub mod femu_ecc;
#[doc = "Flash Lock Register"]
pub struct FLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Lock Register"]
pub mod flock;
#[doc = "EEPROM Emulation Address"]
pub struct FEMUADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Emulation Address"]
pub mod femu_addr;
#[doc = "Diagnostic Control Register"]
pub struct FDIAGCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Diagnostic Control Register"]
pub mod fdiag_ctrl;
#[doc = "Raw Address register during Diagnostic mode"]
pub struct FRAWADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Address register during Diagnostic mode"]
pub mod fraw_addr;
#[doc = "Parity Override"]
pub struct FPAROVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Override"]
pub mod fpar_ovr;
#[doc = "Reset Config and JSM Key Valid"]
pub struct RSTCONFJSMVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Config and JSM Key Valid"]
pub mod rst_conf_jsm_val;
#[doc = "Crossbar access timeout threshold register"]
pub struct XBARTHRESHOLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Crossbar access timeout threshold register"]
pub mod xbar_threshold;
#[doc = "Reset Config Value lower 32bit"]
pub struct RSTCONFVAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Config Value lower 32bit"]
pub mod rst_conf_val0;
#[doc = "Reset Config Value Upper 32bit"]
pub struct RSTCONFVAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Config Value Upper 32bit"]
pub mod rst_conf_val1;
#[doc = "JSM Key Bits 31:0"]
pub struct JSMKEY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JSM Key Bits 31:0"]
pub mod jsm_key0;
#[doc = "JSM Key Bits 63:32"]
pub struct JSMKEY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JSM Key Bits 63:32"]
pub mod jsm_key1;
#[doc = "JSM Key Bits 95:64"]
pub struct JSMKEY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JSM Key Bits 95:64"]
pub mod jsm_key2;
#[doc = "JSM Key Bits 127:96"]
pub struct JSMKEY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JSM Key Bits 127:96"]
pub mod jsm_key3;
