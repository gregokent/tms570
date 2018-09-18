#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Index Offset Vector Register"]
    pub irq_ivec: IRQIVEC,
    #[doc = "0x04 - Index Offset Vector Register"]
    pub fiq_ivec: FIQIVEC,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Program Control Register"]
    pub firq_pr0: FIRQPR0,
    #[doc = "0x14 - Program Control Register"]
    pub firq_pr1: FIRQPR1,
    #[doc = "0x18 - Program Control Register"]
    pub firq_pr2: FIRQPR2,
    #[doc = "0x1c - Program Control Register"]
    pub firq_pr3: FIRQPR3,
    #[doc = "0x20 - Pending Interrupt Read Location"]
    pub int_req0: INTREQ0,
    #[doc = "0x24 - Pending Interrupt Read Location"]
    pub int_req1: INTREQ1,
    #[doc = "0x28 - Pending Interrupt Read Location"]
    pub int_req2: INTREQ2,
    #[doc = "0x2c - Pending Interrupt Read Location"]
    pub int_req3: INTREQ3,
    #[doc = "0x30 - Interrupt Mask Set Register"]
    pub req_mask_set0: REQMASKSET0,
    #[doc = "0x34 - Interrupt Mask Set Register"]
    pub req_mask_set1: REQMASKSET1,
    #[doc = "0x38 - Interrupt Mask Set Register"]
    pub req_mask_set2: REQMASKSET2,
    #[doc = "0x3c - Interrupt Mask Set Register"]
    pub req_mask_set3: REQMASKSET3,
    #[doc = "0x40 - Interrupt Mask Clear Register"]
    pub req_mask_clr0: REQMASKCLR0,
    #[doc = "0x44 - Interrupt Mask Clear Register"]
    pub req_mask_clr1: REQMASKCLR1,
    #[doc = "0x48 - Interrupt Mask Clear Register"]
    pub req_mask_clr2: REQMASKCLR2,
    #[doc = "0x4c - Interrupt Mask Clear Register"]
    pub req_mask_clr3: REQMASKCLR3,
    #[doc = "0x50 - Wake-up Mask Set Register"]
    pub wake_mask_set0: WAKEMASKSET0,
    #[doc = "0x54 - Wake-up Mask Set Register"]
    pub wake_mask_set1: WAKEMASKSET1,
    #[doc = "0x58 - Wake-up Mask Set Register"]
    pub wake_mask_set2: WAKEMASKSET2,
    #[doc = "0x5c - Wake-up Mask Set Register"]
    pub wake_mask_set3: WAKEMASKSET3,
    #[doc = "0x60 - Wake-up Mask Clear Register"]
    pub wake_mask_clr0: WAKEMASKCLR0,
    #[doc = "0x64 - Wake-up Mask Clear Register"]
    pub wake_mask_clr1: WAKEMASKCLR1,
    #[doc = "0x68 - Wake-up Mask Clear Register"]
    pub wake_mask_clr2: WAKEMASKCLR2,
    #[doc = "0x6c - Wake-up Mask Clear Register"]
    pub wake_mask_clr3: WAKEMASKCLR3,
    #[doc = "0x70 - Irq Interrupt Vector Register"]
    pub irq_vec_reg: IRQVECREG,
    #[doc = "0x74 - Fiq Interrupt Vector Register"]
    pub fiq_vec_reg: FIQVECREG,
    #[doc = "0x78 - Capture Event register"]
    pub cap_evt_src: CAPEVTSRC,
    _reserved1: [u8; 4usize],
    #[doc = "0x80 - Channel Mapping Register"]
    pub chan_ctrl0: CHANCTRL0,
    #[doc = "0x84 - Channel Mapping Register"]
    pub chan_ctrl1: CHANCTRL1,
    #[doc = "0x88 - Channel Mapping Register"]
    pub chan_ctrl2: CHANCTRL2,
    #[doc = "0x8c - Channel Mapping Register"]
    pub chan_ctrl3: CHANCTRL3,
    #[doc = "0x90 - Channel Mapping Register"]
    pub chan_ctrl4: CHANCTRL4,
    #[doc = "0x94 - Channel Mapping Register"]
    pub chan_ctrl5: CHANCTRL5,
    #[doc = "0x98 - Channel Mapping Register"]
    pub chan_ctrl6: CHANCTRL6,
    #[doc = "0x9c - Channel Mapping Register"]
    pub chan_ctrl7: CHANCTRL7,
    #[doc = "0xa0 - Channel Mapping Register"]
    pub chan_ctrl8: CHANCTRL8,
    #[doc = "0xa4 - Channel Mapping Register"]
    pub chan_ctrl9: CHANCTRL9,
    #[doc = "0xa8 - Channel Mapping Register"]
    pub chan_ctrl10: CHANCTRL10,
    #[doc = "0xac - Channel Mapping Register"]
    pub chan_ctrl11: CHANCTRL11,
    #[doc = "0xb0 - Channel Mapping Register"]
    pub chan_ctrl12: CHANCTRL12,
    #[doc = "0xb4 - Channel Mapping Register"]
    pub chan_ctrl13: CHANCTRL13,
    #[doc = "0xb8 - Channel Mapping Register"]
    pub chan_ctrl14: CHANCTRL14,
    #[doc = "0xbc - Channel Mapping Register"]
    pub chan_ctrl15: CHANCTRL15,
    #[doc = "0xc0 - Channel Mapping Register"]
    pub chan_ctrl16: CHANCTRL16,
    #[doc = "0xc4 - Channel Mapping Register"]
    pub chan_ctrl17: CHANCTRL17,
    #[doc = "0xc8 - Channel Mapping Register"]
    pub chan_ctrl18: CHANCTRL18,
    #[doc = "0xcc - Channel Mapping Register"]
    pub chan_ctrl19: CHANCTRL19,
    #[doc = "0xd0 - Channel Mapping Register"]
    pub chan_ctrl20: CHANCTRL20,
    #[doc = "0xd4 - Channel Mapping Register"]
    pub chan_ctrl21: CHANCTRL21,
    #[doc = "0xd8 - Channel Mapping Register"]
    pub chan_ctrl22: CHANCTRL22,
    #[doc = "0xdc - Channel Mapping Register"]
    pub chan_ctrl23: CHANCTRL23,
    #[doc = "0xe0 - Channel Mapping Register"]
    pub chan_ctrl24: CHANCTRL24,
    #[doc = "0xe4 - Channel Mapping Register"]
    pub chan_ctrl25: CHANCTRL25,
    #[doc = "0xe8 - Channel Mapping Register"]
    pub chan_ctrl26: CHANCTRL26,
    #[doc = "0xec - Channel Mapping Register"]
    pub chan_ctrl27: CHANCTRL27,
    #[doc = "0xf0 - Channel Mapping Register"]
    pub chan_ctrl28: CHANCTRL28,
    #[doc = "0xf4 - Channel Mapping Register"]
    pub chan_ctrl29: CHANCTRL29,
    #[doc = "0xf8 - Channel Mapping Register"]
    pub chan_ctrl30: CHANCTRL30,
    #[doc = "0xfc - Channel Mapping Register"]
    pub chan_ctrl31: CHANCTRL31,
}
#[doc = "Index Offset Vector Register"]
pub struct IRQIVEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Index Offset Vector Register"]
pub mod irq_ivec;
#[doc = "Index Offset Vector Register"]
pub struct FIQIVEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Index Offset Vector Register"]
pub mod fiq_ivec;
#[doc = "Program Control Register"]
pub struct FIRQPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program Control Register"]
pub mod firq_pr0;
#[doc = "Program Control Register"]
pub struct FIRQPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program Control Register"]
pub mod firq_pr1;
#[doc = "Program Control Register"]
pub struct FIRQPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program Control Register"]
pub mod firq_pr2;
#[doc = "Program Control Register"]
pub struct FIRQPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program Control Register"]
pub mod firq_pr3;
#[doc = "Pending Interrupt Read Location"]
pub struct INTREQ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending Interrupt Read Location"]
pub mod int_req0;
#[doc = "Pending Interrupt Read Location"]
pub struct INTREQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending Interrupt Read Location"]
pub mod int_req1;
#[doc = "Pending Interrupt Read Location"]
pub struct INTREQ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending Interrupt Read Location"]
pub mod int_req2;
#[doc = "Pending Interrupt Read Location"]
pub struct INTREQ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending Interrupt Read Location"]
pub mod int_req3;
#[doc = "Interrupt Mask Set Register"]
pub struct REQMASKSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Set Register"]
pub mod req_mask_set0;
#[doc = "Interrupt Mask Set Register"]
pub struct REQMASKSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Set Register"]
pub mod req_mask_set1;
#[doc = "Interrupt Mask Set Register"]
pub struct REQMASKSET2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Set Register"]
pub mod req_mask_set2;
#[doc = "Interrupt Mask Set Register"]
pub struct REQMASKSET3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Set Register"]
pub mod req_mask_set3;
#[doc = "Interrupt Mask Clear Register"]
pub struct REQMASKCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Clear Register"]
pub mod req_mask_clr0;
#[doc = "Interrupt Mask Clear Register"]
pub struct REQMASKCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Clear Register"]
pub mod req_mask_clr1;
#[doc = "Interrupt Mask Clear Register"]
pub struct REQMASKCLR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Clear Register"]
pub mod req_mask_clr2;
#[doc = "Interrupt Mask Clear Register"]
pub struct REQMASKCLR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Clear Register"]
pub mod req_mask_clr3;
#[doc = "Wake-up Mask Set Register"]
pub struct WAKEMASKSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up Mask Set Register"]
pub mod wake_mask_set0;
#[doc = "Wake-up Mask Set Register"]
pub struct WAKEMASKSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up Mask Set Register"]
pub mod wake_mask_set1;
#[doc = "Wake-up Mask Set Register"]
pub struct WAKEMASKSET2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up Mask Set Register"]
pub mod wake_mask_set2;
#[doc = "Wake-up Mask Set Register"]
pub struct WAKEMASKSET3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up Mask Set Register"]
pub mod wake_mask_set3;
#[doc = "Wake-up Mask Clear Register"]
pub struct WAKEMASKCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up Mask Clear Register"]
pub mod wake_mask_clr0;
#[doc = "Wake-up Mask Clear Register"]
pub struct WAKEMASKCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up Mask Clear Register"]
pub mod wake_mask_clr1;
#[doc = "Wake-up Mask Clear Register"]
pub struct WAKEMASKCLR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up Mask Clear Register"]
pub mod wake_mask_clr2;
#[doc = "Wake-up Mask Clear Register"]
pub struct WAKEMASKCLR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up Mask Clear Register"]
pub mod wake_mask_clr3;
#[doc = "Irq Interrupt Vector Register"]
pub struct IRQVECREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq Interrupt Vector Register"]
pub mod irq_vec_reg;
#[doc = "Fiq Interrupt Vector Register"]
pub struct FIQVECREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fiq Interrupt Vector Register"]
pub mod fiq_vec_reg;
#[doc = "Capture Event register"]
pub struct CAPEVTSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Event register"]
pub mod cap_evt_src;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl0;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl1;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl2;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl3;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl4;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl5;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl6;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl7;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl8;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl9;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl10;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl11;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl12;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl13;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl14;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl15;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl16;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl17;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl18;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl19;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl20;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl21;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl22;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl23;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl24;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl25;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl26;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl27;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl28;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl29;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl30;
#[doc = "Channel Mapping Register"]
pub struct CHANCTRL31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Mapping Register"]
pub mod chan_ctrl31;
