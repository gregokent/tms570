#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    pub gctrl: GCTRL,
    #[doc = "0x04 - Revision ID"]
    pub rev: REV,
    #[doc = "0x08 - Count0 Seed Value"]
    pub cnt_seed0: CNTSEED0,
    #[doc = "0x0c - Valid0 Seed Value"]
    pub valid_seed0: VALIDSEED0,
    #[doc = "0x10 - Count1 Seed Value"]
    pub cnt_seed1: CNTSEED1,
    #[doc = "0x14 - Status Register"]
    pub stat: STAT,
    #[doc = "0x18 - Count0 Value Register"]
    pub cnt0: CNT0,
    #[doc = "0x1c - Valid0 Value Register"]
    pub valid0: VALID0,
    #[doc = "0x20 - Count1 Value Register"]
    pub cnt1: CNT1,
    #[doc = "0x24 - Clock Source Selection Register 1"]
    pub clk_src1: CLKSRC1,
    #[doc = "0x28 - Clock Source Selection Register 0"]
    pub clk_src0: CLKSRC0,
}
#[doc = "Global Control Register"]
pub struct GCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod gctrl;
#[doc = "Revision ID"]
pub struct REV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Revision ID"]
pub mod rev;
#[doc = "Count0 Seed Value"]
pub struct CNTSEED0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count0 Seed Value"]
pub mod cnt_seed0;
#[doc = "Valid0 Seed Value"]
pub struct VALIDSEED0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Valid0 Seed Value"]
pub mod valid_seed0;
#[doc = "Count1 Seed Value"]
pub struct CNTSEED1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count1 Seed Value"]
pub mod cnt_seed1;
#[doc = "Status Register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod stat;
#[doc = "Count0 Value Register"]
pub struct CNT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count0 Value Register"]
pub mod cnt0;
#[doc = "Valid0 Value Register"]
pub struct VALID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Valid0 Value Register"]
pub mod valid0;
#[doc = "Count1 Value Register"]
pub struct CNT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count1 Value Register"]
pub mod cnt1;
#[doc = "Clock Source Selection Register 1"]
pub struct CLKSRC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Source Selection Register 1"]
pub mod clk_src1;
#[doc = "Clock Source Selection Register 0"]
pub struct CLKSRC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Source Selection Register 0"]
pub mod clk_src0;
