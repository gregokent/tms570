#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    pub glb_ctrl: GLBCTRL,
    #[doc = "0x04 - Interrupt Set Register"]
    pub int_set: INTSET,
    #[doc = "0x08 - Interrupt Clear Register"]
    pub int_clr: INTCLR,
    #[doc = "0x0c - Interrupt Level Register"]
    pub int_lvl: INTLVL,
    #[doc = "0x10 - Interrupt Flag Register"]
    pub int_flg: INTFLG,
    #[doc = "0x14 - Interrupt Offset 1 Register"]
    pub int_offst1: INTOFFST1,
    #[doc = "0x18 - Interrupt Offset 2 Register"]
    pub int_offst2: INTOFFST2,
    #[doc = "0x1c - Direct Data Mode Destination Register"]
    pub ddm_dst: DDMDST,
    #[doc = "0x20 - Direct Data Mode Blocksize Register"]
    pub ddm_blk_sz: DDMBLKSZ,
    #[doc = "0x24 - Direct Data Mode Pointer Register"]
    pub ddm_ptr: DDMPTR,
    #[doc = "0x28 - Direct Data Mode Interrupt Pointer Register"]
    pub ddm_int_ptr: DDMINTPTR,
    #[doc = "0x2c - Destination 0 Region 1"]
    pub dst0rgn1: DST0RGN1,
    #[doc = "0x30 - Destination 0 Blocksize 1"]
    pub dst0blk_sz1: DST0BLKSZ1,
    #[doc = "0x34 - Destination 0 Region 2"]
    pub dst0rgn2: DST0RGN2,
    #[doc = "0x38 - Destination 0 Blocksize 2"]
    pub dst0blk_sz2: DST0BLKSZ2,
    #[doc = "0x3c - Destination 1 Region 1"]
    pub dst1rgn1: DST1RGN1,
    #[doc = "0x40 - Destination 1 Blocksize 1"]
    pub dst1blk_sz1: DST1BLKSZ1,
    #[doc = "0x44 - Destination 1 Region 2"]
    pub dst1rgn2: DST1RGN2,
    #[doc = "0x48 - Destination 1 Blocksize 2"]
    pub dst1blk_sz2: DST1BLKSZ2,
    #[doc = "0x4c - Destination 2 Region 1"]
    pub dst2rgn1: DST2RGN1,
    #[doc = "0x50 - Destination 2 Blocksize 1"]
    pub dst2blk_sz1: DST2BLKSZ1,
    #[doc = "0x54 - Destination 2 Region 2"]
    pub dst2rgn2: DST2RGN2,
    #[doc = "0x58 - Destination 2 Blocksize 2"]
    pub dst2blk_sz2: DST2BLKSZ2,
    #[doc = "0x5c - Destination 3 Region 1"]
    pub dst3rgn1: DST3RGN1,
    #[doc = "0x60 - Destination 3 Blocksize 1"]
    pub dst3blk_sz1: DST3BLKSZ1,
    #[doc = "0x64 - Destination 3 Region 2"]
    pub dst3rgn2: DST3RGN2,
    #[doc = "0x68 - Destination 3 Blocksize 2"]
    pub dst3blk_sz2: DST3BLKSZ2,
    #[doc = "0x6c - Pin Control 0"]
    pub fun: FUN,
    #[doc = "0x70 - Pin Control 1"]
    pub dir: DIR,
    #[doc = "0x74 - Pin Control 2"]
    pub din: DIN,
    #[doc = "0x78 - Pin Control 3"]
    pub dout: DOUT,
    #[doc = "0x7c - Pin Control 4"]
    pub dset: DSET,
    #[doc = "0x80 - Pin Control 5"]
    pub dclr: DCLR,
    #[doc = "0x84 - Pin Control 6"]
    pub pdr: PDR,
    #[doc = "0x88 - Pin Control 7"]
    pub pdis: PDIS,
    #[doc = "0x8c - Pin Control 8"]
    pub psel: PSEL,
}
#[doc = "Global Control Register"]
pub struct GLBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod glb_ctrl;
#[doc = "Interrupt Set Register"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set Register"]
pub mod int_set;
#[doc = "Interrupt Clear Register"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod int_clr;
#[doc = "Interrupt Level Register"]
pub struct INTLVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Level Register"]
pub mod int_lvl;
#[doc = "Interrupt Flag Register"]
pub struct INTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Register"]
pub mod int_flg;
#[doc = "Interrupt Offset 1 Register"]
pub struct INTOFFST1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Offset 1 Register"]
pub mod int_offst1;
#[doc = "Interrupt Offset 2 Register"]
pub struct INTOFFST2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Offset 2 Register"]
pub mod int_offst2;
#[doc = "Direct Data Mode Destination Register"]
pub struct DDMDST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct Data Mode Destination Register"]
pub mod ddm_dst;
#[doc = "Direct Data Mode Blocksize Register"]
pub struct DDMBLKSZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct Data Mode Blocksize Register"]
pub mod ddm_blk_sz;
#[doc = "Direct Data Mode Pointer Register"]
pub struct DDMPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct Data Mode Pointer Register"]
pub mod ddm_ptr;
#[doc = "Direct Data Mode Interrupt Pointer Register"]
pub struct DDMINTPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct Data Mode Interrupt Pointer Register"]
pub mod ddm_int_ptr;
#[doc = "Destination 0 Region 1"]
pub struct DST0RGN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 0 Region 1"]
pub mod dst0rgn1;
#[doc = "Destination 0 Blocksize 1"]
pub struct DST0BLKSZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 0 Blocksize 1"]
pub mod dst0blk_sz1;
#[doc = "Destination 0 Region 2"]
pub struct DST0RGN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 0 Region 2"]
pub mod dst0rgn2;
#[doc = "Destination 0 Blocksize 2"]
pub struct DST0BLKSZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 0 Blocksize 2"]
pub mod dst0blk_sz2;
#[doc = "Destination 1 Region 1"]
pub struct DST1RGN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 1 Region 1"]
pub mod dst1rgn1;
#[doc = "Destination 1 Blocksize 1"]
pub struct DST1BLKSZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 1 Blocksize 1"]
pub mod dst1blk_sz1;
#[doc = "Destination 1 Region 2"]
pub struct DST1RGN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 1 Region 2"]
pub mod dst1rgn2;
#[doc = "Destination 1 Blocksize 2"]
pub struct DST1BLKSZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 1 Blocksize 2"]
pub mod dst1blk_sz2;
#[doc = "Destination 2 Region 1"]
pub struct DST2RGN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 2 Region 1"]
pub mod dst2rgn1;
#[doc = "Destination 2 Blocksize 1"]
pub struct DST2BLKSZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 2 Blocksize 1"]
pub mod dst2blk_sz1;
#[doc = "Destination 2 Region 2"]
pub struct DST2RGN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 2 Region 2"]
pub mod dst2rgn2;
#[doc = "Destination 2 Blocksize 2"]
pub struct DST2BLKSZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 2 Blocksize 2"]
pub mod dst2blk_sz2;
#[doc = "Destination 3 Region 1"]
pub struct DST3RGN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 3 Region 1"]
pub mod dst3rgn1;
#[doc = "Destination 3 Blocksize 1"]
pub struct DST3BLKSZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 3 Blocksize 1"]
pub mod dst3blk_sz1;
#[doc = "Destination 3 Region 2"]
pub struct DST3RGN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 3 Region 2"]
pub mod dst3rgn2;
#[doc = "Destination 3 Blocksize 2"]
pub struct DST3BLKSZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Destination 3 Blocksize 2"]
pub mod dst3blk_sz2;
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
