#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Mapping Register"]
    pub firq_pr4: FIRQPR4,
    #[doc = "0x04 - Channel Mapping Register"]
    pub firq_pr5: FIRQPR5,
    #[doc = "0x08 - Channel Mapping Register"]
    pub firq_pr6: FIRQPR6,
    #[doc = "0x0c - Channel Mapping Register"]
    pub firq_pr7: FIRQPR7,
    #[doc = "0x10 - Channel Mapping Register"]
    pub int_req4: INTREQ4,
    #[doc = "0x14 - Channel Mapping Register"]
    pub int_req5: INTREQ5,
    #[doc = "0x18 - Channel Mapping Register"]
    pub int_req6: INTREQ6,
    #[doc = "0x1c - Channel Mapping Register"]
    pub int_req7: INTREQ7,
    #[doc = "0x20 - Channel Mapping Register"]
    pub req_mask_set4: REQMASKSET4,
    #[doc = "0x24 - Channel Mapping Register"]
    pub req_mask_set5: REQMASKSET5,
    #[doc = "0x28 - Channel Mapping Register"]
    pub req_mask_set6: REQMASKSET6,
    #[doc = "0x2c - Channel Mapping Register"]
    pub req_mask_set7: REQMASKSET7,
    #[doc = "0x30 - Channel Mapping Register"]
    pub req_mask_clr4: REQMASKCLR4,
    #[doc = "0x34 - Channel Mapping Register"]
    pub req_mask_clr5: REQMASKCLR5,
    #[doc = "0x38 - Channel Mapping Register"]
    pub req_mask_clr6: REQMASKCLR6,
    #[doc = "0x3c - Channel Mapping Register"]
    pub req_mask_clr7: REQMASKCLR7,
    #[doc = "0x40 - Channel Mapping Register"]
    pub wake_mask_set4: WAKEMASKSET4,
    #[doc = "0x44 - Channel Mapping Register"]
    pub wake_mask_set5: WAKEMASKSET5,
    #[doc = "0x48 - Channel Mapping Register"]
    pub wake_mask_set6: WAKEMASKSET6,
    #[doc = "0x4c - Channel Mapping Register"]
    pub wake_mask_set7: WAKEMASKSET7,
    #[doc = "0x50 - Channel Mapping Register"]
    pub wake_mask_clr4: WAKEMASKCLR4,
    #[doc = "0x54 - Channel Mapping Register"]
    pub wake_mask_clr5: WAKEMASKCLR5,
    #[doc = "0x58 - Channel Mapping Register"]
    pub wake_mask_clr6: WAKEMASKCLR6,
    #[doc = "0x5c - Channel Mapping Register"]
    pub wake_mask_clr7: WAKEMASKCLR7,
    #[doc = "0x60 - Channel Mapping Register"]
    pub chan_ctrl32: CHANCTRL32,
    #[doc = "0x64 - Channel Mapping Register"]
    pub chan_ctrl33: CHANCTRL33,
    #[doc = "0x68 - Channel Mapping Register"]
    pub chan_ctrl34: CHANCTRL34,
    #[doc = "0x6c - Channel Mapping Register"]
    pub chan_ctrl35: CHANCTRL35,
    #[doc = "0x70 - Channel Mapping Register"]
    pub chan_ctrl36: CHANCTRL36,
    #[doc = "0x74 - Channel Mapping Register"]
    pub chan_ctrl37: CHANCTRL37,
    #[doc = "0x78 - Channel Mapping Register"]
    pub chan_ctrl38: CHANCTRL38,
    #[doc = "0x7c - Channel Mapping Register"]
    pub chan_ctrl39: CHANCTRL39,
    #[doc = "0x80 - Channel Mapping Register"]
    pub chan_ctrl40: CHANCTRL40,
    #[doc = "0x84 - Channel Mapping Register"]
    pub chan_ctrl41: CHANCTRL41,
    #[doc = "0x88 - Channel Mapping Register"]
    pub chan_ctrl42: CHANCTRL42,
    #[doc = "0x8c - Channel Mapping Register"]
    pub chan_ctrl43: CHANCTRL43,
    #[doc = "0x90 - Channel Mapping Register"]
    pub chan_ctrl44: CHANCTRL44,
    #[doc = "0x94 - Channel Mapping Register"]
    pub chan_ctrl45: CHANCTRL45,
    #[doc = "0x98 - Channel Mapping Register"]
    pub chan_ctrl46: CHANCTRL46,
    #[doc = "0x9c - Channel Mapping Register"]
    pub chan_ctrl47: CHANCTRL47,
    #[doc = "0xa0 - Channel Mapping Register"]
    pub chan_ctrl48: CHANCTRL48,
    #[doc = "0xa4 - Channel Mapping Register"]
    pub chan_ctrl49: CHANCTRL49,
    #[doc = "0xa8 - Channel Mapping Register"]
    pub chan_ctrl50: CHANCTRL50,
    #[doc = "0xac - Channel Mapping Register"]
    pub chan_ctrl51: CHANCTRL51,
    #[doc = "0xb0 - Channel Mapping Register"]
    pub chan_ctrl52: CHANCTRL52,
    #[doc = "0xb4 - Channel Mapping Register"]
    pub chan_ctrl53: CHANCTRL53,
    #[doc = "0xb8 - Channel Mapping Register"]
    pub chan_ctrl54: CHANCTRL54,
    #[doc = "0xbc - Channel Mapping Register"]
    pub chan_ctrl55: CHANCTRL55,
    #[doc = "0xc0 - Channel Mapping Register"]
    pub chan_ctrl56: CHANCTRL56,
    #[doc = "0xc4 - Channel Mapping Register"]
    pub chan_ctrl57: CHANCTRL57,
    #[doc = "0xc8 - Channel Mapping Register"]
    pub chan_ctrl58: CHANCTRL58,
    #[doc = "0xcc - Channel Mapping Register"]
    pub chan_ctrl59: CHANCTRL59,
    #[doc = "0xd0 - Channel Mapping Register"]
    pub chan_ctrl60: CHANCTRL60,
    #[doc = "0xd4 - Channel Mapping Register"]
    pub chan_ctrl61: CHANCTRL61,
    #[doc = "0xd8 - Channel Mapping Register"]
    pub chan_ctrl62: CHANCTRL62,
    #[doc = "0xdc - Channel Mapping Register"]
    pub chan_ctrl63: CHANCTRL63,
    _reserved0: [u8; 12usize],
    #[doc = "0xec - VIM RAM Parity/ECC Status Register"]
    pub par_ecc_stat: PARECCSTAT,
    #[doc = "0xf0 - VIM RAM Parity/ECC Control Register"]
    pub par_ecc_ctl: PARECCCTL,
    #[doc = "0xf4 - Uncorrectable Error Address Register"]
    pub uerr_addr: UERRADDR,
    #[doc = "0xf8 - FallBack Vector Address Register"]
    pub fb_vec_addr: FBVECADDR,
    #[doc = "0xfc - Single Bit Error Address Register"]
    pub sberr_addr: SBERRADDR,
}
#[doc = "Channel Mapping Register"]
pub struct FIRQPR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod firq_pr4;
#[doc = "Channel Mapping Register"]
pub struct FIRQPR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod firq_pr5;
#[doc = "Channel Mapping Register"]
pub struct FIRQPR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod firq_pr6;
#[doc = "Channel Mapping Register"]
pub struct FIRQPR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod firq_pr7;
#[doc = "Channel Mapping Register"]
pub struct INTREQ4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod int_req4;
#[doc = "Channel Mapping Register"]
pub struct INTREQ5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod int_req5;
#[doc = "Channel Mapping Register"]
pub struct INTREQ6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod int_req6;
#[doc = "Channel Mapping Register"]
pub struct INTREQ7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod int_req7;
#[doc = "Channel Mapping Register"]
pub struct REQMASKSET4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod req_mask_set4;
#[doc = "Channel Mapping Register"]
pub struct REQMASKSET5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod req_mask_set5;
#[doc = "Channel Mapping Register"]
pub struct REQMASKSET6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod req_mask_set6;
#[doc = "Channel Mapping Register"]
pub struct REQMASKSET7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod req_mask_set7;
#[doc = "Channel Mapping Register"]
pub struct REQMASKCLR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod req_mask_clr4;
#[doc = "Channel Mapping Register"]
pub struct REQMASKCLR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod req_mask_clr5;
#[doc = "Channel Mapping Register"]
pub struct REQMASKCLR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod req_mask_clr6;
#[doc = "Channel Mapping Register"]
pub struct REQMASKCLR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod req_mask_clr7;
#[doc = "Channel Mapping Register"]
pub struct WAKEMASKSET4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod wake_mask_set4;
#[doc = "Channel Mapping Register"]
pub struct WAKEMASKSET5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod wake_mask_set5;
#[doc = "Channel Mapping Register"]
pub struct WAKEMASKSET6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod wake_mask_set6;
#[doc = "Channel Mapping Register"]
pub struct WAKEMASKSET7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod wake_mask_set7;
#[doc = "Channel Mapping Register"]
pub struct WAKEMASKCLR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod wake_mask_clr4;
#[doc = "Channel Mapping Register"]
pub struct WAKEMASKCLR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod wake_mask_clr5;
#[doc = "Channel Mapping Register"]
pub struct WAKEMASKCLR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod wake_mask_clr6;
#[doc = "Channel Mapping Register"]
pub struct WAKEMASKCLR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod wake_mask_clr7;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl32;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl33;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl34;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl35;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl36;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl37;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl38;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl39;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl40;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl41;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl42;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl43;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl44;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl45;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl46;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl47;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl48;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl49;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl50;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl51;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl52;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl53;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl54;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl55;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl56;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl57;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl58;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl59;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl60;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl61;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl62;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl63;
#[doc = "VIM RAM Parity/ECC Status Register"]
pub struct PARECCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VIM RAM Parity/ECC Status Register"]
pub mod par_ecc_stat;
#[doc = "VIM RAM Parity/ECC Control Register"]
pub struct PARECCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VIM RAM Parity/ECC Control Register"]
pub mod par_ecc_ctl;
#[doc = "Uncorrectable Error Address Register"]
pub struct UERRADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register"]
pub mod uerr_addr;
#[doc = "FallBack Vector Address Register"]
pub struct FBVECADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FallBack Vector Address Register"]
pub mod fb_vec_addr;
#[doc = "Single Bit Error Address Register"]
pub struct SBERRADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Bit Error Address Register"]
pub mod sberr_addr;
