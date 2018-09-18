#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Sys Pin Control Register 1"]
    pub sys_pc1: SYSPC1,
    #[doc = "0x04 - Sys Pin Control Register 2"]
    pub sys_pc2: SYSPC2,
    #[doc = "0x08 - Sys Pin Control Register 3"]
    pub sys_pc3: SYSPC3,
    #[doc = "0x0c - Sys Pin Control Register 4"]
    pub sys_pc4: SYSPC4,
    #[doc = "0x10 - Sys Pin Control Register 5"]
    pub sys_pc5: SYSPC5,
    #[doc = "0x14 - Sys Pin Control Register 6"]
    pub sys_pc6: SYSPC6,
    #[doc = "0x18 - Sys Pin Control Register 7"]
    pub sys_pc7: SYSPC7,
    #[doc = "0x1c - Sys Pin Control Register 8"]
    pub sys_pc8: SYSPC8,
    #[doc = "0x20 - Sys Pin Control Register 9"]
    pub sys_pc9: SYSPC9,
    _reserved0: [u8; 12usize],
    #[doc = "0x30 - Clock Source Disable Register"]
    pub cs_dis: CSDIS,
    #[doc = "0x34 - Clock Source Disable Set Register"]
    pub cs_dis_set: CSDISSET,
    #[doc = "0x38 - Clock Source Disable Clear Register"]
    pub cs_dis_clr: CSDISCLR,
    #[doc = "0x3c - Clock Domain Disable Register"]
    pub cd_dis: CDDIS,
    #[doc = "0x40 - Clock Domain Disable Set Register"]
    pub cd_dis_set: CDDISSET,
    #[doc = "0x44 - Clock Domain Disable Clear Register"]
    pub cd_dis_clr: CDDISCLR,
    #[doc = "0x48 - GClk, HClk, VClk, and VClk2 Source Register"]
    pub ghv_src: GHVSRC,
    #[doc = "0x4c - Peripheral Asynchronous Clock Source Register"]
    pub vlck_asrc: VLCKASRC,
    #[doc = "0x50 - RTI Clock Source Register"]
    pub rclk_src: RCLKSRC,
    #[doc = "0x54 - Clock Source Valid Status Register"]
    pub cs_vstat: CSVSTAT,
    #[doc = "0x58 - Memory Self-Test Global Control Register"]
    pub mst_glb_ctrl: MSTGLBCTRL,
    #[doc = "0x5c - Memory Hardware Initialization Global Control Register"]
    pub minit_glb_ctrl: MINITGLBCTRL,
    #[doc = "0x60 - MBIST Controller/Memory Initialization Enable Register"]
    pub ms_in_ena: MSINENA,
    #[doc = "0x64 - Memory Self-Test Fail Status Register"]
    pub mst_fail: MSTFAIL,
    #[doc = "0x68 - MstC Global Status Register"]
    pub mst_cg_stat: MSTCGSTAT,
    #[doc = "0x6c - Memory Hardware Initialization Status Register"]
    pub mini_stat: MINISTAT,
    #[doc = "0x70 - Pll Control Register 1"]
    pub pll_ctl1: PLLCTL1,
    #[doc = "0x74 - Pll Control Register 2"]
    pub pll_ctl2: PLLCTL2,
    #[doc = "0x78 - Sys Pin Control Register 10"]
    pub sys_pc10: SYSPC10,
    #[doc = "0x7c - Die Identification Register, Lower Word"]
    pub die_id_l: DIEIDL,
    #[doc = "0x80 - Die Identification Register Upper Word"]
    pub die_id_h: DIEIDH,
    _reserved1: [u8; 4usize],
    #[doc = "0x88 - Lpo/Clock Monitor Control Register"]
    pub lpo_mon_ctl: LPOMONCTL,
    #[doc = "0x8c - Clock Test Register"]
    pub clk_test: CLKTEST,
    #[doc = "0x90 - DFT Control Register"]
    pub dft_ctrl_reg: DFTCTRLREG,
    #[doc = "0x94 - DFT Control Register 2"]
    pub dft_ctrl_reg2: DFTCTRLREG2,
    _reserved2: [u8; 8usize],
    #[doc = "0xa0 - General Purpose Register"]
    pub gpreg1: GPREG1,
    _reserved3: [u8; 4usize],
    #[doc = "0xa8 - Imprecise Fault Status Register"]
    pub imp_fa_sts: IMPFASTS,
    #[doc = "0xac - Imprecise Fault Write Address Register"]
    pub imp_ft_add: IMPFTADD,
    #[doc = "0xb0 - System Software Interrupt Request 1 Register"]
    pub ssir1: SSIR1,
    #[doc = "0xb4 - System Software Interrupt Request 2 Register"]
    pub ssir2: SSIR2,
    #[doc = "0xb8 - System Software Interrupt Request 3 Register"]
    pub ssir3: SSIR3,
    #[doc = "0xbc - System Software Interrupt Request 4 Register"]
    pub ssir4: SSIR4,
    #[doc = "0xc0 - Ram Control Register"]
    pub ram_glb_ctrl: RAMGLBCTRL,
    #[doc = "0xc4 - Bus Matrix Module Control Register1"]
    pub bmm_cr1: BMMCR1,
    #[doc = "0xc8 - Bus Matrix Module Control Register2"]
    pub bmm_cr2: BMMCR2,
    #[doc = "0xcc - Mmu Global Control Register"]
    pub mmu_glb_ctrl: MMUGLBCTRL,
    #[doc = "0xd0 - Clock Control Register"]
    pub clk_cntl: CLKCNTL,
    #[doc = "0xd4 - ECP Control Register"]
    pub ecp_cntl: ECPCNTL,
    _reserved4: [u8; 4usize],
    #[doc = "0xdc - DEV Parity Control Register1"]
    pub dev_cr1: DEVCR1,
    #[doc = "0xe0 - System Exception Control Register"]
    pub sys_ecr: SYSECR,
    #[doc = "0xe4 - System Exception Status Register"]
    pub sys_esr: SYSESR,
    #[doc = "0xe8 - System Test Abort Status Register"]
    pub sys_tasr: SYSTASR,
    #[doc = "0xec - Global Status Register"]
    pub glb_stat: GLBSTAT,
    #[doc = "0xf0 - Device Identification Register"]
    pub dev_id: DEVID,
    #[doc = "0xf4 - Software Interrupt Vector Register"]
    pub ssi_vec: SSIVEC,
    #[doc = "0xf8 - System Software Interrupt Flag Register"]
    pub ssif: SSIF,
}
#[doc = "Sys Pin Control Register 1"]
pub struct SYSPC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sys Pin Control Register 1"]
pub mod sys_pc1;
#[doc = "Sys Pin Control Register 2"]
pub struct SYSPC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sys Pin Control Register 2"]
pub mod sys_pc2;
#[doc = "Sys Pin Control Register 3"]
pub struct SYSPC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sys Pin Control Register 3"]
pub mod sys_pc3;
#[doc = "Sys Pin Control Register 4"]
pub struct SYSPC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sys Pin Control Register 4"]
pub mod sys_pc4;
#[doc = "Sys Pin Control Register 5"]
pub struct SYSPC5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sys Pin Control Register 5"]
pub mod sys_pc5;
#[doc = "Sys Pin Control Register 6"]
pub struct SYSPC6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sys Pin Control Register 6"]
pub mod sys_pc6;
#[doc = "Sys Pin Control Register 7"]
pub struct SYSPC7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sys Pin Control Register 7"]
pub mod sys_pc7;
#[doc = "Sys Pin Control Register 8"]
pub struct SYSPC8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sys Pin Control Register 8"]
pub mod sys_pc8;
#[doc = "Sys Pin Control Register 9"]
pub struct SYSPC9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sys Pin Control Register 9"]
pub mod sys_pc9;
#[doc = "Clock Source Disable Register"]
pub struct CSDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Source Disable Register"]
pub mod cs_dis;
#[doc = "Clock Source Disable Set Register"]
pub struct CSDISSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Source Disable Set Register"]
pub mod cs_dis_set;
#[doc = "Clock Source Disable Clear Register"]
pub struct CSDISCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Source Disable Clear Register"]
pub mod cs_dis_clr;
#[doc = "Clock Domain Disable Register"]
pub struct CDDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Domain Disable Register"]
pub mod cd_dis;
#[doc = "Clock Domain Disable Set Register"]
pub struct CDDISSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Domain Disable Set Register"]
pub mod cd_dis_set;
#[doc = "Clock Domain Disable Clear Register"]
pub struct CDDISCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Domain Disable Clear Register"]
pub mod cd_dis_clr;
#[doc = "GClk, HClk, VClk, and VClk2 Source Register"]
pub struct GHVSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GClk, HClk, VClk, and VClk2 Source Register"]
pub mod ghv_src;
#[doc = "Peripheral Asynchronous Clock Source Register"]
pub struct VLCKASRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Asynchronous Clock Source Register"]
pub mod vlck_asrc;
#[doc = "RTI Clock Source Register"]
pub struct RCLKSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTI Clock Source Register"]
pub mod rclk_src;
#[doc = "Clock Source Valid Status Register"]
pub struct CSVSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Source Valid Status Register"]
pub mod cs_vstat;
#[doc = "Memory Self-Test Global Control Register"]
pub struct MSTGLBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Self-Test Global Control Register"]
pub mod mst_glb_ctrl;
#[doc = "Memory Hardware Initialization Global Control Register"]
pub struct MINITGLBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Hardware Initialization Global Control Register"]
pub mod minit_glb_ctrl;
#[doc = "MBIST Controller/Memory Initialization Enable Register"]
pub struct MSINENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MBIST Controller/Memory Initialization Enable Register"]
pub mod ms_in_ena;
#[doc = "Memory Self-Test Fail Status Register"]
pub struct MSTFAIL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Self-Test Fail Status Register"]
pub mod mst_fail;
#[doc = "MstC Global Status Register"]
pub struct MSTCGSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MstC Global Status Register"]
pub mod mst_cg_stat;
#[doc = "Memory Hardware Initialization Status Register"]
pub struct MINISTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Hardware Initialization Status Register"]
pub mod mini_stat;
#[doc = "Pll Control Register 1"]
pub struct PLLCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pll Control Register 1"]
pub mod pll_ctl1;
#[doc = "Pll Control Register 2"]
pub struct PLLCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pll Control Register 2"]
pub mod pll_ctl2;
#[doc = "Sys Pin Control Register 10"]
pub struct SYSPC10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sys Pin Control Register 10"]
pub mod sys_pc10;
#[doc = "Die Identification Register, Lower Word"]
pub struct DIEIDL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Identification Register, Lower Word"]
pub mod die_id_l;
#[doc = "Die Identification Register Upper Word"]
pub struct DIEIDH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Identification Register Upper Word"]
pub mod die_id_h;
#[doc = "Lpo/Clock Monitor Control Register"]
pub struct LPOMONCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lpo/Clock Monitor Control Register"]
pub mod lpo_mon_ctl;
#[doc = "Clock Test Register"]
pub struct CLKTEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Test Register"]
pub mod clk_test;
#[doc = "DFT Control Register"]
pub struct DFTCTRLREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFT Control Register"]
pub mod dft_ctrl_reg;
#[doc = "DFT Control Register 2"]
pub struct DFTCTRLREG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFT Control Register 2"]
pub mod dft_ctrl_reg2;
#[doc = "General Purpose Register"]
pub struct GPREG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register"]
pub mod gpreg1;
#[doc = "Imprecise Fault Status Register"]
pub struct IMPFASTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Imprecise Fault Status Register"]
pub mod imp_fa_sts;
#[doc = "Imprecise Fault Write Address Register"]
pub struct IMPFTADD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Imprecise Fault Write Address Register"]
pub mod imp_ft_add;
#[doc = "System Software Interrupt Request 1 Register"]
pub struct SSIR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Software Interrupt Request 1 Register"]
pub mod ssir1;
#[doc = "System Software Interrupt Request 2 Register"]
pub struct SSIR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Software Interrupt Request 2 Register"]
pub mod ssir2;
#[doc = "System Software Interrupt Request 3 Register"]
pub struct SSIR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Software Interrupt Request 3 Register"]
pub mod ssir3;
#[doc = "System Software Interrupt Request 4 Register"]
pub struct SSIR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Software Interrupt Request 4 Register"]
pub mod ssir4;
#[doc = "Ram Control Register"]
pub struct RAMGLBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ram Control Register"]
pub mod ram_glb_ctrl;
#[doc = "Bus Matrix Module Control Register1"]
pub struct BMMCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Matrix Module Control Register1"]
pub mod bmm_cr1;
#[doc = "Bus Matrix Module Control Register2"]
pub struct BMMCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Matrix Module Control Register2"]
pub mod bmm_cr2;
#[doc = "Mmu Global Control Register"]
pub struct MMUGLBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mmu Global Control Register"]
pub mod mmu_glb_ctrl;
#[doc = "Clock Control Register"]
pub struct CLKCNTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control Register"]
pub mod clk_cntl;
#[doc = "ECP Control Register"]
pub struct ECPCNTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECP Control Register"]
pub mod ecp_cntl;
#[doc = "DEV Parity Control Register1"]
pub struct DEVCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DEV Parity Control Register1"]
pub mod dev_cr1;
#[doc = "System Exception Control Register"]
pub struct SYSECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Exception Control Register"]
pub mod sys_ecr;
#[doc = "System Exception Status Register"]
pub struct SYSESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Exception Status Register"]
pub mod sys_esr;
#[doc = "System Test Abort Status Register"]
pub struct SYSTASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Test Abort Status Register"]
pub mod sys_tasr;
#[doc = "Global Status Register"]
pub struct GLBSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Status Register"]
pub mod glb_stat;
#[doc = "Device Identification Register"]
pub struct DEVID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Identification Register"]
pub mod dev_id;
#[doc = "Software Interrupt Vector Register"]
pub struct SSIVEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Interrupt Vector Register"]
pub mod ssi_vec;
#[doc = "System Software Interrupt Flag Register"]
pub struct SSIF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Software Interrupt Flag Register"]
pub mod ssif;
