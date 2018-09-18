#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Product Identification Communication Controller"]
    pub picc: PICC,
    #[doc = "0x04 - Global Static Number"]
    pub gsn: GSN,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Global Control Set"]
    pub gcs: GCS,
    #[doc = "0x14 - Global Control Reset"]
    pub gcr: GCR,
    #[doc = "0x18 - Transfer Status"]
    pub tscb: TSCB,
    #[doc = "0x1c - Last Transferred Buffer to Communication Controller"]
    pub ltbcc: LTBCC,
    #[doc = "0x20 - Last Transferred Buffer to System Memory"]
    pub ltbsm: LTBSM,
    #[doc = "0x24 - Transfer Base Address"]
    pub tba: TBA,
    #[doc = "0x28 - Next Transfer Base Address"]
    pub ntba: NTBA,
    #[doc = "0x2c - Base Address of Mirrored Status"]
    pub bams: BAMS,
    #[doc = "0x30 - Start Address of Memory Protection"]
    pub samp: SAMP,
    #[doc = "0x34 - End Address of Memory Protection"]
    pub eamp: EAMP,
    _reserved1: [u8; 8usize],
    #[doc = "0x40 - Transfer to System Memory Occurred 1"]
    pub tsmo1: TSMO1,
    #[doc = "0x44 - Transfer to System Memory Occurred 2"]
    pub tsmo2: TSMO2,
    #[doc = "0x48 - Transfer to System Memory Occurred 3"]
    pub tsmo3: TSMO3,
    #[doc = "0x4c - Transfer to System Memory Occurred 4"]
    pub tsmo4: TSMO4,
    #[doc = "0x50 - Transfer to Communication Controller Occurred 1"]
    pub tcco1: TCCO1,
    #[doc = "0x54 - Transfer to Communication Controller Occurred 2"]
    pub tcco2: TCCO2,
    #[doc = "0x58 - Transfer to Communication Controller Occurred 3"]
    pub tcco3: TCCO3,
    #[doc = "0x5c - Transfer to Communication Controller Occurred 4"]
    pub tcco4: TCCO4,
    #[doc = "0x60 - Transfer Occurred Offset"]
    pub tooff: TOOFF,
    _reserved2: [u8; 8usize],
    #[doc = "0x6c - TCR ECC single Bit Error Status"]
    pub tsbestat: TSBESTAT,
    #[doc = "0x70 - ECC Error Address"]
    pub peadr: PEADR,
    #[doc = "0x74 - Transfer Error InterRupt"]
    pub teir: TEIR,
    #[doc = "0x78 - Transfer Error InterRupt Enable Set"]
    pub teires: TEIRES,
    #[doc = "0x7c - Transfer Error InterRupt Enable Reset"]
    pub teirer: TEIRER,
    #[doc = "0x80 - Trigger Transfer to System Memory Set 1"]
    pub ttsms1: TTSMS1,
    #[doc = "0x84 - Trigger Transfer to System Memory Reset 1"]
    pub ttsmr1: TTSMR1,
    #[doc = "0x88 - Trigger Transfer to System Memory Set 2"]
    pub ttsms2: TTSMS2,
    #[doc = "0x8c - Trigger Transfer to System Memory Reset 2"]
    pub ttsmr2: TTSMR2,
    #[doc = "0x90 - Trigger Transfer to System Memory Set 3"]
    pub ttsms3: TTSMS3,
    #[doc = "0x94 - Trigger Transfer to System Memory Reset 3"]
    pub ttsmr3: TTSMR3,
    #[doc = "0x98 - Trigger Transfer to System Memory Set 4"]
    pub ttsms4: TTSMS4,
    #[doc = "0x9c - Trigger Transfer to System Memory Reset 4"]
    pub ttsmr4: TTSMR4,
    #[doc = "0xa0 - Trigger Transfer to Communication Controller Set 1"]
    pub ttccs1: TTCCS1,
    #[doc = "0xa4 - Trigger Transfer to Communication Controller Reset 1"]
    pub ttccr1: TTCCR1,
    #[doc = "0xa8 - Trigger Transfer to Communication Controller Set 2"]
    pub ttccs2: TTCCS2,
    #[doc = "0xac - Trigger Transfer to Communication Controller Reset 2"]
    pub ttccr2: TTCCR2,
    #[doc = "0xb0 - Trigger Transfer to Communication Controller Set 3"]
    pub ttccs3: TTCCS3,
    #[doc = "0xb4 - Trigger Transfer to Communication Controller Reset 3"]
    pub ttccr3: TTCCR3,
    #[doc = "0xb8 - Trigger Transfer to Communication Controller Set 4"]
    pub ttccs4: TTCCS4,
    #[doc = "0xbc - Trigger Transfer to Communication Controller Reset 4"]
    pub ttccr4: TTCCR4,
    #[doc = "0xc0 - Enable Transfer on Event to System Memory Set 1"]
    pub etesms1: ETESMS1,
    #[doc = "0xc4 - Enable Transfer on Event to System Memory Reset 1"]
    pub etesmr1: ETESMR1,
    #[doc = "0xc8 - Enable Transfer on Event to System Memory Set 2"]
    pub etesms2: ETESMS2,
    #[doc = "0xcc - Enable Transfer on Event to System Memory Reset 2"]
    pub etesmr2: ETESMR2,
    #[doc = "0xd0 - Enable Transfer on Event to System Memory Set 3"]
    pub etesms3: ETESMS3,
    #[doc = "0xd4 - Enable Transfer on Event to System Memory Reset 3"]
    pub etesmr3: ETESMR3,
    #[doc = "0xd8 - Enable Transfer on Event to System Memory Set 4"]
    pub etesms4: ETESMS4,
    #[doc = "0xdc - Enable Transfer on Event to System Memory Reset 4"]
    pub etesmr4: ETESMR4,
    #[doc = "0xe0 - Clear on Event to System Memory Set 1"]
    pub cesms1: CESMS1,
    #[doc = "0xe4 - Clear on Event to System Memory Reset 1"]
    pub cesmr1: CESMR1,
    #[doc = "0xe8 - Clear on Event to System Memory Set 2"]
    pub cesms2: CESMS2,
    #[doc = "0xec - Clear on Event to System Memory Reset 2"]
    pub cesmr2: CESMR2,
    #[doc = "0xf0 - Clear on Event to System Memory Set 3"]
    pub cesms3: CESMS3,
    #[doc = "0xf4 - Clear on Event to System Memory Reset 3"]
    pub cesmr3: CESMR3,
    #[doc = "0xf8 - Clear on Event to System Memory Set 4"]
    pub cesms4: CESMS4,
    #[doc = "0xfc - Clear on Event to System Memory Reset 4"]
    pub cesmr4: CESMR4,
    #[doc = "0x100 - Transfer to System Memory Interrupt Enable Set 1"]
    pub tsmies1: TSMIES1,
    #[doc = "0x104 - Transfer to System Memory Interrupt Enable Reset 1"]
    pub tsmier1: TSMIER1,
    #[doc = "0x108 - Transfer to System Memory Interrupt Enable Set 2"]
    pub tsmies2: TSMIES2,
    #[doc = "0x10c - Transfer to System Memory Interrupt Enable Reset 2"]
    pub tsmier2: TSMIER2,
    #[doc = "0x110 - Transfer to System Memory Interrupt Enable Set 3"]
    pub tsmies3: TSMIES3,
    #[doc = "0x114 - Transfer to System Memory Interrupt Enable Reset 3"]
    pub tsmier3: TSMIER3,
    #[doc = "0x118 - Transfer to System Memory Interrupt Enable Set 4"]
    pub tsmies4: TSMIES4,
    #[doc = "0x11c - Transfer to System Memory Interrupt Enable Reset 4"]
    pub tsmier4: TSMIER4,
    #[doc = "0x120 - Transfer to Communication Controller Interrupt Enable Set 1"]
    pub tccies1: TCCIES1,
    #[doc = "0x124 - Transfer to Communication Controller Interrupt Enable Reset 1"]
    pub tccier1: TCCIER1,
    #[doc = "0x128 - Transfer to Communication Controller Interrupt Enable Set 2"]
    pub tccies2: TCCIES2,
    #[doc = "0x12c - Transfer to Communication Controller Interrupt Enable Reset 2"]
    pub tccier2: TCCIER2,
    #[doc = "0x130 - Transfer to Communication Controller Interrupt Enable Set 3"]
    pub tccies3: TCCIES3,
    #[doc = "0x134 - Transfer to Communication Controller Interrupt Enable Reset 3"]
    pub tccier3: TCCIER3,
    #[doc = "0x138 - Transfer to Communication Controller Interrupt Enable Set 4"]
    pub tccies4: TCCIES4,
    #[doc = "0x13c - Transfer to Communication Controller Interrupt Enable Reset 4"]
    pub tccier4: TCCIER4,
}
#[doc = "Product Identification Communication Controller"]
pub struct PICC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Product Identification Communication Controller"]
pub mod picc;
#[doc = "Global Static Number"]
pub struct GSN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Static Number"]
pub mod gsn;
#[doc = "Global Control Set"]
pub struct GCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Set"]
pub mod gcs;
#[doc = "Global Control Reset"]
pub struct GCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Reset"]
pub mod gcr;
#[doc = "Transfer Status"]
pub struct TSCB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Status"]
pub mod tscb;
#[doc = "Last Transferred Buffer to Communication Controller"]
pub struct LTBCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last Transferred Buffer to Communication Controller"]
pub mod ltbcc;
#[doc = "Last Transferred Buffer to System Memory"]
pub struct LTBSM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last Transferred Buffer to System Memory"]
pub mod ltbsm;
#[doc = "Transfer Base Address"]
pub struct TBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Base Address"]
pub mod tba;
#[doc = "Next Transfer Base Address"]
pub struct NTBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Next Transfer Base Address"]
pub mod ntba;
#[doc = "Base Address of Mirrored Status"]
pub struct BAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Address of Mirrored Status"]
pub mod bams;
#[doc = "Start Address of Memory Protection"]
pub struct SAMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start Address of Memory Protection"]
pub mod samp;
#[doc = "End Address of Memory Protection"]
pub struct EAMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End Address of Memory Protection"]
pub mod eamp;
#[doc = "Transfer to System Memory Occurred 1"]
pub struct TSMO1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Occurred 1"]
pub mod tsmo1;
#[doc = "Transfer to System Memory Occurred 2"]
pub struct TSMO2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Occurred 2"]
pub mod tsmo2;
#[doc = "Transfer to System Memory Occurred 3"]
pub struct TSMO3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Occurred 3"]
pub mod tsmo3;
#[doc = "Transfer to System Memory Occurred 4"]
pub struct TSMO4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Occurred 4"]
pub mod tsmo4;
#[doc = "Transfer to Communication Controller Occurred 1"]
pub struct TCCO1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Occurred 1"]
pub mod tcco1;
#[doc = "Transfer to Communication Controller Occurred 2"]
pub struct TCCO2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Occurred 2"]
pub mod tcco2;
#[doc = "Transfer to Communication Controller Occurred 3"]
pub struct TCCO3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Occurred 3"]
pub mod tcco3;
#[doc = "Transfer to Communication Controller Occurred 4"]
pub struct TCCO4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Occurred 4"]
pub mod tcco4;
#[doc = "Transfer Occurred Offset"]
pub struct TOOFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Occurred Offset"]
pub mod tooff;
#[doc = "TCR ECC single Bit Error Status"]
pub struct TSBESTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCR ECC single Bit Error Status"]
pub mod tsbestat;
#[doc = "ECC Error Address"]
pub struct PEADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Error Address"]
pub mod peadr;
#[doc = "Transfer Error InterRupt"]
pub struct TEIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Error InterRupt"]
pub mod teir;
#[doc = "Transfer Error InterRupt Enable Set"]
pub struct TEIRES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Error InterRupt Enable Set"]
pub mod teires;
#[doc = "Transfer Error InterRupt Enable Reset"]
pub struct TEIRER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Error InterRupt Enable Reset"]
pub mod teirer;
#[doc = "Trigger Transfer to System Memory Set 1"]
pub struct TTSMS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to System Memory Set 1"]
pub mod ttsms1;
#[doc = "Trigger Transfer to System Memory Reset 1"]
pub struct TTSMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to System Memory Reset 1"]
pub mod ttsmr1;
#[doc = "Trigger Transfer to System Memory Set 2"]
pub struct TTSMS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to System Memory Set 2"]
pub mod ttsms2;
#[doc = "Trigger Transfer to System Memory Reset 2"]
pub struct TTSMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to System Memory Reset 2"]
pub mod ttsmr2;
#[doc = "Trigger Transfer to System Memory Set 3"]
pub struct TTSMS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to System Memory Set 3"]
pub mod ttsms3;
#[doc = "Trigger Transfer to System Memory Reset 3"]
pub struct TTSMR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to System Memory Reset 3"]
pub mod ttsmr3;
#[doc = "Trigger Transfer to System Memory Set 4"]
pub struct TTSMS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to System Memory Set 4"]
pub mod ttsms4;
#[doc = "Trigger Transfer to System Memory Reset 4"]
pub struct TTSMR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to System Memory Reset 4"]
pub mod ttsmr4;
#[doc = "Trigger Transfer to Communication Controller Set 1"]
pub struct TTCCS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to Communication Controller Set 1"]
pub mod ttccs1;
#[doc = "Trigger Transfer to Communication Controller Reset 1"]
pub struct TTCCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to Communication Controller Reset 1"]
pub mod ttccr1;
#[doc = "Trigger Transfer to Communication Controller Set 2"]
pub struct TTCCS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to Communication Controller Set 2"]
pub mod ttccs2;
#[doc = "Trigger Transfer to Communication Controller Reset 2"]
pub struct TTCCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to Communication Controller Reset 2"]
pub mod ttccr2;
#[doc = "Trigger Transfer to Communication Controller Set 3"]
pub struct TTCCS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to Communication Controller Set 3"]
pub mod ttccs3;
#[doc = "Trigger Transfer to Communication Controller Reset 3"]
pub struct TTCCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to Communication Controller Reset 3"]
pub mod ttccr3;
#[doc = "Trigger Transfer to Communication Controller Set 4"]
pub struct TTCCS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to Communication Controller Set 4"]
pub mod ttccs4;
#[doc = "Trigger Transfer to Communication Controller Reset 4"]
pub struct TTCCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Transfer to Communication Controller Reset 4"]
pub mod ttccr4;
#[doc = "Enable Transfer on Event to System Memory Set 1"]
pub struct ETESMS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Transfer on Event to System Memory Set 1"]
pub mod etesms1;
#[doc = "Enable Transfer on Event to System Memory Reset 1"]
pub struct ETESMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Transfer on Event to System Memory Reset 1"]
pub mod etesmr1;
#[doc = "Enable Transfer on Event to System Memory Set 2"]
pub struct ETESMS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Transfer on Event to System Memory Set 2"]
pub mod etesms2;
#[doc = "Enable Transfer on Event to System Memory Reset 2"]
pub struct ETESMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Transfer on Event to System Memory Reset 2"]
pub mod etesmr2;
#[doc = "Enable Transfer on Event to System Memory Set 3"]
pub struct ETESMS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Transfer on Event to System Memory Set 3"]
pub mod etesms3;
#[doc = "Enable Transfer on Event to System Memory Reset 3"]
pub struct ETESMR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Transfer on Event to System Memory Reset 3"]
pub mod etesmr3;
#[doc = "Enable Transfer on Event to System Memory Set 4"]
pub struct ETESMS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Transfer on Event to System Memory Set 4"]
pub mod etesms4;
#[doc = "Enable Transfer on Event to System Memory Reset 4"]
pub struct ETESMR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Transfer on Event to System Memory Reset 4"]
pub mod etesmr4;
#[doc = "Clear on Event to System Memory Set 1"]
pub struct CESMS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear on Event to System Memory Set 1"]
pub mod cesms1;
#[doc = "Clear on Event to System Memory Reset 1"]
pub struct CESMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear on Event to System Memory Reset 1"]
pub mod cesmr1;
#[doc = "Clear on Event to System Memory Set 2"]
pub struct CESMS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear on Event to System Memory Set 2"]
pub mod cesms2;
#[doc = "Clear on Event to System Memory Reset 2"]
pub struct CESMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear on Event to System Memory Reset 2"]
pub mod cesmr2;
#[doc = "Clear on Event to System Memory Set 3"]
pub struct CESMS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear on Event to System Memory Set 3"]
pub mod cesms3;
#[doc = "Clear on Event to System Memory Reset 3"]
pub struct CESMR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear on Event to System Memory Reset 3"]
pub mod cesmr3;
#[doc = "Clear on Event to System Memory Set 4"]
pub struct CESMS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear on Event to System Memory Set 4"]
pub mod cesms4;
#[doc = "Clear on Event to System Memory Reset 4"]
pub struct CESMR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear on Event to System Memory Reset 4"]
pub mod cesmr4;
#[doc = "Transfer to System Memory Interrupt Enable Set 1"]
pub struct TSMIES1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Interrupt Enable Set 1"]
pub mod tsmies1;
#[doc = "Transfer to System Memory Interrupt Enable Reset 1"]
pub struct TSMIER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Interrupt Enable Reset 1"]
pub mod tsmier1;
#[doc = "Transfer to System Memory Interrupt Enable Set 2"]
pub struct TSMIES2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Interrupt Enable Set 2"]
pub mod tsmies2;
#[doc = "Transfer to System Memory Interrupt Enable Reset 2"]
pub struct TSMIER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Interrupt Enable Reset 2"]
pub mod tsmier2;
#[doc = "Transfer to System Memory Interrupt Enable Set 3"]
pub struct TSMIES3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Interrupt Enable Set 3"]
pub mod tsmies3;
#[doc = "Transfer to System Memory Interrupt Enable Reset 3"]
pub struct TSMIER3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Interrupt Enable Reset 3"]
pub mod tsmier3;
#[doc = "Transfer to System Memory Interrupt Enable Set 4"]
pub struct TSMIES4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Interrupt Enable Set 4"]
pub mod tsmies4;
#[doc = "Transfer to System Memory Interrupt Enable Reset 4"]
pub struct TSMIER4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to System Memory Interrupt Enable Reset 4"]
pub mod tsmier4;
#[doc = "Transfer to Communication Controller Interrupt Enable Set 1"]
pub struct TCCIES1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Interrupt Enable Set 1"]
pub mod tccies1;
#[doc = "Transfer to Communication Controller Interrupt Enable Reset 1"]
pub struct TCCIER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Interrupt Enable Reset 1"]
pub mod tccier1;
#[doc = "Transfer to Communication Controller Interrupt Enable Set 2"]
pub struct TCCIES2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Interrupt Enable Set 2"]
pub mod tccies2;
#[doc = "Transfer to Communication Controller Interrupt Enable Reset 2"]
pub struct TCCIER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Interrupt Enable Reset 2"]
pub mod tccier2;
#[doc = "Transfer to Communication Controller Interrupt Enable Set 3"]
pub struct TCCIES3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Interrupt Enable Set 3"]
pub mod tccies3;
#[doc = "Transfer to Communication Controller Interrupt Enable Reset 3"]
pub struct TCCIER3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Interrupt Enable Reset 3"]
pub mod tccier3;
#[doc = "Transfer to Communication Controller Interrupt Enable Set 4"]
pub struct TCCIES4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Interrupt Enable Set 4"]
pub mod tccies4;
#[doc = "Transfer to Communication Controller Interrupt Enable Reset 4"]
pub struct TCCIER4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer to Communication Controller Interrupt Enable Reset 4"]
pub mod tccier4;
