#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MPU Revision ID Register"]
    pub mpu_rev: MPUREV,
    #[doc = "0x04 - MPU Lock Register"]
    pub mpu_lock: MPULOCK,
    #[doc = "0x08 - MPU Diagnostic Control Register"]
    pub mpu_diag_ctrl: MPUDIAGCTRL,
    #[doc = "0x0c - MPU Diagnostic Address Register"]
    pub mpu_diag_addr: MPUDIAGADDR,
    #[doc = "0x10 - MPU Error Status Register"]
    pub mpu_err_stat: MPUERRSTAT,
    #[doc = "0x14 - MPU Error Address Register"]
    pub mpu_err_addr: MPUERRADDR,
    #[doc = "0x18 - MPU Input Address Mask Register"]
    pub mpu_iam: MPUIAM,
    #[doc = "0x1c - MPU General Purpose Control Register"]
    pub mpu_gpctrl: MPUGPCTRL,
    #[doc = "0x20 - MPU Control Register1"]
    pub mpu_ctrl1: MPUCTRL1,
    #[doc = "0x24 - MPU Control Register2"]
    pub mpu_ctrl2: MPUCTRL2,
    #[doc = "0x28 - MPU Control Register3"]
    pub mpu_ctrl3: MPUCTRL3,
    #[doc = "0x2c - MPU Type Register"]
    pub mpu_type: MPUTYPE,
    #[doc = "0x30 - MPU Region Base Address Register"]
    pub mpu_reg_base: MPUREGBASE,
    #[doc = "0x34 - MPU Region Size and Enable Register"]
    pub mpu_reg_sena: MPUREGSENA,
    #[doc = "0x38 - MPU Region Access Control Register"]
    pub mpu_reg_acr: MPUREGACR,
    #[doc = "0x3c - MPU Region Number Register"]
    pub mpu_reg_num: MPUREGNUM,
    #[doc = "0x40 - MPU Region Address Mask Register"]
    pub mpu_reg_am: MPUREGAM,
    #[doc = "0x44 - MPU Region Translation Address Register"]
    pub mpu_reg_ta: MPUREGTA,
    #[doc = "0x48 - MPU Region Mode Translation Register"]
    pub mpu_reg_mt: MPUREGMT,
    #[doc = "0x4c - Uncorrectable Error Address Register 11"]
    pub uerr_addr11: UERRADDR11,
}
#[doc = "MPU Revision ID Register"]
pub struct MPUREV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Revision ID Register"]
pub mod mpu_rev;
#[doc = "MPU Lock Register"]
pub struct MPULOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Lock Register"]
pub mod mpu_lock;
#[doc = "MPU Diagnostic Control Register"]
pub struct MPUDIAGCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Diagnostic Control Register"]
pub mod mpu_diag_ctrl;
#[doc = "MPU Diagnostic Address Register"]
pub struct MPUDIAGADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Diagnostic Address Register"]
pub mod mpu_diag_addr;
#[doc = "MPU Error Status Register"]
pub struct MPUERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Error Status Register"]
pub mod mpu_err_stat;
#[doc = "MPU Error Address Register"]
pub struct MPUERRADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Error Address Register"]
pub mod mpu_err_addr;
#[doc = "MPU Input Address Mask Register"]
pub struct MPUIAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Input Address Mask Register"]
pub mod mpu_iam;
#[doc = "MPU General Purpose Control Register"]
pub struct MPUGPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU General Purpose Control Register"]
pub mod mpu_gpctrl;
#[doc = "MPU Control Register1"]
pub struct MPUCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Control Register1"]
pub mod mpu_ctrl1;
#[doc = "MPU Control Register2"]
pub struct MPUCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Control Register2"]
pub mod mpu_ctrl2;
#[doc = "MPU Control Register3"]
pub struct MPUCTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Control Register3"]
pub mod mpu_ctrl3;
#[doc = "MPU Type Register"]
pub struct MPUTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Type Register"]
pub mod mpu_type;
#[doc = "MPU Region Base Address Register"]
pub struct MPUREGBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Base Address Register"]
pub mod mpu_reg_base;
#[doc = "MPU Region Size and Enable Register"]
pub struct MPUREGSENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Size and Enable Register"]
pub mod mpu_reg_sena;
#[doc = "MPU Region Access Control Register"]
pub struct MPUREGACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Access Control Register"]
pub mod mpu_reg_acr;
#[doc = "MPU Region Number Register"]
pub struct MPUREGNUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Number Register"]
pub mod mpu_reg_num;
#[doc = "MPU Region Address Mask Register"]
pub struct MPUREGAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Address Mask Register"]
pub mod mpu_reg_am;
#[doc = "MPU Region Translation Address Register"]
pub struct MPUREGTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Translation Address Register"]
pub mod mpu_reg_ta;
#[doc = "MPU Region Mode Translation Register"]
pub struct MPUREGMT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MPU Region Mode Translation Register"]
pub mod mpu_reg_mt;
#[doc = "Uncorrectable Error Address Register 11"]
pub struct UERRADDR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 11"]
pub mod uerr_addr11;
