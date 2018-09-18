#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EPC REVID Register"]
    pub epc_rev_id: EPCREVID,
    #[doc = "0x04 - EPC Control Register"]
    pub epc_cntrl: EPCCNTRL,
    #[doc = "0x08 - Uncorrectable Error Status Register"]
    pub uerr_stat: UERRSTAT,
    #[doc = "0x0c - H/W Channel Enable Set and Status Register"]
    pub epc_err_stat: EPCERRSTAT,
    #[doc = "0x10 - FIFO Full Status Register"]
    pub fifo_full_stat: FIFOFULLSTAT,
    #[doc = "0x14 - IP Interface FIFO Overflow Status Register"]
    pub ovr_flw_stat: OVRFLWSTAT,
    #[doc = "0x18 - CAM Index Available Status Register"]
    pub cam_avail_stat: CAMAVAILSTAT,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Uncorrectable Error Address Register 0"]
    pub uerr_addr0: UERRADDR0,
    #[doc = "0x24 - Uncorrectable Error Address Register 1"]
    pub uerr_addr1: UERRADDR1,
    #[doc = "0x28 - Uncorrectable Error Address Register 2"]
    pub uerr_addr2: UERRADDR2,
    #[doc = "0x2c - Uncorrectable Error Address Register 3"]
    pub uerr_addr3: UERRADDR3,
    #[doc = "0x30 - Uncorrectable Error Address Register 4"]
    pub uerr_addr4: UERRADDR4,
    #[doc = "0x34 - Uncorrectable Error Address Register 5"]
    pub uerr_addr5: UERRADDR5,
    #[doc = "0x38 - Uncorrectable Error Address Register 6"]
    pub uerr_addr6: UERRADDR6,
    #[doc = "0x3c - Uncorrectable Error Address Register 7"]
    pub uerr_addr7: UERRADDR7,
    #[doc = "0x40 - Uncorrectable Error Address Register 8"]
    pub uerr_addr8: UERRADDR8,
    #[doc = "0x44 - Uncorrectable Error Address Register 9"]
    pub uerr_addr9: UERRADDR9,
    #[doc = "0x48 - Uncorrectable Error Address Register 10"]
    pub uerr_addr10: UERRADDR10,
    #[doc = "0x4c - Uncorrectable Error Address Register 11"]
    pub uerr_addr11: UERRADDR11,
    #[doc = "0x50 - Uncorrectable Error Address Register 12"]
    pub uerr_addr12: UERRADDR12,
    #[doc = "0x54 - Uncorrectable Error Address Register 13"]
    pub uerr_addr13: UERRADDR13,
    #[doc = "0x58 - Uncorrectable Error Address Register 14"]
    pub uerr_addr14: UERRADDR14,
    #[doc = "0x5c - Uncorrectable Error Address Register 15"]
    pub uerr_addr15: UERRADDR15,
    _reserved1: [u8; 64usize],
    #[doc = "0xa0 - CAM Content Update Register0"]
    pub cam_content0: CAM_CONTENT0,
    #[doc = "0xa4 - CAM Content Update Register1"]
    pub cam_content1: CAM_CONTENT1,
    #[doc = "0xa8 - CAM Content Update Register2"]
    pub cam_content2: CAM_CONTENT2,
    #[doc = "0xac - CAM Content Update Register3"]
    pub cam_content3: CAM_CONTENT3,
    #[doc = "0xb0 - CAM Content Update Register4"]
    pub cam_content4: CAM_CONTENT4,
    #[doc = "0xb4 - CAM Content Update Register5"]
    pub cam_content5: CAM_CONTENT5,
    #[doc = "0xb8 - CAM Content Update Register6"]
    pub cam_content6: CAM_CONTENT6,
    #[doc = "0xbc - CAM Content Update Register7"]
    pub cam_content7: CAM_CONTENT7,
    #[doc = "0xc0 - CAM Content Update Register8"]
    pub cam_content8: CAM_CONTENT8,
    #[doc = "0xc4 - CAM Content Update Register9"]
    pub cam_content9: CAM_CONTENT9,
    #[doc = "0xc8 - CAM Content Update Register10"]
    pub cam_content10: CAM_CONTENT10,
    #[doc = "0xcc - CAM Content Update Register11"]
    pub cam_content11: CAM_CONTENT11,
    #[doc = "0xd0 - CAM Content Update Register12"]
    pub cam_content12: CAM_CONTENT12,
    #[doc = "0xd4 - CAM Content Update Register13"]
    pub cam_content13: CAM_CONTENT13,
    #[doc = "0xd8 - CAM Content Update Register14"]
    pub cam_content14: CAM_CONTENT14,
    #[doc = "0xdc - CAM Content Update Register15"]
    pub cam_content15: CAM_CONTENT15,
    #[doc = "0xe0 - CAM Content Update Register16"]
    pub cam_content16: CAM_CONTENT16,
    #[doc = "0xe4 - CAM Content Update Register17"]
    pub cam_content17: CAM_CONTENT17,
    #[doc = "0xe8 - CAM Content Update Register18"]
    pub cam_content18: CAM_CONTENT18,
    #[doc = "0xec - CAM Content Update Register19"]
    pub cam_content19: CAM_CONTENT19,
    #[doc = "0xf0 - CAM Content Update Register20"]
    pub cam_content20: CAM_CONTENT20,
    #[doc = "0xf4 - CAM Content Update Register21"]
    pub cam_content21: CAM_CONTENT21,
    #[doc = "0xf8 - CAM Content Update Register22"]
    pub cam_content22: CAM_CONTENT22,
    #[doc = "0xfc - CAM Content Update Register23"]
    pub cam_content23: CAM_CONTENT23,
    #[doc = "0x100 - CAM Content Update Register24"]
    pub cam_content24: CAM_CONTENT24,
    #[doc = "0x104 - CAM Content Update Register25"]
    pub cam_content25: CAM_CONTENT25,
    #[doc = "0x108 - CAM Content Update Register26"]
    pub cam_content26: CAM_CONTENT26,
    #[doc = "0x10c - CAM Content Update Register27"]
    pub cam_content27: CAM_CONTENT27,
    #[doc = "0x110 - CAM Content Update Register28"]
    pub cam_content28: CAM_CONTENT28,
    #[doc = "0x114 - CAM Content Update Register29"]
    pub cam_content29: CAM_CONTENT29,
    #[doc = "0x118 - CAM Content Update Register30"]
    pub cam_content30: CAM_CONTENT30,
    #[doc = "0x11c - CAM Content Update Register31"]
    pub cam_content31: CAM_CONTENT31,
    _reserved2: [u8; 224usize],
    #[doc = "0x200 - CAM Index Register0"]
    pub cam_index0: CAM_INDEX0,
    #[doc = "0x204 - CAM Index Register1"]
    pub cam_index1: CAM_INDEX1,
    #[doc = "0x208 - CAM Index Register2"]
    pub cam_index2: CAM_INDEX2,
    #[doc = "0x20c - CAM Index Register3"]
    pub cam_index3: CAM_INDEX3,
    #[doc = "0x210 - CAM Index Register4"]
    pub cam_index4: CAM_INDEX4,
    #[doc = "0x214 - CAM Index Register5"]
    pub cam_index5: CAM_INDEX5,
    #[doc = "0x218 - CAM Index Register6"]
    pub cam_index6: CAM_INDEX6,
    #[doc = "0x21c - CAM Index Register7"]
    pub cam_index7: CAM_INDEX7,
}
#[doc = "EPC REVID Register"]
pub struct EPCREVID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPC REVID Register"]
pub mod epc_rev_id;
#[doc = "EPC Control Register"]
pub struct EPCCNTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPC Control Register"]
pub mod epc_cntrl;
#[doc = "Uncorrectable Error Status Register"]
pub struct UERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Status Register"]
pub mod uerr_stat;
#[doc = "H/W Channel Enable Set and Status Register"]
pub struct EPCERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "H/W Channel Enable Set and Status Register"]
pub mod epc_err_stat;
#[doc = "FIFO Full Status Register"]
pub struct FIFOFULLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Full Status Register"]
pub mod fifo_full_stat;
#[doc = "IP Interface FIFO Overflow Status Register"]
pub struct OVRFLWSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Interface FIFO Overflow Status Register"]
pub mod ovr_flw_stat;
#[doc = "CAM Index Available Status Register"]
pub struct CAMAVAILSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Index Available Status Register"]
pub mod cam_avail_stat;
#[doc = "Uncorrectable Error Address Register 0"]
pub struct UERRADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 0"]
pub mod uerr_addr0;
#[doc = "Uncorrectable Error Address Register 1"]
pub struct UERRADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 1"]
pub mod uerr_addr1;
#[doc = "Uncorrectable Error Address Register 2"]
pub struct UERRADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 2"]
pub mod uerr_addr2;
#[doc = "Uncorrectable Error Address Register 3"]
pub struct UERRADDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 3"]
pub mod uerr_addr3;
#[doc = "Uncorrectable Error Address Register 4"]
pub struct UERRADDR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 4"]
pub mod uerr_addr4;
#[doc = "Uncorrectable Error Address Register 5"]
pub struct UERRADDR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 5"]
pub mod uerr_addr5;
#[doc = "Uncorrectable Error Address Register 6"]
pub struct UERRADDR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 6"]
pub mod uerr_addr6;
#[doc = "Uncorrectable Error Address Register 7"]
pub struct UERRADDR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 7"]
pub mod uerr_addr7;
#[doc = "Uncorrectable Error Address Register 8"]
pub struct UERRADDR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 8"]
pub mod uerr_addr8;
#[doc = "Uncorrectable Error Address Register 9"]
pub struct UERRADDR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 9"]
pub mod uerr_addr9;
#[doc = "Uncorrectable Error Address Register 10"]
pub struct UERRADDR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 10"]
pub mod uerr_addr10;
#[doc = "Uncorrectable Error Address Register 11"]
pub struct UERRADDR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 11"]
pub mod uerr_addr11;
#[doc = "Uncorrectable Error Address Register 12"]
pub struct UERRADDR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 12"]
pub mod uerr_addr12;
#[doc = "Uncorrectable Error Address Register 13"]
pub struct UERRADDR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 13"]
pub mod uerr_addr13;
#[doc = "Uncorrectable Error Address Register 14"]
pub struct UERRADDR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 14"]
pub mod uerr_addr14;
#[doc = "Uncorrectable Error Address Register 15"]
pub struct UERRADDR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Error Address Register 15"]
pub mod uerr_addr15;
#[doc = "CAM Content Update Register0"]
pub struct CAM_CONTENT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register0"]
pub mod cam_content0;
#[doc = "CAM Content Update Register1"]
pub struct CAM_CONTENT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register1"]
pub mod cam_content1;
#[doc = "CAM Content Update Register2"]
pub struct CAM_CONTENT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register2"]
pub mod cam_content2;
#[doc = "CAM Content Update Register3"]
pub struct CAM_CONTENT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register3"]
pub mod cam_content3;
#[doc = "CAM Content Update Register4"]
pub struct CAM_CONTENT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register4"]
pub mod cam_content4;
#[doc = "CAM Content Update Register5"]
pub struct CAM_CONTENT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register5"]
pub mod cam_content5;
#[doc = "CAM Content Update Register6"]
pub struct CAM_CONTENT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register6"]
pub mod cam_content6;
#[doc = "CAM Content Update Register7"]
pub struct CAM_CONTENT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register7"]
pub mod cam_content7;
#[doc = "CAM Content Update Register8"]
pub struct CAM_CONTENT8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register8"]
pub mod cam_content8;
#[doc = "CAM Content Update Register9"]
pub struct CAM_CONTENT9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register9"]
pub mod cam_content9;
#[doc = "CAM Content Update Register10"]
pub struct CAM_CONTENT10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register10"]
pub mod cam_content10;
#[doc = "CAM Content Update Register11"]
pub struct CAM_CONTENT11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register11"]
pub mod cam_content11;
#[doc = "CAM Content Update Register12"]
pub struct CAM_CONTENT12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register12"]
pub mod cam_content12;
#[doc = "CAM Content Update Register13"]
pub struct CAM_CONTENT13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register13"]
pub mod cam_content13;
#[doc = "CAM Content Update Register14"]
pub struct CAM_CONTENT14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register14"]
pub mod cam_content14;
#[doc = "CAM Content Update Register15"]
pub struct CAM_CONTENT15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register15"]
pub mod cam_content15;
#[doc = "CAM Content Update Register16"]
pub struct CAM_CONTENT16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register16"]
pub mod cam_content16;
#[doc = "CAM Content Update Register17"]
pub struct CAM_CONTENT17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register17"]
pub mod cam_content17;
#[doc = "CAM Content Update Register18"]
pub struct CAM_CONTENT18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register18"]
pub mod cam_content18;
#[doc = "CAM Content Update Register19"]
pub struct CAM_CONTENT19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register19"]
pub mod cam_content19;
#[doc = "CAM Content Update Register20"]
pub struct CAM_CONTENT20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register20"]
pub mod cam_content20;
#[doc = "CAM Content Update Register21"]
pub struct CAM_CONTENT21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register21"]
pub mod cam_content21;
#[doc = "CAM Content Update Register22"]
pub struct CAM_CONTENT22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register22"]
pub mod cam_content22;
#[doc = "CAM Content Update Register23"]
pub struct CAM_CONTENT23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register23"]
pub mod cam_content23;
#[doc = "CAM Content Update Register24"]
pub struct CAM_CONTENT24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register24"]
pub mod cam_content24;
#[doc = "CAM Content Update Register25"]
pub struct CAM_CONTENT25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register25"]
pub mod cam_content25;
#[doc = "CAM Content Update Register26"]
pub struct CAM_CONTENT26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register26"]
pub mod cam_content26;
#[doc = "CAM Content Update Register27"]
pub struct CAM_CONTENT27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register27"]
pub mod cam_content27;
#[doc = "CAM Content Update Register28"]
pub struct CAM_CONTENT28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register28"]
pub mod cam_content28;
#[doc = "CAM Content Update Register29"]
pub struct CAM_CONTENT29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register29"]
pub mod cam_content29;
#[doc = "CAM Content Update Register30"]
pub struct CAM_CONTENT30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register30"]
pub mod cam_content30;
#[doc = "CAM Content Update Register31"]
pub struct CAM_CONTENT31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Content Update Register31"]
pub mod cam_content31;
#[doc = "CAM Index Register0"]
pub struct CAM_INDEX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Index Register0"]
pub mod cam_index0;
#[doc = "CAM Index Register1"]
pub struct CAM_INDEX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Index Register1"]
pub mod cam_index1;
#[doc = "CAM Index Register2"]
pub struct CAM_INDEX2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Index Register2"]
pub mod cam_index2;
#[doc = "CAM Index Register3"]
pub struct CAM_INDEX3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Index Register3"]
pub mod cam_index3;
#[doc = "CAM Index Register4"]
pub struct CAM_INDEX4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Index Register4"]
pub mod cam_index4;
#[doc = "CAM Index Register5"]
pub struct CAM_INDEX5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Index Register5"]
pub mod cam_index5;
#[doc = "CAM Index Register6"]
pub struct CAM_INDEX6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Index Register6"]
pub mod cam_index6;
#[doc = "CAM Index Register7"]
pub struct CAM_INDEX7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAM Index Register7"]
pub mod cam_index7;
