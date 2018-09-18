#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Control Register"]
    pub ctl: CTL,
    #[doc = "0x04 - Error and Status register"]
    pub es: ES,
    #[doc = "0x08 - Error Counter Register"]
    pub errc: ERRC,
    #[doc = "0x0c - BIT Timing Register"]
    pub btr: BTR,
    #[doc = "0x10 - Interrupt Register"]
    pub int: INT,
    #[doc = "0x14 - Test Register"]
    pub test: TEST,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - Parity Error Code Register"]
    pub perr: PERR,
    #[doc = "0x20 - Core Release Register"]
    pub rel: REL,
    #[doc = "0x24 - ECC Diagnostic Register"]
    pub eccdiag: ECCDIAG,
    #[doc = "0x28 - ECC Diagnostic Status Register"]
    pub eccdiag_stat: ECCDIAG_STAT,
    #[doc = "0x2c - ECC Control and Status Register"]
    pub ecc_cs: ECC_CS,
    #[doc = "0x30 - ECC Single Bit Error Code Register"]
    pub ecc_serr: ECC_SERR,
    _reserved1: [u8; 76usize],
    #[doc = "0x80 - Auto-Bus-On Time Register"]
    pub abotr: ABOTR,
    #[doc = "0x84 - Transmission Request X"]
    pub txrqx: TXRQX,
    #[doc = "0x88 - Transmission Request 12 Register"]
    pub txrq12: TXRQ12,
    #[doc = "0x8c - Transmission Request 34 Register"]
    pub txrq34: TXRQ34,
    #[doc = "0x90 - Transmission Request 56 Register"]
    pub txrq56: TXRQ56,
    #[doc = "0x94 - Transmission Request 78 Register"]
    pub txrq78: TXRQ78,
    #[doc = "0x98 - New Data X Register"]
    pub nwdatx: NWDATX,
    #[doc = "0x9c - New Data 12 Register"]
    pub nwdat12: NWDAT12,
    #[doc = "0xa0 - New Data 34 Register"]
    pub nwdat34: NWDAT34,
    #[doc = "0xa4 - New Data 56 Register"]
    pub nwdat56: NWDAT56,
    #[doc = "0xa8 - New Data 78 Register"]
    pub nwdat78: NWDAT78,
    #[doc = "0xac - Interrupt Pending X Register"]
    pub intpndx: INTPNDX,
    #[doc = "0xb0 - Interrupt Pending 12 Register"]
    pub intpnd12: INTPND12,
    #[doc = "0xb4 - Interrupt Pending 34 Register"]
    pub intpnd34: INTPND34,
    #[doc = "0xb8 - Interrupt Pending 56 Register"]
    pub intpnd56: INTPND56,
    #[doc = "0xbc - Interrupt Pending 78 Register"]
    pub intpnd78: INTPND78,
    #[doc = "0xc0 - Message Valid X Register"]
    pub msgvalx: MSGVALX,
    #[doc = "0xc4 - Message Valid 12 Register"]
    pub msgval12: MSGVAL12,
    #[doc = "0xc8 - Message Valid 34 Register"]
    pub msgval34: MSGVAL34,
    #[doc = "0xcc - Message Valid 56 Register"]
    pub msgval56: MSGVAL56,
    #[doc = "0xd0 - Message Valid 78 Register"]
    pub msgval78: MSGVAL78,
    _reserved2: [u8; 4usize],
    #[doc = "0xd8 - Interrupt Multiplexer 12 Register"]
    pub intmux12: INTMUX12,
    #[doc = "0xdc - Interrupt Multiplexer 34 Register"]
    pub intmux34: INTMUX34,
    #[doc = "0xe0 - Interrupt Multiplexer 56 Register"]
    pub intmux56: INTMUX56,
    #[doc = "0xe4 - Interrupt Multiplexer 78 Register"]
    pub intmux78: INTMUX78,
    _reserved3: [u8; 24usize],
    #[doc = "0x100 - IF1 Command Register"]
    pub if1cmd: IF1CMD,
    #[doc = "0x104 - IF1 Mask Register"]
    pub if1msk: IF1MSK,
    #[doc = "0x108 - IF1 Arbitation Register"]
    pub if1arb: IF1ARB,
    #[doc = "0x10c - IF1 Message Control Register"]
    pub if1mctl: IF1MCTL,
    #[doc = "0x110 - IF1 Data A Register"]
    pub if1data: IF1DATA,
    #[doc = "0x114 - IF1 Data B Register"]
    pub if1datb: IF1DATB,
    _reserved4: [u8; 8usize],
    #[doc = "0x120 - IF2 Command Register"]
    pub if2cmd: IF2CMD,
    #[doc = "0x124 - IF2 Mask Register"]
    pub if2msk: IF2MSK,
    #[doc = "0x128 - IF2 Arbitation Register"]
    pub if2arb: IF2ARB,
    #[doc = "0x12c - IF2 Message Control Register"]
    pub if2mctl: IF2MCTL,
    #[doc = "0x130 - IF2 Data A Register"]
    pub if2data: IF2DATA,
    #[doc = "0x134 - IF2 Data B Register"]
    pub if2datb: IF2DATB,
    _reserved5: [u8; 8usize],
    #[doc = "0x140 - IF3 Observation Register"]
    pub if3obs: IF3OBS,
    #[doc = "0x144 - IF3 Mask Register"]
    pub if3msk: IF3MSK,
    #[doc = "0x148 - IF3 Arbitation Register"]
    pub if3arb: IF3ARB,
    #[doc = "0x14c - IF3 Message Control Register"]
    pub if3mctl: IF3MCTL,
    #[doc = "0x150 - IF3 Data A Register"]
    pub if3data: IF3DATA,
    #[doc = "0x154 - IF3 Data B Register"]
    pub if3datb: IF3DATB,
    _reserved6: [u8; 8usize],
    #[doc = "0x160 - IF3 Update Enable 12 Register"]
    pub if3upd12: IF3UPD12,
    #[doc = "0x164 - IF3 Update Enable 34 Register"]
    pub if3upd34: IF3UPD34,
    #[doc = "0x168 - IF3 Update Enable 56 Register"]
    pub if3upd56: IF3UPD56,
    #[doc = "0x16c - IF3 Update Enable 78 Register"]
    pub if3upd78: IF3UPD78,
    _reserved7: [u8; 112usize],
    #[doc = "0x1e0 - TX IO Control Register"]
    pub tioc: TIOC,
    #[doc = "0x1e4 - RX IO Control Register"]
    pub rioc: RIOC,
}
#[doc = "CAN Control Register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Control Register"]
pub mod ctl;
#[doc = "Error and Status register"]
pub struct ES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error and Status register"]
pub mod es;
#[doc = "Error Counter Register"]
pub struct ERRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Counter Register"]
pub mod errc;
#[doc = "BIT Timing Register"]
pub struct BTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BIT Timing Register"]
pub mod btr;
#[doc = "Interrupt Register"]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register"]
pub mod int;
#[doc = "Test Register"]
pub struct TEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test Register"]
pub mod test;
#[doc = "Parity Error Code Register"]
pub struct PERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Error Code Register"]
pub mod perr;
#[doc = "Core Release Register"]
pub struct REL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core Release Register"]
pub mod rel;
#[doc = "ECC Diagnostic Register"]
pub struct ECCDIAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Diagnostic Register"]
pub mod eccdiag;
#[doc = "ECC Diagnostic Status Register"]
pub struct ECCDIAG_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Diagnostic Status Register"]
pub mod eccdiag_stat;
#[doc = "ECC Control and Status Register"]
pub struct ECC_CS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Control and Status Register"]
pub mod ecc_cs;
#[doc = "ECC Single Bit Error Code Register"]
pub struct ECC_SERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Single Bit Error Code Register"]
pub mod ecc_serr;
#[doc = "Auto-Bus-On Time Register"]
pub struct ABOTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auto-Bus-On Time Register"]
pub mod abotr;
#[doc = "Transmission Request X"]
pub struct TXRQX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission Request X"]
pub mod txrqx;
#[doc = "Transmission Request 12 Register"]
pub struct TXRQ12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission Request 12 Register"]
pub mod txrq12;
#[doc = "Transmission Request 34 Register"]
pub struct TXRQ34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission Request 34 Register"]
pub mod txrq34;
#[doc = "Transmission Request 56 Register"]
pub struct TXRQ56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission Request 56 Register"]
pub mod txrq56;
#[doc = "Transmission Request 78 Register"]
pub struct TXRQ78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission Request 78 Register"]
pub mod txrq78;
#[doc = "New Data X Register"]
pub struct NWDATX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New Data X Register"]
pub mod nwdatx;
#[doc = "New Data 12 Register"]
pub struct NWDAT12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New Data 12 Register"]
pub mod nwdat12;
#[doc = "New Data 34 Register"]
pub struct NWDAT34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New Data 34 Register"]
pub mod nwdat34;
#[doc = "New Data 56 Register"]
pub struct NWDAT56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New Data 56 Register"]
pub mod nwdat56;
#[doc = "New Data 78 Register"]
pub struct NWDAT78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New Data 78 Register"]
pub mod nwdat78;
#[doc = "Interrupt Pending X Register"]
pub struct INTPNDX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Pending X Register"]
pub mod intpndx;
#[doc = "Interrupt Pending 12 Register"]
pub struct INTPND12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Pending 12 Register"]
pub mod intpnd12;
#[doc = "Interrupt Pending 34 Register"]
pub struct INTPND34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Pending 34 Register"]
pub mod intpnd34;
#[doc = "Interrupt Pending 56 Register"]
pub struct INTPND56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Pending 56 Register"]
pub mod intpnd56;
#[doc = "Interrupt Pending 78 Register"]
pub struct INTPND78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Pending 78 Register"]
pub mod intpnd78;
#[doc = "Message Valid X Register"]
pub struct MSGVALX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Valid X Register"]
pub mod msgvalx;
#[doc = "Message Valid 12 Register"]
pub struct MSGVAL12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Valid 12 Register"]
pub mod msgval12;
#[doc = "Message Valid 34 Register"]
pub struct MSGVAL34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Valid 34 Register"]
pub mod msgval34;
#[doc = "Message Valid 56 Register"]
pub struct MSGVAL56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Valid 56 Register"]
pub mod msgval56;
#[doc = "Message Valid 78 Register"]
pub struct MSGVAL78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Valid 78 Register"]
pub mod msgval78;
#[doc = "Interrupt Multiplexer 12 Register"]
pub struct INTMUX12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Multiplexer 12 Register"]
pub mod intmux12;
#[doc = "Interrupt Multiplexer 34 Register"]
pub struct INTMUX34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Multiplexer 34 Register"]
pub mod intmux34;
#[doc = "Interrupt Multiplexer 56 Register"]
pub struct INTMUX56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Multiplexer 56 Register"]
pub mod intmux56;
#[doc = "Interrupt Multiplexer 78 Register"]
pub struct INTMUX78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Multiplexer 78 Register"]
pub mod intmux78;
#[doc = "IF1 Command Register"]
pub struct IF1CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF1 Command Register"]
pub mod if1cmd;
#[doc = "IF1 Mask Register"]
pub struct IF1MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF1 Mask Register"]
pub mod if1msk;
#[doc = "IF1 Arbitation Register"]
pub struct IF1ARB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF1 Arbitation Register"]
pub mod if1arb;
#[doc = "IF1 Message Control Register"]
pub struct IF1MCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF1 Message Control Register"]
pub mod if1mctl;
#[doc = "IF1 Data A Register"]
pub struct IF1DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF1 Data A Register"]
pub mod if1data;
#[doc = "IF1 Data B Register"]
pub struct IF1DATB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF1 Data B Register"]
pub mod if1datb;
#[doc = "IF2 Command Register"]
pub struct IF2CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF2 Command Register"]
pub mod if2cmd;
#[doc = "IF2 Mask Register"]
pub struct IF2MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF2 Mask Register"]
pub mod if2msk;
#[doc = "IF2 Arbitation Register"]
pub struct IF2ARB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF2 Arbitation Register"]
pub mod if2arb;
#[doc = "IF2 Message Control Register"]
pub struct IF2MCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF2 Message Control Register"]
pub mod if2mctl;
#[doc = "IF2 Data A Register"]
pub struct IF2DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF2 Data A Register"]
pub mod if2data;
#[doc = "IF2 Data B Register"]
pub struct IF2DATB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF2 Data B Register"]
pub mod if2datb;
#[doc = "IF3 Observation Register"]
pub struct IF3OBS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF3 Observation Register"]
pub mod if3obs;
#[doc = "IF3 Mask Register"]
pub struct IF3MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF3 Mask Register"]
pub mod if3msk;
#[doc = "IF3 Arbitation Register"]
pub struct IF3ARB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF3 Arbitation Register"]
pub mod if3arb;
#[doc = "IF3 Message Control Register"]
pub struct IF3MCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF3 Message Control Register"]
pub mod if3mctl;
#[doc = "IF3 Data A Register"]
pub struct IF3DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF3 Data A Register"]
pub mod if3data;
#[doc = "IF3 Data B Register"]
pub struct IF3DATB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF3 Data B Register"]
pub mod if3datb;
#[doc = "IF3 Update Enable 12 Register"]
pub struct IF3UPD12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF3 Update Enable 12 Register"]
pub mod if3upd12;
#[doc = "IF3 Update Enable 34 Register"]
pub struct IF3UPD34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF3 Update Enable 34 Register"]
pub mod if3upd34;
#[doc = "IF3 Update Enable 56 Register"]
pub struct IF3UPD56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF3 Update Enable 56 Register"]
pub mod if3upd56;
#[doc = "IF3 Update Enable 78 Register"]
pub struct IF3UPD78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IF3 Update Enable 78 Register"]
pub mod if3upd78;
#[doc = "TX IO Control Register"]
pub struct TIOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX IO Control Register"]
pub mod tioc;
#[doc = "RX IO Control Register"]
pub struct RIOC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX IO Control Register"]
pub mod rioc;
