#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ECC Control Register"]
    pub eccctrl: ECCCTRL,
    #[doc = "0x04 - ECC Diagnostic Status Register"]
    pub eccdstat: ECCDSTAT,
    #[doc = "0x08 - ECC Test Register"]
    pub ecctest: ECCTEST,
    #[doc = "0x0c - Single Bit Error Register"]
    pub sbestat: SBESTAT,
    #[doc = "0x10 - Test register 1"]
    pub test1: TEST1,
    #[doc = "0x14 - Test register 2"]
    pub test2: TEST2,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - Lock register"]
    pub lck: LCK,
    #[doc = "0x20 - Error interrupt register"]
    pub eir: EIR,
    #[doc = "0x24 - Status interrupt register"]
    pub sir: SIR,
    #[doc = "0x28 - Error interrupt line select"]
    pub eils: EILS,
    #[doc = "0x2c - Status interrupt line select"]
    pub sils: SILS,
    #[doc = "0x30 - Error interrupt enable set"]
    pub eies: EIES,
    #[doc = "0x34 - Error interrupt enable reset"]
    pub eier: EIER,
    #[doc = "0x38 - Status interrupt enable set"]
    pub sies: SIES,
    #[doc = "0x3c - Status interrupt enable reset"]
    pub sier: SIER,
    #[doc = "0x40 - Interrupt line enable"]
    pub ile: ILE,
    #[doc = "0x44 - Timer 0 configuration"]
    pub t0c: T0C,
    #[doc = "0x48 - Timer 1 configuration"]
    pub t1c: T1C,
    #[doc = "0x4c - Stop watch register1"]
    pub stpw1: STPW1,
    #[doc = "0x50 - Stop watch register2"]
    pub stpw2: STPW2,
    _reserved1: [u8; 44usize],
    #[doc = "0x80 - SUC configuration register 1"]
    pub succ1: SUCC1,
    #[doc = "0x84 - SUC configuration register 2"]
    pub succ2: SUCC2,
    #[doc = "0x88 - SUC configuration register 3"]
    pub succ3: SUCC3,
    #[doc = "0x8c - NEM configuration register"]
    pub nemc: NEMC,
    #[doc = "0x90 - PRT configuration register 1"]
    pub prtc1: PRTC1,
    #[doc = "0x94 - PRT configuration register 2"]
    pub prtc2: PRTC2,
    #[doc = "0x98 - MHD configuration register"]
    pub mhdc: MHDC,
    _reserved2: [u8; 4usize],
    #[doc = "0xa0 - GTU configuration register 1"]
    pub gtuc1: GTUC1,
    #[doc = "0xa4 - GTU configuration register 2"]
    pub gtuc2: GTUC2,
    #[doc = "0xa8 - GTU configuration register 3"]
    pub gtuc3: GTUC3,
    #[doc = "0xac - GTU configuration register 4"]
    pub gtuc4: GTUC4,
    #[doc = "0xb0 - GTU configuration register 5"]
    pub gtuc5: GTUC5,
    #[doc = "0xb4 - GTU configuration register 6"]
    pub gtuc6: GTUC6,
    #[doc = "0xb8 - GTU configuration register 7"]
    pub gtuc7: GTUC7,
    #[doc = "0xbc - GTU configuration register 8"]
    pub gtuc8: GTUC8,
    #[doc = "0xc0 - GTU configuration register 9"]
    pub gtuc9: GTUC9,
    #[doc = "0xc4 - GTU configuration register 10"]
    pub gtuc10: GTUC10,
    #[doc = "0xc8 - GTU configuration register 11"]
    pub gtuc11: GTUC11,
    _reserved3: [u8; 52usize],
    #[doc = "0x100 - communication controller status vector"]
    pub ccsv: CCSV,
    #[doc = "0x104 - communication controller error vector"]
    pub ccev: CCEV,
    _reserved4: [u8; 8usize],
    #[doc = "0x110 - Slot counter value"]
    pub scv: SCV,
    #[doc = "0x114 - Macrotick and cycle counter"]
    pub mtccv: MTCCV,
    #[doc = "0x118 - Rate correction value"]
    pub rcv: RCV,
    #[doc = "0x11c - Offset correction value"]
    pub ocv: OCV,
    #[doc = "0x120 - Sync frame status"]
    pub sfs: SFS,
    #[doc = "0x124 - Symbol window and NIT status"]
    pub swnit: SWNIT,
    #[doc = "0x128 - Aggregated channel status"]
    pub acs: ACS,
    _reserved5: [u8; 4usize],
    #[doc = "0x130 - Even sync ID [1-15]"]
    pub esid1: ESID1,
    #[doc = "0x134 - Even sync ID [1-15]"]
    pub esid2: ESID2,
    #[doc = "0x138 - Even sync ID [1-15]"]
    pub esid3: ESID3,
    #[doc = "0x13c - Even sync ID [1-15]"]
    pub esid4: ESID4,
    #[doc = "0x140 - Even sync ID [1-15]"]
    pub esid5: ESID5,
    #[doc = "0x144 - Even sync ID [1-15]"]
    pub esid6: ESID6,
    #[doc = "0x148 - Even sync ID [1-15]"]
    pub esid7: ESID7,
    #[doc = "0x14c - Even sync ID [1-15]"]
    pub esid8: ESID8,
    #[doc = "0x150 - Even sync ID [1-15]"]
    pub esid9: ESID9,
    #[doc = "0x154 - Even sync ID [1-15]"]
    pub esid10: ESID10,
    #[doc = "0x158 - Even sync ID [1-15]"]
    pub esid11: ESID11,
    #[doc = "0x15c - Even sync ID [1-15]"]
    pub esid12: ESID12,
    #[doc = "0x160 - Even sync ID [1-15]"]
    pub esid13: ESID13,
    #[doc = "0x164 - Even sync ID [1-15]"]
    pub esid14: ESID14,
    #[doc = "0x168 - Even sync ID [1-15]"]
    pub esid15: ESID15,
    _reserved6: [u8; 4usize],
    #[doc = "0x170 - Odd sync ID [1-15]"]
    pub osid1: OSID1,
    #[doc = "0x174 - Odd sync ID [1-15]"]
    pub osid2: OSID2,
    #[doc = "0x178 - Odd sync ID [1-15]"]
    pub osid3: OSID3,
    #[doc = "0x17c - Odd sync ID [1-15]"]
    pub osid4: OSID4,
    #[doc = "0x180 - Odd sync ID [1-15]"]
    pub osid5: OSID5,
    #[doc = "0x184 - Odd sync ID [1-15]"]
    pub osid6: OSID6,
    #[doc = "0x188 - Odd sync ID [1-15]"]
    pub osid7: OSID7,
    #[doc = "0x18c - Odd sync ID [1-15]"]
    pub osid8: OSID8,
    #[doc = "0x190 - Odd sync ID [1-15]"]
    pub osid9: OSID9,
    #[doc = "0x194 - Odd sync ID [1-15]"]
    pub osid10: OSID10,
    #[doc = "0x198 - Odd sync ID [1-15]"]
    pub osid11: OSID11,
    #[doc = "0x19c - Odd sync ID [1-15]"]
    pub osid12: OSID12,
    #[doc = "0x1a0 - Odd sync ID [1-15]"]
    pub osid13: OSID13,
    #[doc = "0x1a4 - Odd sync ID [1-15]"]
    pub osid14: OSID14,
    #[doc = "0x1a8 - Odd sync ID [1-15]"]
    pub osid15: OSID15,
    _reserved7: [u8; 4usize],
    #[doc = "0x1b0 - Network management vector [1-3]"]
    pub nmv1: NMV1,
    #[doc = "0x1b4 - Network management vector [1-3]"]
    pub nmv2: NMV2,
    #[doc = "0x1b8 - Network management vector [1-3]"]
    pub nmv3: NMV3,
    _reserved8: [u8; 324usize],
    #[doc = "0x300 - Message RAM configuration"]
    pub mrc: MRC,
    #[doc = "0x304 - FIFO rejection filter"]
    pub frf: FRF,
    #[doc = "0x308 - FIFO rejection filter mask"]
    pub frfm: FRFM,
    #[doc = "0x30c - FIFO Critical Level"]
    pub fcim: FCIM,
    #[doc = "0x310 - Message handler status"]
    pub mhds: MHDS,
    #[doc = "0x314 - Last dynamic transmit slot"]
    pub ldts: LDTS,
    #[doc = "0x318 - FIFO status register"]
    pub fsr: FSR,
    #[doc = "0x31c - Message handler constraint flags"]
    pub mhdf: MHDF,
    #[doc = "0x320 - Transmission request 1"]
    pub txrq1: TXRQ1,
    #[doc = "0x324 - Transmission request 2"]
    pub txrq2: TXRQ2,
    #[doc = "0x328 - Transmission request 3"]
    pub txrq3: TXRQ3,
    #[doc = "0x32c - Transmission request 4"]
    pub txrq4: TXRQ4,
    #[doc = "0x330 - New data 1"]
    pub ndat1: NDAT1,
    #[doc = "0x334 - New data 2"]
    pub ndat2: NDAT2,
    #[doc = "0x338 - New data 3"]
    pub ndat3: NDAT3,
    #[doc = "0x33c - New data 4"]
    pub ndat4: NDAT4,
    #[doc = "0x340 - Message buffer status changed 1"]
    pub mbsc1: MBSC1,
    #[doc = "0x344 - Message buffer status changed 2"]
    pub mbsc2: MBSC2,
    #[doc = "0x348 - Message buffer status changed 3"]
    pub mbsc3: MBSC3,
    #[doc = "0x34c - Message buffer status changed 4"]
    pub mbsc4: MBSC4,
    _reserved9: [u8; 160usize],
    #[doc = "0x3f0 - Core release register"]
    pub crel: CREL,
    #[doc = "0x3f4 - Endian register"]
    pub endn: ENDN,
    _reserved10: [u8; 8usize],
    #[doc = "0x400 - Write data section [1-64]"]
    pub wrds1: WRDS1,
    #[doc = "0x404 - Write data section [1-64]"]
    pub wrds2: WRDS2,
    #[doc = "0x408 - Write data section [1-64]"]
    pub wrds3: WRDS3,
    #[doc = "0x40c - Write data section [1-64]"]
    pub wrds4: WRDS4,
    #[doc = "0x410 - Write data section [1-64]"]
    pub wrds5: WRDS5,
    #[doc = "0x414 - Write data section [1-64]"]
    pub wrds6: WRDS6,
    #[doc = "0x418 - Write data section [1-64]"]
    pub wrds7: WRDS7,
    #[doc = "0x41c - Write data section [1-64]"]
    pub wrds8: WRDS8,
    #[doc = "0x420 - Write data section [1-64]"]
    pub wrds9: WRDS9,
    #[doc = "0x424 - Write data section [1-64]"]
    pub wrds10: WRDS10,
    #[doc = "0x428 - Write data section [1-64]"]
    pub wrds11: WRDS11,
    #[doc = "0x42c - Write data section [1-64]"]
    pub wrds12: WRDS12,
    #[doc = "0x430 - Write data section [1-64]"]
    pub wrds13: WRDS13,
    #[doc = "0x434 - Write data section [1-64]"]
    pub wrds14: WRDS14,
    #[doc = "0x438 - Write data section [1-64]"]
    pub wrds15: WRDS15,
    #[doc = "0x43c - Write data section [1-64]"]
    pub wrds16: WRDS16,
    #[doc = "0x440 - Write data section [1-64]"]
    pub wrds17: WRDS17,
    #[doc = "0x444 - Write data section [1-64]"]
    pub wrds18: WRDS18,
    #[doc = "0x448 - Write data section [1-64]"]
    pub wrds19: WRDS19,
    #[doc = "0x44c - Write data section [1-64]"]
    pub wrds20: WRDS20,
    #[doc = "0x450 - Write data section [1-64]"]
    pub wrds21: WRDS21,
    #[doc = "0x454 - Write data section [1-64]"]
    pub wrds22: WRDS22,
    #[doc = "0x458 - Write data section [1-64]"]
    pub wrds23: WRDS23,
    #[doc = "0x45c - Write data section [1-64]"]
    pub wrds24: WRDS24,
    #[doc = "0x460 - Write data section [1-64]"]
    pub wrds25: WRDS25,
    #[doc = "0x464 - Write data section [1-64]"]
    pub wrds26: WRDS26,
    #[doc = "0x468 - Write data section [1-64]"]
    pub wrds27: WRDS27,
    #[doc = "0x46c - Write data section [1-64]"]
    pub wrds28: WRDS28,
    #[doc = "0x470 - Write data section [1-64]"]
    pub wrds29: WRDS29,
    #[doc = "0x474 - Write data section [1-64]"]
    pub wrds30: WRDS30,
    #[doc = "0x478 - Write data section [1-64]"]
    pub wrds31: WRDS31,
    #[doc = "0x47c - Write data section [1-64]"]
    pub wrds32: WRDS32,
    #[doc = "0x480 - Write data section [1-64]"]
    pub wrds33: WRDS33,
    #[doc = "0x484 - Write data section [1-64]"]
    pub wrds34: WRDS34,
    #[doc = "0x488 - Write data section [1-64]"]
    pub wrds35: WRDS35,
    #[doc = "0x48c - Write data section [1-64]"]
    pub wrds36: WRDS36,
    #[doc = "0x490 - Write data section [1-64]"]
    pub wrds37: WRDS37,
    #[doc = "0x494 - Write data section [1-64]"]
    pub wrds38: WRDS38,
    #[doc = "0x498 - Write data section [1-64]"]
    pub wrds39: WRDS39,
    #[doc = "0x49c - Write data section [1-64]"]
    pub wrds40: WRDS40,
    #[doc = "0x4a0 - Write data section [1-64]"]
    pub wrds41: WRDS41,
    #[doc = "0x4a4 - Write data section [1-64]"]
    pub wrds42: WRDS42,
    #[doc = "0x4a8 - Write data section [1-64]"]
    pub wrds43: WRDS43,
    #[doc = "0x4ac - Write data section [1-64]"]
    pub wrds44: WRDS44,
    #[doc = "0x4b0 - Write data section [1-64]"]
    pub wrds45: WRDS45,
    #[doc = "0x4b4 - Write data section [1-64]"]
    pub wrds46: WRDS46,
    #[doc = "0x4b8 - Write data section [1-64]"]
    pub wrds47: WRDS47,
    #[doc = "0x4bc - Write data section [1-64]"]
    pub wrds48: WRDS48,
    #[doc = "0x4c0 - Write data section [1-64]"]
    pub wrds49: WRDS49,
    #[doc = "0x4c4 - Write data section [1-64]"]
    pub wrds50: WRDS50,
    #[doc = "0x4c8 - Write data section [1-64]"]
    pub wrds51: WRDS51,
    #[doc = "0x4cc - Write data section [1-64]"]
    pub wrds52: WRDS52,
    #[doc = "0x4d0 - Write data section [1-64]"]
    pub wrds53: WRDS53,
    #[doc = "0x4d4 - Write data section [1-64]"]
    pub wrds54: WRDS54,
    #[doc = "0x4d8 - Write data section [1-64]"]
    pub wrds55: WRDS55,
    #[doc = "0x4dc - Write data section [1-64]"]
    pub wrds56: WRDS56,
    #[doc = "0x4e0 - Write data section [1-64]"]
    pub wrds57: WRDS57,
    #[doc = "0x4e4 - Write data section [1-64]"]
    pub wrds58: WRDS58,
    #[doc = "0x4e8 - Write data section [1-64]"]
    pub wrds59: WRDS59,
    #[doc = "0x4ec - Write data section [1-64]"]
    pub wrds60: WRDS60,
    #[doc = "0x4f0 - Write data section [1-64]"]
    pub wrds61: WRDS61,
    #[doc = "0x4f4 - Write data section [1-64]"]
    pub wrds62: WRDS62,
    #[doc = "0x4f8 - Write data section [1-64]"]
    pub wrds63: WRDS63,
    #[doc = "0x4fc - Write data section [1-64]"]
    pub wrds64: WRDS64,
    #[doc = "0x500 - Write header section 1"]
    pub wrhs1: WRHS1,
    #[doc = "0x504 - Write header section 2"]
    pub wrhs2: WRHS2,
    #[doc = "0x508 - Write header section 3"]
    pub wrhs3: WRHS3,
    _reserved11: [u8; 4usize],
    #[doc = "0x510 - Input buffer command mask"]
    pub ibcm: IBCM,
    #[doc = "0x514 - Input buffer command request"]
    pub ibcr: IBCR,
    _reserved12: [u8; 232usize],
    #[doc = "0x600 - Read data section [1-64]"]
    pub rdds1: RDDS1,
    #[doc = "0x604 - Read data section [1-64]"]
    pub rdds2: RDDS2,
    #[doc = "0x608 - Read data section [1-64]"]
    pub rdds3: RDDS3,
    #[doc = "0x60c - Read data section [1-64]"]
    pub rdds4: RDDS4,
    #[doc = "0x610 - Read data section [1-64]"]
    pub rdds5: RDDS5,
    #[doc = "0x614 - Read data section [1-64]"]
    pub rdds6: RDDS6,
    #[doc = "0x618 - Read data section [1-64]"]
    pub rdds7: RDDS7,
    #[doc = "0x61c - Read data section [1-64]"]
    pub rdds8: RDDS8,
    #[doc = "0x620 - Read data section [1-64]"]
    pub rdds9: RDDS9,
    #[doc = "0x624 - Read data section [1-64]"]
    pub rdds10: RDDS10,
    #[doc = "0x628 - Read data section [1-64]"]
    pub rdds11: RDDS11,
    #[doc = "0x62c - Read data section [1-64]"]
    pub rdds12: RDDS12,
    #[doc = "0x630 - Read data section [1-64]"]
    pub rdds13: RDDS13,
    #[doc = "0x634 - Read data section [1-64]"]
    pub rdds14: RDDS14,
    #[doc = "0x638 - Read data section [1-64]"]
    pub rdds15: RDDS15,
    #[doc = "0x63c - Read data section [1-64]"]
    pub rdds16: RDDS16,
    #[doc = "0x640 - Read data section [1-64]"]
    pub rdds17: RDDS17,
    #[doc = "0x644 - Read data section [1-64]"]
    pub rdds18: RDDS18,
    #[doc = "0x648 - Read data section [1-64]"]
    pub rdds19: RDDS19,
    #[doc = "0x64c - Read data section [1-64]"]
    pub rdds20: RDDS20,
    #[doc = "0x650 - Read data section [1-64]"]
    pub rdds21: RDDS21,
    #[doc = "0x654 - Read data section [1-64]"]
    pub rdds22: RDDS22,
    #[doc = "0x658 - Read data section [1-64]"]
    pub rdds23: RDDS23,
    #[doc = "0x65c - Read data section [1-64]"]
    pub rdds24: RDDS24,
    #[doc = "0x660 - Read data section [1-64]"]
    pub rdds25: RDDS25,
    #[doc = "0x664 - Read data section [1-64]"]
    pub rdds26: RDDS26,
    #[doc = "0x668 - Read data section [1-64]"]
    pub rdds27: RDDS27,
    #[doc = "0x66c - Read data section [1-64]"]
    pub rdds28: RDDS28,
    #[doc = "0x670 - Read data section [1-64]"]
    pub rdds29: RDDS29,
    #[doc = "0x674 - Read data section [1-64]"]
    pub rdds30: RDDS30,
    #[doc = "0x678 - Read data section [1-64]"]
    pub rdds31: RDDS31,
    #[doc = "0x67c - Read data section [1-64]"]
    pub rdds32: RDDS32,
    #[doc = "0x680 - Read data section [1-64]"]
    pub rdds33: RDDS33,
    #[doc = "0x684 - Read data section [1-64]"]
    pub rdds34: RDDS34,
    #[doc = "0x688 - Read data section [1-64]"]
    pub rdds35: RDDS35,
    #[doc = "0x68c - Read data section [1-64]"]
    pub rdds36: RDDS36,
    #[doc = "0x690 - Read data section [1-64]"]
    pub rdds37: RDDS37,
    #[doc = "0x694 - Read data section [1-64]"]
    pub rdds38: RDDS38,
    #[doc = "0x698 - Read data section [1-64]"]
    pub rdds39: RDDS39,
    #[doc = "0x69c - Read data section [1-64]"]
    pub rdds40: RDDS40,
    #[doc = "0x6a0 - Read data section [1-64]"]
    pub rdds41: RDDS41,
    #[doc = "0x6a4 - Read data section [1-64]"]
    pub rdds42: RDDS42,
    #[doc = "0x6a8 - Read data section [1-64]"]
    pub rdds43: RDDS43,
    #[doc = "0x6ac - Read data section [1-64]"]
    pub rdds44: RDDS44,
    #[doc = "0x6b0 - Read data section [1-64]"]
    pub rdds45: RDDS45,
    #[doc = "0x6b4 - Read data section [1-64]"]
    pub rdds46: RDDS46,
    #[doc = "0x6b8 - Read data section [1-64]"]
    pub rdds47: RDDS47,
    #[doc = "0x6bc - Read data section [1-64]"]
    pub rdds48: RDDS48,
    #[doc = "0x6c0 - Read data section [1-64]"]
    pub rdds49: RDDS49,
    #[doc = "0x6c4 - Read data section [1-64]"]
    pub rdds50: RDDS50,
    #[doc = "0x6c8 - Read data section [1-64]"]
    pub rdds51: RDDS51,
    #[doc = "0x6cc - Read data section [1-64]"]
    pub rdds52: RDDS52,
    #[doc = "0x6d0 - Read data section [1-64]"]
    pub rdds53: RDDS53,
    #[doc = "0x6d4 - Read data section [1-64]"]
    pub rdds54: RDDS54,
    #[doc = "0x6d8 - Read data section [1-64]"]
    pub rdds55: RDDS55,
    #[doc = "0x6dc - Read data section [1-64]"]
    pub rdds56: RDDS56,
    #[doc = "0x6e0 - Read data section [1-64]"]
    pub rdds57: RDDS57,
    #[doc = "0x6e4 - Read data section [1-64]"]
    pub rdds58: RDDS58,
    #[doc = "0x6e8 - Read data section [1-64]"]
    pub rdds59: RDDS59,
    #[doc = "0x6ec - Read data section [1-64]"]
    pub rdds60: RDDS60,
    #[doc = "0x6f0 - Read data section [1-64]"]
    pub rdds61: RDDS61,
    #[doc = "0x6f4 - Read data section [1-64]"]
    pub rdds62: RDDS62,
    #[doc = "0x6f8 - Read data section [1-64]"]
    pub rdds63: RDDS63,
    #[doc = "0x6fc - Read data section [1-64]"]
    pub rdds64: RDDS64,
    #[doc = "0x700 - Read header section 1"]
    pub rdhs1: RDHS1,
    #[doc = "0x704 - Read header section 2"]
    pub rdhs2: RDHS2,
    #[doc = "0x708 - Read header section 3"]
    pub rdhs3: RDHS3,
    #[doc = "0x70c - Message buffer status"]
    pub mbs: MBS,
    #[doc = "0x710 - Output buffer command mask"]
    pub obcm: OBCM,
    #[doc = "0x714 - Output buffer command request"]
    pub obcr: OBCR,
}
#[doc = "ECC Control Register"]
pub struct ECCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Control Register"]
pub mod eccctrl;
#[doc = "ECC Diagnostic Status Register"]
pub struct ECCDSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Diagnostic Status Register"]
pub mod eccdstat;
#[doc = "ECC Test Register"]
pub struct ECCTEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Test Register"]
pub mod ecctest;
#[doc = "Single Bit Error Register"]
pub struct SBESTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Bit Error Register"]
pub mod sbestat;
#[doc = "Test register 1"]
pub struct TEST1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test register 1"]
pub mod test1;
#[doc = "Test register 2"]
pub struct TEST2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test register 2"]
pub mod test2;
#[doc = "Lock register"]
pub struct LCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock register"]
pub mod lck;
#[doc = "Error interrupt register"]
pub struct EIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error interrupt register"]
pub mod eir;
#[doc = "Status interrupt register"]
pub struct SIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status interrupt register"]
pub mod sir;
#[doc = "Error interrupt line select"]
pub struct EILS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error interrupt line select"]
pub mod eils;
#[doc = "Status interrupt line select"]
pub struct SILS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status interrupt line select"]
pub mod sils;
#[doc = "Error interrupt enable set"]
pub struct EIES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error interrupt enable set"]
pub mod eies;
#[doc = "Error interrupt enable reset"]
pub struct EIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error interrupt enable reset"]
pub mod eier;
#[doc = "Status interrupt enable set"]
pub struct SIES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status interrupt enable set"]
pub mod sies;
#[doc = "Status interrupt enable reset"]
pub struct SIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status interrupt enable reset"]
pub mod sier;
#[doc = "Interrupt line enable"]
pub struct ILE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt line enable"]
pub mod ile;
#[doc = "Timer 0 configuration"]
pub struct T0C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 0 configuration"]
pub mod t0c;
#[doc = "Timer 1 configuration"]
pub struct T1C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 1 configuration"]
pub mod t1c;
#[doc = "Stop watch register1"]
pub struct STPW1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop watch register1"]
pub mod stpw1;
#[doc = "Stop watch register2"]
pub struct STPW2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop watch register2"]
pub mod stpw2;
#[doc = "SUC configuration register 1"]
pub struct SUCC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SUC configuration register 1"]
pub mod succ1;
#[doc = "SUC configuration register 2"]
pub struct SUCC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SUC configuration register 2"]
pub mod succ2;
#[doc = "SUC configuration register 3"]
pub struct SUCC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SUC configuration register 3"]
pub mod succ3;
#[doc = "NEM configuration register"]
pub struct NEMC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NEM configuration register"]
pub mod nemc;
#[doc = "PRT configuration register 1"]
pub struct PRTC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PRT configuration register 1"]
pub mod prtc1;
#[doc = "PRT configuration register 2"]
pub struct PRTC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PRT configuration register 2"]
pub mod prtc2;
#[doc = "MHD configuration register"]
pub struct MHDC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MHD configuration register"]
pub mod mhdc;
#[doc = "GTU configuration register 1"]
pub struct GTUC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 1"]
pub mod gtuc1;
#[doc = "GTU configuration register 2"]
pub struct GTUC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 2"]
pub mod gtuc2;
#[doc = "GTU configuration register 3"]
pub struct GTUC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 3"]
pub mod gtuc3;
#[doc = "GTU configuration register 4"]
pub struct GTUC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 4"]
pub mod gtuc4;
#[doc = "GTU configuration register 5"]
pub struct GTUC5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 5"]
pub mod gtuc5;
#[doc = "GTU configuration register 6"]
pub struct GTUC6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 6"]
pub mod gtuc6;
#[doc = "GTU configuration register 7"]
pub struct GTUC7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 7"]
pub mod gtuc7;
#[doc = "GTU configuration register 8"]
pub struct GTUC8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 8"]
pub mod gtuc8;
#[doc = "GTU configuration register 9"]
pub struct GTUC9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 9"]
pub mod gtuc9;
#[doc = "GTU configuration register 10"]
pub struct GTUC10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 10"]
pub mod gtuc10;
#[doc = "GTU configuration register 11"]
pub struct GTUC11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GTU configuration register 11"]
pub mod gtuc11;
#[doc = "communication controller status vector"]
pub struct CCSV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "communication controller status vector"]
pub mod ccsv;
#[doc = "communication controller error vector"]
pub struct CCEV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "communication controller error vector"]
pub mod ccev;
#[doc = "Slot counter value"]
pub struct SCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot counter value"]
pub mod scv;
#[doc = "Macrotick and cycle counter"]
pub struct MTCCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Macrotick and cycle counter"]
pub mod mtccv;
#[doc = "Rate correction value"]
pub struct RCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rate correction value"]
pub mod rcv;
#[doc = "Offset correction value"]
pub struct OCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Offset correction value"]
pub mod ocv;
#[doc = "Sync frame status"]
pub struct SFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sync frame status"]
pub mod sfs;
#[doc = "Symbol window and NIT status"]
pub struct SWNIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Symbol window and NIT status"]
pub mod swnit;
#[doc = "Aggregated channel status"]
pub struct ACS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Aggregated channel status"]
pub mod acs;
#[doc = "Even sync ID [1-15]"]
pub struct ESID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid1;
#[doc = "Even sync ID [1-15]"]
pub struct ESID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid2;
#[doc = "Even sync ID [1-15]"]
pub struct ESID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid3;
#[doc = "Even sync ID [1-15]"]
pub struct ESID4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid4;
#[doc = "Even sync ID [1-15]"]
pub struct ESID5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid5;
#[doc = "Even sync ID [1-15]"]
pub struct ESID6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid6;
#[doc = "Even sync ID [1-15]"]
pub struct ESID7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid7;
#[doc = "Even sync ID [1-15]"]
pub struct ESID8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid8;
#[doc = "Even sync ID [1-15]"]
pub struct ESID9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid9;
#[doc = "Even sync ID [1-15]"]
pub struct ESID10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid10;
#[doc = "Even sync ID [1-15]"]
pub struct ESID11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid11;
#[doc = "Even sync ID [1-15]"]
pub struct ESID12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid12;
#[doc = "Even sync ID [1-15]"]
pub struct ESID13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid13;
#[doc = "Even sync ID [1-15]"]
pub struct ESID14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid14;
#[doc = "Even sync ID [1-15]"]
pub struct ESID15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Even sync ID [1-15]"]
pub mod esid15;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid1;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid2;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid3;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid4;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid5;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid6;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid7;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid8;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid9;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid10;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid11;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid12;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid13;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid14;
#[doc = "Odd sync ID [1-15]"]
pub struct OSID15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Odd sync ID [1-15]"]
pub mod osid15;
#[doc = "Network management vector [1-3]"]
pub struct NMV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Network management vector [1-3]"]
pub mod nmv1;
#[doc = "Network management vector [1-3]"]
pub struct NMV2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Network management vector [1-3]"]
pub mod nmv2;
#[doc = "Network management vector [1-3]"]
pub struct NMV3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Network management vector [1-3]"]
pub mod nmv3;
#[doc = "Message RAM configuration"]
pub struct MRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message RAM configuration"]
pub mod mrc;
#[doc = "FIFO rejection filter"]
pub struct FRF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO rejection filter"]
pub mod frf;
#[doc = "FIFO rejection filter mask"]
pub struct FRFM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO rejection filter mask"]
pub mod frfm;
#[doc = "FIFO Critical Level"]
pub struct FCIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Critical Level"]
pub mod fcim;
#[doc = "Message handler status"]
pub struct MHDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message handler status"]
pub mod mhds;
#[doc = "Last dynamic transmit slot"]
pub struct LDTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last dynamic transmit slot"]
pub mod ldts;
#[doc = "FIFO status register"]
pub struct FSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO status register"]
pub mod fsr;
#[doc = "Message handler constraint flags"]
pub struct MHDF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message handler constraint flags"]
pub mod mhdf;
#[doc = "Transmission request 1"]
pub struct TXRQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission request 1"]
pub mod txrq1;
#[doc = "Transmission request 2"]
pub struct TXRQ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission request 2"]
pub mod txrq2;
#[doc = "Transmission request 3"]
pub struct TXRQ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission request 3"]
pub mod txrq3;
#[doc = "Transmission request 4"]
pub struct TXRQ4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission request 4"]
pub mod txrq4;
#[doc = "New data 1"]
pub struct NDAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New data 1"]
pub mod ndat1;
#[doc = "New data 2"]
pub struct NDAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New data 2"]
pub mod ndat2;
#[doc = "New data 3"]
pub struct NDAT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New data 3"]
pub mod ndat3;
#[doc = "New data 4"]
pub struct NDAT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New data 4"]
pub mod ndat4;
#[doc = "Message buffer status changed 1"]
pub struct MBSC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message buffer status changed 1"]
pub mod mbsc1;
#[doc = "Message buffer status changed 2"]
pub struct MBSC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message buffer status changed 2"]
pub mod mbsc2;
#[doc = "Message buffer status changed 3"]
pub struct MBSC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message buffer status changed 3"]
pub mod mbsc3;
#[doc = "Message buffer status changed 4"]
pub struct MBSC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message buffer status changed 4"]
pub mod mbsc4;
#[doc = "Core release register"]
pub struct CREL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core release register"]
pub mod crel;
#[doc = "Endian register"]
pub struct ENDN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endian register"]
pub mod endn;
#[doc = "Write data section [1-64]"]
pub struct WRDS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds1;
#[doc = "Write data section [1-64]"]
pub struct WRDS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds2;
#[doc = "Write data section [1-64]"]
pub struct WRDS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds3;
#[doc = "Write data section [1-64]"]
pub struct WRDS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds4;
#[doc = "Write data section [1-64]"]
pub struct WRDS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds5;
#[doc = "Write data section [1-64]"]
pub struct WRDS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds6;
#[doc = "Write data section [1-64]"]
pub struct WRDS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds7;
#[doc = "Write data section [1-64]"]
pub struct WRDS8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds8;
#[doc = "Write data section [1-64]"]
pub struct WRDS9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds9;
#[doc = "Write data section [1-64]"]
pub struct WRDS10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds10;
#[doc = "Write data section [1-64]"]
pub struct WRDS11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds11;
#[doc = "Write data section [1-64]"]
pub struct WRDS12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds12;
#[doc = "Write data section [1-64]"]
pub struct WRDS13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds13;
#[doc = "Write data section [1-64]"]
pub struct WRDS14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds14;
#[doc = "Write data section [1-64]"]
pub struct WRDS15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds15;
#[doc = "Write data section [1-64]"]
pub struct WRDS16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds16;
#[doc = "Write data section [1-64]"]
pub struct WRDS17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds17;
#[doc = "Write data section [1-64]"]
pub struct WRDS18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds18;
#[doc = "Write data section [1-64]"]
pub struct WRDS19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds19;
#[doc = "Write data section [1-64]"]
pub struct WRDS20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds20;
#[doc = "Write data section [1-64]"]
pub struct WRDS21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds21;
#[doc = "Write data section [1-64]"]
pub struct WRDS22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds22;
#[doc = "Write data section [1-64]"]
pub struct WRDS23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds23;
#[doc = "Write data section [1-64]"]
pub struct WRDS24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds24;
#[doc = "Write data section [1-64]"]
pub struct WRDS25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds25;
#[doc = "Write data section [1-64]"]
pub struct WRDS26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds26;
#[doc = "Write data section [1-64]"]
pub struct WRDS27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds27;
#[doc = "Write data section [1-64]"]
pub struct WRDS28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds28;
#[doc = "Write data section [1-64]"]
pub struct WRDS29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds29;
#[doc = "Write data section [1-64]"]
pub struct WRDS30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds30;
#[doc = "Write data section [1-64]"]
pub struct WRDS31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds31;
#[doc = "Write data section [1-64]"]
pub struct WRDS32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds32;
#[doc = "Write data section [1-64]"]
pub struct WRDS33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds33;
#[doc = "Write data section [1-64]"]
pub struct WRDS34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds34;
#[doc = "Write data section [1-64]"]
pub struct WRDS35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds35;
#[doc = "Write data section [1-64]"]
pub struct WRDS36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds36;
#[doc = "Write data section [1-64]"]
pub struct WRDS37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds37;
#[doc = "Write data section [1-64]"]
pub struct WRDS38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds38;
#[doc = "Write data section [1-64]"]
pub struct WRDS39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds39;
#[doc = "Write data section [1-64]"]
pub struct WRDS40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds40;
#[doc = "Write data section [1-64]"]
pub struct WRDS41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds41;
#[doc = "Write data section [1-64]"]
pub struct WRDS42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds42;
#[doc = "Write data section [1-64]"]
pub struct WRDS43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds43;
#[doc = "Write data section [1-64]"]
pub struct WRDS44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds44;
#[doc = "Write data section [1-64]"]
pub struct WRDS45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds45;
#[doc = "Write data section [1-64]"]
pub struct WRDS46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds46;
#[doc = "Write data section [1-64]"]
pub struct WRDS47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds47;
#[doc = "Write data section [1-64]"]
pub struct WRDS48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds48;
#[doc = "Write data section [1-64]"]
pub struct WRDS49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds49;
#[doc = "Write data section [1-64]"]
pub struct WRDS50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds50;
#[doc = "Write data section [1-64]"]
pub struct WRDS51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds51;
#[doc = "Write data section [1-64]"]
pub struct WRDS52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds52;
#[doc = "Write data section [1-64]"]
pub struct WRDS53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds53;
#[doc = "Write data section [1-64]"]
pub struct WRDS54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds54;
#[doc = "Write data section [1-64]"]
pub struct WRDS55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds55;
#[doc = "Write data section [1-64]"]
pub struct WRDS56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds56;
#[doc = "Write data section [1-64]"]
pub struct WRDS57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds57;
#[doc = "Write data section [1-64]"]
pub struct WRDS58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds58;
#[doc = "Write data section [1-64]"]
pub struct WRDS59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds59;
#[doc = "Write data section [1-64]"]
pub struct WRDS60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds60;
#[doc = "Write data section [1-64]"]
pub struct WRDS61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds61;
#[doc = "Write data section [1-64]"]
pub struct WRDS62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds62;
#[doc = "Write data section [1-64]"]
pub struct WRDS63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds63;
#[doc = "Write data section [1-64]"]
pub struct WRDS64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write data section [1-64]"]
pub mod wrds64;
#[doc = "Write header section 1"]
pub struct WRHS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write header section 1"]
pub mod wrhs1;
#[doc = "Write header section 2"]
pub struct WRHS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write header section 2"]
pub mod wrhs2;
#[doc = "Write header section 3"]
pub struct WRHS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write header section 3"]
pub mod wrhs3;
#[doc = "Input buffer command mask"]
pub struct IBCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input buffer command mask"]
pub mod ibcm;
#[doc = "Input buffer command request"]
pub struct IBCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input buffer command request"]
pub mod ibcr;
#[doc = "Read data section [1-64]"]
pub struct RDDS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds1;
#[doc = "Read data section [1-64]"]
pub struct RDDS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds2;
#[doc = "Read data section [1-64]"]
pub struct RDDS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds3;
#[doc = "Read data section [1-64]"]
pub struct RDDS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds4;
#[doc = "Read data section [1-64]"]
pub struct RDDS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds5;
#[doc = "Read data section [1-64]"]
pub struct RDDS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds6;
#[doc = "Read data section [1-64]"]
pub struct RDDS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds7;
#[doc = "Read data section [1-64]"]
pub struct RDDS8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds8;
#[doc = "Read data section [1-64]"]
pub struct RDDS9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds9;
#[doc = "Read data section [1-64]"]
pub struct RDDS10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds10;
#[doc = "Read data section [1-64]"]
pub struct RDDS11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds11;
#[doc = "Read data section [1-64]"]
pub struct RDDS12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds12;
#[doc = "Read data section [1-64]"]
pub struct RDDS13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds13;
#[doc = "Read data section [1-64]"]
pub struct RDDS14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds14;
#[doc = "Read data section [1-64]"]
pub struct RDDS15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds15;
#[doc = "Read data section [1-64]"]
pub struct RDDS16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds16;
#[doc = "Read data section [1-64]"]
pub struct RDDS17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds17;
#[doc = "Read data section [1-64]"]
pub struct RDDS18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds18;
#[doc = "Read data section [1-64]"]
pub struct RDDS19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds19;
#[doc = "Read data section [1-64]"]
pub struct RDDS20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds20;
#[doc = "Read data section [1-64]"]
pub struct RDDS21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds21;
#[doc = "Read data section [1-64]"]
pub struct RDDS22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds22;
#[doc = "Read data section [1-64]"]
pub struct RDDS23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds23;
#[doc = "Read data section [1-64]"]
pub struct RDDS24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds24;
#[doc = "Read data section [1-64]"]
pub struct RDDS25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds25;
#[doc = "Read data section [1-64]"]
pub struct RDDS26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds26;
#[doc = "Read data section [1-64]"]
pub struct RDDS27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds27;
#[doc = "Read data section [1-64]"]
pub struct RDDS28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds28;
#[doc = "Read data section [1-64]"]
pub struct RDDS29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds29;
#[doc = "Read data section [1-64]"]
pub struct RDDS30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds30;
#[doc = "Read data section [1-64]"]
pub struct RDDS31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds31;
#[doc = "Read data section [1-64]"]
pub struct RDDS32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds32;
#[doc = "Read data section [1-64]"]
pub struct RDDS33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds33;
#[doc = "Read data section [1-64]"]
pub struct RDDS34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds34;
#[doc = "Read data section [1-64]"]
pub struct RDDS35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds35;
#[doc = "Read data section [1-64]"]
pub struct RDDS36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds36;
#[doc = "Read data section [1-64]"]
pub struct RDDS37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds37;
#[doc = "Read data section [1-64]"]
pub struct RDDS38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds38;
#[doc = "Read data section [1-64]"]
pub struct RDDS39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds39;
#[doc = "Read data section [1-64]"]
pub struct RDDS40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds40;
#[doc = "Read data section [1-64]"]
pub struct RDDS41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds41;
#[doc = "Read data section [1-64]"]
pub struct RDDS42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds42;
#[doc = "Read data section [1-64]"]
pub struct RDDS43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds43;
#[doc = "Read data section [1-64]"]
pub struct RDDS44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds44;
#[doc = "Read data section [1-64]"]
pub struct RDDS45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds45;
#[doc = "Read data section [1-64]"]
pub struct RDDS46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds46;
#[doc = "Read data section [1-64]"]
pub struct RDDS47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds47;
#[doc = "Read data section [1-64]"]
pub struct RDDS48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds48;
#[doc = "Read data section [1-64]"]
pub struct RDDS49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds49;
#[doc = "Read data section [1-64]"]
pub struct RDDS50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds50;
#[doc = "Read data section [1-64]"]
pub struct RDDS51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds51;
#[doc = "Read data section [1-64]"]
pub struct RDDS52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds52;
#[doc = "Read data section [1-64]"]
pub struct RDDS53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds53;
#[doc = "Read data section [1-64]"]
pub struct RDDS54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds54;
#[doc = "Read data section [1-64]"]
pub struct RDDS55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds55;
#[doc = "Read data section [1-64]"]
pub struct RDDS56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds56;
#[doc = "Read data section [1-64]"]
pub struct RDDS57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds57;
#[doc = "Read data section [1-64]"]
pub struct RDDS58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds58;
#[doc = "Read data section [1-64]"]
pub struct RDDS59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds59;
#[doc = "Read data section [1-64]"]
pub struct RDDS60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds60;
#[doc = "Read data section [1-64]"]
pub struct RDDS61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds61;
#[doc = "Read data section [1-64]"]
pub struct RDDS62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds62;
#[doc = "Read data section [1-64]"]
pub struct RDDS63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds63;
#[doc = "Read data section [1-64]"]
pub struct RDDS64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read data section [1-64]"]
pub mod rdds64;
#[doc = "Read header section 1"]
pub struct RDHS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read header section 1"]
pub mod rdhs1;
#[doc = "Read header section 2"]
pub struct RDHS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read header section 2"]
pub mod rdhs2;
#[doc = "Read header section 3"]
pub struct RDHS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read header section 3"]
pub mod rdhs3;
#[doc = "Message buffer status"]
pub struct MBS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message buffer status"]
pub mod mbs;
#[doc = "Output buffer command mask"]
pub struct OBCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output buffer command mask"]
pub mod obcm;
#[doc = "Output buffer command request"]
pub struct OBCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output buffer command request"]
pub mod obcr;
