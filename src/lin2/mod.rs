#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - global control register"]
    pub glb_ctrl0: GLBCTRL0,
    #[doc = "0x04 - global control register"]
    pub glb_ctrl1: GLBCTRL1,
    #[doc = "0x08 - global control register"]
    pub glb_ctrl2: GLBCTRL2,
    #[doc = "0x0c - Set Interrupt Register"]
    pub set_int: SETINT,
    #[doc = "0x10 - Clear Interrupt Register"]
    pub clear_int: CLEARINT,
    #[doc = "0x14 - Set Interrupt Level Register"]
    pub set_int_lvl: SETINTLVL,
    #[doc = "0x18 - Clear Interrupt Level Register"]
    pub clear_int_lvl: CLEARINTLVL,
    #[doc = "0x1c - Flags Register"]
    pub flr: FLR,
    #[doc = "0x20 - Interrupt Vector Offset 0"]
    pub int_vect0: INTVECT0,
    #[doc = "0x24 - Interrupt Vector Offset 1"]
    pub int_vect1: INTVECT1,
    #[doc = "0x28 - Format Control Register"]
    pub format: FORMAT,
    #[doc = "0x2c - Baud Rate Selection Register"]
    pub brsr: BRSR,
    #[doc = "0x30 - SCI Data Buffer"]
    pub ed: ED,
    #[doc = "0x34 - SCI Data Buffer"]
    pub rd: RD,
    #[doc = "0x38 - SCI Data Buffer"]
    pub td: TD,
    #[doc = "0x3c - Pin Control 0"]
    pub fun: FUN,
    #[doc = "0x40 - Pin Control 1"]
    pub dir: DIR,
    #[doc = "0x44 - Pin Control 2"]
    pub din: DIN,
    #[doc = "0x48 - Pin Control 3"]
    pub dout: DOUT,
    #[doc = "0x4c - Pin Control 4"]
    pub dset: DSET,
    #[doc = "0x50 - Pin Control 5"]
    pub dclr: DCLR,
    #[doc = "0x54 - Pin Control 6"]
    pub pdr: PDR,
    #[doc = "0x58 - Pin Control 7"]
    pub pdis: PDIS,
    #[doc = "0x5c - Pin Control 8"]
    pub psel: PSEL,
    #[doc = "0x60 - BLinCompARE Register"]
    pub lin_comp: LINCOMP,
    #[doc = "0x64 - LinRd0 Register"]
    pub lin_rd0: LINRD0,
    #[doc = "0x68 - LinRd1 Register"]
    pub lin_rd1: LINRD1,
    #[doc = "0x6c - LinMask Register"]
    pub lin_mask: LINMASK,
    #[doc = "0x70 - LinId Register"]
    pub lin_id: LINID,
    #[doc = "0x74 - LIntD0 Register"]
    pub lin_td0: LINTD0,
    #[doc = "0x78 - LIntD1 Register"]
    pub lin_td1: LINTD1,
    #[doc = "0x7c - Maximum Baud Rate Selection Register"]
    pub mbrsr: MBRSR,
    #[doc = "0x80 - Slew Rate Control Register"]
    pub sr_sel: SRSEL,
    _reserved0: [u8; 12usize],
    #[doc = "0x90 - IODFT for BLin moduler"]
    pub io_dft_ctrl: IODFTCTRL,
}
#[doc = "global control register"]
pub struct GLBCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "global control register"]
pub mod glb_ctrl0;
#[doc = "global control register"]
pub struct GLBCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "global control register"]
pub mod glb_ctrl1;
#[doc = "global control register"]
pub struct GLBCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "global control register"]
pub mod glb_ctrl2;
#[doc = "Set Interrupt Register"]
pub struct SETINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Interrupt Register"]
pub mod set_int;
#[doc = "Clear Interrupt Register"]
pub struct CLEARINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Interrupt Register"]
pub mod clear_int;
#[doc = "Set Interrupt Level Register"]
pub struct SETINTLVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Interrupt Level Register"]
pub mod set_int_lvl;
#[doc = "Clear Interrupt Level Register"]
pub struct CLEARINTLVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Interrupt Level Register"]
pub mod clear_int_lvl;
#[doc = "Flags Register"]
pub struct FLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flags Register"]
pub mod flr;
#[doc = "Interrupt Vector Offset 0"]
pub struct INTVECT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Vector Offset 0"]
pub mod int_vect0;
#[doc = "Interrupt Vector Offset 1"]
pub struct INTVECT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Vector Offset 1"]
pub mod int_vect1;
#[doc = "Format Control Register"]
pub struct FORMAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Format Control Register"]
pub mod format;
#[doc = "Baud Rate Selection Register"]
pub struct BRSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Selection Register"]
pub mod brsr;
#[doc = "SCI Data Buffer"]
pub struct ED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI Data Buffer"]
pub mod ed;
#[doc = "SCI Data Buffer"]
pub struct RD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI Data Buffer"]
pub mod rd;
#[doc = "SCI Data Buffer"]
pub struct TD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCI Data Buffer"]
pub mod td;
#[doc = "Pin Control 0"]
pub struct FUN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 0"]
pub mod fun;
#[doc = "Pin Control 1"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 1"]
pub mod dir;
#[doc = "Pin Control 2"]
pub struct DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 2"]
pub mod din;
#[doc = "Pin Control 3"]
pub struct DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 3"]
pub mod dout;
#[doc = "Pin Control 4"]
pub struct DSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 4"]
pub mod dset;
#[doc = "Pin Control 5"]
pub struct DCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 5"]
pub mod dclr;
#[doc = "Pin Control 6"]
pub struct PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 6"]
pub mod pdr;
#[doc = "Pin Control 7"]
pub struct PDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 7"]
pub mod pdis;
#[doc = "Pin Control 8"]
pub struct PSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 8"]
pub mod psel;
#[doc = "BLinCompARE Register"]
pub struct LINCOMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BLinCompARE Register"]
pub mod lin_comp;
#[doc = "LinRd0 Register"]
pub struct LINRD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LinRd0 Register"]
pub mod lin_rd0;
#[doc = "LinRd1 Register"]
pub struct LINRD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LinRd1 Register"]
pub mod lin_rd1;
#[doc = "LinMask Register"]
pub struct LINMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LinMask Register"]
pub mod lin_mask;
#[doc = "LinId Register"]
pub struct LINID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LinId Register"]
pub mod lin_id;
#[doc = "LIntD0 Register"]
pub struct LINTD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LIntD0 Register"]
pub mod lin_td0;
#[doc = "LIntD1 Register"]
pub struct LINTD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LIntD1 Register"]
pub mod lin_td1;
#[doc = "Maximum Baud Rate Selection Register"]
pub struct MBRSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum Baud Rate Selection Register"]
pub mod mbrsr;
#[doc = "Slew Rate Control Register"]
pub struct SRSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slew Rate Control Register"]
pub mod sr_sel;
#[doc = "IODFT for BLin moduler"]
pub struct IODFTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IODFT for BLin moduler"]
pub mod io_dft_ctrl;
