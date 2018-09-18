#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Own Address Manager"]
    pub i2coar: I2COAR,
    #[doc = "0x04 - I2C Interrupt Mask Register"]
    pub i2cimr: I2CIMR,
    #[doc = "0x08 - I2C Status Register"]
    pub i2cstr: I2CSTR,
    #[doc = "0x0c - I2C Clock Divider Low Register"]
    pub i2cckl: I2CCKL,
    #[doc = "0x10 - I2C Clock Control High Register"]
    pub i2cckh: I2CCKH,
    #[doc = "0x14 - I2C Data Count Register"]
    pub i2ccnt: I2CCNT,
    #[doc = "0x18 - I2C Data Receive Register"]
    pub i2cdrr: I2CDRR,
    #[doc = "0x1c - I2C Slave Address Register"]
    pub i2csar: I2CSAR,
    #[doc = "0x20 - I2C Data Transmit Register"]
    pub i2cdxr: I2CDXR,
    #[doc = "0x24 - I2C Mode Register"]
    pub i2cmdr: I2CMDR,
    #[doc = "0x28 - I2C Interrupt Vector Register"]
    pub i2civr: I2CIVR,
    #[doc = "0x2c - I2C Extended Mode Register"]
    pub i2cemdr: I2CEMDR,
    #[doc = "0x30 - I2C Prescale Register"]
    pub i2cpsc: I2CPSC,
    #[doc = "0x34 - I2C Peripheral ID Register 1"]
    pub i2cpid1: I2CPID1,
    #[doc = "0x38 - I2C Peripheral ID Register 2"]
    pub i2cpid2: I2CPID2,
    #[doc = "0x3c - I2C DMA Control Register"]
    pub i2cdmacr: I2CDMACR,
    _reserved0: [u8; 8usize],
    #[doc = "0x48 - I2C Pin Function Register"]
    pub i2cpfnc: I2CPFNC,
    #[doc = "0x4c - I2C Pin Direction Register"]
    pub i2cpdir: I2CPDIR,
    #[doc = "0x50 - I2C Data Input Register"]
    pub i2cdin: I2CDIN,
    #[doc = "0x54 - I2C Data Output Register"]
    pub i2cdout: I2CDOUT,
    #[doc = "0x58 - I2C Data Set Register"]
    pub i2cdset: I2CDSET,
    #[doc = "0x5c - I2C Data Clear Register"]
    pub i2cdclr: I2CDCLR,
    #[doc = "0x60 - I2C Pin Open Drain Register"]
    pub i2cpdr: I2CPDR,
    #[doc = "0x64 - I2C Pull Disable Register"]
    pub i2cpdis: I2CPDIS,
    #[doc = "0x68 - I2C Pull Select Register"]
    pub i2cpsel: I2CPSEL,
    #[doc = "0x6c - I2C Pins Slew Rate Select Register"]
    pub i2csrs: I2CSRS,
}
#[doc = "I2C Own Address Manager"]
pub struct I2COAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Own Address Manager"]
pub mod i2coar;
#[doc = "I2C Interrupt Mask Register"]
pub struct I2CIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Interrupt Mask Register"]
pub mod i2cimr;
#[doc = "I2C Status Register"]
pub struct I2CSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Status Register"]
pub mod i2cstr;
#[doc = "I2C Clock Divider Low Register"]
pub struct I2CCKL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Clock Divider Low Register"]
pub mod i2cckl;
#[doc = "I2C Clock Control High Register"]
pub struct I2CCKH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Clock Control High Register"]
pub mod i2cckh;
#[doc = "I2C Data Count Register"]
pub struct I2CCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Data Count Register"]
pub mod i2ccnt;
#[doc = "I2C Data Receive Register"]
pub struct I2CDRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Data Receive Register"]
pub mod i2cdrr;
#[doc = "I2C Slave Address Register"]
pub struct I2CSAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Address Register"]
pub mod i2csar;
#[doc = "I2C Data Transmit Register"]
pub struct I2CDXR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Data Transmit Register"]
pub mod i2cdxr;
#[doc = "I2C Mode Register"]
pub struct I2CMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Mode Register"]
pub mod i2cmdr;
#[doc = "I2C Interrupt Vector Register"]
pub struct I2CIVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Interrupt Vector Register"]
pub mod i2civr;
#[doc = "I2C Extended Mode Register"]
pub struct I2CEMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Extended Mode Register"]
pub mod i2cemdr;
#[doc = "I2C Prescale Register"]
pub struct I2CPSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Prescale Register"]
pub mod i2cpsc;
#[doc = "I2C Peripheral ID Register 1"]
pub struct I2CPID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Peripheral ID Register 1"]
pub mod i2cpid1;
#[doc = "I2C Peripheral ID Register 2"]
pub struct I2CPID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Peripheral ID Register 2"]
pub mod i2cpid2;
#[doc = "I2C DMA Control Register"]
pub struct I2CDMACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C DMA Control Register"]
pub mod i2cdmacr;
#[doc = "I2C Pin Function Register"]
pub struct I2CPFNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Pin Function Register"]
pub mod i2cpfnc;
#[doc = "I2C Pin Direction Register"]
pub struct I2CPDIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Pin Direction Register"]
pub mod i2cpdir;
#[doc = "I2C Data Input Register"]
pub struct I2CDIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Data Input Register"]
pub mod i2cdin;
#[doc = "I2C Data Output Register"]
pub struct I2CDOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Data Output Register"]
pub mod i2cdout;
#[doc = "I2C Data Set Register"]
pub struct I2CDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Data Set Register"]
pub mod i2cdset;
#[doc = "I2C Data Clear Register"]
pub struct I2CDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Data Clear Register"]
pub mod i2cdclr;
#[doc = "I2C Pin Open Drain Register"]
pub struct I2CPDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Pin Open Drain Register"]
pub mod i2cpdr;
#[doc = "I2C Pull Disable Register"]
pub struct I2CPDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Pull Disable Register"]
pub mod i2cpdis;
#[doc = "I2C Pull Select Register"]
pub struct I2CPSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Pull Select Register"]
pub mod i2cpsel;
#[doc = "I2C Pins Slew Rate Select Register"]
pub struct I2CSRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Pins Slew Rate Select Register"]
pub mod i2csrs;
