#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Influence Error Pin Set/Status Register 1"]
    pub ifl_err_pin_set1: IFLERRPINSET1,
    #[doc = "0x04 - Influence Error Pin Clear/Status Register 1"]
    pub ifl_err_pin_clr1: IFLERRPINCLR1,
    #[doc = "0x08 - Interrupt Enable Set/Status Register 1"]
    pub int_ena_set1: INTENASET1,
    #[doc = "0x0c - Interrupt Enable Clear/Status Register 1"]
    pub int_ena_clr1: INTENACLR1,
    #[doc = "0x10 - Interrupt Level Set/Status Register 1"]
    pub int_lvl_set1: INTLVLSET1,
    #[doc = "0x14 - Interrupt Level Clear/Status Register 1"]
    pub int_lvl_clr1: INTLVLCLR1,
    #[doc = "0x18 - Status Register 1"]
    pub stat1: STAT1,
    #[doc = "0x1c - Status Register 2"]
    pub stat2: STAT2,
    #[doc = "0x20 - Status Register 3"]
    pub stat3: STAT3,
    #[doc = "0x24 - Error Pin Status Register"]
    pub err_pin_stat: ERRPINSTAT,
    #[doc = "0x28 - Interrupt Offset High Register"]
    pub int_offst_hgh: INTOFFSTHGH,
    #[doc = "0x2c - Interrupt Offset Low Register"]
    pub int_offst_low: INTOFFSTLOW,
    #[doc = "0x30 - Low-Time Counter Register"]
    pub lt_cnt: LTCNT,
    #[doc = "0x34 - Low-Time Counter Preload Register"]
    pub lt_cnt_pre: LTCNTPRE,
    #[doc = "0x38 - Error Key Register"]
    pub err_key: ERRKEY,
    #[doc = "0x3c - Status Shadow Register"]
    pub shdw_stat2: SHDWSTAT2,
    #[doc = "0x40 - Influence Error Pin Set/Status Register 4"]
    pub ifl_err_pin_set4: IFLERRPINSET4,
    #[doc = "0x44 - Influence Error Pin Clear/Status Register 4"]
    pub ifl_err_pin_clr4: IFLERRPINCLR4,
    #[doc = "0x48 - Interrupt Enable Set/Status Register 4"]
    pub int_ena_set4: INTENASET4,
    #[doc = "0x4c - Interrupt Enable Clear/Status Register 4"]
    pub int_ena_clr4: INTENACLR4,
    #[doc = "0x50 - Interrupt Level Set/Status Register 4"]
    pub int_lvl_set4: INTLVLSET4,
    #[doc = "0x54 - Interrupt Level Clear/Status Register 4"]
    pub int_lvl_clr4: INTLVLCLR4,
    #[doc = "0x58 - Status Register 4"]
    pub stat4: STAT4,
    #[doc = "0x5c - Status Register 5"]
    pub stat5: STAT5,
    #[doc = "0x60 - Status Register 6"]
    pub stat6: STAT6,
    #[doc = "0x64 - Status Shadow Register5"]
    pub shdw_stat5: SHDWSTAT5,
    _reserved0: [u8; 24usize],
    #[doc = "0x80 - Influence Error Pin Set/Status Register 7"]
    pub ifl_err_pin_set7: IFLERRPINSET7,
    #[doc = "0x84 - Influence Error Pin Clear/Status Register 7"]
    pub ifl_err_pin_clr7: IFLERRPINCLR7,
    #[doc = "0x88 - Interrupt Enable Set/Status Register 7"]
    pub int_ena_set7: INTENASET7,
    #[doc = "0x8c - Interrupt Enable Clear/Status Register 7"]
    pub int_ena_clr7: INTENACLR7,
    #[doc = "0x90 - Interrupt Level Set/Status Register 7"]
    pub int_lvl_set7: INTLVLSET7,
    #[doc = "0x94 - Interrupt Level Clear/Status Register 7"]
    pub int_lvl_clr7: INTLVLCLR7,
    #[doc = "0x98 - Status Register 7"]
    pub stat7: STAT7,
    #[doc = "0x9c - Status Register 8"]
    pub stat8: STAT8,
    #[doc = "0xa0 - Status Register 9"]
    pub stat9: STAT9,
    #[doc = "0xa4 - Status Shadow Register8"]
    pub shdw_stat8: SHDWSTAT8,
    _reserved1: [u8; 24usize],
    #[doc = "0xc0 - Influence Error Pin Set/Status Register 10"]
    pub ifl_err_pin_set10: IFLERRPINSET10,
    #[doc = "0xc4 - Influence Error Pin Clear/Status Register 10"]
    pub ifl_err_pin_clr10: IFLERRPINCLR10,
    #[doc = "0xc8 - Interrupt Enable Set/Status Register 10"]
    pub int_ena_set10: INTENASET10,
    #[doc = "0xcc - Interrupt Enable Clear/Status Register 10"]
    pub int_ena_clr10: INTENACLR10,
    #[doc = "0xd0 - Interrupt Level Set/Status Register 10"]
    pub int_lvl_set10: INTLVLSET10,
    #[doc = "0xd4 - Interrupt Level Clear/Status Register 10"]
    pub int_lvl_clr10: INTLVLCLR10,
    #[doc = "0xd8 - Status Register 10"]
    pub stat10: STAT10,
    #[doc = "0xdc - Status Register 11"]
    pub stat11: STAT11,
    #[doc = "0xe0 - Status Register 12"]
    pub stat12: STAT12,
    #[doc = "0xe4 - Status Shadow Register11"]
    pub shdw_stat11: SHDWSTAT11,
}
#[doc = "Influence Error Pin Set/Status Register 1"]
pub struct IFLERRPINSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Influence Error Pin Set/Status Register 1"]
pub mod ifl_err_pin_set1;
#[doc = "Influence Error Pin Clear/Status Register 1"]
pub struct IFLERRPINCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Influence Error Pin Clear/Status Register 1"]
pub mod ifl_err_pin_clr1;
#[doc = "Interrupt Enable Set/Status Register 1"]
pub struct INTENASET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set/Status Register 1"]
pub mod int_ena_set1;
#[doc = "Interrupt Enable Clear/Status Register 1"]
pub struct INTENACLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear/Status Register 1"]
pub mod int_ena_clr1;
#[doc = "Interrupt Level Set/Status Register 1"]
pub struct INTLVLSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Level Set/Status Register 1"]
pub mod int_lvl_set1;
#[doc = "Interrupt Level Clear/Status Register 1"]
pub struct INTLVLCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Level Clear/Status Register 1"]
pub mod int_lvl_clr1;
#[doc = "Status Register 1"]
pub struct STAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 1"]
pub mod stat1;
#[doc = "Status Register 2"]
pub struct STAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 2"]
pub mod stat2;
#[doc = "Status Register 3"]
pub struct STAT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 3"]
pub mod stat3;
#[doc = "Error Pin Status Register"]
pub struct ERRPINSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Pin Status Register"]
pub mod err_pin_stat;
#[doc = "Interrupt Offset High Register"]
pub struct INTOFFSTHGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Offset High Register"]
pub mod int_offst_hgh;
#[doc = "Interrupt Offset Low Register"]
pub struct INTOFFSTLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Offset Low Register"]
pub mod int_offst_low;
#[doc = "Low-Time Counter Register"]
pub struct LTCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low-Time Counter Register"]
pub mod lt_cnt;
#[doc = "Low-Time Counter Preload Register"]
pub struct LTCNTPRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low-Time Counter Preload Register"]
pub mod lt_cnt_pre;
#[doc = "Error Key Register"]
pub struct ERRKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Key Register"]
pub mod err_key;
#[doc = "Status Shadow Register"]
pub struct SHDWSTAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Shadow Register"]
pub mod shdw_stat2;
#[doc = "Influence Error Pin Set/Status Register 4"]
pub struct IFLERRPINSET4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Influence Error Pin Set/Status Register 4"]
pub mod ifl_err_pin_set4;
#[doc = "Influence Error Pin Clear/Status Register 4"]
pub struct IFLERRPINCLR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Influence Error Pin Clear/Status Register 4"]
pub mod ifl_err_pin_clr4;
#[doc = "Interrupt Enable Set/Status Register 4"]
pub struct INTENASET4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set/Status Register 4"]
pub mod int_ena_set4;
#[doc = "Interrupt Enable Clear/Status Register 4"]
pub struct INTENACLR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear/Status Register 4"]
pub mod int_ena_clr4;
#[doc = "Interrupt Level Set/Status Register 4"]
pub struct INTLVLSET4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Level Set/Status Register 4"]
pub mod int_lvl_set4;
#[doc = "Interrupt Level Clear/Status Register 4"]
pub struct INTLVLCLR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Level Clear/Status Register 4"]
pub mod int_lvl_clr4;
#[doc = "Status Register 4"]
pub struct STAT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 4"]
pub mod stat4;
#[doc = "Status Register 5"]
pub struct STAT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 5"]
pub mod stat5;
#[doc = "Status Register 6"]
pub struct STAT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 6"]
pub mod stat6;
#[doc = "Status Shadow Register5"]
pub struct SHDWSTAT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Shadow Register5"]
pub mod shdw_stat5;
#[doc = "Influence Error Pin Set/Status Register 7"]
pub struct IFLERRPINSET7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Influence Error Pin Set/Status Register 7"]
pub mod ifl_err_pin_set7;
#[doc = "Influence Error Pin Clear/Status Register 7"]
pub struct IFLERRPINCLR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Influence Error Pin Clear/Status Register 7"]
pub mod ifl_err_pin_clr7;
#[doc = "Interrupt Enable Set/Status Register 7"]
pub struct INTENASET7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set/Status Register 7"]
pub mod int_ena_set7;
#[doc = "Interrupt Enable Clear/Status Register 7"]
pub struct INTENACLR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear/Status Register 7"]
pub mod int_ena_clr7;
#[doc = "Interrupt Level Set/Status Register 7"]
pub struct INTLVLSET7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Level Set/Status Register 7"]
pub mod int_lvl_set7;
#[doc = "Interrupt Level Clear/Status Register 7"]
pub struct INTLVLCLR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Level Clear/Status Register 7"]
pub mod int_lvl_clr7;
#[doc = "Status Register 7"]
pub struct STAT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 7"]
pub mod stat7;
#[doc = "Status Register 8"]
pub struct STAT8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 8"]
pub mod stat8;
#[doc = "Status Register 9"]
pub struct STAT9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 9"]
pub mod stat9;
#[doc = "Status Shadow Register8"]
pub struct SHDWSTAT8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Shadow Register8"]
pub mod shdw_stat8;
#[doc = "Influence Error Pin Set/Status Register 10"]
pub struct IFLERRPINSET10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Influence Error Pin Set/Status Register 10"]
pub mod ifl_err_pin_set10;
#[doc = "Influence Error Pin Clear/Status Register 10"]
pub struct IFLERRPINCLR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Influence Error Pin Clear/Status Register 10"]
pub mod ifl_err_pin_clr10;
#[doc = "Interrupt Enable Set/Status Register 10"]
pub struct INTENASET10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set/Status Register 10"]
pub mod int_ena_set10;
#[doc = "Interrupt Enable Clear/Status Register 10"]
pub struct INTENACLR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear/Status Register 10"]
pub mod int_ena_clr10;
#[doc = "Interrupt Level Set/Status Register 10"]
pub struct INTLVLSET10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Level Set/Status Register 10"]
pub mod int_lvl_set10;
#[doc = "Interrupt Level Clear/Status Register 10"]
pub struct INTLVLCLR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Level Clear/Status Register 10"]
pub mod int_lvl_clr10;
#[doc = "Status Register 10"]
pub struct STAT10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 10"]
pub mod stat10;
#[doc = "Status Register 11"]
pub struct STAT11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 11"]
pub mod stat11;
#[doc = "Status Register 12"]
pub struct STAT12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 12"]
pub mod stat12;
#[doc = "Status Shadow Register11"]
pub struct SHDWSTAT11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Shadow Register11"]
pub mod shdw_stat11;
