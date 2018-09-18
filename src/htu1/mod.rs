#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    pub glb_ctrl: GLBCTRL,
    #[doc = "0x04 - Control Packet Enable Register"]
    pub cpena: CPENA,
    #[doc = "0x08 - Control Packet (CP) Busy Register 0"]
    pub busy0: BUSY0,
    #[doc = "0x0c - Control Packet (CP) Busy Register 1"]
    pub busy1: BUSY1,
    #[doc = "0x10 - Control Packet (CP) Busy Register 2"]
    pub busy2: BUSY2,
    #[doc = "0x14 - Control Packet (CP) Busy Register 3"]
    pub busy3: BUSY3,
    #[doc = "0x18 - Active Control Packet Register"]
    pub acp: ACP,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Request Lost and Bus Error Control Register"]
    pub rlbectrl: RLBECTRL,
    #[doc = "0x24 - Buffer Full Interrupt Enable Set Register"]
    pub bfint_set: BFINTSET,
    #[doc = "0x28 - Buffer Full Interrupt Enable Clear Register"]
    pub bfint_clr: BFINTCLR,
    #[doc = "0x2c - Interrupt Mapping Register"]
    pub int_map: INTMAP,
    _reserved1: [u8; 4usize],
    #[doc = "0x34 - Interrupt Offset Register 0"]
    pub int_offst0: INTOFFST0,
    #[doc = "0x38 - Interrupt Offset Register 1"]
    pub int_offst1: INTOFFST1,
    #[doc = "0x3c - Buffer Initialization Mode Register"]
    pub bim: BIM,
    #[doc = "0x40 - Request Lost Flag Register"]
    pub rlost_flg: RLOSTFLG,
    #[doc = "0x44 - Buffer Full Interrupt Flag Register"]
    pub bfint_flg: BFINTFLG,
    #[doc = "0x48 - BER Interrupt Flag Register"]
    pub ber_int_flg: BERINTFLG,
    #[doc = "0x4c - Memory Protection 1 Start Address"]
    pub mp1strt: MP1STRT,
    #[doc = "0x50 - Memory Protection 1 End Address"]
    pub mp1end: MP1END,
    #[doc = "0x54 - Debug Control Register"]
    pub dbg_ctrl: DBGCTRL,
    #[doc = "0x58 - Watch Point Register"]
    pub wp_reg: WPREG,
    #[doc = "0x5c - Watch Mask Register"]
    pub wp_msk: WPMSK,
    #[doc = "0x60 - Module Identification Register"]
    pub id: ID,
    #[doc = "0x64 - Parity Control Register"]
    pub par_ctrl: PARCTRL,
    #[doc = "0x68 - Parity Address Register"]
    pub par_addr: PARADDR,
    _reserved2: [u8; 4usize],
    #[doc = "0x70 - Memory Protection Control and Status Register"]
    pub mp_ctrl_stat: MPCTRLSTAT,
    #[doc = "0x74 - Memory Protection Start Address Register"]
    pub mp0strt: MP0STRT,
    #[doc = "0x78 - Memory Protection End Address Register"]
    pub mp0end: MP0END,
}
#[doc = "Global Control Register"]
pub struct GLBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod glb_ctrl;
#[doc = "Control Packet Enable Register"]
pub struct CPENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Packet Enable Register"]
pub mod cpena;
#[doc = "Control Packet (CP) Busy Register 0"]
pub struct BUSY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Packet (CP) Busy Register 0"]
pub mod busy0;
#[doc = "Control Packet (CP) Busy Register 1"]
pub struct BUSY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Packet (CP) Busy Register 1"]
pub mod busy1;
#[doc = "Control Packet (CP) Busy Register 2"]
pub struct BUSY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Packet (CP) Busy Register 2"]
pub mod busy2;
#[doc = "Control Packet (CP) Busy Register 3"]
pub struct BUSY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Packet (CP) Busy Register 3"]
pub mod busy3;
#[doc = "Active Control Packet Register"]
pub struct ACP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Active Control Packet Register"]
pub mod acp;
#[doc = "Request Lost and Bus Error Control Register"]
pub struct RLBECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Lost and Bus Error Control Register"]
pub mod rlbectrl;
#[doc = "Buffer Full Interrupt Enable Set Register"]
pub struct BFINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer Full Interrupt Enable Set Register"]
pub mod bfint_set;
#[doc = "Buffer Full Interrupt Enable Clear Register"]
pub struct BFINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer Full Interrupt Enable Clear Register"]
pub mod bfint_clr;
#[doc = "Interrupt Mapping Register"]
pub struct INTMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mapping Register"]
pub mod int_map;
#[doc = "Interrupt Offset Register 0"]
pub struct INTOFFST0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Offset Register 0"]
pub mod int_offst0;
#[doc = "Interrupt Offset Register 1"]
pub struct INTOFFST1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Offset Register 1"]
pub mod int_offst1;
#[doc = "Buffer Initialization Mode Register"]
pub struct BIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer Initialization Mode Register"]
pub mod bim;
#[doc = "Request Lost Flag Register"]
pub struct RLOSTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Lost Flag Register"]
pub mod rlost_flg;
#[doc = "Buffer Full Interrupt Flag Register"]
pub struct BFINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer Full Interrupt Flag Register"]
pub mod bfint_flg;
#[doc = "BER Interrupt Flag Register"]
pub struct BERINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BER Interrupt Flag Register"]
pub mod ber_int_flg;
#[doc = "Memory Protection 1 Start Address"]
pub struct MP1STRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Protection 1 Start Address"]
pub mod mp1strt;
#[doc = "Memory Protection 1 End Address"]
pub struct MP1END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Protection 1 End Address"]
pub mod mp1end;
#[doc = "Debug Control Register"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Control Register"]
pub mod dbg_ctrl;
#[doc = "Watch Point Register"]
pub struct WPREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watch Point Register"]
pub mod wp_reg;
#[doc = "Watch Mask Register"]
pub struct WPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watch Mask Register"]
pub mod wp_msk;
#[doc = "Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "Parity Control Register"]
pub struct PARCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Control Register"]
pub mod par_ctrl;
#[doc = "Parity Address Register"]
pub struct PARADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Address Register"]
pub mod par_addr;
#[doc = "Memory Protection Control and Status Register"]
pub struct MPCTRLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Protection Control and Status Register"]
pub mod mp_ctrl_stat;
#[doc = "Memory Protection Start Address Register"]
pub struct MP0STRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Protection Start Address Register"]
pub mod mp0strt;
#[doc = "Memory Protection End Address Register"]
pub struct MP0END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Protection End Address Register"]
pub mod mp0end;
