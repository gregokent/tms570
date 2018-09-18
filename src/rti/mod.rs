#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTI Global Control Register"]
    pub rtigctrl: RTIGCTRL,
    #[doc = "0x04 - RTI Timebase Control Register"]
    pub rtitbctrl: RTITBCTRL,
    #[doc = "0x08 - RTI Capture Control Register"]
    pub rticapctrl: RTICAPCTRL,
    #[doc = "0x0c - RTI Compare Control Register"]
    pub rticompctrl: RTICOMPCTRL,
    #[doc = "0x10 - RTI Free Running Counter 0 Register"]
    pub rtifrc0: RTIFRC0,
    #[doc = "0x14 - RTI Up Counter 0 Register"]
    pub rtiuc0: RTIUC0,
    #[doc = "0x18 - RTI Compare Up Counter 0 Register"]
    pub rticpuc0: RTICPUC0,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - RTI Capture Free Running Counter 0 Register"]
    pub rticafrc0: RTICAFRC0,
    #[doc = "0x24 - RTI Capture Up Counter 0 Register"]
    pub rticauc0: RTICAUC0,
    _reserved1: [u8; 8usize],
    #[doc = "0x30 - RTI Free Running Counter 1 Register"]
    pub rtifrc1: RTIFRC1,
    #[doc = "0x34 - RTI Up Counter 1 Register"]
    pub rtiuc1: RTIUC1,
    #[doc = "0x38 - RTI Compare Up Counter 1 Register"]
    pub rticpuc1: RTICPUC1,
    _reserved2: [u8; 4usize],
    #[doc = "0x40 - RTI Capture Free Running Counter 1 Register"]
    pub rticafrc1: RTICAFRC1,
    #[doc = "0x44 - RTI Capture Up Counter 1 Register"]
    pub rticauc1: RTICAUC1,
    _reserved3: [u8; 8usize],
    #[doc = "0x50 - RTI Compare 0 Register"]
    pub rticomp0: RTICOMP0,
    #[doc = "0x54 - RTI Update Compare 0 Register"]
    pub rtiudcp0: RTIUDCP0,
    #[doc = "0x58 - RTI Compare 1 Register"]
    pub rticomp1: RTICOMP1,
    #[doc = "0x5c - RTI Update Compare 1 Register"]
    pub rtiudcp1: RTIUDCP1,
    #[doc = "0x60 - RTI Compare 2 Register"]
    pub rticomp2: RTICOMP2,
    #[doc = "0x64 - RTI Update Compare 2 Register"]
    pub rtiudcp2: RTIUDCP2,
    #[doc = "0x68 - RTI Compare 3 Register"]
    pub rticomp3: RTICOMP3,
    #[doc = "0x6c - RTI Update Compare 2 Register"]
    pub rtiudcp3: RTIUDCP3,
    #[doc = "0x70 - RTI Timebase Low Compare Register"]
    pub rtitblcomp: RTITBLCOMP,
    #[doc = "0x74 - RTI Timebase High Compare Register"]
    pub rtitbhcomp: RTITBHCOMP,
    _reserved4: [u8; 8usize],
    #[doc = "0x80 - RTI Set Interrupt Register"]
    pub rtisetintena: RTISETINTENA,
    #[doc = "0x84 - RTI Clear Interrupt Enable Register"]
    pub rticlearintena: RTICLEARINTENA,
    #[doc = "0x88 - RTI Interrupt Flag Register"]
    pub rtiintflag: RTIINTFLAG,
    _reserved5: [u8; 4usize],
    #[doc = "0x90 - Digital Watchdog Control Register"]
    pub rtidwdctrl: RTIDWDCTRL,
    #[doc = "0x94 - Digital Watchdog Preload Register"]
    pub rtidwdprld: RTIDWDPRLD,
    #[doc = "0x98 - Watchdog Status Register"]
    pub rtiwdstatus: RTIWDSTATUS,
    #[doc = "0x9c - RTI Watchdog Key Register"]
    pub rtiwdkey: RTIWDKEY,
    #[doc = "0xa0 - RTI Digital Watchdog Down Counter"]
    pub rtidwdcntr: RTIDWDCNTR,
    #[doc = "0xa4 - Digital Windowed Watchdog Reaction Control"]
    pub rtiwwdrxnctrl: RTIWWDRXNCTRL,
    #[doc = "0xa8 - Digital Windowed Watchdog Window Size Control"]
    pub rtiwwdsizectrl: RTIWWDSIZECTRL,
    #[doc = "0xac - RTI Compare Interrupt Clear Enable Register"]
    pub rtiintclrenable: RTIINTCLRENABLE,
    #[doc = "0xb0 - RTI Compare 0 Clear Register"]
    pub rticomp0clr: RTICOMP0CLR,
    #[doc = "0xb4 - RTI Compare 1 Clear Register"]
    pub rticomp1clr: RTICOMP1CLR,
    #[doc = "0xb8 - RTI Compare 2 Clear Register"]
    pub rticomp2clr: RTICOMP2CLR,
    #[doc = "0xbc - RTI Compare 3 Clear Register"]
    pub rticomp3clr: RTICOMP3CLR,
}
#[doc = "RTI Global Control Register"]
pub struct RTIGCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Global Control Register"]
pub mod rtigctrl;
#[doc = "RTI Timebase Control Register"]
pub struct RTITBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Timebase Control Register"]
pub mod rtitbctrl;
#[doc = "RTI Capture Control Register"]
pub struct RTICAPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Capture Control Register"]
pub mod rticapctrl;
#[doc = "RTI Compare Control Register"]
pub struct RTICOMPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare Control Register"]
pub mod rticompctrl;
#[doc = "RTI Free Running Counter 0 Register"]
pub struct RTIFRC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Free Running Counter 0 Register"]
pub mod rtifrc0;
#[doc = "RTI Up Counter 0 Register"]
pub struct RTIUC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Up Counter 0 Register"]
pub mod rtiuc0;
#[doc = "RTI Compare Up Counter 0 Register"]
pub struct RTICPUC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare Up Counter 0 Register"]
pub mod rticpuc0;
#[doc = "RTI Capture Free Running Counter 0 Register"]
pub struct RTICAFRC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Capture Free Running Counter 0 Register"]
pub mod rticafrc0;
#[doc = "RTI Capture Up Counter 0 Register"]
pub struct RTICAUC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Capture Up Counter 0 Register"]
pub mod rticauc0;
#[doc = "RTI Free Running Counter 1 Register"]
pub struct RTIFRC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Free Running Counter 1 Register"]
pub mod rtifrc1;
#[doc = "RTI Up Counter 1 Register"]
pub struct RTIUC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Up Counter 1 Register"]
pub mod rtiuc1;
#[doc = "RTI Compare Up Counter 1 Register"]
pub struct RTICPUC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare Up Counter 1 Register"]
pub mod rticpuc1;
#[doc = "RTI Capture Free Running Counter 1 Register"]
pub struct RTICAFRC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Capture Free Running Counter 1 Register"]
pub mod rticafrc1;
#[doc = "RTI Capture Up Counter 1 Register"]
pub struct RTICAUC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Capture Up Counter 1 Register"]
pub mod rticauc1;
#[doc = "RTI Compare 0 Register"]
pub struct RTICOMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare 0 Register"]
pub mod rticomp0;
#[doc = "RTI Update Compare 0 Register"]
pub struct RTIUDCP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Update Compare 0 Register"]
pub mod rtiudcp0;
#[doc = "RTI Compare 1 Register"]
pub struct RTICOMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare 1 Register"]
pub mod rticomp1;
#[doc = "RTI Update Compare 1 Register"]
pub struct RTIUDCP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Update Compare 1 Register"]
pub mod rtiudcp1;
#[doc = "RTI Compare 2 Register"]
pub struct RTICOMP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare 2 Register"]
pub mod rticomp2;
#[doc = "RTI Update Compare 2 Register"]
pub struct RTIUDCP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Update Compare 2 Register"]
pub mod rtiudcp2;
#[doc = "RTI Compare 3 Register"]
pub struct RTICOMP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare 3 Register"]
pub mod rticomp3;
#[doc = "RTI Update Compare 2 Register"]
pub struct RTIUDCP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Update Compare 2 Register"]
pub mod rtiudcp3;
#[doc = "RTI Timebase Low Compare Register"]
pub struct RTITBLCOMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Timebase Low Compare Register"]
pub mod rtitblcomp;
#[doc = "RTI Timebase High Compare Register"]
pub struct RTITBHCOMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Timebase High Compare Register"]
pub mod rtitbhcomp;
#[doc = "RTI Set Interrupt Register"]
pub struct RTISETINTENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Set Interrupt Register"]
pub mod rtisetintena;
#[doc = "RTI Clear Interrupt Enable Register"]
pub struct RTICLEARINTENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Clear Interrupt Enable Register"]
pub mod rticlearintena;
#[doc = "RTI Interrupt Flag Register"]
pub struct RTIINTFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Interrupt Flag Register"]
pub mod rtiintflag;
#[doc = "Digital Watchdog Control Register"]
pub struct RTIDWDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Watchdog Control Register"]
pub mod rtidwdctrl;
#[doc = "Digital Watchdog Preload Register"]
pub struct RTIDWDPRLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Watchdog Preload Register"]
pub mod rtidwdprld;
#[doc = "Watchdog Status Register"]
pub struct RTIWDSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Status Register"]
pub mod rtiwdstatus;
#[doc = "RTI Watchdog Key Register"]
pub struct RTIWDKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Watchdog Key Register"]
pub mod rtiwdkey;
#[doc = "RTI Digital Watchdog Down Counter"]
pub struct RTIDWDCNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Digital Watchdog Down Counter"]
pub mod rtidwdcntr;
#[doc = "Digital Windowed Watchdog Reaction Control"]
pub struct RTIWWDRXNCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Windowed Watchdog Reaction Control"]
pub mod rtiwwdrxnctrl;
#[doc = "Digital Windowed Watchdog Window Size Control"]
pub struct RTIWWDSIZECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Windowed Watchdog Window Size Control"]
pub mod rtiwwdsizectrl;
#[doc = "RTI Compare Interrupt Clear Enable Register"]
pub struct RTIINTCLRENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare Interrupt Clear Enable Register"]
pub mod rtiintclrenable;
#[doc = "RTI Compare 0 Clear Register"]
pub struct RTICOMP0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare 0 Clear Register"]
pub mod rticomp0clr;
#[doc = "RTI Compare 1 Clear Register"]
pub struct RTICOMP1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare 1 Clear Register"]
pub mod rticomp1clr;
#[doc = "RTI Compare 2 Clear Register"]
pub struct RTICOMP2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare 2 Clear Register"]
pub mod rticomp2clr;
#[doc = "RTI Compare 3 Clear Register"]
pub struct RTICOMP3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Compare 3 Clear Register"]
pub mod rticomp3clr;
