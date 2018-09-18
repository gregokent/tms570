#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    pub glb_ctrl: GLBCTRL,
    #[doc = "0x04 - Channel Pending Register"]
    pub chn_pnd: CHNPND,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Status Register"]
    pub stat: STAT,
    _reserved1: [u8; 4usize],
    #[doc = "0x14 - H/W Channel Enable Set and Status Register"]
    pub hwchn_ena_set: HWCHNENASET,
    _reserved2: [u8; 4usize],
    #[doc = "0x1c - H/W Channel Enable Reset and Status Register"]
    pub hwchn_ena_rst: HWCHNENARST,
    _reserved3: [u8; 4usize],
    #[doc = "0x24 - S/W Channel Enable Set and Status Register"]
    pub swchn_ena_set: SWCHNENASET,
    _reserved4: [u8; 4usize],
    #[doc = "0x2c - S/W Channel Enable Reset and Status Register"]
    pub swchn_ena_rst: SWCHNENARST,
    _reserved5: [u8; 4usize],
    #[doc = "0x34 - Channel Priority Set Register"]
    pub chn_prio_set: CHNPRIOSET,
    _reserved6: [u8; 4usize],
    #[doc = "0x3c - Channel Priority Reset"]
    pub chn_prio_rst: CHNPRIORST,
    _reserved7: [u8; 4usize],
    #[doc = "0x44 - Global Channel Interrupt Enable Set"]
    pub glb_chn_int_ena_set: GLBCHNINTENASET,
    _reserved8: [u8; 4usize],
    #[doc = "0x4c - Global Channel Interrupt Enable Reset"]
    pub glb_chn_int_ena_rst: GLBCHNINTENARST,
    _reserved9: [u8; 4usize],
    #[doc = "0x54 - Request Assignment Register 0"]
    pub req_assg0: REQASSG0,
    #[doc = "0x58 - Request Assignment Register 1"]
    pub req_assg1: REQASSG1,
    #[doc = "0x5c - Request Assignment Register 2"]
    pub req_assg2: REQASSG2,
    #[doc = "0x60 - Request Assignment Register 3"]
    pub req_assg3: REQASSG3,
    #[doc = "0x64 - Request Assignment Register 4"]
    pub req_assg4: REQASSG4,
    #[doc = "0x68 - Request Assignment Register 5"]
    pub req_assg5: REQASSG5,
    #[doc = "0x6c - Request Assignment Register 6"]
    pub req_assg6: REQASSG6,
    #[doc = "0x70 - Request Assignment Register 7"]
    pub req_assg7: REQASSG7,
    _reserved10: [u8; 32usize],
    #[doc = "0x94 - Port Assignment Register 0"]
    pub prt_assg0: PRTASSG0,
    #[doc = "0x98 - Port Assignment Register 1"]
    pub prt_assg1: PRTASSG1,
    #[doc = "0x9c - Port Assignment Register 2"]
    pub prt_assg2: PRTASSG2,
    #[doc = "0xa0 - Port Assignment Register 3"]
    pub prt_assg3: PRTASSG3,
    _reserved11: [u8; 16usize],
    #[doc = "0xb4 - FTC Interrupt Mapping Register"]
    pub ftcmap: FTCMAP,
    _reserved12: [u8; 4usize],
    #[doc = "0xbc - LFS Interrupt Mapping Register"]
    pub lfsmap: LFSMAP,
    _reserved13: [u8; 4usize],
    #[doc = "0xc4 - HBC Interrupt Mapping Register"]
    pub hbcmap: HBCMAP,
    _reserved14: [u8; 4usize],
    #[doc = "0xcc - BTC Interrupt Mapping Register"]
    pub btcmap: BTCMAP,
    _reserved15: [u8; 4usize],
    #[doc = "0xd4 - BER Interrupt Mapping Register"]
    pub bermap: BERMAP,
    _reserved16: [u8; 4usize],
    #[doc = "0xdc - FTC Interrupt Enable Set"]
    pub ftcint_ena_set: FTCINTENASET,
    _reserved17: [u8; 4usize],
    #[doc = "0xe4 - FTC Interrupt Enable Reset"]
    pub ftcint_ena_rst: FTCINTENARST,
    _reserved18: [u8; 4usize],
    #[doc = "0xec - LFS Interrupt Enable Set"]
    pub lfsint_ena_set: LFSINTENASET,
    _reserved19: [u8; 4usize],
    #[doc = "0xf4 - LFS Interrupt Enable Reset"]
    pub lfsint_ena_rst: LFSINTENARST,
    _reserved20: [u8; 4usize],
    #[doc = "0xfc - HBC Interrupt Enable Set"]
    pub hbcint_ena_set: HBCINTENASET,
    _reserved21: [u8; 4usize],
    #[doc = "0x104 - HBC Interrupt Enable Reset"]
    pub hbcint_ena_rst: HBCINTENARST,
    _reserved22: [u8; 4usize],
    #[doc = "0x10c - BTC Interrupt Enable Set"]
    pub btcint_ena_set: BTCINTENASET,
    _reserved23: [u8; 4usize],
    #[doc = "0x114 - BTC Interrupt Enable Reset"]
    pub btcint_ena_rst: BTCINTENARST,
    _reserved24: [u8; 4usize],
    #[doc = "0x11c - Global Interrupt Flg Register"]
    pub glb_int_flg: GLBINTFLG,
    _reserved25: [u8; 4usize],
    #[doc = "0x124 - FTC Interrupt Flag Register"]
    pub ftcint_flg: FTCINTFLG,
    _reserved26: [u8; 4usize],
    #[doc = "0x12c - LFS Interrupt Flag Register"]
    pub lfsint_flg: LFSINTFLG,
    _reserved27: [u8; 4usize],
    #[doc = "0x134 - HBC Interrupt Flag Register"]
    pub hbcint_flg: HBCINTFLG,
    _reserved28: [u8; 4usize],
    #[doc = "0x13c - BER Interrupt Flag Register"]
    pub btcint_flg: BTCINTFLG,
    _reserved29: [u8; 4usize],
    #[doc = "0x144 - BER Interrupt Flag Register"]
    pub berint_flg: BERINTFLG,
    _reserved30: [u8; 4usize],
    #[doc = "0x14c - FTCA Interrupt Channel Offset Register"]
    pub ftcaoffst: FTCAOFFST,
    #[doc = "0x150 - LFSA Interrupt Channel Offset Register"]
    pub lfsaoffst: LFSAOFFST,
    #[doc = "0x154 - HBCA Interrupt Channel Offset Register"]
    pub hbcaoffst: HBCAOFFST,
    #[doc = "0x158 - BTCA Interrupt Channel Offset Register"]
    pub btcaoffst: BTCAOFFST,
    #[doc = "0x15c - BERA Interrupt Channel Offset Register"]
    pub beraoffst: BERAOFFST,
    #[doc = "0x160 - FTCB Interrupt Channel Offset Register"]
    pub ftcboffst: FTCBOFFST,
    #[doc = "0x164 - LFSB Interrupt Channel Offset Register"]
    pub lsfboffst: LSFBOFFST,
    #[doc = "0x168 - HBCB Interrupt Channel Offset Register"]
    pub hbcboffst: HBCBOFFST,
    #[doc = "0x16c - BTCB Interrupt Channel Offset Register"]
    pub btcboffst: BTCBOFFST,
    #[doc = "0x170 - BERB Interrupt Channel Offset Register"]
    pub berboffst: BERBOFFST,
    _reserved31: [u8; 4usize],
    #[doc = "0x178 - Port Control Register"]
    pub prt_ctrl: PRTCTRL,
    #[doc = "0x17c - RAM TEST Control"]
    pub ram_tst_ctrl: RAMTSTCTRL,
    #[doc = "0x180 - Debug Control"]
    pub dbg_ctrl: DBGCTRL,
    #[doc = "0x184 - Watchpoint Register"]
    pub wp_reg: WPREG,
    #[doc = "0x188 - Watchpoint Mask Register"]
    pub wp_msk: WPMSK,
    #[doc = "0x18c - Port A Active Channel Source Address Register"]
    pub prt_achn_src_addr: PRTACHNSRCADDR,
    #[doc = "0x190 - Port A Active Channel Destination Address Register"]
    pub prt_achn_dst_addr: PRTACHNDSTADDR,
    #[doc = "0x194 - Port A Active Channel Transfer Count Register"]
    pub prt_achn_tr_cnt: PRTACHNTRCNT,
    #[doc = "0x198 - Port B Active Channel Source Address Register"]
    pub prt_bchn_src_addr: PRTBCHNSRCADDR,
    #[doc = "0x19c - Port B Active Channel Destination Address Register"]
    pub prt_bchn_dest_addr: PRTBCHNDESTADDR,
    #[doc = "0x1a0 - Port B Active Channel Transfer Count Register"]
    pub prt_bchn_tr_cnt: PRTBCHNTRCNT,
    _reserved32: [u8; 4usize],
    #[doc = "0x1a8 - Parity Control Register"]
    pub par_ctrl: PARCTRL,
    #[doc = "0x1ac - Parity Error Address Register"]
    pub par_err_addr: PARERRADDR,
    #[doc = "0x1b0 - Memory Protection Control Register"]
    pub mp_ctrl: MPCTRL,
    #[doc = "0x1b4 - Memory Protection Status Register"]
    pub mp_stat: MPSTAT,
    #[doc = "0x1b8 - Start Address of region 0"]
    pub pr0strt: PR0STRT,
    #[doc = "0x1bc - End Address of region 0"]
    pub pr0end: PR0END,
    #[doc = "0x1c0 - Start Address of region 0"]
    pub pr1strt: PR1STRT,
    #[doc = "0x1c4 - End Address of region 1"]
    pub pr1end: PR1END,
    #[doc = "0x1c8 - Start Address of region 2"]
    pub pr2strt: PR2STRT,
    #[doc = "0x1cc - End Address of region 2"]
    pub pr2end: PR2END,
    #[doc = "0x1d0 - Start Address of region 3"]
    pub pr3strt: PR3STRT,
    #[doc = "0x1d4 - End Address of region 3"]
    pub pr3end: PR3END,
    #[doc = "0x1d8 - Memory Protection Control Register2"]
    pub mp_ctrl2: MPCTRL2,
    #[doc = "0x1dc - Memory Protection Status Register2"]
    pub mp_stat2: MPSTAT2,
    #[doc = "0x1e0 - Start Address of region 4"]
    pub pr4strt: PR4STRT,
    #[doc = "0x1e4 - End Address of region 4"]
    pub pr4end: PR4END,
    #[doc = "0x1e8 - Start Address of region 5"]
    pub pr5strt: PR5STRT,
    #[doc = "0x1ec - End Address of region 5"]
    pub pr5end: PR5END,
    #[doc = "0x1f0 - Start Address of region 6"]
    pub pr6strt: PR6STRT,
    #[doc = "0x1f4 - End Address of region 6"]
    pub pr6end: PR6END,
    #[doc = "0x1f8 - Start Address of region 7"]
    pub pr7strt: PR7STRT,
    #[doc = "0x1fc - End Address of region 7"]
    pub pr7end: PR7END,
    #[doc = "0x200 - Pattern Fill Control Register"]
    pub pfctrl: PFCTRL,
    #[doc = "0x204 - Upper Pattern Fill Register"]
    pub upfreg: UPFREG,
    #[doc = "0x208 - Lower Pattern Fill Register"]
    pub lpfreg: LPFREG,
    _reserved33: [u8; 4usize],
    #[doc = "0x210 - Pattern Mask Control Register"]
    pub pmctrl: PMCTRL,
    #[doc = "0x214 - Upper Pattern Mask Register"]
    pub upmreg: UPMREG,
    #[doc = "0x218 - Lower Pattern Mask Register"]
    pub lpmreg: LPMREG,
    _reserved34: [u8; 4usize],
    #[doc = "0x220 - Cull Configuration Register"]
    pub cull_con_reg: CULLCONREG,
    _reserved35: [u8; 4usize],
    #[doc = "0x228 - Single Bit ECC Control Register"]
    pub secc_ctrl: SECCCTRL,
    _reserved36: [u8; 4usize],
    #[doc = "0x230 - Single Bit ECC Error Address Register"]
    pub secc_addr: SECCADDR,
    _reserved37: [u8; 12usize],
    #[doc = "0x240 - Fifo A Status Register"]
    pub fifo_astat: FIFOASTAT,
    #[doc = "0x244 - Fifo B Status Register"]
    pub fifo_bstat: FIFOBSTAT,
    _reserved38: [u8; 12usize],
    #[doc = "0x254 - FTC Interrupt Mapping Register 0"]
    pub ftcmap0: FTCMAP0,
    #[doc = "0x258 - FTC Interrupt Mapping Register 1"]
    pub ftcmap1: FTCMAP1,
    #[doc = "0x25c - LFS Interrupt Mapping Register 0"]
    pub lfsmap0: LFSMAP0,
    #[doc = "0x260 - LFS Interrupt Mapping Register 1"]
    pub lfsmap1: LFSMAP1,
    #[doc = "0x264 - HBC Interrupt Mapping Register 0"]
    pub hbcmap0: HBCMAP0,
    #[doc = "0x268 - HBC Interrupt Mapping Register 1"]
    pub hbcmap1: HBCMAP1,
    #[doc = "0x26c - BTC Interrupt Mapping Register 0"]
    pub btcmap0: BTCMAP0,
    #[doc = "0x270 - BTC Interrupt Mapping Register 1"]
    pub btcmap1: BTCMAP1,
    #[doc = "0x274 - BER Interrupt Mapping Register 0"]
    pub bermap0: BERMAP0,
    #[doc = "0x278 - BER Interrupt Mapping Register 1"]
    pub bermap1: BERMAP1,
    _reserved39: [u8; 128usize],
    #[doc = "0x2fc - FTC Interrupt Channel Offset Register for CPU3"]
    pub ftccoff_set: FTCCOFFSET,
    #[doc = "0x300 - LFS Interrupt Channel Offset Register for CPU3"]
    pub lfscoff_set: LFSCOFFSET,
    #[doc = "0x304 - HBC Interrupt Channel Offset Register for CPU3"]
    pub hbccoff_set: HBCCOFFSET,
    #[doc = "0x308 - BTC Interrupt Channel Offset Register for CPU3"]
    pub btccoff_set: BTCCOFFSET,
    #[doc = "0x30c - BER Interrupt Channel Offset Register for CPU3"]
    pub bercoff_set: BERCOFFSET,
    #[doc = "0x310 - FTC Interrupt Channel Offset Register for CPU4"]
    pub ftcdoff_set: FTCDOFFSET,
    #[doc = "0x314 - LFS Interrupt Channel Offset Register for CPU4"]
    pub lfsdoff_set: LFSDOFFSET,
    #[doc = "0x318 - HBC Interrupt Channel Offset Register for CPU4"]
    pub hbcdoff_set: HBCDOFFSET,
    #[doc = "0x31c - BTC Interrupt Channel Offset Register for CPU4"]
    pub btcdoff_set: BTCDOFFSET,
    #[doc = "0x320 - BER Interrupt Channel Offset Register for CPU4"]
    pub berdoff_set: BERDOFFSET,
    _reserved40: [u8; 12usize],
    #[doc = "0x330 - Request Polarity Select Register1"]
    pub req_pol_sel1: REQPOLSEL1,
    #[doc = "0x334 - Request Polarity Select Register0"]
    pub req_pol_sel0: REQPOLSEL0,
    _reserved41: [u8; 8usize],
    #[doc = "0x340 - TER Event Control Register"]
    pub terevt_ctrl: TEREVTCTRL,
    #[doc = "0x344 - TER Event Flag Register"]
    pub terevt_flag: TEREVTFLAG,
    #[doc = "0x348 - TER Event Offset Register"]
    pub terevt_offset: TEREVTOFFSET,
}
#[doc = "Global Control Register"]
pub struct GLBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod glb_ctrl;
#[doc = "Channel Pending Register"]
pub struct CHNPND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Pending Register"]
pub mod chn_pnd;
#[doc = "Status Register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod stat;
#[doc = "H/W Channel Enable Set and Status Register"]
pub struct HWCHNENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "H/W Channel Enable Set and Status Register"]
pub mod hwchn_ena_set;
#[doc = "H/W Channel Enable Reset and Status Register"]
pub struct HWCHNENARST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "H/W Channel Enable Reset and Status Register"]
pub mod hwchn_ena_rst;
#[doc = "S/W Channel Enable Set and Status Register"]
pub struct SWCHNENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "S/W Channel Enable Set and Status Register"]
pub mod swchn_ena_set;
#[doc = "S/W Channel Enable Reset and Status Register"]
pub struct SWCHNENARST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "S/W Channel Enable Reset and Status Register"]
pub mod swchn_ena_rst;
#[doc = "Channel Priority Set Register"]
pub struct CHNPRIOSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Priority Set Register"]
pub mod chn_prio_set;
#[doc = "Channel Priority Reset"]
pub struct CHNPRIORST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Priority Reset"]
pub mod chn_prio_rst;
#[doc = "Global Channel Interrupt Enable Set"]
pub struct GLBCHNINTENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Interrupt Enable Set"]
pub mod glb_chn_int_ena_set;
#[doc = "Global Channel Interrupt Enable Reset"]
pub struct GLBCHNINTENARST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Interrupt Enable Reset"]
pub mod glb_chn_int_ena_rst;
#[doc = "Request Assignment Register 0"]
pub struct REQASSG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Assignment Register 0"]
pub mod req_assg0;
#[doc = "Request Assignment Register 1"]
pub struct REQASSG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Assignment Register 1"]
pub mod req_assg1;
#[doc = "Request Assignment Register 2"]
pub struct REQASSG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Assignment Register 2"]
pub mod req_assg2;
#[doc = "Request Assignment Register 3"]
pub struct REQASSG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Assignment Register 3"]
pub mod req_assg3;
#[doc = "Request Assignment Register 4"]
pub struct REQASSG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Assignment Register 4"]
pub mod req_assg4;
#[doc = "Request Assignment Register 5"]
pub struct REQASSG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Assignment Register 5"]
pub mod req_assg5;
#[doc = "Request Assignment Register 6"]
pub struct REQASSG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Assignment Register 6"]
pub mod req_assg6;
#[doc = "Request Assignment Register 7"]
pub struct REQASSG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Assignment Register 7"]
pub mod req_assg7;
#[doc = "Port Assignment Register 0"]
pub struct PRTASSG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Assignment Register 0"]
pub mod prt_assg0;
#[doc = "Port Assignment Register 1"]
pub struct PRTASSG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Assignment Register 1"]
pub mod prt_assg1;
#[doc = "Port Assignment Register 2"]
pub struct PRTASSG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Assignment Register 2"]
pub mod prt_assg2;
#[doc = "Port Assignment Register 3"]
pub struct PRTASSG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Assignment Register 3"]
pub mod prt_assg3;
#[doc = "FTC Interrupt Mapping Register"]
pub struct FTCMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTC Interrupt Mapping Register"]
pub mod ftcmap;
#[doc = "LFS Interrupt Mapping Register"]
pub struct LFSMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFS Interrupt Mapping Register"]
pub mod lfsmap;
#[doc = "HBC Interrupt Mapping Register"]
pub struct HBCMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HBC Interrupt Mapping Register"]
pub mod hbcmap;
#[doc = "BTC Interrupt Mapping Register"]
pub struct BTCMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BTC Interrupt Mapping Register"]
pub mod btcmap;
#[doc = "BER Interrupt Mapping Register"]
pub struct BERMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BER Interrupt Mapping Register"]
pub mod bermap;
#[doc = "FTC Interrupt Enable Set"]
pub struct FTCINTENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTC Interrupt Enable Set"]
pub mod ftcint_ena_set;
#[doc = "FTC Interrupt Enable Reset"]
pub struct FTCINTENARST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTC Interrupt Enable Reset"]
pub mod ftcint_ena_rst;
#[doc = "LFS Interrupt Enable Set"]
pub struct LFSINTENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFS Interrupt Enable Set"]
pub mod lfsint_ena_set;
#[doc = "LFS Interrupt Enable Reset"]
pub struct LFSINTENARST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFS Interrupt Enable Reset"]
pub mod lfsint_ena_rst;
#[doc = "HBC Interrupt Enable Set"]
pub struct HBCINTENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HBC Interrupt Enable Set"]
pub mod hbcint_ena_set;
#[doc = "HBC Interrupt Enable Reset"]
pub struct HBCINTENARST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HBC Interrupt Enable Reset"]
pub mod hbcint_ena_rst;
#[doc = "BTC Interrupt Enable Set"]
pub struct BTCINTENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BTC Interrupt Enable Set"]
pub mod btcint_ena_set;
#[doc = "BTC Interrupt Enable Reset"]
pub struct BTCINTENARST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BTC Interrupt Enable Reset"]
pub mod btcint_ena_rst;
#[doc = "Global Interrupt Flg Register"]
pub struct GLBINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Interrupt Flg Register"]
pub mod glb_int_flg;
#[doc = "FTC Interrupt Flag Register"]
pub struct FTCINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTC Interrupt Flag Register"]
pub mod ftcint_flg;
#[doc = "LFS Interrupt Flag Register"]
pub struct LFSINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFS Interrupt Flag Register"]
pub mod lfsint_flg;
#[doc = "HBC Interrupt Flag Register"]
pub struct HBCINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HBC Interrupt Flag Register"]
pub mod hbcint_flg;
#[doc = "BER Interrupt Flag Register"]
pub struct BTCINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BER Interrupt Flag Register"]
pub mod btcint_flg;
#[doc = "BER Interrupt Flag Register"]
pub struct BERINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BER Interrupt Flag Register"]
pub mod berint_flg;
#[doc = "FTCA Interrupt Channel Offset Register"]
pub struct FTCAOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTCA Interrupt Channel Offset Register"]
pub mod ftcaoffst;
#[doc = "LFSA Interrupt Channel Offset Register"]
pub struct LFSAOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFSA Interrupt Channel Offset Register"]
pub mod lfsaoffst;
#[doc = "HBCA Interrupt Channel Offset Register"]
pub struct HBCAOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HBCA Interrupt Channel Offset Register"]
pub mod hbcaoffst;
#[doc = "BTCA Interrupt Channel Offset Register"]
pub struct BTCAOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BTCA Interrupt Channel Offset Register"]
pub mod btcaoffst;
#[doc = "BERA Interrupt Channel Offset Register"]
pub struct BERAOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BERA Interrupt Channel Offset Register"]
pub mod beraoffst;
#[doc = "FTCB Interrupt Channel Offset Register"]
pub struct FTCBOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTCB Interrupt Channel Offset Register"]
pub mod ftcboffst;
#[doc = "LFSB Interrupt Channel Offset Register"]
pub struct LSFBOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFSB Interrupt Channel Offset Register"]
pub mod lsfboffst;
#[doc = "HBCB Interrupt Channel Offset Register"]
pub struct HBCBOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HBCB Interrupt Channel Offset Register"]
pub mod hbcboffst;
#[doc = "BTCB Interrupt Channel Offset Register"]
pub struct BTCBOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BTCB Interrupt Channel Offset Register"]
pub mod btcboffst;
#[doc = "BERB Interrupt Channel Offset Register"]
pub struct BERBOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BERB Interrupt Channel Offset Register"]
pub mod berboffst;
#[doc = "Port Control Register"]
pub struct PRTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Control Register"]
pub mod prt_ctrl;
#[doc = "RAM TEST Control"]
pub struct RAMTSTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RAM TEST Control"]
pub mod ram_tst_ctrl;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Control"]
pub mod dbg_ctrl;
#[doc = "Watchpoint Register"]
pub struct WPREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchpoint Register"]
pub mod wp_reg;
#[doc = "Watchpoint Mask Register"]
pub struct WPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchpoint Mask Register"]
pub mod wp_msk;
#[doc = "Port A Active Channel Source Address Register"]
pub struct PRTACHNSRCADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port A Active Channel Source Address Register"]
pub mod prt_achn_src_addr;
#[doc = "Port A Active Channel Destination Address Register"]
pub struct PRTACHNDSTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port A Active Channel Destination Address Register"]
pub mod prt_achn_dst_addr;
#[doc = "Port A Active Channel Transfer Count Register"]
pub struct PRTACHNTRCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port A Active Channel Transfer Count Register"]
pub mod prt_achn_tr_cnt;
#[doc = "Port B Active Channel Source Address Register"]
pub struct PRTBCHNSRCADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port B Active Channel Source Address Register"]
pub mod prt_bchn_src_addr;
#[doc = "Port B Active Channel Destination Address Register"]
pub struct PRTBCHNDESTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port B Active Channel Destination Address Register"]
pub mod prt_bchn_dest_addr;
#[doc = "Port B Active Channel Transfer Count Register"]
pub struct PRTBCHNTRCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port B Active Channel Transfer Count Register"]
pub mod prt_bchn_tr_cnt;
#[doc = "Parity Control Register"]
pub struct PARCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Control Register"]
pub mod par_ctrl;
#[doc = "Parity Error Address Register"]
pub struct PARERRADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Error Address Register"]
pub mod par_err_addr;
#[doc = "Memory Protection Control Register"]
pub struct MPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Protection Control Register"]
pub mod mp_ctrl;
#[doc = "Memory Protection Status Register"]
pub struct MPSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Protection Status Register"]
pub mod mp_stat;
#[doc = "Start Address of region 0"]
pub struct PR0STRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Address of region 0"]
pub mod pr0strt;
#[doc = "End Address of region 0"]
pub struct PR0END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End Address of region 0"]
pub mod pr0end;
#[doc = "Start Address of region 0"]
pub struct PR1STRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Address of region 0"]
pub mod pr1strt;
#[doc = "End Address of region 1"]
pub struct PR1END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End Address of region 1"]
pub mod pr1end;
#[doc = "Start Address of region 2"]
pub struct PR2STRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Address of region 2"]
pub mod pr2strt;
#[doc = "End Address of region 2"]
pub struct PR2END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End Address of region 2"]
pub mod pr2end;
#[doc = "Start Address of region 3"]
pub struct PR3STRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Address of region 3"]
pub mod pr3strt;
#[doc = "End Address of region 3"]
pub struct PR3END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End Address of region 3"]
pub mod pr3end;
#[doc = "Memory Protection Control Register2"]
pub struct MPCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Protection Control Register2"]
pub mod mp_ctrl2;
#[doc = "Memory Protection Status Register2"]
pub struct MPSTAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Protection Status Register2"]
pub mod mp_stat2;
#[doc = "Start Address of region 4"]
pub struct PR4STRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Address of region 4"]
pub mod pr4strt;
#[doc = "End Address of region 4"]
pub struct PR4END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End Address of region 4"]
pub mod pr4end;
#[doc = "Start Address of region 5"]
pub struct PR5STRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Address of region 5"]
pub mod pr5strt;
#[doc = "End Address of region 5"]
pub struct PR5END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End Address of region 5"]
pub mod pr5end;
#[doc = "Start Address of region 6"]
pub struct PR6STRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Address of region 6"]
pub mod pr6strt;
#[doc = "End Address of region 6"]
pub struct PR6END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End Address of region 6"]
pub mod pr6end;
#[doc = "Start Address of region 7"]
pub struct PR7STRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Address of region 7"]
pub mod pr7strt;
#[doc = "End Address of region 7"]
pub struct PR7END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End Address of region 7"]
pub mod pr7end;
#[doc = "Pattern Fill Control Register"]
pub struct PFCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern Fill Control Register"]
pub mod pfctrl;
#[doc = "Upper Pattern Fill Register"]
pub struct UPFREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Upper Pattern Fill Register"]
pub mod upfreg;
#[doc = "Lower Pattern Fill Register"]
pub struct LPFREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lower Pattern Fill Register"]
pub mod lpfreg;
#[doc = "Pattern Mask Control Register"]
pub struct PMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern Mask Control Register"]
pub mod pmctrl;
#[doc = "Upper Pattern Mask Register"]
pub struct UPMREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Upper Pattern Mask Register"]
pub mod upmreg;
#[doc = "Lower Pattern Mask Register"]
pub struct LPMREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lower Pattern Mask Register"]
pub mod lpmreg;
#[doc = "Cull Configuration Register"]
pub struct CULLCONREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cull Configuration Register"]
pub mod cull_con_reg;
#[doc = "Single Bit ECC Control Register"]
pub struct SECCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Bit ECC Control Register"]
pub mod secc_ctrl;
#[doc = "Single Bit ECC Error Address Register"]
pub struct SECCADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Bit ECC Error Address Register"]
pub mod secc_addr;
#[doc = "Fifo A Status Register"]
pub struct FIFOASTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fifo A Status Register"]
pub mod fifo_astat;
#[doc = "Fifo B Status Register"]
pub struct FIFOBSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fifo B Status Register"]
pub mod fifo_bstat;
#[doc = "FTC Interrupt Mapping Register 0"]
pub struct FTCMAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTC Interrupt Mapping Register 0"]
pub mod ftcmap0;
#[doc = "FTC Interrupt Mapping Register 1"]
pub struct FTCMAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTC Interrupt Mapping Register 1"]
pub mod ftcmap1;
#[doc = "LFS Interrupt Mapping Register 0"]
pub struct LFSMAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFS Interrupt Mapping Register 0"]
pub mod lfsmap0;
#[doc = "LFS Interrupt Mapping Register 1"]
pub struct LFSMAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFS Interrupt Mapping Register 1"]
pub mod lfsmap1;
#[doc = "HBC Interrupt Mapping Register 0"]
pub struct HBCMAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HBC Interrupt Mapping Register 0"]
pub mod hbcmap0;
#[doc = "HBC Interrupt Mapping Register 1"]
pub struct HBCMAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HBC Interrupt Mapping Register 1"]
pub mod hbcmap1;
#[doc = "BTC Interrupt Mapping Register 0"]
pub struct BTCMAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BTC Interrupt Mapping Register 0"]
pub mod btcmap0;
#[doc = "BTC Interrupt Mapping Register 1"]
pub struct BTCMAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BTC Interrupt Mapping Register 1"]
pub mod btcmap1;
#[doc = "BER Interrupt Mapping Register 0"]
pub struct BERMAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BER Interrupt Mapping Register 0"]
pub mod bermap0;
#[doc = "BER Interrupt Mapping Register 1"]
pub struct BERMAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BER Interrupt Mapping Register 1"]
pub mod bermap1;
#[doc = "FTC Interrupt Channel Offset Register for CPU3"]
pub struct FTCCOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTC Interrupt Channel Offset Register for CPU3"]
pub mod ftccoff_set;
#[doc = "LFS Interrupt Channel Offset Register for CPU3"]
pub struct LFSCOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFS Interrupt Channel Offset Register for CPU3"]
pub mod lfscoff_set;
#[doc = "HBC Interrupt Channel Offset Register for CPU3"]
pub struct HBCCOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HBC Interrupt Channel Offset Register for CPU3"]
pub mod hbccoff_set;
#[doc = "BTC Interrupt Channel Offset Register for CPU3"]
pub struct BTCCOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BTC Interrupt Channel Offset Register for CPU3"]
pub mod btccoff_set;
#[doc = "BER Interrupt Channel Offset Register for CPU3"]
pub struct BERCOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BER Interrupt Channel Offset Register for CPU3"]
pub mod bercoff_set;
#[doc = "FTC Interrupt Channel Offset Register for CPU4"]
pub struct FTCDOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTC Interrupt Channel Offset Register for CPU4"]
pub mod ftcdoff_set;
#[doc = "LFS Interrupt Channel Offset Register for CPU4"]
pub struct LFSDOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFS Interrupt Channel Offset Register for CPU4"]
pub mod lfsdoff_set;
#[doc = "HBC Interrupt Channel Offset Register for CPU4"]
pub struct HBCDOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HBC Interrupt Channel Offset Register for CPU4"]
pub mod hbcdoff_set;
#[doc = "BTC Interrupt Channel Offset Register for CPU4"]
pub struct BTCDOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BTC Interrupt Channel Offset Register for CPU4"]
pub mod btcdoff_set;
#[doc = "BER Interrupt Channel Offset Register for CPU4"]
pub struct BERDOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BER Interrupt Channel Offset Register for CPU4"]
pub mod berdoff_set;
#[doc = "Request Polarity Select Register1"]
pub struct REQPOLSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Polarity Select Register1"]
pub mod req_pol_sel1;
#[doc = "Request Polarity Select Register0"]
pub struct REQPOLSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Polarity Select Register0"]
pub mod req_pol_sel0;
#[doc = "TER Event Control Register"]
pub struct TEREVTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TER Event Control Register"]
pub mod terevt_ctrl;
#[doc = "TER Event Flag Register"]
pub struct TEREVTFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TER Event Flag Register"]
pub mod terevt_flag;
#[doc = "TER Event Offset Register"]
pub struct TEREVTOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TER Event Offset Register"]
pub mod terevt_offset;
