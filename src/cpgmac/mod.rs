#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Transmit Identification and Version Register"]
    pub revid: REVID,
    #[doc = "0x04 - Software Reset Register"]
    pub softreset: SOFTRESET,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Interrupt Control Register"]
    pub intcontrol: INTCONTROL,
    #[doc = "0x10 - Core 0 Receive Threshold Enable Register"]
    pub c0rxthreshen: C0RXTHRESHEN,
    #[doc = "0x14 - Core 0 Receive Enable Register"]
    pub c0rxen: C0RXEN,
    #[doc = "0x18 - Core 0 Transmit Enable Register"]
    pub c0txen: C0TXEN,
    #[doc = "0x1c - Core 0 Misc Interrupt Enable Register"]
    pub c0miscen: C0MISCEN,
    #[doc = "0x20 - Core 1 Receive Threshold Enable Register"]
    pub c1rxthreshen: C1RXTHRESHEN,
    #[doc = "0x24 - Core 1 Receive Enable Register"]
    pub c1rxen: C1RXEN,
    #[doc = "0x28 - Core 1 Transmit Enable Register"]
    pub c1txen: C1TXEN,
    #[doc = "0x2c - Core 1 Misc Enable Register"]
    pub c1miscen: C1MISCEN,
    #[doc = "0x30 - Core 2 Receive Threshold Enable Register"]
    pub c2rxthreshen: C2RXTHRESHEN,
    #[doc = "0x34 - Core 2 Receive Enable Register"]
    pub c2rxen: C2RXEN,
    #[doc = "0x38 - Core 2 Transmit Enable Register"]
    pub c2txen: C2TXEN,
    #[doc = "0x3c - Core 2 Misc Enable Register"]
    pub c2miscen: C2MISCEN,
    #[doc = "0x40 - Core 0 Receive Threshold Status Register"]
    pub c0rxthreshstat: C0RXTHRESHSTAT,
    #[doc = "0x44 - Core 0 Receive Status Register"]
    pub c0rxstat: C0RXSTAT,
    #[doc = "0x48 - Core 0 Transmit Status Register"]
    pub c0txstat: C0TXSTAT,
    #[doc = "0x4c - Core 0 Misc Interrupt Status Register"]
    pub c0miscstat: C0MISCSTAT,
    #[doc = "0x50 - Core 1 Receive Threshold Status Register"]
    pub c1rxthreshstat: C1RXTHRESHSTAT,
    #[doc = "0x54 - Core 1 Receive Status Register"]
    pub c1rxstat: C1RXSTAT,
    #[doc = "0x58 - Core 1 Transmit Status Register"]
    pub c1txstat: C1TXSTAT,
    #[doc = "0x5c - Core 1 Misc Interrupt Status Register"]
    pub c1miscstat: C1MISCSTAT,
    #[doc = "0x60 - Core 2 Receive Threshold Status Register"]
    pub c2rxthreshstat: C2RXTHRESHSTAT,
    #[doc = "0x64 - Core 2 Receive Status Register"]
    pub c2rxstat: C2RXSTAT,
    #[doc = "0x68 - Core 2 Transmit Status Register"]
    pub c2txstat: C2TXSTAT,
    #[doc = "0x6c - Core 2 Misc Interrupt Status Register"]
    pub c2miscstat: C2MISCSTAT,
    #[doc = "0x70 - Core 0 Receive Interrupts per Millisecond Register"]
    pub c0rximax: C0RXIMAX,
    #[doc = "0x74 - Core 0 Transmit Interrupts per Millisecond Register"]
    pub c0tximax: C0TXIMAX,
    #[doc = "0x78 - Core 1 Receive Interrupts per Millisecond Register"]
    pub c1rximax: C1RXIMAX,
    #[doc = "0x7c - Core 1 Transmit Interrupts per Millisecond Register"]
    pub c1tximax: C1TXIMAX,
    #[doc = "0x80 - Core 2 Receive Interrupts per Millisecond Register"]
    pub c2rximax: C2RXIMAX,
    #[doc = "0x84 - Core 2 Transmit Interrupts per Millisecond Register"]
    pub c2tximax: C2TXIMAX,
}
#[doc = "Transmit Identification and Version Register"]
pub struct REVID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Identification and Version Register"]
pub mod revid;
#[doc = "Software Reset Register"]
pub struct SOFTRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Reset Register"]
pub mod softreset;
#[doc = "Interrupt Control Register"]
pub struct INTCONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Control Register"]
pub mod intcontrol;
#[doc = "Core 0 Receive Threshold Enable Register"]
pub struct C0RXTHRESHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 0 Receive Threshold Enable Register"]
pub mod c0rxthreshen;
#[doc = "Core 0 Receive Enable Register"]
pub struct C0RXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 0 Receive Enable Register"]
pub mod c0rxen;
#[doc = "Core 0 Transmit Enable Register"]
pub struct C0TXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 0 Transmit Enable Register"]
pub mod c0txen;
#[doc = "Core 0 Misc Interrupt Enable Register"]
pub struct C0MISCEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 0 Misc Interrupt Enable Register"]
pub mod c0miscen;
#[doc = "Core 1 Receive Threshold Enable Register"]
pub struct C1RXTHRESHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 1 Receive Threshold Enable Register"]
pub mod c1rxthreshen;
#[doc = "Core 1 Receive Enable Register"]
pub struct C1RXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 1 Receive Enable Register"]
pub mod c1rxen;
#[doc = "Core 1 Transmit Enable Register"]
pub struct C1TXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 1 Transmit Enable Register"]
pub mod c1txen;
#[doc = "Core 1 Misc Enable Register"]
pub struct C1MISCEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 1 Misc Enable Register"]
pub mod c1miscen;
#[doc = "Core 2 Receive Threshold Enable Register"]
pub struct C2RXTHRESHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 2 Receive Threshold Enable Register"]
pub mod c2rxthreshen;
#[doc = "Core 2 Receive Enable Register"]
pub struct C2RXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 2 Receive Enable Register"]
pub mod c2rxen;
#[doc = "Core 2 Transmit Enable Register"]
pub struct C2TXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 2 Transmit Enable Register"]
pub mod c2txen;
#[doc = "Core 2 Misc Enable Register"]
pub struct C2MISCEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 2 Misc Enable Register"]
pub mod c2miscen;
#[doc = "Core 0 Receive Threshold Status Register"]
pub struct C0RXTHRESHSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 0 Receive Threshold Status Register"]
pub mod c0rxthreshstat;
#[doc = "Core 0 Receive Status Register"]
pub struct C0RXSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 0 Receive Status Register"]
pub mod c0rxstat;
#[doc = "Core 0 Transmit Status Register"]
pub struct C0TXSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 0 Transmit Status Register"]
pub mod c0txstat;
#[doc = "Core 0 Misc Interrupt Status Register"]
pub struct C0MISCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 0 Misc Interrupt Status Register"]
pub mod c0miscstat;
#[doc = "Core 1 Receive Threshold Status Register"]
pub struct C1RXTHRESHSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 1 Receive Threshold Status Register"]
pub mod c1rxthreshstat;
#[doc = "Core 1 Receive Status Register"]
pub struct C1RXSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 1 Receive Status Register"]
pub mod c1rxstat;
#[doc = "Core 1 Transmit Status Register"]
pub struct C1TXSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 1 Transmit Status Register"]
pub mod c1txstat;
#[doc = "Core 1 Misc Interrupt Status Register"]
pub struct C1MISCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 1 Misc Interrupt Status Register"]
pub mod c1miscstat;
#[doc = "Core 2 Receive Threshold Status Register"]
pub struct C2RXTHRESHSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 2 Receive Threshold Status Register"]
pub mod c2rxthreshstat;
#[doc = "Core 2 Receive Status Register"]
pub struct C2RXSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 2 Receive Status Register"]
pub mod c2rxstat;
#[doc = "Core 2 Transmit Status Register"]
pub struct C2TXSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 2 Transmit Status Register"]
pub mod c2txstat;
#[doc = "Core 2 Misc Interrupt Status Register"]
pub struct C2MISCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 2 Misc Interrupt Status Register"]
pub mod c2miscstat;
#[doc = "Core 0 Receive Interrupts per Millisecond Register"]
pub struct C0RXIMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 0 Receive Interrupts per Millisecond Register"]
pub mod c0rximax;
#[doc = "Core 0 Transmit Interrupts per Millisecond Register"]
pub struct C0TXIMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 0 Transmit Interrupts per Millisecond Register"]
pub mod c0tximax;
#[doc = "Core 1 Receive Interrupts per Millisecond Register"]
pub struct C1RXIMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 1 Receive Interrupts per Millisecond Register"]
pub mod c1rximax;
#[doc = "Core 1 Transmit Interrupts per Millisecond Register"]
pub struct C1TXIMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 1 Transmit Interrupts per Millisecond Register"]
pub mod c1tximax;
#[doc = "Core 2 Receive Interrupts per Millisecond Register"]
pub struct C2RXIMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 2 Receive Interrupts per Millisecond Register"]
pub mod c2rximax;
#[doc = "Core 2 Transmit Interrupts per Millisecond Register"]
pub struct C2TXIMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core 2 Transmit Interrupts per Millisecond Register"]
pub mod c2tximax;
