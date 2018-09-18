#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCM REVID Register"]
    pub scm_rev_id: SCMREVID,
    #[doc = "0x04 - SCM Control Register"]
    pub scm_cntrl: SCMCNTRL,
    #[doc = "0x08 - SCM Compare Threshold Counter Register"]
    pub scm_threshold: SCMTHRESHOLD,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - SCM Inittiator Error0 Status Register"]
    pub scm_ia_err0stat: SCMIAERR0STAT,
    #[doc = "0x14 - SCM Initiator Error1 Status Register"]
    pub scm_ia_err1stat: SCMIAERR1STAT,
    #[doc = "0x18 - SCM Initiator Active Status Register"]
    pub scm_ia_stat: SCMIASTAT,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - SCM Target Active Status Register"]
    pub scm_ta_stat: SCMTASTAT,
}
#[doc = "SCM REVID Register"]
pub struct SCMREVID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCM REVID Register"]
pub mod scm_rev_id;
#[doc = "SCM Control Register"]
pub struct SCMCNTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCM Control Register"]
pub mod scm_cntrl;
#[doc = "SCM Compare Threshold Counter Register"]
pub struct SCMTHRESHOLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCM Compare Threshold Counter Register"]
pub mod scm_threshold;
#[doc = "SCM Inittiator Error0 Status Register"]
pub struct SCMIAERR0STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCM Inittiator Error0 Status Register"]
pub mod scm_ia_err0stat;
#[doc = "SCM Initiator Error1 Status Register"]
pub struct SCMIAERR1STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCM Initiator Error1 Status Register"]
pub mod scm_ia_err1stat;
#[doc = "SCM Initiator Active Status Register"]
pub struct SCMIASTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCM Initiator Active Status Register"]
pub mod scm_ia_stat;
#[doc = "SCM Target Active Status Register"]
pub struct SCMTASTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCM Target Active Status Register"]
pub mod scm_ta_stat;
