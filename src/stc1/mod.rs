#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register 0"]
    pub glb_ctrl0: GLBCTRL0,
    #[doc = "0x04 - Global Control Register 1"]
    pub glb_ctrl1: GLBCTRL1,
    #[doc = "0x08 - Run TimeOut Counter Preload Register"]
    pub tpr: TPR,
    #[doc = "0x0c - Current ROM Address Register"]
    pub caddr: CADDR,
    #[doc = "0x10 - Current Interval Count Register"]
    pub cicnt: CICNT,
    #[doc = "0x14 - SelfTest Global Status Register"]
    pub gstat: GSTAT,
    #[doc = "0x18 - Fail Status Register"]
    pub fstat: FSTAT,
    #[doc = "0x1c - Cpu1 Current Misr Register"]
    pub cpu1cur_misr0: CPU1CURMISR0,
    #[doc = "0x20 - Cpu1 Current Misr Register"]
    pub cpu1cur_misr1: CPU1CURMISR1,
    #[doc = "0x24 - Cpu1 Current Misr Register"]
    pub cpu1cur_misr2: CPU1CURMISR2,
    #[doc = "0x28 - Cpu1 Current Misr Register"]
    pub cpu1cur_misr3: CPU1CURMISR3,
    #[doc = "0x2c - Cpu2 Current Misr Register"]
    pub cpu2cur_misr0: CPU2CURMISR0,
    #[doc = "0x30 - Cpu2 Current Misr Register"]
    pub cpu2cur_misr1: CPU2CURMISR1,
    #[doc = "0x34 - Cpu2 Current Misr Register"]
    pub cpu2cur_misr2: CPU2CURMISR2,
    #[doc = "0x38 - Cpu2 Current Misr Register"]
    pub cpu2cur_misr3: CPU2CURMISR3,
    #[doc = "0x3c - Signature SelfCheck Register"]
    pub stc_self_check_reg: STCSELFCHECKREG,
    #[doc = "0x40 - STC Current ROM Address Register - Core2"]
    pub stc_cur_addr2: STCCURADDR2,
    #[doc = "0x44 - STC Clock Prescalar"]
    pub stc_clk_div: STCCLKDIV,
    #[doc = "0x48 - STC Segment Preload Register"]
    pub stc_seg_plr: STCSEGPLR,
}
#[doc = "Global Control Register 0"]
pub struct GLBCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register 0"]
pub mod glb_ctrl0;
#[doc = "Global Control Register 1"]
pub struct GLBCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register 1"]
pub mod glb_ctrl1;
#[doc = "Run TimeOut Counter Preload Register"]
pub struct TPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Run TimeOut Counter Preload Register"]
pub mod tpr;
#[doc = "Current ROM Address Register"]
pub struct CADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current ROM Address Register"]
pub mod caddr;
#[doc = "Current Interval Count Register"]
pub struct CICNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Interval Count Register"]
pub mod cicnt;
#[doc = "SelfTest Global Status Register"]
pub struct GSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SelfTest Global Status Register"]
pub mod gstat;
#[doc = "Fail Status Register"]
pub struct FSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fail Status Register"]
pub mod fstat;
#[doc = "Cpu1 Current Misr Register"]
pub struct CPU1CURMISR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cpu1 Current Misr Register"]
pub mod cpu1cur_misr0;
#[doc = "Cpu1 Current Misr Register"]
pub struct CPU1CURMISR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cpu1 Current Misr Register"]
pub mod cpu1cur_misr1;
#[doc = "Cpu1 Current Misr Register"]
pub struct CPU1CURMISR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cpu1 Current Misr Register"]
pub mod cpu1cur_misr2;
#[doc = "Cpu1 Current Misr Register"]
pub struct CPU1CURMISR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cpu1 Current Misr Register"]
pub mod cpu1cur_misr3;
#[doc = "Cpu2 Current Misr Register"]
pub struct CPU2CURMISR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cpu2 Current Misr Register"]
pub mod cpu2cur_misr0;
#[doc = "Cpu2 Current Misr Register"]
pub struct CPU2CURMISR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cpu2 Current Misr Register"]
pub mod cpu2cur_misr1;
#[doc = "Cpu2 Current Misr Register"]
pub struct CPU2CURMISR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cpu2 Current Misr Register"]
pub mod cpu2cur_misr2;
#[doc = "Cpu2 Current Misr Register"]
pub struct CPU2CURMISR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cpu2 Current Misr Register"]
pub mod cpu2cur_misr3;
#[doc = "Signature SelfCheck Register"]
pub struct STCSELFCHECKREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signature SelfCheck Register"]
pub mod stc_self_check_reg;
#[doc = "STC Current ROM Address Register - Core2"]
pub struct STCCURADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STC Current ROM Address Register - Core2"]
pub mod stc_cur_addr2;
#[doc = "STC Clock Prescalar"]
pub struct STCCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STC Clock Prescalar"]
pub mod stc_clk_div;
#[doc = "STC Segment Preload Register"]
pub struct STCSEGPLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STC Segment Preload Register"]
pub mod stc_seg_plr;
