#![doc = "Peripheral access API for TMS570LC43XX microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(const_fn)]
#![feature(try_from)]
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc(hidden)]
pub mod interrupt;
pub use interrupt::Interrupt;
#[doc = "Analog to Digital Module"]
pub struct MIBADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MIBADC2 {}
impl MIBADC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mib_adc2::RegisterBlock {
        4294427136 as *const _
    }
}
impl Deref for MIBADC2 {
    type Target = mib_adc2::RegisterBlock;
    fn deref(&self) -> &mib_adc2::RegisterBlock {
        unsafe { &*MIBADC2::ptr() }
    }
}
#[doc = "Analog to Digital Module"]
pub mod mib_adc2;
#[doc = "Analog to Digital Module"]
pub struct MIBADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MIBADC1 {}
impl MIBADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mib_adc1::RegisterBlock {
        4294426624 as *const _
    }
}
impl Deref for MIBADC1 {
    type Target = mib_adc1::RegisterBlock;
    fn deref(&self) -> &mib_adc1::RegisterBlock {
        unsafe { &*MIBADC1::ptr() }
    }
}
#[doc = "Analog to Digital Module"]
pub mod mib_adc1;
#[doc = "DCAN"]
pub struct DCAN4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCAN4 {}
impl DCAN4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dcan4::RegisterBlock {
        4294435328 as *const _
    }
}
impl Deref for DCAN4 {
    type Target = dcan4::RegisterBlock;
    fn deref(&self) -> &dcan4::RegisterBlock {
        unsafe { &*DCAN4::ptr() }
    }
}
#[doc = "DCAN"]
pub mod dcan4;
#[doc = "Dcan3"]
pub struct DCAN3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCAN3 {}
impl DCAN3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dcan4::RegisterBlock {
        4294434816 as *const _
    }
}
impl Deref for DCAN3 {
    type Target = dcan4::RegisterBlock;
    fn deref(&self) -> &dcan4::RegisterBlock {
        unsafe { &*DCAN3::ptr() }
    }
}
#[doc = "Dcan2"]
pub struct DCAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCAN2 {}
impl DCAN2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dcan4::RegisterBlock {
        4294434304 as *const _
    }
}
impl Deref for DCAN2 {
    type Target = dcan4::RegisterBlock;
    fn deref(&self) -> &dcan4::RegisterBlock {
        unsafe { &*DCAN2::ptr() }
    }
}
#[doc = "Dcan1"]
pub struct DCAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCAN1 {}
impl DCAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dcan4::RegisterBlock {
        4294433792 as *const _
    }
}
impl Deref for DCAN1 {
    type Target = dcan4::RegisterBlock;
    fn deref(&self) -> &dcan4::RegisterBlock {
        unsafe { &*DCAN1::ptr() }
    }
}
#[doc = "Common Platform Gigabit ethernet MAC"]
pub struct EMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMAC {}
impl EMAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emac::RegisterBlock {
        4244078592 as *const _
    }
}
impl Deref for EMAC {
    type Target = emac::RegisterBlock;
    fn deref(&self) -> &emac::RegisterBlock {
        unsafe { &*EMAC::ptr() }
    }
}
#[doc = "Common Platform Gigabit ethernet MAC"]
pub mod emac;
#[doc = "Common Platform Gigabit ethernet MAC SubSystem RMII"]
pub struct CPGMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPGMAC {}
impl CPGMAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cpgmac::RegisterBlock {
        4244080640 as *const _
    }
}
impl Deref for CPGMAC {
    type Target = cpgmac::RegisterBlock;
    fn deref(&self) -> &cpgmac::RegisterBlock {
        unsafe { &*CPGMAC::ptr() }
    }
}
#[doc = "Common Platform Gigabit ethernet MAC SubSystem RMII"]
pub mod cpgmac;
#[doc = "VBUS MII Management Interface"]
pub struct MDIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MDIO {}
impl MDIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mdio::RegisterBlock {
        4244080896 as *const _
    }
}
impl Deref for MDIO {
    type Target = mdio::RegisterBlock;
    fn deref(&self) -> &mdio::RegisterBlock {
        unsafe { &*MDIO::ptr() }
    }
}
#[doc = "VBUS MII Management Interface"]
pub mod mdio;
#[doc = "EPWM REGS Registers"]
pub struct EPWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EPWM1 {}
impl EPWM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_pwm1::RegisterBlock {
        4244081664 as *const _
    }
}
impl Deref for EPWM1 {
    type Target = e_pwm1::RegisterBlock;
    fn deref(&self) -> &e_pwm1::RegisterBlock {
        unsafe { &*EPWM1::ptr() }
    }
}
#[doc = "EPWM REGS Registers"]
pub mod e_pwm1;
#[doc = "ePWM2"]
pub struct EPWM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EPWM2 {}
impl EPWM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_pwm1::RegisterBlock {
        4244081920 as *const _
    }
}
impl Deref for EPWM2 {
    type Target = e_pwm1::RegisterBlock;
    fn deref(&self) -> &e_pwm1::RegisterBlock {
        unsafe { &*EPWM2::ptr() }
    }
}
#[doc = "ePWM3"]
pub struct EPWM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EPWM3 {}
impl EPWM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_pwm1::RegisterBlock {
        4244082176 as *const _
    }
}
impl Deref for EPWM3 {
    type Target = e_pwm1::RegisterBlock;
    fn deref(&self) -> &e_pwm1::RegisterBlock {
        unsafe { &*EPWM3::ptr() }
    }
}
#[doc = "ePWM4"]
pub struct EPWM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EPWM4 {}
impl EPWM4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_pwm1::RegisterBlock {
        4244082432 as *const _
    }
}
impl Deref for EPWM4 {
    type Target = e_pwm1::RegisterBlock;
    fn deref(&self) -> &e_pwm1::RegisterBlock {
        unsafe { &*EPWM4::ptr() }
    }
}
#[doc = "ePWM5"]
pub struct EPWM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EPWM5 {}
impl EPWM5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_pwm1::RegisterBlock {
        4244082688 as *const _
    }
}
impl Deref for EPWM5 {
    type Target = e_pwm1::RegisterBlock;
    fn deref(&self) -> &e_pwm1::RegisterBlock {
        unsafe { &*EPWM5::ptr() }
    }
}
#[doc = "ePWM6"]
pub struct EPWM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EPWM6 {}
impl EPWM6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_pwm1::RegisterBlock {
        4244082944 as *const _
    }
}
impl Deref for EPWM6 {
    type Target = e_pwm1::RegisterBlock;
    fn deref(&self) -> &e_pwm1::RegisterBlock {
        unsafe { &*EPWM6::ptr() }
    }
}
#[doc = "ePWM7"]
pub struct EPWM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EPWM7 {}
impl EPWM7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_pwm1::RegisterBlock {
        4244083200 as *const _
    }
}
impl Deref for EPWM7 {
    type Target = e_pwm1::RegisterBlock;
    fn deref(&self) -> &e_pwm1::RegisterBlock {
        unsafe { &*EPWM7::ptr() }
    }
}
#[doc = "eCAP Control and Status Registers"]
pub struct ECAP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECAP1 {}
impl ECAP1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_cap1::RegisterBlock {
        4244083456 as *const _
    }
}
impl Deref for ECAP1 {
    type Target = e_cap1::RegisterBlock;
    fn deref(&self) -> &e_cap1::RegisterBlock {
        unsafe { &*ECAP1::ptr() }
    }
}
#[doc = "eCAP Control and Status Registers"]
pub mod e_cap1;
#[doc = "eCAP2"]
pub struct ECAP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECAP2 {}
impl ECAP2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_cap1::RegisterBlock {
        4244083712 as *const _
    }
}
impl Deref for ECAP2 {
    type Target = e_cap1::RegisterBlock;
    fn deref(&self) -> &e_cap1::RegisterBlock {
        unsafe { &*ECAP2::ptr() }
    }
}
#[doc = "eCAP3"]
pub struct ECAP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECAP3 {}
impl ECAP3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_cap1::RegisterBlock {
        4244083968 as *const _
    }
}
impl Deref for ECAP3 {
    type Target = e_cap1::RegisterBlock;
    fn deref(&self) -> &e_cap1::RegisterBlock {
        unsafe { &*ECAP3::ptr() }
    }
}
#[doc = "eCAP4"]
pub struct ECAP4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECAP4 {}
impl ECAP4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_cap1::RegisterBlock {
        4244084224 as *const _
    }
}
impl Deref for ECAP4 {
    type Target = e_cap1::RegisterBlock;
    fn deref(&self) -> &e_cap1::RegisterBlock {
        unsafe { &*ECAP4::ptr() }
    }
}
#[doc = "eCAP5"]
pub struct ECAP5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECAP5 {}
impl ECAP5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_cap1::RegisterBlock {
        4244084480 as *const _
    }
}
impl Deref for ECAP5 {
    type Target = e_cap1::RegisterBlock;
    fn deref(&self) -> &e_cap1::RegisterBlock {
        unsafe { &*ECAP5::ptr() }
    }
}
#[doc = "eCAP6"]
pub struct ECAP6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECAP6 {}
impl ECAP6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_cap1::RegisterBlock {
        4244084736 as *const _
    }
}
impl Deref for ECAP6 {
    type Target = e_cap1::RegisterBlock;
    fn deref(&self) -> &e_cap1::RegisterBlock {
        unsafe { &*ECAP6::ptr() }
    }
}
#[doc = "eQEP Control and Status Registers"]
pub struct EQEP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EQEP1 {}
impl EQEP1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_qep1::RegisterBlock {
        4244084992 as *const _
    }
}
impl Deref for EQEP1 {
    type Target = e_qep1::RegisterBlock;
    fn deref(&self) -> &e_qep1::RegisterBlock {
        unsafe { &*EQEP1::ptr() }
    }
}
#[doc = "eQEP Control and Status Registers"]
pub mod e_qep1;
#[doc = "eQEP2"]
pub struct EQEP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EQEP2 {}
impl EQEP2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const e_qep1::RegisterBlock {
        4244085248 as *const _
    }
}
impl Deref for EQEP2 {
    type Target = e_qep1::RegisterBlock;
    fn deref(&self) -> &e_qep1::RegisterBlock {
        unsafe { &*EQEP2::ptr() }
    }
}
#[doc = "FlexRay"]
pub struct FLEXRAY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXRAY {}
impl FLEXRAY {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flex_ray::RegisterBlock {
        4294428672 as *const _
    }
}
impl Deref for FLEXRAY {
    type Target = flex_ray::RegisterBlock;
    fn deref(&self) -> &flex_ray::RegisterBlock {
        unsafe { &*FLEXRAY::ptr() }
    }
}
#[doc = "FlexRay"]
pub mod flex_ray;
#[doc = "Flexray Transfer Unit"]
pub struct FLEXRAYTU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXRAYTU {}
impl FLEXRAYTU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flex_ray_tu::RegisterBlock {
        4294418432 as *const _
    }
}
impl Deref for FLEXRAYTU {
    type Target = flex_ray_tu::RegisterBlock;
    fn deref(&self) -> &flex_ray_tu::RegisterBlock {
        unsafe { &*FLEXRAYTU::ptr() }
    }
}
#[doc = "Flexray Transfer Unit"]
pub mod flex_ray_tu;
#[doc = "General Purpose Input Output"]
pub struct GIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GIO {}
impl GIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gio::RegisterBlock {
        4294425600 as *const _
    }
}
impl Deref for GIO {
    type Target = gio::RegisterBlock;
    fn deref(&self) -> &gio::RegisterBlock {
        unsafe { &*GIO::ptr() }
    }
}
#[doc = "General Purpose Input Output"]
pub mod gio;
#[doc = "General Purpose Input Output"]
pub struct GIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GIOA {}
impl GIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gio_a::RegisterBlock {
        4294425600 as *const _
    }
}
impl Deref for GIOA {
    type Target = gio_a::RegisterBlock;
    fn deref(&self) -> &gio_a::RegisterBlock {
        unsafe { &*GIOA::ptr() }
    }
}
#[doc = "General Purpose Input Output"]
pub mod gio_a;
#[doc = "General Purpose Input Output"]
pub struct GIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GIOB {}
impl GIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gio_b::RegisterBlock {
        4294425600 as *const _
    }
}
impl Deref for GIOB {
    type Target = gio_b::RegisterBlock;
    fn deref(&self) -> &gio_b::RegisterBlock {
        unsafe { &*GIOB::ptr() }
    }
}
#[doc = "General Purpose Input Output"]
pub mod gio_b;
#[doc = "I2C Module"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        4294431744 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "I2C Module"]
pub mod i2c1;
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        4294432000 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "Platform High End Timer"]
pub struct NHET1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NHET1 {}
impl NHET1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nhet1::RegisterBlock {
        4294424576 as *const _
    }
}
impl Deref for NHET1 {
    type Target = nhet1::RegisterBlock;
    fn deref(&self) -> &nhet1::RegisterBlock {
        unsafe { &*NHET1::ptr() }
    }
}
#[doc = "Platform High End Timer"]
pub mod nhet1;
#[doc = "Nhet2"]
pub struct NHET2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NHET2 {}
impl NHET2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nhet1::RegisterBlock {
        4294424832 as *const _
    }
}
impl Deref for NHET2 {
    type Target = nhet1::RegisterBlock;
    fn deref(&self) -> &nhet1::RegisterBlock {
        unsafe { &*NHET2::ptr() }
    }
}
#[doc = "HET Transfer Unit"]
pub struct HTU1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HTU1 {}
impl HTU1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const htu1::RegisterBlock {
        4294419456 as *const _
    }
}
impl Deref for HTU1 {
    type Target = htu1::RegisterBlock;
    fn deref(&self) -> &htu1::RegisterBlock {
        unsafe { &*HTU1::ptr() }
    }
}
#[doc = "HET Transfer Unit"]
pub mod htu1;
#[doc = "Htu2"]
pub struct HTU2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HTU2 {}
impl HTU2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const htu1::RegisterBlock {
        4294419712 as *const _
    }
}
impl Deref for HTU2 {
    type Target = htu1::RegisterBlock;
    fn deref(&self) -> &htu1::RegisterBlock {
        unsafe { &*HTU2::ptr() }
    }
}
#[doc = "IOMM Multiplexing Control Module for TMS570LC44xx"]
pub struct IOMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOMM {}
impl IOMM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iomm::RegisterBlock {
        4294908928 as *const _
    }
}
impl Deref for IOMM {
    type Target = iomm::RegisterBlock;
    fn deref(&self) -> &iomm::RegisterBlock {
        unsafe { &*IOMM::ptr() }
    }
}
#[doc = "IOMM Multiplexing Control Module for TMS570LC44xx"]
pub mod iomm;
#[doc = "Multibuffered Serial Peripheral Interface"]
pub struct MIBSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MIBSPI1 {}
impl MIBSPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mib_spi1::RegisterBlock {
        4294439936 as *const _
    }
}
impl Deref for MIBSPI1 {
    type Target = mib_spi1::RegisterBlock;
    fn deref(&self) -> &mib_spi1::RegisterBlock {
        unsafe { &*MIBSPI1::ptr() }
    }
}
#[doc = "Multibuffered Serial Peripheral Interface"]
pub mod mib_spi1;
#[doc = "MibSpi2"]
pub struct MIBSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MIBSPI2 {}
impl MIBSPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mib_spi1::RegisterBlock {
        4294440448 as *const _
    }
}
impl Deref for MIBSPI2 {
    type Target = mib_spi1::RegisterBlock;
    fn deref(&self) -> &mib_spi1::RegisterBlock {
        unsafe { &*MIBSPI2::ptr() }
    }
}
#[doc = "MibSpi3"]
pub struct MIBSPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MIBSPI3 {}
impl MIBSPI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mib_spi1::RegisterBlock {
        4294440960 as *const _
    }
}
impl Deref for MIBSPI3 {
    type Target = mib_spi1::RegisterBlock;
    fn deref(&self) -> &mib_spi1::RegisterBlock {
        unsafe { &*MIBSPI3::ptr() }
    }
}
#[doc = "MibSpi4"]
pub struct MIBSPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MIBSPI4 {}
impl MIBSPI4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mib_spi1::RegisterBlock {
        4294441472 as *const _
    }
}
impl Deref for MIBSPI4 {
    type Target = mib_spi1::RegisterBlock;
    fn deref(&self) -> &mib_spi1::RegisterBlock {
        unsafe { &*MIBSPI4::ptr() }
    }
}
#[doc = "MibSpip5"]
pub struct MIBSPIP5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MIBSPIP5 {}
impl MIBSPIP5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mib_spi1::RegisterBlock {
        4294441984 as *const _
    }
}
impl Deref for MIBSPIP5 {
    type Target = mib_spi1::RegisterBlock;
    fn deref(&self) -> &mib_spi1::RegisterBlock {
        unsafe { &*MIBSPIP5::ptr() }
    }
}
#[doc = "Serial Communications Interface"]
pub struct LIN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LIN2 {}
impl LIN2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lin2::RegisterBlock {
        4294436352 as *const _
    }
}
impl Deref for LIN2 {
    type Target = lin2::RegisterBlock;
    fn deref(&self) -> &lin2::RegisterBlock {
        unsafe { &*LIN2::ptr() }
    }
}
#[doc = "Serial Communications Interface"]
pub mod lin2;
#[doc = "Lin1"]
pub struct LIN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LIN1 {}
impl LIN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lin2::RegisterBlock {
        4294435840 as *const _
    }
}
impl Deref for LIN1 {
    type Target = lin2::RegisterBlock;
    fn deref(&self) -> &lin2::RegisterBlock {
        unsafe { &*LIN1::ptr() }
    }
}
#[doc = "Sci4"]
pub struct SCI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCI4 {}
impl SCI4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lin2::RegisterBlock {
        4294436608 as *const _
    }
}
impl Deref for SCI4 {
    type Target = lin2::RegisterBlock;
    fn deref(&self) -> &lin2::RegisterBlock {
        unsafe { &*SCI4::ptr() }
    }
}
#[doc = "Sci3"]
pub struct SCI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCI3 {}
impl SCI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lin2::RegisterBlock {
        4294436096 as *const _
    }
}
impl Deref for SCI3 {
    type Target = lin2::RegisterBlock;
    fn deref(&self) -> &lin2::RegisterBlock {
        unsafe { &*SCI3::ptr() }
    }
}
#[doc = "CPU Compare Module for Cortex-R5"]
pub struct CCMR5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCMR5 {}
impl CCMR5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccm_r5::RegisterBlock {
        4294964736 as *const _
    }
}
impl Deref for CCMR5 {
    type Target = ccm_r5::RegisterBlock;
    fn deref(&self) -> &ccm_r5::RegisterBlock {
        unsafe { &*CCMR5::ptr() }
    }
}
#[doc = "CPU Compare Module for Cortex-R5"]
pub mod ccm_r5;
#[doc = "Cyclic Redundancy Check"]
pub struct CRC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC1 {}
impl CRC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc1::RegisterBlock {
        4261412864 as *const _
    }
}
impl Deref for CRC1 {
    type Target = crc1::RegisterBlock;
    fn deref(&self) -> &crc1::RegisterBlock {
        unsafe { &*CRC1::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check"]
pub mod crc1;
#[doc = "Crc2"]
pub struct CRC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC2 {}
impl CRC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc1::RegisterBlock {
        4261412864 as *const _
    }
}
impl Deref for CRC2 {
    type Target = crc1::RegisterBlock;
    fn deref(&self) -> &crc1::RegisterBlock {
        unsafe { &*CRC2::ptr() }
    }
}
#[doc = "Dual Clock Comparator"]
pub struct DCC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCC1 {}
impl DCC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dcc1::RegisterBlock {
        4294962176 as *const _
    }
}
impl Deref for DCC1 {
    type Target = dcc1::RegisterBlock;
    fn deref(&self) -> &dcc1::RegisterBlock {
        unsafe { &*DCC1::ptr() }
    }
}
#[doc = "Dual Clock Comparator"]
pub mod dcc1;
#[doc = "Dcc2"]
pub struct DCC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCC2 {}
impl DCC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dcc1::RegisterBlock {
        4294964224 as *const _
    }
}
impl Deref for DCC2 {
    type Target = dcc1::RegisterBlock;
    fn deref(&self) -> &dcc1::RegisterBlock {
        unsafe { &*DCC2::ptr() }
    }
}
#[doc = "Direct Memory Access"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma::RegisterBlock {
        4294963200 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "Direct Memory Access"]
