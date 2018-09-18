#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interconnect selftest status register"]
    pub stc_stat: STCSTAT,
    #[doc = "0x04 - Interconnect selftest control register"]
    pub stc_ctrl: STCCTRL,
    #[doc = "0x08 - Parity Error Status"]
    pub parity_err_stat: PARITYERRSTAT,
    #[doc = "0x0c - Transaction Error Status"]
    pub trans_err_stat: TRANSERRSTAT,
    #[doc = "0x10 - Transaction ID Error Status"]
    pub trans_id_err_stat: TRANSIDERRSTAT,
    #[doc = "0x14 - Transaction Signature Error Status"]
    pub trans_sig_err_stat: TRANSSIGERRSTAT,
    #[doc = "0x18 - Transaction Type Error Status"]
    pub trans_typ_err_stat: TRANSTYPERRSTAT,
    #[doc = "0x1c - Error User Parity"]
    pub err_user_parity: ERRUSERPARITY,
    #[doc = "0x20 - MasterID mistmatch status register"]
    pub serr_unexp_mid: SERRUNEXPMID,
    #[doc = "0x24 - Address mismatch status register"]
    pub serr_addr_decode: SERRADDRDECODE,
    #[doc = "0x28 - Parity on address status register"]
    pub serr_user_parity: SERRUSERPARITY,
}
#[doc = "Interconnect selftest status register"]
pub struct STCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interconnect selftest status register"]
pub mod stc_stat;
#[doc = "Interconnect selftest control register"]
pub struct STCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interconnect selftest control register"]
pub mod stc_ctrl;
#[doc = "Parity Error Status"]
pub struct PARITYERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Error Status"]
pub mod parity_err_stat;
#[doc = "Transaction Error Status"]
pub struct TRANSERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transaction Error Status"]
pub mod trans_err_stat;
#[doc = "Transaction ID Error Status"]
pub struct TRANSIDERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transaction ID Error Status"]
pub mod trans_id_err_stat;
#[doc = "Transaction Signature Error Status"]
pub struct TRANSSIGERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transaction Signature Error Status"]
pub mod trans_sig_err_stat;
#[doc = "Transaction Type Error Status"]
pub struct TRANSTYPERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transaction Type Error Status"]
pub mod trans_typ_err_stat;
#[doc = "Error User Parity"]
pub struct ERRUSERPARITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error User Parity"]
pub mod err_user_parity;
#[doc = "MasterID mistmatch status register"]
pub struct SERRUNEXPMID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MasterID mistmatch status register"]
pub mod serr_unexp_mid;
#[doc = "Address mismatch status register"]
pub struct SERRADDRDECODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address mismatch status register"]
pub mod serr_addr_decode;
#[doc = "Parity on address status register"]
pub struct SERRUSERPARITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity on address status register"]
pub mod serr_user_parity;
