#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 84usize],
    #[doc = "0x54 - Data Direction Gio B"]
    pub dir: DIR,
    #[doc = "0x58 - Data Input Gio B"]
    pub din: DIN,
    #[doc = "0x5c - Data Output Gio B"]
    pub dout: DOUT,
    #[doc = "0x60 - Data Set Gio B"]
    pub dset: DSET,
    #[doc = "0x64 - Data Clear Gio B"]
    pub dclr: DCLR,
    #[doc = "0x68 - Open Drain Gio B"]
    pub pdr: PDR,
    #[doc = "0x6c - Pull Disable Gio B"]
    pub pdis: PDIS,
    #[doc = "0x70 - Pull Select Gio B"]
    pub psel: PSEL,
    _reserved1: [u8; 196usize],
    #[doc = "0x138 - Slew Rate Select Gio B"]
    pub srs: SRS,
}
#[doc = "Data Direction Gio B"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Direction Gio B"]
pub mod dir;
#[doc = "Data Input Gio B"]
pub struct DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input Gio B"]
pub mod din;
#[doc = "Data Output Gio B"]
pub struct DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Gio B"]
pub mod dout;
#[doc = "Data Set Gio B"]
pub struct DSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Set Gio B"]
pub mod dset;
#[doc = "Data Clear Gio B"]
pub struct DCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Clear Gio B"]
pub mod dclr;
#[doc = "Open Drain Gio B"]
pub struct PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open Drain Gio B"]
pub mod pdr;
#[doc = "Pull Disable Gio B"]
pub struct PDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull Disable Gio B"]
pub mod pdis;
#[doc = "Pull Select Gio B"]
pub struct PSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull Select Gio B"]
pub mod psel;
#[doc = "Slew Rate Select Gio B"]
pub struct SRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slew Rate Select Gio B"]
pub mod srs;
