#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    pub glb_ctrl: GLBCTRL,
    #[doc = "0x04 - Trace Enable Register"]
    pub tr_ena: TRENA,
    #[doc = "0x08 - Global Status Register"]
    pub gsr: GSR,
    #[doc = "0x0c - Ram 1 Trace Region 1 Register"]
    pub ram1reg1: RAM1REG1,
    #[doc = "0x10 - Ram 1 Trace Region 2 Register"]
    pub ram1reg2: RAM1REG2,
    #[doc = "0x14 - Ram 2 Trace Region 1 Register"]
    pub ram2reg1: RAM2REG1,
    #[doc = "0x18 - Ram 2 Trace Region 2 Register"]
    pub ram2reg2: RAM2REG2,
    #[doc = "0x1c - Ram 3 Trace Region 1 Register"]
    pub ram3reg1: RAM3REG1,
    #[doc = "0x20 - Ram 3 Trace Region 2 Register"]
    pub ram3reg2: RAM3REG2,
    #[doc = "0x24 - Peripheral Trace Region 1 Register"]
    pub per_reg1: PERREG1,
    #[doc = "0x28 - Peripheral Trace Region 2 Register"]
    pub per_reg2: PERREG2,
    #[doc = "0x2c - Direct Data Mode Write Register"]
    pub ddm_w: DDMW,
    _reserved0: [u8; 4usize],
    #[doc = "0x34 - Pin Control 0"]
    pub fun: FUN,
    #[doc = "0x38 - Pin Control 1"]
    pub dir: DIR,
    #[doc = "0x3c - Pin Control 2"]
    pub din: DIN,
    #[doc = "0x40 - Pin Control 3"]
    pub dout: DOUT,
    #[doc = "0x44 - Pin Control 4"]
    pub dset: DSET,
    #[doc = "0x48 - Pin Control 5"]
    pub dclr: DCLR,
    #[doc = "0x4c - Pin Control 6"]
    pub pdr: PDR,
    #[doc = "0x50 - Pin Control 7"]
    pub pdis: PDIS,
    #[doc = "0x54 - Pin Control 8"]
    pub psel: PSEL,
    #[doc = "0x58 - IODFT CONTROL"]
    pub io_dft_ctrl: IODFTCTRL,
}
#[doc = "Global Control Register"]
pub struct GLBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod glb_ctrl;
#[doc = "Trace Enable Register"]
pub struct TRENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trace Enable Register"]
pub mod tr_ena;
#[doc = "Global Status Register"]
pub struct GSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Status Register"]
pub mod gsr;
#[doc = "Ram 1 Trace Region 1 Register"]
pub struct RAM1REG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram 1 Trace Region 1 Register"]
pub mod ram1reg1;
#[doc = "Ram 1 Trace Region 2 Register"]
pub struct RAM1REG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram 1 Trace Region 2 Register"]
pub mod ram1reg2;
#[doc = "Ram 2 Trace Region 1 Register"]
pub struct RAM2REG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram 2 Trace Region 1 Register"]
pub mod ram2reg1;
#[doc = "Ram 2 Trace Region 2 Register"]
pub struct RAM2REG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram 2 Trace Region 2 Register"]
pub mod ram2reg2;
#[doc = "Ram 3 Trace Region 1 Register"]
pub struct RAM3REG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram 3 Trace Region 1 Register"]
pub mod ram3reg1;
#[doc = "Ram 3 Trace Region 2 Register"]
pub struct RAM3REG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram 3 Trace Region 2 Register"]
pub mod ram3reg2;
#[doc = "Peripheral Trace Region 1 Register"]
pub struct PERREG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Trace Region 1 Register"]
pub mod per_reg1;
#[doc = "Peripheral Trace Region 2 Register"]
pub struct PERREG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Trace Region 2 Register"]
pub mod per_reg2;
#[doc = "Direct Data Mode Write Register"]
pub struct DDMW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct Data Mode Write Register"]
pub mod ddm_w;
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
#[doc = "IODFT CONTROL"]
pub struct IODFTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IODFT CONTROL"]
pub mod io_dft_ctrl;
