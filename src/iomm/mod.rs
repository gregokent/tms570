#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Revision Register"]
    pub rev: REV,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Die Id Register 0"]
    pub die_id0: DIEID0,
    #[doc = "0x0c - Die Id Register 1"]
    pub die_id1: DIEID1,
    #[doc = "0x10 - Die Id Register 2"]
    pub die_id2: DIEID2,
    #[doc = "0x14 - Die Id Register 3"]
    pub die_id3: DIEID3,
    #[doc = "0x18 - Device Id Register 0"]
    pub dev_id0: DEVID0,
    #[doc = "0x1c - Device Id Register 1"]
    pub dev_id1: DEVID1,
    #[doc = "0x20 - Boot Config Register 0"]
    pub boot_config0: BOOTCONFIG0,
    _reserved1: [u8; 20usize],
    #[doc = "0x38 - Kicker Register 0"]
    pub kick0: KICK0,
    #[doc = "0x3c - Kicker Register 1"]
    pub kick1: KICK1,
    _reserved2: [u8; 160usize],
    #[doc = "0xe0 - Error Raw Status/Set Register"]
    pub err_raw_stat_set: ERRRAWSTATSET,
    #[doc = "0xe4 - Error Enabled Status/Clear Register"]
    pub err_ena_stat_clr: ERRENASTATCLR,
    #[doc = "0xe8 - Error Enable Register"]
    pub err_ena: ERRENA,
    #[doc = "0xec - Error Enable Clear Register"]
    pub err_ena_clr: ERRENACLR,
    _reserved3: [u8; 4usize],
    #[doc = "0xf4 - Fault Address Register"]
    pub fault_addr: FAULT_ADDR,
    #[doc = "0xf8 - Fault Status Register"]
    pub fault_stat: FAULT_STAT,
    #[doc = "0xfc - Fault Clear Register"]
    pub fault_clr: FAULT_CLR,
    _reserved4: [u8; 16usize],
    #[doc = "0x110 - Pin Multiplexing Control Register 0"]
    pub pinmmr0: PINMMR0,
    #[doc = "0x114 - Pin Multiplexing Control Register 1"]
    pub pinmmr1: PINMMR1,
    #[doc = "0x118 - Pin Multiplexing Control Register 2"]
    pub pinmmr2: PINMMR2,
    #[doc = "0x11c - Pin Multiplexing Control Register 3"]
    pub pinmmr3: PINMMR3,
    #[doc = "0x120 - Pin Multiplexing Control Register 4"]
    pub pinmmr4: PINMMR4,
    #[doc = "0x124 - Pin Multiplexing Control Register 5"]
    pub pinmmr5: PINMMR5,
    #[doc = "0x128 - Pin Multiplexing Control Register 6"]
    pub pinmmr6: PINMMR6,
    #[doc = "0x12c - Pin Multiplexing Control Register 7"]
    pub pinmmr7: PINMMR7,
    #[doc = "0x130 - Pin Multiplexing Control Register 8"]
    pub pinmmr8: PINMMR8,
    #[doc = "0x134 - Pin Multiplexing Control Register 9"]
    pub pinmmr9: PINMMR9,
    #[doc = "0x138 - Pin Multiplexing Control Register 10"]
    pub pinmmr10: PINMMR10,
    #[doc = "0x13c - Pin Multiplexing Control Register 11"]
    pub pinmmr11: PINMMR11,
    #[doc = "0x140 - Pin Multiplexing Control Register 12"]
    pub pinmmr12: PINMMR12,
    #[doc = "0x144 - Pin Multiplexing Control Register 13"]
    pub pinmmr13: PINMMR13,
    #[doc = "0x148 - Pin Multiplexing Control Register 14"]
    pub pinmmr14: PINMMR14,
    #[doc = "0x14c - Pin Multiplexing Control Register 15"]
    pub pinmmr15: PINMMR15,
    #[doc = "0x150 - Pin Multiplexing Control Register 16"]
    pub pinmmr16: PINMMR16,
    #[doc = "0x154 - Pin Multiplexing Control Register 17"]
    pub pinmmr17: PINMMR17,
    #[doc = "0x158 - Pin Multiplexing Control Register 18"]
    pub pinmmr18: PINMMR18,
    #[doc = "0x15c - Pin Multiplexing Control Register 19"]
    pub pinmmr19: PINMMR19,
    #[doc = "0x160 - Pin Multiplexing Control Register 20"]
    pub pinmmr20: PINMMR20,
    #[doc = "0x164 - Pin Multiplexing Control Register 21"]
    pub pinmmr21: PINMMR21,
    #[doc = "0x168 - Pin Multiplexing Control Register 22"]
    pub pinmmr22: PINMMR22,
    #[doc = "0x16c - Pin Multiplexing Control Register 23"]
    pub pinmmr23: PINMMR23,
    #[doc = "0x170 - Pin Multiplexing Control Register 24"]
    pub pinmmr24: PINMMR24,
    #[doc = "0x174 - Pin Multiplexing Control Register 25"]
    pub pinmmr25: PINMMR25,
    #[doc = "0x178 - Pin Multiplexing Control Register 26"]
    pub pinmmr26: PINMMR26,
    #[doc = "0x17c - Pin Multiplexing Control Register 27"]
    pub pinmmr27: PINMMR27,
    #[doc = "0x180 - Pin Multiplexing Control Register 28"]
    pub pinmmr28: PINMMR28,
    #[doc = "0x184 - Pin Multiplexing Control Register 29"]
    pub pinmmr29: PINMMR29,
    #[doc = "0x188 - Pin Multiplexing Control Register 30"]
    pub pinmmr30: PINMMR30,
    #[doc = "0x18c - Pin Multiplexing Control Register 31"]
    pub pinmmr31: PINMMR31,
    #[doc = "0x190 - Pin Multiplexing Control Register 32"]
    pub pinmmr32: PINMMR32,
    #[doc = "0x194 - Pin Multiplexing Control Register 33"]
    pub pinmmr33: PINMMR33,
    #[doc = "0x198 - Pin Multiplexing Control Register 34"]
    pub pinmmr34: PINMMR34,
    #[doc = "0x19c - Pin Multiplexing Control Register 35"]
    pub pinmmr35: PINMMR35,
    #[doc = "0x1a0 - Pin Multiplexing Control Register 36"]
    pub pinmmr36: PINMMR36,
    #[doc = "0x1a4 - Pin Multiplexing Control Register 37"]
    pub pinmmr37: PINMMR37,
    _reserved5: [u8; 168usize],
    #[doc = "0x250 - Pin Multiplexing Control Register 80"]
    pub pinmmr80: PINMMR80,
    #[doc = "0x254 - Pin Multiplexing Control Register 81"]
    pub pinmmr81: PINMMR81,
    #[doc = "0x258 - Pin Multiplexing Control Register 82"]
    pub pinmmr82: PINMMR82,
    #[doc = "0x25c - Pin Multiplexing Control Register 83"]
    pub pinmmr83: PINMMR83,
    #[doc = "0x260 - Pin Multiplexing Control Register 84"]
    pub pinmmr84: PINMMR84,
    #[doc = "0x264 - Pin Multiplexing Control Register 85"]
    pub pinmmr85: PINMMR85,
    #[doc = "0x268 - Pin Multiplexing Control Register 86"]
    pub pinmmr86: PINMMR86,
    #[doc = "0x26c - Pin Multiplexing Control Register 87"]
    pub pinmmr87: PINMMR87,
    #[doc = "0x270 - Pin Multiplexing Control Register 88"]
    pub pinmmr88: PINMMR88,
    #[doc = "0x274 - Pin Multiplexing Control Register 89"]
    pub pinmmr89: PINMMR89,
    #[doc = "0x278 - Pin Multiplexing Control Register 90"]
    pub pinmmr90: PINMMR90,
    #[doc = "0x27c - Pin Multiplexing Control Register 91"]
    pub pinmmr91: PINMMR91,
    #[doc = "0x280 - Pin Multiplexing Control Register 92"]
    pub pinmmr92: PINMMR92,
    #[doc = "0x284 - Pin Multiplexing Control Register 93"]
    pub pinmmr93: PINMMR93,
    #[doc = "0x288 - Pin Multiplexing Control Register 94"]
    pub pinmmr94: PINMMR94,
    #[doc = "0x28c - Pin Multiplexing Control Register 95"]
    pub pinmmr95: PINMMR95,
    #[doc = "0x290 - Pin Multiplexing Control Register 96"]
    pub pinmmr96: PINMMR96,
    #[doc = "0x294 - Pin Multiplexing Control Register 97"]
    pub pinmmr97: PINMMR97,
    #[doc = "0x298 - Pin Multiplexing Control Register 98"]
    pub pinmmr98: PINMMR98,
    #[doc = "0x29c - Pin Multiplexing Control Register 99"]
    pub pinmmr99: PINMMR99,
    _reserved6: [u8; 240usize],
    #[doc = "0x390 - Pin Multiplexing Control Register 160"]
    pub pinmmr160: PINMMR160,
    #[doc = "0x394 - Pin Multiplexing Control Register 161"]
    pub pinmmr161: PINMMR161,
    #[doc = "0x398 - Pin Multiplexing Control Register 162"]
    pub pinmmr162: PINMMR162,
    #[doc = "0x39c - Pin Multiplexing Control Register 163"]
    pub pinmmr163: PINMMR163,
    #[doc = "0x3a0 - Pin Multiplexing Control Register 164"]
    pub pinmmr164: PINMMR164,
    #[doc = "0x3a4 - Pin Multiplexing Control Register 165"]
    pub pinmmr165: PINMMR165,
    #[doc = "0x3a8 - Pin Multiplexing Control Register 166"]
    pub pinmmr166: PINMMR166,
    #[doc = "0x3ac - Pin Multiplexing Control Register 167"]
    pub pinmmr167: PINMMR167,
    #[doc = "0x3b0 - Pin Multiplexing Control Register 168"]
    pub pinmmr168: PINMMR168,
    #[doc = "0x3b4 - Pin Multiplexing Control Register 169"]
    pub pinmmr169: PINMMR169,
    #[doc = "0x3b8 - Pin Multiplexing Control Register 170"]
    pub pinmmr170: PINMMR170,
    #[doc = "0x3bc - Pin Multiplexing Control Register 171"]
    pub pinmmr171: PINMMR171,
    _reserved7: [u8; 16usize],
    #[doc = "0x3d0 - Pin Multiplexing Control Register 172"]
    pub pinmmr172: PINMMR172,
    #[doc = "0x3d4 - Pin Multiplexing Control Register 173"]
    pub pinmmr173: PINMMR173,
    #[doc = "0x3d8 - Pin Multiplexing Control Register 174"]
    pub pinmmr174: PINMMR174,
    #[doc = "0x3dc - Pin Multiplexing Control Register 175"]
    pub pinmmr175: PINMMR175,
    #[doc = "0x3e0 - Pin Multiplexing Control Register 176"]
    pub pinmmr176: PINMMR176,
    #[doc = "0x3e4 - Pin Multiplexing Control Register 177"]
    pub pinmmr177: PINMMR177,
    #[doc = "0x3e8 - Pin Multiplexing Control Register 178"]
    pub pinmmr178: PINMMR178,
    #[doc = "0x3ec - Pin Multiplexing Control Register 179"]
    pub pinmmr179: PINMMR179,
}
#[doc = "Module Revision Register"]
pub struct REV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Revision Register"]
pub mod rev;
#[doc = "Die Id Register 0"]
pub struct DIEID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Id Register 0"]
pub mod die_id0;
#[doc = "Die Id Register 1"]
pub struct DIEID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Id Register 1"]
pub mod die_id1;
#[doc = "Die Id Register 2"]
pub struct DIEID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Id Register 2"]
pub mod die_id2;
#[doc = "Die Id Register 3"]
pub struct DIEID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Id Register 3"]
pub mod die_id3;
#[doc = "Device Id Register 0"]
pub struct DEVID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Id Register 0"]
pub mod dev_id0;
#[doc = "Device Id Register 1"]
pub struct DEVID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Id Register 1"]
pub mod dev_id1;
#[doc = "Boot Config Register 0"]
pub struct BOOTCONFIG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Boot Config Register 0"]
pub mod boot_config0;
#[doc = "Kicker Register 0"]
pub struct KICK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Kicker Register 0"]
pub mod kick0;
#[doc = "Kicker Register 1"]
pub struct KICK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Kicker Register 1"]
pub mod kick1;
#[doc = "Error Raw Status/Set Register"]
pub struct ERRRAWSTATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Raw Status/Set Register"]
pub mod err_raw_stat_set;
#[doc = "Error Enabled Status/Clear Register"]
pub struct ERRENASTATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Enabled Status/Clear Register"]
pub mod err_ena_stat_clr;
#[doc = "Error Enable Register"]
pub struct ERRENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Enable Register"]
pub mod err_ena;
#[doc = "Error Enable Clear Register"]
pub struct ERRENACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Enable Clear Register"]
pub mod err_ena_clr;
#[doc = "Fault Address Register"]
pub struct FAULT_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault Address Register"]
pub mod fault_addr;
#[doc = "Fault Status Register"]
pub struct FAULT_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault Status Register"]
pub mod fault_stat;
#[doc = "Fault Clear Register"]
pub struct FAULT_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault Clear Register"]
pub mod fault_clr;
#[doc = "Pin Multiplexing Control Register 0"]
pub struct PINMMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 0"]
pub mod pinmmr0;
#[doc = "Pin Multiplexing Control Register 1"]
pub struct PINMMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 1"]
pub mod pinmmr1;
#[doc = "Pin Multiplexing Control Register 2"]
pub struct PINMMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 2"]
pub mod pinmmr2;
#[doc = "Pin Multiplexing Control Register 3"]
pub struct PINMMR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 3"]
pub mod pinmmr3;
#[doc = "Pin Multiplexing Control Register 4"]
pub struct PINMMR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 4"]
pub mod pinmmr4;
#[doc = "Pin Multiplexing Control Register 5"]
pub struct PINMMR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 5"]
pub mod pinmmr5;
#[doc = "Pin Multiplexing Control Register 6"]
pub struct PINMMR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 6"]
pub mod pinmmr6;
#[doc = "Pin Multiplexing Control Register 7"]
pub struct PINMMR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 7"]
pub mod pinmmr7;
#[doc = "Pin Multiplexing Control Register 8"]
pub struct PINMMR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 8"]
pub mod pinmmr8;
#[doc = "Pin Multiplexing Control Register 9"]
pub struct PINMMR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 9"]
pub mod pinmmr9;
#[doc = "Pin Multiplexing Control Register 10"]
pub struct PINMMR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 10"]
pub mod pinmmr10;
#[doc = "Pin Multiplexing Control Register 11"]
pub struct PINMMR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 11"]
pub mod pinmmr11;
#[doc = "Pin Multiplexing Control Register 12"]
pub struct PINMMR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 12"]
pub mod pinmmr12;
#[doc = "Pin Multiplexing Control Register 13"]
pub struct PINMMR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 13"]
pub mod pinmmr13;
#[doc = "Pin Multiplexing Control Register 14"]
pub struct PINMMR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 14"]
pub mod pinmmr14;
#[doc = "Pin Multiplexing Control Register 15"]
pub struct PINMMR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 15"]
pub mod pinmmr15;
#[doc = "Pin Multiplexing Control Register 16"]
pub struct PINMMR16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 16"]
pub mod pinmmr16;
#[doc = "Pin Multiplexing Control Register 17"]
pub struct PINMMR17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 17"]
pub mod pinmmr17;
#[doc = "Pin Multiplexing Control Register 18"]
pub struct PINMMR18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 18"]
pub mod pinmmr18;
#[doc = "Pin Multiplexing Control Register 19"]
pub struct PINMMR19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 19"]
pub mod pinmmr19;
#[doc = "Pin Multiplexing Control Register 20"]
pub struct PINMMR20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 20"]
pub mod pinmmr20;
#[doc = "Pin Multiplexing Control Register 21"]
pub struct PINMMR21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 21"]
pub mod pinmmr21;
#[doc = "Pin Multiplexing Control Register 22"]
pub struct PINMMR22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 22"]
pub mod pinmmr22;
#[doc = "Pin Multiplexing Control Register 23"]
pub struct PINMMR23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 23"]
pub mod pinmmr23;
#[doc = "Pin Multiplexing Control Register 24"]
pub struct PINMMR24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 24"]
pub mod pinmmr24;
#[doc = "Pin Multiplexing Control Register 25"]
pub struct PINMMR25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 25"]
pub mod pinmmr25;
#[doc = "Pin Multiplexing Control Register 26"]
pub struct PINMMR26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 26"]
pub mod pinmmr26;
#[doc = "Pin Multiplexing Control Register 27"]
pub struct PINMMR27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 27"]
pub mod pinmmr27;
#[doc = "Pin Multiplexing Control Register 28"]
pub struct PINMMR28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 28"]
pub mod pinmmr28;
#[doc = "Pin Multiplexing Control Register 29"]
pub struct PINMMR29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 29"]
pub mod pinmmr29;
#[doc = "Pin Multiplexing Control Register 30"]
pub struct PINMMR30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 30"]
pub mod pinmmr30;
#[doc = "Pin Multiplexing Control Register 31"]
pub struct PINMMR31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 31"]
pub mod pinmmr31;
#[doc = "Pin Multiplexing Control Register 32"]
pub struct PINMMR32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 32"]
pub mod pinmmr32;
#[doc = "Pin Multiplexing Control Register 33"]
pub struct PINMMR33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 33"]
pub mod pinmmr33;
#[doc = "Pin Multiplexing Control Register 34"]
pub struct PINMMR34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 34"]
pub mod pinmmr34;
#[doc = "Pin Multiplexing Control Register 35"]
pub struct PINMMR35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 35"]
pub mod pinmmr35;
#[doc = "Pin Multiplexing Control Register 36"]
pub struct PINMMR36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 36"]
pub mod pinmmr36;
#[doc = "Pin Multiplexing Control Register 37"]
pub struct PINMMR37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 37"]
pub mod pinmmr37;
#[doc = "Pin Multiplexing Control Register 80"]
pub struct PINMMR80 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 80"]
pub mod pinmmr80;
#[doc = "Pin Multiplexing Control Register 81"]
pub struct PINMMR81 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 81"]
pub mod pinmmr81;
#[doc = "Pin Multiplexing Control Register 82"]
pub struct PINMMR82 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 82"]
pub mod pinmmr82;
#[doc = "Pin Multiplexing Control Register 83"]
pub struct PINMMR83 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 83"]
pub mod pinmmr83;
#[doc = "Pin Multiplexing Control Register 84"]
pub struct PINMMR84 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 84"]
pub mod pinmmr84;
#[doc = "Pin Multiplexing Control Register 85"]
pub struct PINMMR85 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 85"]
pub mod pinmmr85;
#[doc = "Pin Multiplexing Control Register 86"]
pub struct PINMMR86 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 86"]
pub mod pinmmr86;
#[doc = "Pin Multiplexing Control Register 87"]
pub struct PINMMR87 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 87"]
pub mod pinmmr87;
#[doc = "Pin Multiplexing Control Register 88"]
pub struct PINMMR88 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 88"]
pub mod pinmmr88;
#[doc = "Pin Multiplexing Control Register 89"]
pub struct PINMMR89 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 89"]
pub mod pinmmr89;
#[doc = "Pin Multiplexing Control Register 90"]
pub struct PINMMR90 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 90"]
pub mod pinmmr90;
#[doc = "Pin Multiplexing Control Register 91"]
pub struct PINMMR91 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 91"]
pub mod pinmmr91;
#[doc = "Pin Multiplexing Control Register 92"]
pub struct PINMMR92 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 92"]
pub mod pinmmr92;
#[doc = "Pin Multiplexing Control Register 93"]
pub struct PINMMR93 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 93"]
pub mod pinmmr93;
#[doc = "Pin Multiplexing Control Register 94"]
pub struct PINMMR94 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 94"]
pub mod pinmmr94;
#[doc = "Pin Multiplexing Control Register 95"]
pub struct PINMMR95 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 95"]
pub mod pinmmr95;
#[doc = "Pin Multiplexing Control Register 96"]
pub struct PINMMR96 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 96"]
pub mod pinmmr96;
#[doc = "Pin Multiplexing Control Register 97"]
pub struct PINMMR97 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 97"]
pub mod pinmmr97;
#[doc = "Pin Multiplexing Control Register 98"]
pub struct PINMMR98 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 98"]
pub mod pinmmr98;
#[doc = "Pin Multiplexing Control Register 99"]
pub struct PINMMR99 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 99"]
pub mod pinmmr99;
#[doc = "Pin Multiplexing Control Register 160"]
pub struct PINMMR160 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 160"]
pub mod pinmmr160;
#[doc = "Pin Multiplexing Control Register 161"]
pub struct PINMMR161 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 161"]
pub mod pinmmr161;
#[doc = "Pin Multiplexing Control Register 162"]
pub struct PINMMR162 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 162"]
pub mod pinmmr162;
#[doc = "Pin Multiplexing Control Register 163"]
pub struct PINMMR163 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 163"]
pub mod pinmmr163;
#[doc = "Pin Multiplexing Control Register 164"]
pub struct PINMMR164 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 164"]
pub mod pinmmr164;
#[doc = "Pin Multiplexing Control Register 165"]
pub struct PINMMR165 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 165"]
pub mod pinmmr165;
#[doc = "Pin Multiplexing Control Register 166"]
pub struct PINMMR166 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 166"]
pub mod pinmmr166;
#[doc = "Pin Multiplexing Control Register 167"]
pub struct PINMMR167 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 167"]
pub mod pinmmr167;
#[doc = "Pin Multiplexing Control Register 168"]
pub struct PINMMR168 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 168"]
pub mod pinmmr168;
#[doc = "Pin Multiplexing Control Register 169"]
pub struct PINMMR169 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 169"]
pub mod pinmmr169;
#[doc = "Pin Multiplexing Control Register 170"]
pub struct PINMMR170 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 170"]
pub mod pinmmr170;
#[doc = "Pin Multiplexing Control Register 171"]
pub struct PINMMR171 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 171"]
pub mod pinmmr171;
#[doc = "Pin Multiplexing Control Register 172"]
pub struct PINMMR172 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 172"]
pub mod pinmmr172;
#[doc = "Pin Multiplexing Control Register 173"]
pub struct PINMMR173 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 173"]
pub mod pinmmr173;
#[doc = "Pin Multiplexing Control Register 174"]
pub struct PINMMR174 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 174"]
pub mod pinmmr174;
#[doc = "Pin Multiplexing Control Register 175"]
pub struct PINMMR175 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 175"]
pub mod pinmmr175;
#[doc = "Pin Multiplexing Control Register 176"]
pub struct PINMMR176 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 176"]
pub mod pinmmr176;
#[doc = "Pin Multiplexing Control Register 177"]
pub struct PINMMR177 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 177"]
pub mod pinmmr177;
#[doc = "Pin Multiplexing Control Register 178"]
pub struct PINMMR178 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 178"]
pub mod pinmmr178;
#[doc = "Pin Multiplexing Control Register 179"]
pub struct PINMMR179 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Multiplexing Control Register 179"]
pub mod pinmmr179;
