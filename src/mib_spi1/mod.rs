#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global control register 0"]
    pub glb_ctrl0: GLBCTRL0,
    #[doc = "0x04 - Global control register 1"]
    pub glb_ctrl1: GLBCTRL1,
    #[doc = "0x08 - Interrupt Register"]
    pub int0: INT0,
    #[doc = "0x0c - Interrupt Level Register"]
    pub int_lvl: INTLVL,
    #[doc = "0x10 - Flag Register"]
    pub int_flg: INTFLG,
    #[doc = "0x14 - Pin Control 0"]
    pub fun: FUN,
    #[doc = "0x18 - Pin Control 1"]
    pub dir: DIR,
    #[doc = "0x1c - Pin Control 2"]
    pub din: DIN,
    #[doc = "0x20 - Pin Control 3"]
    pub dout: DOUT,
    #[doc = "0x24 - Pin Control 4"]
    pub dset: DSET,
    #[doc = "0x28 - Pin Control 5"]
    pub dclr: DCLR,
    #[doc = "0x2c - Pin Control 6"]
    pub pdr: PDR,
    #[doc = "0x30 - Pin Control 7"]
    pub pdis: PDIS,
    #[doc = "0x34 - Pin Control 8"]
    pub psel: PSEL,
    #[doc = "0x38 - Transmit Data Register 0"]
    pub tx_dat0: TXDAT0,
    #[doc = "0x3c - Transmit Data Register 1"]
    pub tx_dat1: TXDAT1,
    #[doc = "0x40 - Receive Buffer Register"]
    pub rx_buf: RXBUF,
    #[doc = "0x44 - Emulation Register"]
    pub emu: EMU,
    #[doc = "0x48 - Delay Register"]
    pub delay: DELAY,
    #[doc = "0x4c - Default Chip select Register"]
    pub def_cs: DEFCS,
    #[doc = "0x50 - Data Format Register 0"]
    pub dat_fmt0: DATFMT0,
    #[doc = "0x54 - Data Format Register 1"]
    pub dat_fmt1: DATFMT1,
    #[doc = "0x58 - Data Format Register 2"]
    pub dat_fmt2: DATFMT2,
    #[doc = "0x5c - Data Format Register 3"]
    pub dat_fmt3: DATFMT3,
    #[doc = "0x60 - Transfer Group Interrupt Vector Register 0"]
    pub tg_int_vec0: TGINTVEC0,
    #[doc = "0x64 - Transfer Group Interrupt Vector Register 1"]
    pub tg_int_vec1: TGINTVEC1,
    #[doc = "0x68 - Pin Control Register 9"]
    pub sr_sel: SRSEL,
    #[doc = "0x6c - Parallel/Modulo Mode Control Register"]
    pub pm_ctrl: PMCTRL,
    #[doc = "0x70 - MibSPI Enable Register"]
    pub mib_spi_ena: MIBSPIENA,
    #[doc = "0x74 - MibSPI Transfer Group Interrupt Enable Set Register"]
    pub tg_int_ena_set: TGINTENASET,
    #[doc = "0x78 - MibSPI Transfer Group Interrupt Enable Clear Register"]
    pub tg_int_ena_clr: TGINTENACLR,
    #[doc = "0x7c - MibSPI Transfer Group Interrupt Level Set Register"]
    pub tg_int_lvl_set: TGINTLVLSET,
    #[doc = "0x80 - MibSPI Transfer Group Interrupt Level Clear Register"]
    pub tg_int_lvl_clr: TGINTLVLCLR,
    #[doc = "0x84 - Transfer Group Interrupt Flag Register"]
    pub tg_int_flg: TGINTFLG,
    _reserved0: [u8; 8usize],
    #[doc = "0x90 - Tick Cnt Register"]
    pub tick_cnt: TICKCNT,
    #[doc = "0x94 - Last Transfer Group End Pointer"]
    pub ltg_pend: LTGPEND,
    #[doc = "0x98 - MibSPI Transfer Group Control Register 0"]
    pub tg0ctrl: TG0CTRL,
    #[doc = "0x9c - MibSPI Transfer Group Control Register 1"]
    pub tg1ctrl: TG1CTRL,
    #[doc = "0xa0 - MibSPI Transfer Group Control Register 2"]
    pub tg2ctrl: TG2CTRL,
    #[doc = "0xa4 - MibSPI Transfer Group Control Register 3"]
    pub tg3ctrl: TG3CTRL,
    #[doc = "0xa8 - MibSPI Transfer Group Control Register 4"]
    pub tg4ctrl: TG4CTRL,
    #[doc = "0xac - MibSPI Transfer Group Control Register 5"]
    pub tg5ctrl: TG5CTRL,
    #[doc = "0xb0 - MibSPI Transfer Group Control Register 6"]
    pub tg6ctrl: TG6CTRL,
    #[doc = "0xb4 - MibSPI Transfer Group Control Register 7"]
    pub tg7ctrl: TG7CTRL,
    #[doc = "0xb8 - MibSPI Transfer Group Control Register 8"]
    pub tg8ctrl: TG8CTRL,
    #[doc = "0xbc - MibSPI Transfer Group Control Register 9"]
    pub tg9ctrl: TG9CTRL,
    #[doc = "0xc0 - MibSPI Transfer Group Control Register 10"]
    pub tg10ctrl: TG10CTRL,
    #[doc = "0xc4 - MibSPI Transfer Group Control Register 11"]
    pub tg11ctrl: TG11CTRL,
    #[doc = "0xc8 - MibSPI Transfer Group Control Register 12"]
    pub tg12ctrl: TG12CTRL,
    #[doc = "0xcc - MibSPI Transfer Group Control Register 13"]
    pub tg13ctrl: TG13CTRL,
    #[doc = "0xd0 - MibSPI Transfer Group Control Register 14"]
    pub tg14ctrl: TG14CTRL,
    #[doc = "0xd4 - MibSPI Transfer Group Control Register 15"]
    pub tg15ctrl: TG15CTRL,
    #[doc = "0xd8 - MibSPI Dma Channel Control Register 0"]
    pub dma0ctrl: DMA0CTRL,
    #[doc = "0xdc - MibSPI Dma Channel Control Register 1"]
    pub dma1ctrl: DMA1CTRL,
    #[doc = "0xe0 - MibSPI Dma Channel Control Register 2"]
    pub dma2ctrl: DMA2CTRL,
    #[doc = "0xe4 - MibSPI Dma Channel Control Register 3"]
    pub dma3ctrl: DMA3CTRL,
    #[doc = "0xe8 - MibSPI Dma Channel Control Register 4"]
    pub dma4ctrl: DMA4CTRL,
    #[doc = "0xec - MibSPI Dma Channel Control Register 5"]
    pub dma5ctrl: DMA5CTRL,
    #[doc = "0xf0 - MibSPI Dma Channel Control Register 6"]
    pub dma6ctrl: DMA6CTRL,
    #[doc = "0xf4 - MibSPI Dma Channel Control Register 7"]
    pub dma7ctrl: DMA7CTRL,
    #[doc = "0xf8 - ICnt Register 0"]
    pub dma0cnt: DMA0CNT,
    #[doc = "0xfc - ICnt Register 1"]
    pub dma1cnt: DMA1CNT,
    #[doc = "0x100 - ICnt Register 2"]
    pub dma2cnt: DMA2CNT,
    #[doc = "0x104 - ICnt Register 3"]
    pub dma3cnt: DMA3CNT,
    #[doc = "0x108 - ICnt Register 4"]
    pub dma4cnt: DMA4CNT,
    #[doc = "0x10c - ICnt Register 5"]
    pub dma5cnt: DMA5CNT,
    #[doc = "0x110 - ICnt Register 6"]
    pub dma6cnt: DMA6CNT,
    #[doc = "0x114 - ICnt Register 7"]
    pub dma7cnt: DMA7CNT,
    #[doc = "0x118 - Dma LARGE Cnt register"]
    pub dma_cnt_len: DMACNTLEN,
    _reserved1: [u8; 4usize],
    #[doc = "0x120 - Uncorrectable Parity Error Control Register"]
    pub uerr_ctrl: UERRCTRL,
    #[doc = "0x124 - Uncorrectable Parity Error Status Register"]
    pub uerr_stat: UERRSTAT,
    #[doc = "0x128 - Uncorrectable Parity Error Address Register"]
    pub uerr_addr1: UERRADDR1,
    #[doc = "0x12c - Uncorrectable Parity Error Address Register"]
    pub uerr_addr0: UERRADDR0,
    #[doc = "0x130 - Receive RAM Overrun Buffer Address Register"]
    pub rx_ovr_nbuf_addr: RXOVRNBUFADDR,
    #[doc = "0x134 - IO Loopback Test Control Register"]
    pub io_lpbk_tst_ctrl: IOLPBKTSTCTRL,
    #[doc = "0x138 - Extended Prescale Register 1"]
    pub extended_prescale1: EXTENDED_PRESCALE1,
    #[doc = "0x13c - Extended Prescale Register 2"]
    pub extended_prescale2: EXTENDED_PRESCALE2,
    #[doc = "0x140 - ECC Control register"]
    pub eccdiag_ctrl: ECCDIAG_CTRL,
    #[doc = "0x144 - ECC Diagnostic Status register"]
    pub eccdiag_stat: ECCDIAG_STAT,
    #[doc = "0x148 - Single Bit Error Address for RXRAM"]
    pub sbit_err_addr1: SBITERRADDR1,
    #[doc = "0x14c - Single Bit Error Address for TXRAM"]
    pub sbit_err_addr0: SBITERRADDR0,
    _reserved2: [u8; 172usize],
    #[doc = "0x1fc - Revision ID Register"]
    pub spirev: SPIREV,
}
#[doc = "Global control register 0"]
pub struct GLBCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global control register 0"]
pub mod glb_ctrl0;
#[doc = "Global control register 1"]
pub struct GLBCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global control register 1"]
pub mod glb_ctrl1;
#[doc = "Interrupt Register"]
pub struct INT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register"]
pub mod int0;
#[doc = "Interrupt Level Register"]
pub struct INTLVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Level Register"]
pub mod int_lvl;
#[doc = "Flag Register"]
pub struct INTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flag Register"]
pub mod int_flg;
#[doc = "Pin Control 0"]
pub struct FUN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 0"]
pub mod fun;
#[doc = "Pin Control 1"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 1"]
pub mod dir;
#[doc = "Pin Control 2"]
pub struct DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 2"]
pub mod din;
#[doc = "Pin Control 3"]
pub struct DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 3"]
pub mod dout;
#[doc = "Pin Control 4"]
pub struct DSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 4"]
pub mod dset;
#[doc = "Pin Control 5"]
pub struct DCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 5"]
pub mod dclr;
#[doc = "Pin Control 6"]
pub struct PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 6"]
pub mod pdr;
#[doc = "Pin Control 7"]
pub struct PDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 7"]
pub mod pdis;
#[doc = "Pin Control 8"]
pub struct PSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control 8"]
pub mod psel;
#[doc = "Transmit Data Register 0"]
pub struct TXDAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Register 0"]
pub mod tx_dat0;
#[doc = "Transmit Data Register 1"]
pub struct TXDAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Register 1"]
pub mod tx_dat1;
#[doc = "Receive Buffer Register"]
pub struct RXBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Register"]
pub mod rx_buf;
#[doc = "Emulation Register"]
pub struct EMU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Emulation Register"]
pub mod emu;
#[doc = "Delay Register"]
pub struct DELAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Delay Register"]
pub mod delay;
#[doc = "Default Chip select Register"]
pub struct DEFCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Default Chip select Register"]
pub mod def_cs;
#[doc = "Data Format Register 0"]
pub struct DATFMT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Format Register 0"]
pub mod dat_fmt0;
#[doc = "Data Format Register 1"]
pub struct DATFMT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Format Register 1"]
pub mod dat_fmt1;
#[doc = "Data Format Register 2"]
pub struct DATFMT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Format Register 2"]
pub mod dat_fmt2;
#[doc = "Data Format Register 3"]
pub struct DATFMT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Format Register 3"]
pub mod dat_fmt3;
#[doc = "Transfer Group Interrupt Vector Register 0"]
pub struct TGINTVEC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Group Interrupt Vector Register 0"]
pub mod tg_int_vec0;
#[doc = "Transfer Group Interrupt Vector Register 1"]
pub struct TGINTVEC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Group Interrupt Vector Register 1"]
pub mod tg_int_vec1;
#[doc = "Pin Control Register 9"]
pub struct SRSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register 9"]
pub mod sr_sel;
#[doc = "Parallel/Modulo Mode Control Register"]
pub struct PMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parallel/Modulo Mode Control Register"]
pub mod pm_ctrl;
#[doc = "MibSPI Enable Register"]
pub struct MIBSPIENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Enable Register"]
pub mod mib_spi_ena;
#[doc = "MibSPI Transfer Group Interrupt Enable Set Register"]
pub struct TGINTENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Interrupt Enable Set Register"]
pub mod tg_int_ena_set;
#[doc = "MibSPI Transfer Group Interrupt Enable Clear Register"]
pub struct TGINTENACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Interrupt Enable Clear Register"]
pub mod tg_int_ena_clr;
#[doc = "MibSPI Transfer Group Interrupt Level Set Register"]
pub struct TGINTLVLSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Interrupt Level Set Register"]
pub mod tg_int_lvl_set;
#[doc = "MibSPI Transfer Group Interrupt Level Clear Register"]
pub struct TGINTLVLCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Interrupt Level Clear Register"]
pub mod tg_int_lvl_clr;
#[doc = "Transfer Group Interrupt Flag Register"]
pub struct TGINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Group Interrupt Flag Register"]
pub mod tg_int_flg;
#[doc = "Tick Cnt Register"]
pub struct TICKCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tick Cnt Register"]
pub mod tick_cnt;
#[doc = "Last Transfer Group End Pointer"]
pub struct LTGPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last Transfer Group End Pointer"]
pub mod ltg_pend;
#[doc = "MibSPI Transfer Group Control Register 0"]
pub struct TG0CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 0"]
pub mod tg0ctrl;
#[doc = "MibSPI Transfer Group Control Register 1"]
pub struct TG1CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 1"]
pub mod tg1ctrl;
#[doc = "MibSPI Transfer Group Control Register 2"]
pub struct TG2CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 2"]
pub mod tg2ctrl;
#[doc = "MibSPI Transfer Group Control Register 3"]
pub struct TG3CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 3"]
pub mod tg3ctrl;
#[doc = "MibSPI Transfer Group Control Register 4"]
pub struct TG4CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 4"]
pub mod tg4ctrl;
#[doc = "MibSPI Transfer Group Control Register 5"]
pub struct TG5CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 5"]
pub mod tg5ctrl;
#[doc = "MibSPI Transfer Group Control Register 6"]
pub struct TG6CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 6"]
pub mod tg6ctrl;
#[doc = "MibSPI Transfer Group Control Register 7"]
pub struct TG7CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 7"]
pub mod tg7ctrl;
#[doc = "MibSPI Transfer Group Control Register 8"]
pub struct TG8CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 8"]
pub mod tg8ctrl;
#[doc = "MibSPI Transfer Group Control Register 9"]
pub struct TG9CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 9"]
pub mod tg9ctrl;
#[doc = "MibSPI Transfer Group Control Register 10"]
pub struct TG10CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 10"]
pub mod tg10ctrl;
#[doc = "MibSPI Transfer Group Control Register 11"]
pub struct TG11CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 11"]
pub mod tg11ctrl;
#[doc = "MibSPI Transfer Group Control Register 12"]
pub struct TG12CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 12"]
pub mod tg12ctrl;
#[doc = "MibSPI Transfer Group Control Register 13"]
pub struct TG13CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 13"]
pub mod tg13ctrl;
#[doc = "MibSPI Transfer Group Control Register 14"]
pub struct TG14CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 14"]
pub mod tg14ctrl;
#[doc = "MibSPI Transfer Group Control Register 15"]
pub struct TG15CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Transfer Group Control Register 15"]
pub mod tg15ctrl;
#[doc = "MibSPI Dma Channel Control Register 0"]
pub struct DMA0CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Dma Channel Control Register 0"]
pub mod dma0ctrl;
#[doc = "MibSPI Dma Channel Control Register 1"]
pub struct DMA1CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Dma Channel Control Register 1"]
pub mod dma1ctrl;
#[doc = "MibSPI Dma Channel Control Register 2"]
pub struct DMA2CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Dma Channel Control Register 2"]
pub mod dma2ctrl;
#[doc = "MibSPI Dma Channel Control Register 3"]
pub struct DMA3CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Dma Channel Control Register 3"]
pub mod dma3ctrl;
#[doc = "MibSPI Dma Channel Control Register 4"]
pub struct DMA4CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Dma Channel Control Register 4"]
pub mod dma4ctrl;
#[doc = "MibSPI Dma Channel Control Register 5"]
pub struct DMA5CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Dma Channel Control Register 5"]
pub mod dma5ctrl;
#[doc = "MibSPI Dma Channel Control Register 6"]
pub struct DMA6CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Dma Channel Control Register 6"]
pub mod dma6ctrl;
#[doc = "MibSPI Dma Channel Control Register 7"]
pub struct DMA7CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MibSPI Dma Channel Control Register 7"]
pub mod dma7ctrl;
#[doc = "ICnt Register 0"]
pub struct DMA0CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICnt Register 0"]
pub mod dma0cnt;
#[doc = "ICnt Register 1"]
pub struct DMA1CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICnt Register 1"]
pub mod dma1cnt;
#[doc = "ICnt Register 2"]
pub struct DMA2CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICnt Register 2"]
pub mod dma2cnt;
#[doc = "ICnt Register 3"]
pub struct DMA3CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICnt Register 3"]
pub mod dma3cnt;
#[doc = "ICnt Register 4"]
pub struct DMA4CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICnt Register 4"]
pub mod dma4cnt;
#[doc = "ICnt Register 5"]
pub struct DMA5CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICnt Register 5"]
pub mod dma5cnt;
#[doc = "ICnt Register 6"]
pub struct DMA6CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICnt Register 6"]
pub mod dma6cnt;
#[doc = "ICnt Register 7"]
pub struct DMA7CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ICnt Register 7"]
pub mod dma7cnt;
#[doc = "Dma LARGE Cnt register"]
pub struct DMACNTLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dma LARGE Cnt register"]
pub mod dma_cnt_len;
#[doc = "Uncorrectable Parity Error Control Register"]
pub struct UERRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Parity Error Control Register"]
pub mod uerr_ctrl;
#[doc = "Uncorrectable Parity Error Status Register"]
pub struct UERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Parity Error Status Register"]
pub mod uerr_stat;
#[doc = "Uncorrectable Parity Error Address Register"]
pub struct UERRADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Parity Error Address Register"]
pub mod uerr_addr1;
#[doc = "Uncorrectable Parity Error Address Register"]
pub struct UERRADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Uncorrectable Parity Error Address Register"]
pub mod uerr_addr0;
#[doc = "Receive RAM Overrun Buffer Address Register"]
pub struct RXOVRNBUFADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive RAM Overrun Buffer Address Register"]
pub mod rx_ovr_nbuf_addr;
#[doc = "IO Loopback Test Control Register"]
pub struct IOLPBKTSTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Loopback Test Control Register"]
pub mod io_lpbk_tst_ctrl;
#[doc = "Extended Prescale Register 1"]
pub struct EXTENDED_PRESCALE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extended Prescale Register 1"]
pub mod extended_prescale1;
#[doc = "Extended Prescale Register 2"]
pub struct EXTENDED_PRESCALE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extended Prescale Register 2"]
pub mod extended_prescale2;
#[doc = "ECC Control register"]
pub struct ECCDIAG_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Control register"]
pub mod eccdiag_ctrl;
#[doc = "ECC Diagnostic Status register"]
pub struct ECCDIAG_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC Diagnostic Status register"]
pub mod eccdiag_stat;
#[doc = "Single Bit Error Address for RXRAM"]
pub struct SBITERRADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Bit Error Address for RXRAM"]
pub mod sbit_err_addr1;
#[doc = "Single Bit Error Address for TXRAM"]
pub struct SBITERRADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Bit Error Address for TXRAM"]
pub mod sbit_err_addr0;
#[doc = "Revision ID Register"]
pub struct SPIREV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Revision ID Register"]
pub mod spirev;