pub mod dma;
#[doc = "Data Modification Module"]
pub struct DMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMM {}
impl DMM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmm::RegisterBlock {
        4294964992 as *const _
    }
}
impl Deref for DMM {
    type Target = dmm::RegisterBlock;
    fn deref(&self) -> &dmm::RegisterBlock {
        unsafe { &*DMM::ptr() }
    }
}
#[doc = "Data Modification Module"]
pub mod dmm;
#[doc = "Error Signaling Module"]
pub struct ESM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ESM1 {}
impl ESM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const esm1::RegisterBlock {
        4294964480 as *const _
    }
}
impl Deref for ESM1 {
    type Target = esm1::RegisterBlock;
    fn deref(&self) -> &esm1::RegisterBlock {
        unsafe { &*ESM1::ptr() }
    }
}
#[doc = "Error Signaling Module"]
pub mod esm1;
#[doc = "Esm2"]
pub struct ESM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ESM2 {}
impl ESM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const esm1::RegisterBlock {
        4294961408 as *const _
    }
}
impl Deref for ESM2 {
    type Target = esm1::RegisterBlock;
    fn deref(&self) -> &esm1::RegisterBlock {
        unsafe { &*ESM2::ptr() }
    }
}
#[doc = "L2 Flash Memory Controller"]
pub struct FLASHWRAPPER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASHWRAPPER {}
impl FLASHWRAPPER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash_wrapper::RegisterBlock {
        4294471680 as *const _
    }
}
impl Deref for FLASHWRAPPER {
    type Target = flash_wrapper::RegisterBlock;
    fn deref(&self) -> &flash_wrapper::RegisterBlock {
        unsafe { &*FLASHWRAPPER::ptr() }
    }
}
#[doc = "L2 Flash Memory Controller"]
pub mod flash_wrapper;
#[doc = "ProgRammable Built-In Self Test"]
pub struct PBIST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PBIST {}
impl PBIST {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pbist::RegisterBlock {
        4294960128 as *const _
    }
}
impl Deref for PBIST {
    type Target = pbist::RegisterBlock;
    fn deref(&self) -> &pbist::RegisterBlock {
        unsafe { &*PBIST::ptr() }
    }
}
#[doc = "ProgRammable Built-In Self Test"]
pub mod pbist;
#[doc = "Power Management Module"]
pub struct PMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMM {}
impl PMM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmm::RegisterBlock {
        4294901760 as *const _
    }
}
impl Deref for PMM {
    type Target = pmm::RegisterBlock;
    fn deref(&self) -> &pmm::RegisterBlock {
        unsafe { &*PMM::ptr() }
    }
}
#[doc = "Power Management Module"]
pub mod pmm;
#[doc = "Ram Trace Port"]
pub struct RTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTP {}
impl RTP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtp::RegisterBlock {
        4294965760 as *const _
    }
}
impl Deref for RTP {
    type Target = rtp::RegisterBlock;
    fn deref(&self) -> &rtp::RegisterBlock {
        unsafe { &*RTP::ptr() }
    }
}
#[doc = "Ram Trace Port"]
pub mod rtp;
#[doc = "Real Time Interrupt"]
pub struct RTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTI {}
impl RTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rti::RegisterBlock {
        4294966272 as *const _
    }
}
impl Deref for RTI {
    type Target = rti::RegisterBlock;
    fn deref(&self) -> &rti::RegisterBlock {
        unsafe { &*RTI::ptr() }
    }
}
#[doc = "Real Time Interrupt"]
pub mod rti;
#[doc = "Self Test Controller"]
pub struct STC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STC1 {}
impl STC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const stc1::RegisterBlock {
        4294960640 as *const _
    }
}
impl Deref for STC1 {
    type Target = stc1::RegisterBlock;
    fn deref(&self) -> &stc1::RegisterBlock {
        unsafe { &*STC1::ptr() }
    }
}
#[doc = "Self Test Controller"]
pub mod stc1;
#[doc = "Stc2"]
pub struct STC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STC2 {}
impl STC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const stc1::RegisterBlock {
        4294903808 as *const _
    }
}
impl Deref for STC2 {
    type Target = stc1::RegisterBlock;
    fn deref(&self) -> &stc1::RegisterBlock {
        unsafe { &*STC2::ptr() }
    }
}
#[doc = "SCR Control Module"]
pub struct SCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCM {}
impl SCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scm::RegisterBlock {
        4294904320 as *const _
    }
}
impl Deref for SCM {
    type Target = scm::RegisterBlock;
    fn deref(&self) -> &scm::RegisterBlock {
        unsafe { &*SCM::ptr() }
    }
}
#[doc = "SCR Control Module"]
pub mod scm;
#[doc = "Error Profiling Controller"]
pub struct EPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EPC {}
impl EPC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const epc::RegisterBlock {
        4294904832 as *const _
    }
}
impl Deref for EPC {
    type Target = epc::RegisterBlock;
    fn deref(&self) -> &epc::RegisterBlock {
        unsafe { &*EPC::ptr() }
    }
}
#[doc = "Error Profiling Controller"]
pub mod epc;
#[doc = "CPU Subsystem Interconnect for Conqueror device"]
pub struct INTERCONNECT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INTERCONNECT {}
impl INTERCONNECT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const interconnect::RegisterBlock {
        4194304000 as *const _
    }
}
impl Deref for INTERCONNECT {
    type Target = interconnect::RegisterBlock;
    fn deref(&self) -> &interconnect::RegisterBlock {
        unsafe { &*INTERCONNECT::ptr() }
    }
}
#[doc = "CPU Subsystem Interconnect for Conqueror device"]
pub mod interconnect;
#[doc = "Enhanced Memory Protection Unit"]
pub struct NMPU1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NMPU1 {}
impl NMPU1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nmpu1::RegisterBlock {
        4294907904 as *const _
    }
}
impl Deref for NMPU1 {
    type Target = nmpu1::RegisterBlock;
    fn deref(&self) -> &nmpu1::RegisterBlock {
        unsafe { &*NMPU1::ptr() }
    }
}
#[doc = "Enhanced Memory Protection Unit"]
pub mod nmpu1;
#[doc = "Nmpu2"]
pub struct NMPU2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NMPU2 {}
impl NMPU2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nmpu1::RegisterBlock {
        4294908416 as *const _
    }
}
impl Deref for NMPU2 {
    type Target = nmpu1::RegisterBlock;
    fn deref(&self) -> &nmpu1::RegisterBlock {
        unsafe { &*NMPU2::ptr() }
    }
}
#[doc = "Nmpu3"]
pub struct NMPU3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NMPU3 {}
impl NMPU3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nmpu1::RegisterBlock {
        4244576256 as *const _
    }
}
impl Deref for NMPU3 {
    type Target = nmpu1::RegisterBlock;
    fn deref(&self) -> &nmpu1::RegisterBlock {
        unsafe { &*NMPU3::ptr() }
    }
}
#[doc = "System Modules"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sys::RegisterBlock {
        4294967040 as *const _
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    fn deref(&self) -> &sys::RegisterBlock {
        unsafe { &*SYS::ptr() }
    }
}
#[doc = "System Modules"]
pub mod sys;
#[doc = "R4 System Registers Second Frame"]
pub struct SYS2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS2 {}
impl SYS2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sys2::RegisterBlock {
        4294959360 as *const _
    }
}
impl Deref for SYS2 {
    type Target = sys2::RegisterBlock;
    fn deref(&self) -> &sys2::RegisterBlock {
        unsafe { &*SYS2::ptr() }
    }
}
#[doc = "R4 System Registers Second Frame"]
pub mod sys2;
#[doc = "Vectored Interrupt Manager"]
pub struct VIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VIM {}
impl VIM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vim::RegisterBlock {
        4294966784 as *const _
    }
}
impl Deref for VIM {
    type Target = vim::RegisterBlock;
    fn deref(&self) -> &vim::RegisterBlock {
        unsafe { &*VIM::ptr() }
    }
}
#[doc = "Vectored Interrupt Manager"]
pub mod vim;
#[doc = "Vectored Interrupt Manager"]
pub struct VIM_BASE2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VIM_BASE2 {}
impl VIM_BASE2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vim_base2::RegisterBlock {
        4294966528 as *const _
    }
}
impl Deref for VIM_BASE2 {
    type Target = vim_base2::RegisterBlock;
    fn deref(&self) -> &vim_base2::RegisterBlock {
        unsafe { &*VIM_BASE2::ptr() }
    }
}
#[doc = "Vectored Interrupt Manager"]
pub mod vim_base2;
#[doc = "External Memory Interface"]
pub struct EMIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMIF {}
impl EMIF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emif::RegisterBlock {
        4244629504 as *const _
    }
}
impl Deref for EMIF {
    type Target = emif::RegisterBlock;
    fn deref(&self) -> &emif::RegisterBlock {
        unsafe { &*EMIF::ptr() }
    }
}
#[doc = "External Memory Interface"]
pub mod emif;
#[doc = "Pcr3"]
pub struct PCR3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCR3 {}
impl PCR3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pcr3::RegisterBlock {
        4294410240 as *const _
    }
}
impl Deref for PCR3 {
    type Target = pcr3::RegisterBlock;
    fn deref(&self) -> &pcr3::RegisterBlock {
        unsafe { &*PCR3::ptr() }
    }
}
#[doc = "Pcr3"]
pub mod pcr3;
#[doc = "Pcr2"]
pub struct PCR2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCR2 {}
impl PCR2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pcr3::RegisterBlock {
        4244574208 as *const _
    }
}
impl Deref for PCR2 {
    type Target = pcr3::RegisterBlock;
    fn deref(&self) -> &pcr3::RegisterBlock {
        unsafe { &*PCR2::ptr() }
    }
}
#[doc = "Pcr1"]
pub struct PCR1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCR1 {}
impl PCR1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pcr3::RegisterBlock {
        4294905856 as *const _
    }
}
impl Deref for PCR1 {
    type Target = pcr3::RegisterBlock;
    fn deref(&self) -> &pcr3::RegisterBlock {
        unsafe { &*PCR1::ptr() }
    }
}
#[doc = "L2 RAM Wrapper"]
pub struct L2RAMW {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for L2RAMW {}
impl L2RAMW {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const l2ramw::RegisterBlock {
        4294965504 as *const _
    }
}
impl Deref for L2RAMW {
    type Target = l2ramw::RegisterBlock;
    fn deref(&self) -> &l2ramw::RegisterBlock {
        unsafe { &*L2RAMW::ptr() }
    }
}
#[doc = "L2 RAM Wrapper"]
pub mod l2ramw;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "MIBADC2"]
    pub MIBADC2: MIBADC2,
    #[doc = "MIBADC1"]
    pub MIBADC1: MIBADC1,
    #[doc = "DCAN4"]
    pub DCAN4: DCAN4,
    #[doc = "DCAN3"]
    pub DCAN3: DCAN3,
    #[doc = "DCAN2"]
    pub DCAN2: DCAN2,
    #[doc = "DCAN1"]
    pub DCAN1: DCAN1,
    #[doc = "EMAC"]
    pub EMAC: EMAC,
    #[doc = "CPGMAC"]
    pub CPGMAC: CPGMAC,
    #[doc = "MDIO"]
    pub MDIO: MDIO,
    #[doc = "EPWM1"]
    pub EPWM1: EPWM1,
    #[doc = "EPWM2"]
    pub EPWM2: EPWM2,
    #[doc = "EPWM3"]
    pub EPWM3: EPWM3,
    #[doc = "EPWM4"]
    pub EPWM4: EPWM4,
    #[doc = "EPWM5"]
    pub EPWM5: EPWM5,
    #[doc = "EPWM6"]
    pub EPWM6: EPWM6,
    #[doc = "EPWM7"]
    pub EPWM7: EPWM7,
    #[doc = "ECAP1"]
    pub ECAP1: ECAP1,
    #[doc = "ECAP2"]
    pub ECAP2: ECAP2,
    #[doc = "ECAP3"]
    pub ECAP3: ECAP3,
    #[doc = "ECAP4"]
    pub ECAP4: ECAP4,
    #[doc = "ECAP5"]
    pub ECAP5: ECAP5,
    #[doc = "ECAP6"]
    pub ECAP6: ECAP6,
    #[doc = "EQEP1"]
    pub EQEP1: EQEP1,
    #[doc = "EQEP2"]
    pub EQEP2: EQEP2,
    #[doc = "FLEXRAY"]
    pub FLEXRAY: FLEXRAY,
    #[doc = "FLEXRAYTU"]
    pub FLEXRAYTU: FLEXRAYTU,
    #[doc = "GIO"]
    pub GIO: GIO,
    #[doc = "GIOA"]
    pub GIOA: GIOA,
    #[doc = "GIOB"]
    pub GIOB: GIOB,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "NHET1"]
    pub NHET1: NHET1,
    #[doc = "NHET2"]
    pub NHET2: NHET2,
    #[doc = "HTU1"]
    pub HTU1: HTU1,
    #[doc = "HTU2"]
    pub HTU2: HTU2,
    #[doc = "IOMM"]
    pub IOMM: IOMM,
    #[doc = "MIBSPI1"]
    pub MIBSPI1: MIBSPI1,
    #[doc = "MIBSPI2"]
    pub MIBSPI2: MIBSPI2,
    #[doc = "MIBSPI3"]
    pub MIBSPI3: MIBSPI3,
    #[doc = "MIBSPI4"]
    pub MIBSPI4: MIBSPI4,
    #[doc = "MIBSPIP5"]
    pub MIBSPIP5: MIBSPIP5,
    #[doc = "LIN2"]
    pub LIN2: LIN2,
    #[doc = "LIN1"]
    pub LIN1: LIN1,
    #[doc = "SCI4"]
    pub SCI4: SCI4,
    #[doc = "SCI3"]
    pub SCI3: SCI3,
    #[doc = "CCMR5"]
    pub CCMR5: CCMR5,
    #[doc = "CRC1"]
    pub CRC1: CRC1,
    #[doc = "CRC2"]
    pub CRC2: CRC2,
    #[doc = "DCC1"]
    pub DCC1: DCC1,
    #[doc = "DCC2"]
    pub DCC2: DCC2,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "DMM"]
    pub DMM: DMM,
    #[doc = "ESM1"]
    pub ESM1: ESM1,
    #[doc = "ESM2"]
    pub ESM2: ESM2,
    #[doc = "FLASHWRAPPER"]
    pub FLASHWRAPPER: FLASHWRAPPER,
    #[doc = "PBIST"]
    pub PBIST: PBIST,
    #[doc = "PMM"]
    pub PMM: PMM,
    #[doc = "RTP"]
    pub RTP: RTP,
    #[doc = "RTI"]
    pub RTI: RTI,
    #[doc = "STC1"]
    pub STC1: STC1,
    #[doc = "STC2"]
    pub STC2: STC2,
    #[doc = "SCM"]
    pub SCM: SCM,
    #[doc = "EPC"]
    pub EPC: EPC,
    #[doc = "INTERCONNECT"]
    pub INTERCONNECT: INTERCONNECT,
    #[doc = "NMPU1"]
    pub NMPU1: NMPU1,
    #[doc = "NMPU2"]
    pub NMPU2: NMPU2,
    #[doc = "NMPU3"]
    pub NMPU3: NMPU3,
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "SYS2"]
    pub SYS2: SYS2,
    #[doc = "VIM"]
    pub VIM: VIM,
    #[doc = "VIM_BASE2"]
    pub VIM_BASE2: VIM_BASE2,
    #[doc = "EMIF"]
    pub EMIF: EMIF,
    #[doc = "PCR3"]
    pub PCR3: PCR3,
    #[doc = "PCR2"]
    pub PCR2: PCR2,
    #[doc = "PCR1"]
    pub PCR1: PCR1,
    #[doc = "L2RAMW"]
    pub L2RAMW: L2RAMW,
}
impl Peripherals {
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            MIBADC2: MIBADC2 { _marker: PhantomData },
            MIBADC1: MIBADC1 { _marker: PhantomData },
            DCAN4: DCAN4 { _marker: PhantomData },
            DCAN3: DCAN3 { _marker: PhantomData },
            DCAN2: DCAN2 { _marker: PhantomData },
            DCAN1: DCAN1 { _marker: PhantomData },
            EMAC: EMAC { _marker: PhantomData },
            CPGMAC: CPGMAC { _marker: PhantomData },
            MDIO: MDIO { _marker: PhantomData },
            EPWM1: EPWM1 { _marker: PhantomData },
            EPWM2: EPWM2 { _marker: PhantomData },
            EPWM3: EPWM3 { _marker: PhantomData },
            EPWM4: EPWM4 { _marker: PhantomData },
            EPWM5: EPWM5 { _marker: PhantomData },
            EPWM6: EPWM6 { _marker: PhantomData },
            EPWM7: EPWM7 { _marker: PhantomData },
            ECAP1: ECAP1 { _marker: PhantomData },
            ECAP2: ECAP2 { _marker: PhantomData },
            ECAP3: ECAP3 { _marker: PhantomData },
            ECAP4: ECAP4 { _marker: PhantomData },
            ECAP5: ECAP5 { _marker: PhantomData },
            ECAP6: ECAP6 { _marker: PhantomData },
            EQEP1: EQEP1 { _marker: PhantomData },
            EQEP2: EQEP2 { _marker: PhantomData },
            FLEXRAY: FLEXRAY { _marker: PhantomData },
            FLEXRAYTU: FLEXRAYTU { _marker: PhantomData },
            GIO: GIO { _marker: PhantomData },
            GIOA: GIOA { _marker: PhantomData },
            GIOB: GIOB { _marker: PhantomData },
            I2C1: I2C1 { _marker: PhantomData },
            I2C2: I2C2 { _marker: PhantomData },
            NHET1: NHET1 { _marker: PhantomData },
            NHET2: NHET2 { _marker: PhantomData },
            HTU1: HTU1 { _marker: PhantomData },
            HTU2: HTU2 { _marker: PhantomData },
            IOMM: IOMM { _marker: PhantomData },
            MIBSPI1: MIBSPI1 { _marker: PhantomData },
            MIBSPI2: MIBSPI2 { _marker: PhantomData },
            MIBSPI3: MIBSPI3 { _marker: PhantomData },
            MIBSPI4: MIBSPI4 { _marker: PhantomData },
            MIBSPIP5: MIBSPIP5 { _marker: PhantomData },
            LIN2: LIN2 { _marker: PhantomData },
            LIN1: LIN1 { _marker: PhantomData },
            SCI4: SCI4 { _marker: PhantomData },
            SCI3: SCI3 { _marker: PhantomData },
            CCMR5: CCMR5 { _marker: PhantomData },
            CRC1: CRC1 { _marker: PhantomData },
            CRC2: CRC2 { _marker: PhantomData },
            DCC1: DCC1 { _marker: PhantomData },
            DCC2: DCC2 { _marker: PhantomData },
            DMA: DMA { _marker: PhantomData },
            DMM: DMM { _marker: PhantomData },
            ESM1: ESM1 { _marker: PhantomData },
            ESM2: ESM2 { _marker: PhantomData },
            FLASHWRAPPER: FLASHWRAPPER { _marker: PhantomData },
            PBIST: PBIST { _marker: PhantomData },
            PMM: PMM { _marker: PhantomData },
            RTP: RTP { _marker: PhantomData },
            RTI: RTI { _marker: PhantomData },
            STC1: STC1 { _marker: PhantomData },
            STC2: STC2 { _marker: PhantomData },
            SCM: SCM { _marker: PhantomData },
            EPC: EPC { _marker: PhantomData },
            INTERCONNECT: INTERCONNECT { _marker: PhantomData },
            NMPU1: NMPU1 { _marker: PhantomData },
            NMPU2: NMPU2 { _marker: PhantomData },
            NMPU3: NMPU3 { _marker: PhantomData },
            SYS: SYS { _marker: PhantomData },
            SYS2: SYS2 { _marker: PhantomData },
            VIM: VIM { _marker: PhantomData },
            VIM_BASE2: VIM_BASE2 { _marker: PhantomData },
            EMIF: EMIF { _marker: PhantomData },
            PCR3: PCR3 { _marker: PhantomData },
            PCR2: PCR2 { _marker: PhantomData },
            PCR1: PCR1 { _marker: PhantomData },
            L2RAMW: L2RAMW { _marker: PhantomData },
        }
    }
}
