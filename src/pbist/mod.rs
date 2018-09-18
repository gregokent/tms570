#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - Variable Address Register 0"]
    pub var_addr0: VARADDR0,
    #[doc = "0x104 - Variable Address Register 1"]
    pub var_addr1: VARADDR1,
    #[doc = "0x108 - Variable Address Register 2"]
    pub var_addr2: VARADDR2,
    #[doc = "0x10c - Variable Address Register 3"]
    pub var_addr3: VARADDR3,
    #[doc = "0x110 - Variable Loop Count Register 0"]
    pub var_lp_cnt0: VARLPCNT0,
    #[doc = "0x114 - Variable Loop Count Register 1"]
    pub var_lp_cnt1: VARLPCNT1,
    #[doc = "0x118 - Variable Loop Count Register 0"]
    pub var_lp_cnt2: VARLPCNT2,
    #[doc = "0x11c - Variable Loop Count Register 1"]
    pub var_lp_cnt3: VARLPCNT3,
    #[doc = "0x120 - DD0 Data Register"]
    pub dd0: DD0,
    #[doc = "0x124 - DE0 Data Register"]
    pub de0: DE0,
    _reserved1: [u8; 8usize],
    #[doc = "0x130 - Constant Address Register 0"]
    pub const_addr0: CONSTADDR0,
    #[doc = "0x134 - Constant Address Register 1"]
    pub const_addr1: CONSTADDR1,
    #[doc = "0x138 - Constant Address Register 2"]
    pub const_addr2: CONSTADDR2,
    #[doc = "0x13c - Constant Address Register 3"]
    pub const_addr3: CONSTADDR3,
    #[doc = "0x140 - Constant Loop Count Register 0"]
    pub const_lp_cnt0: CONSTLPCNT0,
    #[doc = "0x144 - Constant Loop Count Register 1"]
    pub const_lp_cnt1: CONSTLPCNT1,
    #[doc = "0x148 - Constant Loop Count Register 2"]
    pub const_lp_cnt2: CONSTLPCNT2,
    #[doc = "0x14c - Constant Loop Count Register 3"]
    pub const_lp_cnt3: CONSTLPCNT3,
    #[doc = "0x150 - Constant Increment Register 0"]
    pub const_inc0: CONSTINC0,
    #[doc = "0x154 - Constant Increment Register 1"]
    pub const_inc1: CONSTINC1,
    #[doc = "0x158 - Constant Increment Register 2"]
    pub const_inc2: CONSTINC2,
    #[doc = "0x15c - Constant Increment Register 3"]
    pub const_inc3: CONSTINC3,
    #[doc = "0x160 - Ram Configuration"]
    pub ram_t: RAMT,
    #[doc = "0x164 - Datalogger"]
    pub dlr: DLR,
    #[doc = "0x168 - ConstLpCntock Mux Select"]
    pub cms: CMS,
    #[doc = "0x16c - ProgRam Control"]
    pub str: STR,
    _reserved2: [u8; 8usize],
    #[doc = "0x178 - Chip Select"]
    pub csr: CSR,
    #[doc = "0x17c - Fail Delay"]
    pub fdly: FDLY,
    #[doc = "0x180 - PBIST Activate ROM Clock Enable"]
    pub pact: PACT,
    #[doc = "0x184 - PBIST ID"]
    pub id_r: IDR,
    #[doc = "0x188 - PBIST Override"]
    pub over: OVER,
    _reserved3: [u8; 4usize],
    #[doc = "0x190 - Fail Status Fail - Port 0"]
    pub fsfr0: FSFR0,
    #[doc = "0x194 - Fail Status Fail - Port 1"]
    pub fsfr1: FSFR1,
    #[doc = "0x198 - Fail Status Count - Port 0"]
    pub fsrc0: FSRC0,
    #[doc = "0x19c - Fail Status Count - Port 1"]
    pub fsrc1: FSRC1,
    #[doc = "0x1a0 - Fail Status Address - Port 0"]
    pub fsra0: FSRA0,
    #[doc = "0x1a4 - Fail Status Address - Port 1"]
    pub fsra1: FSRA1,
    #[doc = "0x1a8 - Fail Status Data - Port 0"]
    pub fsrdl0: FSRDL0,
    _reserved4: [u8; 4usize],
    #[doc = "0x1b0 - Fail Status Data - Port 1"]
    pub fsrdl1: FSRDL1,
    _reserved5: [u8; 12usize],
    #[doc = "0x1c0 - ROM Mask"]
    pub rom: ROM,
    #[doc = "0x1c4 - ROM Algorithm Mask"]
    pub algo: ALGO,
    #[doc = "0x1c8 - Ram Info Mask Lower"]
    pub rinfo_l: RINFOL,
    #[doc = "0x1cc - Ram Info Mask Upper"]
    pub rinfo_u: RINFOU,
}
#[doc = "Variable Address Register 0"]
pub struct VARADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Variable Address Register 0"]
pub mod var_addr0;
#[doc = "Variable Address Register 1"]
pub struct VARADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Variable Address Register 1"]
pub mod var_addr1;
#[doc = "Variable Address Register 2"]
pub struct VARADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Variable Address Register 2"]
pub mod var_addr2;
#[doc = "Variable Address Register 3"]
pub struct VARADDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Variable Address Register 3"]
pub mod var_addr3;
#[doc = "Variable Loop Count Register 0"]
pub struct VARLPCNT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Variable Loop Count Register 0"]
pub mod var_lp_cnt0;
#[doc = "Variable Loop Count Register 1"]
pub struct VARLPCNT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Variable Loop Count Register 1"]
pub mod var_lp_cnt1;
#[doc = "Variable Loop Count Register 0"]
pub struct VARLPCNT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Variable Loop Count Register 0"]
pub mod var_lp_cnt2;
#[doc = "Variable Loop Count Register 1"]
pub struct VARLPCNT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Variable Loop Count Register 1"]
pub mod var_lp_cnt3;
#[doc = "DD0 Data Register"]
pub struct DD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DD0 Data Register"]
pub mod dd0;
#[doc = "DE0 Data Register"]
pub struct DE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DE0 Data Register"]
pub mod de0;
#[doc = "Constant Address Register 0"]
pub struct CONSTADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Address Register 0"]
pub mod const_addr0;
#[doc = "Constant Address Register 1"]
pub struct CONSTADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Address Register 1"]
pub mod const_addr1;
#[doc = "Constant Address Register 2"]
pub struct CONSTADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Address Register 2"]
pub mod const_addr2;
#[doc = "Constant Address Register 3"]
pub struct CONSTADDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Address Register 3"]
pub mod const_addr3;
#[doc = "Constant Loop Count Register 0"]
pub struct CONSTLPCNT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Loop Count Register 0"]
pub mod const_lp_cnt0;
#[doc = "Constant Loop Count Register 1"]
pub struct CONSTLPCNT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Loop Count Register 1"]
pub mod const_lp_cnt1;
#[doc = "Constant Loop Count Register 2"]
pub struct CONSTLPCNT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Loop Count Register 2"]
pub mod const_lp_cnt2;
#[doc = "Constant Loop Count Register 3"]
pub struct CONSTLPCNT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Loop Count Register 3"]
pub mod const_lp_cnt3;
#[doc = "Constant Increment Register 0"]
pub struct CONSTINC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Increment Register 0"]
pub mod const_inc0;
#[doc = "Constant Increment Register 1"]
pub struct CONSTINC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Increment Register 1"]
pub mod const_inc1;
#[doc = "Constant Increment Register 2"]
pub struct CONSTINC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Increment Register 2"]
pub mod const_inc2;
#[doc = "Constant Increment Register 3"]
pub struct CONSTINC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Constant Increment Register 3"]
pub mod const_inc3;
#[doc = "Ram Configuration"]
pub struct RAMT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram Configuration"]
pub mod ram_t;
#[doc = "Datalogger"]
pub struct DLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Datalogger"]
pub mod dlr;
#[doc = "ConstLpCntock Mux Select"]
pub struct CMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ConstLpCntock Mux Select"]
pub mod cms;
#[doc = "ProgRam Control"]
pub struct STR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ProgRam Control"]
pub mod str;
#[doc = "Chip Select"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Select"]
pub mod csr;
#[doc = "Fail Delay"]
pub struct FDLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fail Delay"]
pub mod fdly;
#[doc = "PBIST Activate ROM Clock Enable"]
pub struct PACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBIST Activate ROM Clock Enable"]
pub mod pact;
#[doc = "PBIST ID"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBIST ID"]
pub mod id_r;
#[doc = "PBIST Override"]
pub struct OVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PBIST Override"]
pub mod over;
#[doc = "Fail Status Fail - Port 0"]
pub struct FSFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fail Status Fail - Port 0"]
pub mod fsfr0;
#[doc = "Fail Status Fail - Port 1"]
pub struct FSFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fail Status Fail - Port 1"]
pub mod fsfr1;
#[doc = "Fail Status Count - Port 0"]
pub struct FSRC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fail Status Count - Port 0"]
pub mod fsrc0;
#[doc = "Fail Status Count - Port 1"]
pub struct FSRC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fail Status Count - Port 1"]
pub mod fsrc1;
#[doc = "Fail Status Address - Port 0"]
pub struct FSRA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fail Status Address - Port 0"]
pub mod fsra0;
#[doc = "Fail Status Address - Port 1"]
pub struct FSRA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fail Status Address - Port 1"]
pub mod fsra1;
#[doc = "Fail Status Data - Port 0"]
pub struct FSRDL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fail Status Data - Port 0"]
pub mod fsrdl0;
#[doc = "Fail Status Data - Port 1"]
pub struct FSRDL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fail Status Data - Port 1"]
pub mod fsrdl1;
#[doc = "ROM Mask"]
pub struct ROM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROM Mask"]
pub mod rom;
#[doc = "ROM Algorithm Mask"]
pub struct ALGO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROM Algorithm Mask"]
pub mod algo;
#[doc = "Ram Info Mask Lower"]
pub struct RINFOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram Info Mask Lower"]
pub mod rinfo_l;
#[doc = "Ram Info Mask Upper"]
pub struct RINFOU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram Info Mask Upper"]
pub mod rinfo_u;
