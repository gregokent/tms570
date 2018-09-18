#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 52usize],
    #[doc = "0x34 - Data Direction Gio A"]
    pub dir: DIR,
    #[doc = "0x38 - Data Input Gio A"]
    pub din: DIN,
    #[doc = "0x3c - Data Output Gio A"]
    pub dout: DOUT,
    #[doc = "0x40 - Data Set Gio A"]
    pub dset: DSET,
    #[doc = "0x44 - Data Clear Gio A"]
    pub dclr: DCLR,
    #[doc = "0x48 - Open Drain Gio A"]
    pub pdr: PDR,
    #[doc = "0x4c - Pull Disable Gio A"]
    pub pdis: PDIS,
    #[doc = "0x50 - Pull Select Gio A"]
    pub psel: PSEL,
    _reserved1: [u8; 224usize],
    #[doc = "0x134 - Slew Rate Select Gio A"]
    pub srs: SRS,
}
#[doc = "Data Direction Gio A"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction Gio A"]
pub mod dir;
#[doc = "Data Input Gio A"]
pub struct DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input Gio A"]
pub mod din;
#[doc = "Data Output Gio A"]
pub struct DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Gio A"]
pub mod dout;
#[doc = "Data Set Gio A"]
pub struct DSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Set Gio A"]
pub mod dset;
#[doc = "Data Clear Gio A"]
pub struct DCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Clear Gio A"]
pub mod dclr;
#[doc = "Open Drain Gio A"]
pub struct PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open Drain Gio A"]
pub mod pdr;
#[doc = "Pull Disable Gio A"]
pub struct PDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull Disable Gio A"]
pub mod pdis;
#[doc = "Pull Select Gio A"]
pub struct PSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull Select Gio A"]
pub mod psel;
#[doc = "Slew Rate Select Gio A"]
pub struct SRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slew Rate Select Gio A"]
pub mod srs;
