#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Global Control Register 0"]
    pub ctrl0: CTRL0,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - CRC Global Control Register 1"]
    pub ctrl1: CTRL1,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - CRC Global Control Register 2"]
    pub ctrl2: CTRL2,
    _reserved2: [u8; 4usize],
    #[doc = "0x18 - Write one to a bit to enable a interrupt"]
    pub int_set: INTSET,
    _reserved3: [u8; 4usize],
    #[doc = "0x20 - Write one to a bit to disable a interrupt"]
    pub int_clr: INTCLR,
    _reserved4: [u8; 4usize],
    #[doc = "0x28 - CRC Interrupt Status Register"]
    pub int_stat: INTSTAT,
    _reserved5: [u8; 4usize],
    #[doc = "0x30 - CRC Interrupt Offset Register"]
    pub int_offst: INTOFFST,
    _reserved6: [u8; 4usize],
    #[doc = "0x38 - CRC Busy Register"]
    pub busy: BUSY,
    _reserved7: [u8; 4usize],
    #[doc = "0x40 - CRC Pattern Counter Preload Register 1"]
    pub pcount1: PCOUNT1,
    #[doc = "0x44 - CRC Sector Counter Preload Register 1"]
    pub scount1: SCOUNT1,
    #[doc = "0x48 - CRC Current Sector Register 1"]
    pub cur_sec1: CURSEC1,
    #[doc = "0x4c - Watchdog Timeout Preload Register"]
    pub wd_to_pld1: WDTOPLD1,
    #[doc = "0x50 - CRC Channel 1 Block Complete Timeout Preload Register"]
    pub bc_to_pld1: BCTOPLD1,
    _reserved8: [u8; 12usize],
    #[doc = "0x60 - Channel 1 PSA signature low register"]
    pub psa_sig_l1: PSASIGL1,
    #[doc = "0x64 - Channel 1 PSA signature high register"]
    pub psa_sig_h1: PSASIGH1,
    #[doc = "0x68 - Channel 1 CRC value low register"]
    pub crc_val_l1: CRCVALL1,
    #[doc = "0x6c - Channel 1 CRC value high register"]
    pub crc_val_h1: CRCVALH1,
    #[doc = "0x70 - PSA Sector Signature Low Register 1"]
    pub psa_sec_sig_l1: PSASECSIGL1,
    #[doc = "0x74 - PSA Sector Signature High Register 1"]
    pub psa_sec_sig_h1: PSASECSIGH1,
    #[doc = "0x78 - Raw Data Low Register 1"]
    pub raw_data_l1: RAWDATAL1,
    #[doc = "0x7c - Raw Data High Register 1"]
    pub raw_data_h1: RAWDATAH1,
    #[doc = "0x80 - CRC Pattern Counter Preload Register 1"]
    pub pcount2: PCOUNT2,
    #[doc = "0x84 - CRC Sector Counter Preload Register 1"]
    pub scount2: SCOUNT2,
    #[doc = "0x88 - CRC Current Sector Register 1"]
    pub cur_sec2: CURSEC2,
    #[doc = "0x8c - Watchdog Timeout Preload Register"]
    pub wd_to_pld2: WDTOPLD2,
    #[doc = "0x90 - CRC Channel 1 Block Complete Timeout Preload Register"]
    pub bc_to_pld2: BCTOPLD2,
    _reserved9: [u8; 12usize],
    #[doc = "0xa0 - Channel 2 PSA signature low register"]
    pub psa_sig_l2: PSASIGL2,
    #[doc = "0xa4 - Channel 2 PSA signature high register"]
    pub psa_sig_h2: PSASIGH2,
    #[doc = "0xa8 - Channel 2 CRC value low register"]
    pub crc_val_l2: CRCVALL2,
    #[doc = "0xac - Channel 2 CRC value high register"]
    pub crc_val_h2: CRCVALH2,
    #[doc = "0xb0 - PSA Sector Signature Low Register 2"]
    pub psa_sec_sig_l2: PSASECSIGL2,
    #[doc = "0xb4 - PSA Sector Signature High Register 2"]
    pub psa_sec_sig_h2: PSASECSIGH2,
    #[doc = "0xb8 - Raw Data Low Register 2"]
    pub raw_data_l2: RAWDATAL2,
    #[doc = "0xbc - Raw Data High Register 2"]
    pub raw_data_h2: RAWDATAH2,
    #[doc = "0xc0 - CRC Pattern Counter Preload Register 3"]
    pub pcount3: PCOUNT3,
    #[doc = "0xc4 - CRC Sector Counter Preload Register 3"]
    pub scount3: SCOUNT3,
    #[doc = "0xc8 - CRC Current Sector Register 3"]
    pub cur_sec3: CURSEC3,
    #[doc = "0xcc - Watchdog Timeout Preload Register"]
    pub wd_to_pld3: WDTOPLD3,
    #[doc = "0xd0 - CRC Channel 3 Block Complete Timeout Preload Register"]
    pub bc_to_pld3: BCTOPLD3,
    _reserved10: [u8; 12usize],
    #[doc = "0xe0 - Channel 3 PSA signature low register"]
    pub psa_sig_l3: PSASIGL3,
    #[doc = "0xe4 - Channel 3 PSA signature high register"]
    pub psa_sig_h3: PSASIGH3,
    #[doc = "0xe8 - Channel 3 CRC value low register"]
    pub crc_val_l3: CRCVALL3,
    #[doc = "0xec - Channel 3 CRC value high register"]
    pub crc_val_h3: CRCVALH3,
    #[doc = "0xf0 - PSA Sector Signature Low Register 3"]
    pub psa_sec_sig_l3: PSASECSIGL3,
    #[doc = "0xf4 - PSA Sector Signature High Register 3"]
    pub psa_sec_sig_h3: PSASECSIGH3,
    #[doc = "0xf8 - Raw Data Low Register 3"]
    pub raw_data_l3: RAWDATAL3,
    #[doc = "0xfc - Raw Data High Register 3"]
    pub raw_data_h3: RAWDATAH3,
    #[doc = "0x100 - CRC Pattern Counter Preload Register 4"]
    pub pcount4: PCOUNT4,
    #[doc = "0x104 - CRC Sector Counter Preload Register 4"]
    pub scount4: SCOUNT4,
    #[doc = "0x108 - CRC Current Sector Register 4"]
    pub cur_sec4: CURSEC4,
    #[doc = "0x10c - Watchdog Timeout Preload Register"]
    pub wd_to_pld4: WDTOPLD4,
    #[doc = "0x110 - CRC Channel 4 Block Complete Timeout Preload Register"]
    pub bc_to_pld4: BCTOPLD4,
    _reserved11: [u8; 12usize],
    #[doc = "0x120 - Channel 4 PSA signature low register"]
    pub psa_sig_l4: PSASIGL4,
    #[doc = "0x124 - Channel 4 PSA signature high register"]
    pub psa_sig_h4: PSASIGH4,
    #[doc = "0x128 - Channel 4 CRC value low register"]
    pub crc_val_l4: CRCVALL4,
    #[doc = "0x12c - Channel 4 CRC value high register"]
    pub crc_val_h4: CRCVALH4,
    #[doc = "0x130 - PSA Sector Signature Low Register 4"]
    pub psa_sec_sig_l4: PSASECSIGL4,
    #[doc = "0x134 - PSA Sector Signature High Register 4"]
    pub psa_sec_sig_h4: PSASECSIGH4,
    #[doc = "0x138 - Raw Data Low Register 4"]
    pub raw_data_l4: RAWDATAL4,
    #[doc = "0x13c - Raw Data High Register 4"]
    pub raw_data_h4: RAWDATAH4,
}
#[doc = "CRC Global Control Register 0"]
pub struct CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Global Control Register 0"]
pub mod ctrl0;
#[doc = "CRC Global Control Register 1"]
pub struct CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Global Control Register 1"]
pub mod ctrl1;
#[doc = "CRC Global Control Register 2"]
pub struct CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Global Control Register 2"]
pub mod ctrl2;
#[doc = "Write one to a bit to enable a interrupt"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write one to a bit to enable a interrupt"]
pub mod int_set;
#[doc = "Write one to a bit to disable a interrupt"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write one to a bit to disable a interrupt"]
pub mod int_clr;
#[doc = "CRC Interrupt Status Register"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Interrupt Status Register"]
pub mod int_stat;
#[doc = "CRC Interrupt Offset Register"]
pub struct INTOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Interrupt Offset Register"]
pub mod int_offst;
#[doc = "CRC Busy Register"]
pub struct BUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Busy Register"]
pub mod busy;
#[doc = "CRC Pattern Counter Preload Register 1"]
pub struct PCOUNT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Pattern Counter Preload Register 1"]
pub mod pcount1;
#[doc = "CRC Sector Counter Preload Register 1"]
pub struct SCOUNT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Sector Counter Preload Register 1"]
pub mod scount1;
#[doc = "CRC Current Sector Register 1"]
pub struct CURSEC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Current Sector Register 1"]
pub mod cur_sec1;
#[doc = "Watchdog Timeout Preload Register"]
pub struct WDTOPLD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timeout Preload Register"]
pub mod wd_to_pld1;
#[doc = "CRC Channel 1 Block Complete Timeout Preload Register"]
pub struct BCTOPLD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Channel 1 Block Complete Timeout Preload Register"]
pub mod bc_to_pld1;
#[doc = "Channel 1 PSA signature low register"]
pub struct PSASIGL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 PSA signature low register"]
pub mod psa_sig_l1;
#[doc = "Channel 1 PSA signature high register"]
pub struct PSASIGH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 PSA signature high register"]
pub mod psa_sig_h1;
#[doc = "Channel 1 CRC value low register"]
pub struct CRCVALL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 CRC value low register"]
pub mod crc_val_l1;
#[doc = "Channel 1 CRC value high register"]
pub struct CRCVALH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 CRC value high register"]
pub mod crc_val_h1;
#[doc = "PSA Sector Signature Low Register 1"]
pub struct PSASECSIGL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSA Sector Signature Low Register 1"]
pub mod psa_sec_sig_l1;
#[doc = "PSA Sector Signature High Register 1"]
pub struct PSASECSIGH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSA Sector Signature High Register 1"]
pub mod psa_sec_sig_h1;
#[doc = "Raw Data Low Register 1"]
pub struct RAWDATAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Data Low Register 1"]
pub mod raw_data_l1;
#[doc = "Raw Data High Register 1"]
pub struct RAWDATAH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Data High Register 1"]
pub mod raw_data_h1;
#[doc = "CRC Pattern Counter Preload Register 1"]
pub struct PCOUNT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Pattern Counter Preload Register 1"]
pub mod pcount2;
#[doc = "CRC Sector Counter Preload Register 1"]
pub struct SCOUNT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Sector Counter Preload Register 1"]
pub mod scount2;
#[doc = "CRC Current Sector Register 1"]
pub struct CURSEC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Current Sector Register 1"]
pub mod cur_sec2;
#[doc = "Watchdog Timeout Preload Register"]
pub struct WDTOPLD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timeout Preload Register"]
pub mod wd_to_pld2;
#[doc = "CRC Channel 1 Block Complete Timeout Preload Register"]
pub struct BCTOPLD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Channel 1 Block Complete Timeout Preload Register"]
pub mod bc_to_pld2;
#[doc = "Channel 2 PSA signature low register"]
pub struct PSASIGL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 PSA signature low register"]
pub mod psa_sig_l2;
#[doc = "Channel 2 PSA signature high register"]
pub struct PSASIGH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 PSA signature high register"]
pub mod psa_sig_h2;
#[doc = "Channel 2 CRC value low register"]
pub struct CRCVALL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 CRC value low register"]
pub mod crc_val_l2;
#[doc = "Channel 2 CRC value high register"]
pub struct CRCVALH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 CRC value high register"]
pub mod crc_val_h2;
#[doc = "PSA Sector Signature Low Register 2"]
pub struct PSASECSIGL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSA Sector Signature Low Register 2"]
pub mod psa_sec_sig_l2;
#[doc = "PSA Sector Signature High Register 2"]
pub struct PSASECSIGH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSA Sector Signature High Register 2"]
pub mod psa_sec_sig_h2;
#[doc = "Raw Data Low Register 2"]
pub struct RAWDATAL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Data Low Register 2"]
pub mod raw_data_l2;
#[doc = "Raw Data High Register 2"]
pub struct RAWDATAH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Data High Register 2"]
pub mod raw_data_h2;
#[doc = "CRC Pattern Counter Preload Register 3"]
pub struct PCOUNT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Pattern Counter Preload Register 3"]
pub mod pcount3;
#[doc = "CRC Sector Counter Preload Register 3"]
pub struct SCOUNT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Sector Counter Preload Register 3"]
pub mod scount3;
#[doc = "CRC Current Sector Register 3"]
pub struct CURSEC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Current Sector Register 3"]
pub mod cur_sec3;
#[doc = "Watchdog Timeout Preload Register"]
pub struct WDTOPLD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timeout Preload Register"]
pub mod wd_to_pld3;
#[doc = "CRC Channel 3 Block Complete Timeout Preload Register"]
pub struct BCTOPLD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Channel 3 Block Complete Timeout Preload Register"]
pub mod bc_to_pld3;
#[doc = "Channel 3 PSA signature low register"]
pub struct PSASIGL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 PSA signature low register"]
pub mod psa_sig_l3;
#[doc = "Channel 3 PSA signature high register"]
pub struct PSASIGH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 PSA signature high register"]
pub mod psa_sig_h3;
#[doc = "Channel 3 CRC value low register"]
pub struct CRCVALL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 CRC value low register"]
pub mod crc_val_l3;
#[doc = "Channel 3 CRC value high register"]
pub struct CRCVALH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 CRC value high register"]
pub mod crc_val_h3;
#[doc = "PSA Sector Signature Low Register 3"]
pub struct PSASECSIGL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSA Sector Signature Low Register 3"]
pub mod psa_sec_sig_l3;
#[doc = "PSA Sector Signature High Register 3"]
pub struct PSASECSIGH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSA Sector Signature High Register 3"]
pub mod psa_sec_sig_h3;
#[doc = "Raw Data Low Register 3"]
pub struct RAWDATAL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Data Low Register 3"]
pub mod raw_data_l3;
#[doc = "Raw Data High Register 3"]
pub struct RAWDATAH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Data High Register 3"]
pub mod raw_data_h3;
#[doc = "CRC Pattern Counter Preload Register 4"]
pub struct PCOUNT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Pattern Counter Preload Register 4"]
pub mod pcount4;
#[doc = "CRC Sector Counter Preload Register 4"]
pub struct SCOUNT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Sector Counter Preload Register 4"]
pub mod scount4;
#[doc = "CRC Current Sector Register 4"]
pub struct CURSEC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Current Sector Register 4"]
pub mod cur_sec4;
#[doc = "Watchdog Timeout Preload Register"]
pub struct WDTOPLD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timeout Preload Register"]
pub mod wd_to_pld4;
#[doc = "CRC Channel 4 Block Complete Timeout Preload Register"]
pub struct BCTOPLD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Channel 4 Block Complete Timeout Preload Register"]
pub mod bc_to_pld4;
#[doc = "Channel 4 PSA signature low register"]
pub struct PSASIGL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 4 PSA signature low register"]
pub mod psa_sig_l4;
#[doc = "Channel 4 PSA signature high register"]
pub struct PSASIGH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 4 PSA signature high register"]
pub mod psa_sig_h4;
#[doc = "Channel 4 CRC value low register"]
pub struct CRCVALL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 4 CRC value low register"]
pub mod crc_val_l4;
#[doc = "Channel 4 CRC value high register"]
pub struct CRCVALH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 4 CRC value high register"]
pub mod crc_val_h4;
#[doc = "PSA Sector Signature Low Register 4"]
pub struct PSASECSIGL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSA Sector Signature Low Register 4"]
pub mod psa_sec_sig_l4;
#[doc = "PSA Sector Signature High Register 4"]
pub struct PSASECSIGH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSA Sector Signature High Register 4"]
pub mod psa_sec_sig_h4;
#[doc = "Raw Data Low Register 4"]
pub struct RAWDATAL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Data Low Register 4"]
pub mod raw_data_l4;
#[doc = "Raw Data High Register 4"]
pub struct RAWDATAH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Data High Register 4"]
pub mod raw_data_h4;
