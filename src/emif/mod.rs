#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Revision Code and Status Register"]
    pub rev_cd_stat: REVCDSTAT,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - Asynchronous 1 Configuration Register (CS0 space)"]
    pub async1cfg: ASYNC1CFG,
    #[doc = "0x14 - Asynchronous 2 Configuration Register (CS1 space)"]
    pub async2cfg: ASYNC2CFG,
    #[doc = "0x18 - Asynchronous 3 Configuration Register (CS2 space)"]
    pub async3cfg: ASYNC3CFG,
    #[doc = "0x1c - Asynchronous 4 Configuration Register (CS3 space)"]
    pub async4cfg: ASYNC4CFG,
    #[doc = "0x20 - SDRAM Timing Register"]
    pub sdram_timing: SDRAMTIMING,
    #[doc = "0x24 - SDRAM Status Register"]
    pub sdram_stat: SDRAMSTAT,
    #[doc = "0x28 - DDR PHY Control Register"]
    pub ddr_phy_ctrl: DDRPHYCTRL,
    #[doc = "0x2c - DDR PHY Status Register"]
    pub ddr_phy_stat: DDRPHYSTAT,
    #[doc = "0x30 - Total SDRAM Accesses Register"]
    pub sdram_access: SDRAMACCESS,
    #[doc = "0x34 - Total SDRAM Activate Register"]
    pub sdram_activat: SDRAMACTIVAT,
    #[doc = "0x38 - DDR PHY ID and Revision Register"]
    pub ddr_phy_id_rev: DDRPHYIDREV,
    #[doc = "0x3c - SDRAM SR/PD Exit Timing Register"]
    pub sdram_sr_ex_timing: SDRAMSREXTIMING,
    #[doc = "0x40 - Interrupt Raw Register"]
    pub int_raw: INTRAW,
    #[doc = "0x44 - Interrupt Masked Register"]
    pub int_mask: INTMASK,
    #[doc = "0x48 - Interrupt Mask Set Register"]
    pub int_mask_set: INTMASKSET,
    #[doc = "0x4c - Interrupt Mask Clear Register"]
    pub int_mask_clr: INTMASKCLR,
    #[doc = "0x50 - IO Control Register"]
    pub io_ctrl: IOCTRL,
    #[doc = "0x54 - IO Status Register"]
    pub io_stat: IOSTAT,
    #[doc = "0x58 - SDRAM Config 2 Register"]
    pub sdram_config2: SDRAMCONFIG2,
    #[doc = "0x5c - NAND Flash Control Register"]
    pub one_nand_flash_ctrl: ONENANDFLASHCTRL,
    #[doc = "0x60 - NAND Flash Control Register"]
    pub nand_flash_ctrl: NANDFLASHCTRL,
    #[doc = "0x64 - NAND Flash Status Register"]
    pub nand_flash_stat: NANDFLASHSTAT,
    #[doc = "0x68 - Page Mode Control Register"]
    pub page_mod_ctrl: PAGEMODCTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x70 - NAND Flash CS2 1-Bit ECC Register"]
    pub nand_flash_cs2ecc: NANDFLASHCS2ECC,
    #[doc = "0x74 - NAND Flash CS3 1-Bit ECC Register"]
    pub nand_flash_cs3ecc: NANDFLASHCS3ECC,
    #[doc = "0x78 - NAND Flash CS4 1-Bit ECC Register"]
    pub nand_flash_cs4ecc: NANDFLASHCS4ECC,
    #[doc = "0x7c - NAND Flash CS2 1-Bit ECC Register"]
    pub nand_flash_cs5ecc: NANDFLASHCS5ECC,
    _reserved2: [u8; 4usize],
    #[doc = "0x84 - IODFT Test Logic Execution Counter Register"]
    pub io_dft_exec_count: IODFTEXECCOUNT,
    #[doc = "0x88 - IODFT Test Logic Global Control Register"]
    pub io_dft_glob_ctrl: IODFTGLOBCTRL,
    _reserved3: [u8; 4usize],
    #[doc = "0x90 - IODFT Test Logic Address MISR Result Register"]
    pub io_dft_addr_misr_result: IODFTADDRMISRRESULT,
    #[doc = "0x94 - IODFT Test Logic Data MISR Result Register"]
    pub io_dft_data_misr_result: IODFTDATAMISRRESULT,
    #[doc = "0x98 - IODFT Test Logic Data and Control MISR Result"]
    pub io_dft_ctrl_result: IODFTCTRLRESULT,
    _reserved4: [u8; 20usize],
    #[doc = "0xb0 - Module Release Number Register"]
    pub module_rev: MODULEREV,
    _reserved5: [u8; 8usize],
    #[doc = "0xbc - NAND Flash 4-Bit ECC Load Register"]
    pub nand_flash_ecc_load: NANDFLASHECCLOAD,
    #[doc = "0xc0 - NAND Flash 4-Bit ECC 1 Register"]
    pub nand_flash_ecc1: NANDFLASHECC1,
    #[doc = "0xc4 - NAND Flash 4-Bit ECC 2 Register"]
    pub nand_flash_ecc2: NANDFLASHECC2,
    #[doc = "0xc8 - NAND Flash 4-Bit ECC 3 Register"]
    pub nand_flash_ecc3: NANDFLASHECC3,
    #[doc = "0xcc - NAND Flash 4-Bit ECC 4 Register"]
    pub nand_flash_ecc4: NANDFLASHECC4,
    #[doc = "0xd0 - NAND Flash Error Address 1 Register"]
    pub nand_flash_err_addr1: NANDFLASHERRADDR1,
    #[doc = "0xd4 - NAND Flash Error Address 2 Register"]
    pub nand_flash_err_addr2: NANDFLASHERRADDR2,
    #[doc = "0xd8 - NAND Flash Error Address 3 Register"]
    pub nand_flash_err_addr3: NANDFLASHERRADDR3,
    #[doc = "0xdc - NAND Flash Error Address 4 Register"]
    pub nand_flash_err_addr4: NANDFLASHERRADDR4,
}
#[doc = "Revision Code and Status Register"]
pub struct REVCDSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Revision Code and Status Register"]
pub mod rev_cd_stat;
#[doc = "Asynchronous 1 Configuration Register (CS0 space)"]
pub struct ASYNC1CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Asynchronous 1 Configuration Register (CS0 space)"]
pub mod async1cfg;
#[doc = "Asynchronous 2 Configuration Register (CS1 space)"]
pub struct ASYNC2CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Asynchronous 2 Configuration Register (CS1 space)"]
pub mod async2cfg;
#[doc = "Asynchronous 3 Configuration Register (CS2 space)"]
pub struct ASYNC3CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Asynchronous 3 Configuration Register (CS2 space)"]
pub mod async3cfg;
#[doc = "Asynchronous 4 Configuration Register (CS3 space)"]
pub struct ASYNC4CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Asynchronous 4 Configuration Register (CS3 space)"]
pub mod async4cfg;
#[doc = "SDRAM Timing Register"]
pub struct SDRAMTIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM Timing Register"]
pub mod sdram_timing;
#[doc = "SDRAM Status Register"]
pub struct SDRAMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM Status Register"]
pub mod sdram_stat;
#[doc = "DDR PHY Control Register"]
pub struct DDRPHYCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDR PHY Control Register"]
pub mod ddr_phy_ctrl;
#[doc = "DDR PHY Status Register"]
pub struct DDRPHYSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDR PHY Status Register"]
pub mod ddr_phy_stat;
#[doc = "Total SDRAM Accesses Register"]
pub struct SDRAMACCESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total SDRAM Accesses Register"]
pub mod sdram_access;
#[doc = "Total SDRAM Activate Register"]
pub struct SDRAMACTIVAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total SDRAM Activate Register"]
pub mod sdram_activat;
#[doc = "DDR PHY ID and Revision Register"]
pub struct DDRPHYIDREV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDR PHY ID and Revision Register"]
pub mod ddr_phy_id_rev;
#[doc = "SDRAM SR/PD Exit Timing Register"]
pub struct SDRAMSREXTIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM SR/PD Exit Timing Register"]
pub mod sdram_sr_ex_timing;
#[doc = "Interrupt Raw Register"]
pub struct INTRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Raw Register"]
pub mod int_raw;
#[doc = "Interrupt Masked Register"]
pub struct INTMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Masked Register"]
pub mod int_mask;
#[doc = "Interrupt Mask Set Register"]
pub struct INTMASKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Set Register"]
pub mod int_mask_set;
#[doc = "Interrupt Mask Clear Register"]
pub struct INTMASKCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Clear Register"]
pub mod int_mask_clr;
#[doc = "IO Control Register"]
pub struct IOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Control Register"]
pub mod io_ctrl;
#[doc = "IO Status Register"]
pub struct IOSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Status Register"]
pub mod io_stat;
#[doc = "SDRAM Config 2 Register"]
pub struct SDRAMCONFIG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM Config 2 Register"]
pub mod sdram_config2;
#[doc = "NAND Flash Control Register"]
pub struct ONENANDFLASHCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash Control Register"]
pub mod one_nand_flash_ctrl;
#[doc = "NAND Flash Control Register"]
pub struct NANDFLASHCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash Control Register"]
pub mod nand_flash_ctrl;
#[doc = "NAND Flash Status Register"]
pub struct NANDFLASHSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash Status Register"]
pub mod nand_flash_stat;
#[doc = "Page Mode Control Register"]
pub struct PAGEMODCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Page Mode Control Register"]
pub mod page_mod_ctrl;
#[doc = "NAND Flash CS2 1-Bit ECC Register"]
pub struct NANDFLASHCS2ECC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash CS2 1-Bit ECC Register"]
pub mod nand_flash_cs2ecc;
#[doc = "NAND Flash CS3 1-Bit ECC Register"]
pub struct NANDFLASHCS3ECC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash CS3 1-Bit ECC Register"]
pub mod nand_flash_cs3ecc;
#[doc = "NAND Flash CS4 1-Bit ECC Register"]
pub struct NANDFLASHCS4ECC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash CS4 1-Bit ECC Register"]
pub mod nand_flash_cs4ecc;
#[doc = "NAND Flash CS2 1-Bit ECC Register"]
pub struct NANDFLASHCS5ECC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash CS2 1-Bit ECC Register"]
pub mod nand_flash_cs5ecc;
#[doc = "IODFT Test Logic Execution Counter Register"]
pub struct IODFTEXECCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IODFT Test Logic Execution Counter Register"]
pub mod io_dft_exec_count;
#[doc = "IODFT Test Logic Global Control Register"]
pub struct IODFTGLOBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IODFT Test Logic Global Control Register"]
pub mod io_dft_glob_ctrl;
#[doc = "IODFT Test Logic Address MISR Result Register"]
pub struct IODFTADDRMISRRESULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IODFT Test Logic Address MISR Result Register"]
pub mod io_dft_addr_misr_result;
#[doc = "IODFT Test Logic Data MISR Result Register"]
pub struct IODFTDATAMISRRESULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IODFT Test Logic Data MISR Result Register"]
pub mod io_dft_data_misr_result;
#[doc = "IODFT Test Logic Data and Control MISR Result"]
pub struct IODFTCTRLRESULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IODFT Test Logic Data and Control MISR Result"]
pub mod io_dft_ctrl_result;
#[doc = "Module Release Number Register"]
pub struct MODULEREV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Release Number Register"]
pub mod module_rev;
#[doc = "NAND Flash 4-Bit ECC Load Register"]
pub struct NANDFLASHECCLOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash 4-Bit ECC Load Register"]
pub mod nand_flash_ecc_load;
#[doc = "NAND Flash 4-Bit ECC 1 Register"]
pub struct NANDFLASHECC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash 4-Bit ECC 1 Register"]
pub mod nand_flash_ecc1;
#[doc = "NAND Flash 4-Bit ECC 2 Register"]
pub struct NANDFLASHECC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash 4-Bit ECC 2 Register"]
pub mod nand_flash_ecc2;
#[doc = "NAND Flash 4-Bit ECC 3 Register"]
pub struct NANDFLASHECC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash 4-Bit ECC 3 Register"]
pub mod nand_flash_ecc3;
#[doc = "NAND Flash 4-Bit ECC 4 Register"]
pub struct NANDFLASHECC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash 4-Bit ECC 4 Register"]
pub mod nand_flash_ecc4;
#[doc = "NAND Flash Error Address 1 Register"]
pub struct NANDFLASHERRADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash Error Address 1 Register"]
pub mod nand_flash_err_addr1;
#[doc = "NAND Flash Error Address 2 Register"]
pub struct NANDFLASHERRADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash Error Address 2 Register"]
pub mod nand_flash_err_addr2;
#[doc = "NAND Flash Error Address 3 Register"]
pub struct NANDFLASHERRADDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash Error Address 3 Register"]
pub mod nand_flash_err_addr3;
#[doc = "NAND Flash Error Address 4 Register"]
pub struct NANDFLASHERRADDR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND Flash Error Address 4 Register"]
pub mod nand_flash_err_addr4;
