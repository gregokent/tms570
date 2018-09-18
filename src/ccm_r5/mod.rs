#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CCMR5 Status Register1"]
    pub ccm_sr1: CCMSR1,
    #[doc = "0x04 - CCMR5 Key Register1"]
    pub ccm_key_r1: CCMKEYR1,
    #[doc = "0x08 - CCMR5 Status Register2"]
    pub ccm_sr2: CCMSR2,
    #[doc = "0x0c - CCMR5 Key Register2"]
    pub ccm_key_r2: CCMKEYR2,
    #[doc = "0x10 - CCMR5 Status Register3"]
    pub ccm_sr3: CCMSR3,
    #[doc = "0x14 - CCMR5 Key Register3"]
    pub ccm_key_r3: CCMKEYR3,
    #[doc = "0x18 - CCMR5 Polarity Control Register"]
    pub ccm_pol_cntrl: CCMPOLCNTRL,
    #[doc = "0x1c - CCMR5 Status Register4"]
    pub ccm_sr4: CCMSR4,
    #[doc = "0x20 - CCMR5 Key Register4"]
    pub ccm_key_r4: CCMKEYR4,
    #[doc = "0x24 - CCMR5 Power Domain Status Register0"]
    pub ccm_pdstat0: CCMPDSTAT0,
    #[doc = "0x28 - CCMR5 Power Domain Status Register1"]
    pub ccm_pdstat1: CCMPDSTAT1,
}
#[doc = "CCMR5 Status Register1"]
pub struct CCMSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Status Register1"]
pub mod ccm_sr1;
#[doc = "CCMR5 Key Register1"]
pub struct CCMKEYR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Key Register1"]
pub mod ccm_key_r1;
#[doc = "CCMR5 Status Register2"]
pub struct CCMSR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Status Register2"]
pub mod ccm_sr2;
#[doc = "CCMR5 Key Register2"]
pub struct CCMKEYR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Key Register2"]
pub mod ccm_key_r2;
#[doc = "CCMR5 Status Register3"]
pub struct CCMSR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Status Register3"]
pub mod ccm_sr3;
#[doc = "CCMR5 Key Register3"]
pub struct CCMKEYR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Key Register3"]
pub mod ccm_key_r3;
#[doc = "CCMR5 Polarity Control Register"]
pub struct CCMPOLCNTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Polarity Control Register"]
pub mod ccm_pol_cntrl;
#[doc = "CCMR5 Status Register4"]
pub struct CCMSR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Status Register4"]
pub mod ccm_sr4;
#[doc = "CCMR5 Key Register4"]
pub struct CCMKEYR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Key Register4"]
pub mod ccm_key_r4;
#[doc = "CCMR5 Power Domain Status Register0"]
pub struct CCMPDSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Power Domain Status Register0"]
pub mod ccm_pdstat0;
#[doc = "CCMR5 Power Domain Status Register1"]
pub struct CCMPDSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCMR5 Power Domain Status Register1"]
pub mod ccm_pdstat1;
