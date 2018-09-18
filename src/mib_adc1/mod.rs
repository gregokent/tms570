#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Control Register"]
    pub rst_ctrl: RSTCTRL,
    #[doc = "0x04 - Operation Mode Control Register"]
    pub op_mode_ctrl: OPMODECTRL,
    #[doc = "0x08 - Clock Prescaler"]
    pub clck_ctrl: CLCKCTRL,
    #[doc = "0x0c - Calibration Control Register"]
    pub cal_ctrl: CALCTRL,
    #[doc = "0x10 - Ev Mode Control Register"]
    pub ev_mode_ctrl: EVMODECTRL,
    #[doc = "0x14 - G1 Mode Control Register"]
    pub g1mode_ctrl: G1MODECTRL,
    #[doc = "0x18 - G2 Mode Control Register"]
    pub g2mode_ctrl: G2MODECTRL,
    #[doc = "0x1c - Event Group Trigger Source Select"]
    pub ev_src: EVSRC,
    #[doc = "0x20 - Group 1 Trigger Source Select"]
    pub g1src: G1SRC,
    #[doc = "0x24 - Group 2 Trigger Source Select"]
    pub g2src: G2SRC,
    #[doc = "0x28 - Event Group Interrupt Enable"]
    pub ev_int_ena: EVINTENA,
    #[doc = "0x2c - Group 1 Interrupt Enable"]
    pub g1int_ena: G1INTENA,
    #[doc = "0x30 - Group 2 Interrupt Enable"]
    pub g2int_ena: G2INTENA,
    #[doc = "0x34 - Event Group Interrupt Flg"]
    pub ev_int_flg: EVINTFLG,
    #[doc = "0x38 - Group 1 Interrupt Flg"]
    pub g1int_flg: G1INTFLG,
    #[doc = "0x3c - Group 2 Interrupt Flg"]
    pub g2int_flg: G2INTFLG,
    #[doc = "0x40 - Event Group Interrupt Threshold Counter"]
    pub ev_int_ctrl: EVINTCTRL,
    #[doc = "0x44 - Group 1 Interrupt Threshold Counter"]
    pub g1int_ctrl: G1INTCTRL,
    #[doc = "0x48 - Group 2 Interrupt Threshold Counter"]
    pub g2int_ctrl: G2INTCTRL,
    #[doc = "0x4c - Event Group Dma Control Register"]
    pub ev_dma_ctrl: EVDMACTRL,
    #[doc = "0x50 - Group 1 Dma Control Register"]
    pub g1dma_ctrl: G1DMACTRL,
    #[doc = "0x54 - Group 2 Dma Control Register"]
    pub g2dma_ctrl: G2DMACTRL,
    #[doc = "0x58 - Buffer Boundary Control Register"]
    pub bnd_ctrl: BNDCTRL,
    #[doc = "0x5c - Buffer End Boundary"]
    pub bnd_end: BNDEND,
    #[doc = "0x60 - Event Group Sample Window"]
    pub ev_samp: EVSAMP,
    #[doc = "0x64 - Group 1 Sample Window"]
    pub g1samp: G1SAMP,
    #[doc = "0x68 - Group 2 Sample Window"]
    pub g2samp: G2SAMP,
    #[doc = "0x6c - Event Group Status Register"]
    pub ev_sr: EVSR,
    #[doc = "0x70 - Group 1 Status Register"]
    pub g1sr: G1SR,
    #[doc = "0x74 - Group 2 Status Register"]
    pub g2sr: G2SR,
    #[doc = "0x78 - Event Group select register"]
    pub ev_sel: EVSEL,
    #[doc = "0x7c - Group 1 select register"]
    pub g1sel: G1SEL,
    #[doc = "0x80 - Group 2 select register"]
    pub g2sel: G2SEL,
    #[doc = "0x84 - Calibration Register"]
    pub cal_r: CALR,
    #[doc = "0x88 - State Macine Current State"]
    pub sm_state: SMSTATE,
    #[doc = "0x8c - Last Conversion"]
    pub last_conv: LASTCONV,
    #[doc = "0x90 - Event Group Buffer"]
    pub ev_buffer1: EVBUFFER1,
    #[doc = "0x94 - Event Group Buffer"]
    pub ev_buffer2: EVBUFFER2,
    #[doc = "0x98 - Event Group Buffer"]
    pub ev_buffer3: EVBUFFER3,
    #[doc = "0x9c - Event Group Buffer"]
    pub ev_buffer4: EVBUFFER4,
    #[doc = "0xa0 - Event Group Buffer"]
    pub ev_buffer5: EVBUFFER5,
    #[doc = "0xa4 - Event Group Buffer"]
    pub ev_buffer6: EVBUFFER6,
    #[doc = "0xa8 - Event Group Buffer"]
    pub ev_buffer7: EVBUFFER7,
    #[doc = "0xac - Event Group Buffer"]
    pub ev_buffer8: EVBUFFER8,
    #[doc = "0xb0 - Group 1 Buffer"]
    pub g1buffer1: G1BUFFER1,
    #[doc = "0xb4 - Group 1 Buffer"]
    pub g1buffer2: G1BUFFER2,
    #[doc = "0xb8 - Group 1 Buffer"]
    pub g1buffer3: G1BUFFER3,
    #[doc = "0xbc - Group 1 Buffer"]
    pub g1buffer4: G1BUFFER4,
    #[doc = "0xc0 - Group 1 Buffer"]
    pub g1buffer5: G1BUFFER5,
    #[doc = "0xc4 - Group 1 Buffer"]
    pub g1buffer6: G1BUFFER6,
    #[doc = "0xc8 - Group 1 Buffer"]
    pub g1buffer7: G1BUFFER7,
    #[doc = "0xcc - Group 1 Buffer"]
    pub g1buffer8: G1BUFFER8,
    #[doc = "0xd0 - Group 2 Buffer"]
    pub g2buffer1: G2BUFFER1,
    #[doc = "0xd4 - Group 2 Buffer"]
    pub g2buffer2: G2BUFFER2,
    #[doc = "0xd8 - Group 2 Buffer"]
    pub g2buffer3: G2BUFFER3,
    #[doc = "0xdc - Group 2 Buffer"]
    pub g2buffer4: G2BUFFER4,
    #[doc = "0xe0 - Group 2 Buffer"]
    pub g2buffer5: G2BUFFER5,
    #[doc = "0xe4 - Group 2 Buffer"]
    pub g2buffer6: G2BUFFER6,
    #[doc = "0xe8 - Group 2 Buffer"]
    pub g2buffer7: G2BUFFER7,
    #[doc = "0xec - Group 2 Buffer"]
    pub g2buffer8: G2BUFFER8,
    #[doc = "0xf0 - Event Group Emu Buffer"]
    pub ev_emu_buffer: EVEMUBUFFER,
    #[doc = "0xf4 - Group 1 Emu Buffer"]
    pub g1emu_buffer: G1EMUBUFFER,
    #[doc = "0xf8 - Group 2 Emu Buffer"]
    pub g2emu_buffer: G2EMUBUFFER,
    #[doc = "0xfc - Event Group pin direction selection"]
    pub ev_dir: EVDIR,
    #[doc = "0x100 - Event Group pin data output"]
    pub ev_dout: EVDOUT,
    #[doc = "0x104 - Event Group pin input value"]
    pub ev_din: EVDIN,
    #[doc = "0x108 - Event Group pin set"]
    pub ev_dset: EVDSET,
    #[doc = "0x10c - Event Group pin clear"]
    pub ev_dclr: EVDCLR,
    #[doc = "0x110 - Event Group pin open-drain enable"]
    pub ev_pdr: EVPDR,
    #[doc = "0x114 - Event Group pin pull control enable"]
    pub ev_pdis: EVPDIS,
    #[doc = "0x118 - Event Group pull select"]
    pub ev_psel: EVPSEL,
    #[doc = "0x11c - Event Group Discharge Control"]
    pub ev_samp_dis_en: EVSAMPDISEN,
    #[doc = "0x120 - Group 1 Discharge Control"]
    pub g1samp_dis_en: G1SAMPDISEN,
    #[doc = "0x124 - Group 2 Discharge Control"]
    pub g2samp_dis_en: G2SAMPDISEN,
    #[doc = "0x128 - Magnitude Interrupt Control"]
    pub mag_int_ctrl1: MAGINTCTRL1,
    #[doc = "0x12c - Magnitude Interrupt Mask"]
    pub mag_int1msk: MAGINT1MSK,
    #[doc = "0x130 - Magnitude Interrupt Control"]
    pub mag_int_ctrl2: MAGINTCTRL2,
    #[doc = "0x134 - Magnitude Interrupt Mask"]
    pub mag_int2msk: MAGINT2MSK,
    #[doc = "0x138 - Magnitude Interrupt Control"]
    pub mag_int_ctrl3: MAGINTCTRL3,
    #[doc = "0x13c - Magnitude Interrupt Mask"]
    pub mag_int3msk: MAGINT3MSK,
    #[doc = "0x140 - Magnitude Interrupt Control"]
    pub mag_int_ctrl4: MAGINTCTRL4,
    #[doc = "0x144 - Magnitude Interrupt Mask"]
    pub mag_int4msk: MAGINT4MSK,
    #[doc = "0x148 - Magnitude Interrupt Control"]
    pub mag_int_ctrl5: MAGINTCTRL5,
    #[doc = "0x14c - Magnitude Interrupt Mask"]
    pub mag_int5msk: MAGINT5MSK,
    #[doc = "0x150 - Magnitude Interrupt Control"]
    pub mag_int_ctrl6: MAGINTCTRL6,
    #[doc = "0x154 - Magnitude Interrupt Mask"]
    pub mag_int6msk: MAGINT6MSK,
    #[doc = "0x158 - Magnitude Interrupt Enable Set"]
    pub mag_thr_int_ena_set: MAGTHRINTENASET,
    #[doc = "0x15c - Magnitude Interrupt Enable Clear"]
    pub mag_thr_int_ena_clr: MAGTHRINTENACLR,
    #[doc = "0x160 - Magnitude Interrupt Enable Flag"]
    pub mag_thr_int_flg: MAGTHRINTFLG,
    #[doc = "0x164 - Magnitude Interrupt Offset"]
    pub mag_thr_int_offst: MAGTHRINTOFFST,
    #[doc = "0x168 - Event Group FIFO Reset Control"]
    pub ev_fifo_rst_ctrl: EVFIFORSTCTRL,
    #[doc = "0x16c - Group 1 FIFO Reset Control"]
    pub g1fifo_rst_ctrl: G1FIFORSTCTRL,
    #[doc = "0x170 - Group 2 FIFO Reset Control"]
    pub g2fifo_rst_ctrl: G2FIFORSTCTRL,
    #[doc = "0x174 - Event Group RAM Pointer"]
    pub ev_ram_addr: EVRAMADDR,
    #[doc = "0x178 - Group 1 RAM Pointer"]
    pub g1ram_addr: G1RAMADDR,
    #[doc = "0x17c - Group 2 RAM Pointer"]
    pub g2ram_addr: G2RAMADDR,
    #[doc = "0x180 - Parity Control"]
    pub par_ctrl: PARCTRL,
    #[doc = "0x184 - Parity Address"]
    pub par_addr: PARADDR,
    #[doc = "0x188 - Power up DLY Control"]
    pub pwrup_dly_ctrl: PWRUPDLYCTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x190 - Event Group Channel Selection Mode Control"]
    pub ev_chn_sel_mode_ctrl: EVCHNSELMODECTRL,
    #[doc = "0x194 - Group1 Channel Selection Mode Control"]
    pub g1chn_sel_mode_ctrl: G1CHNSELMODECTRL,
    #[doc = "0x198 - Group2 Channel Selection Mode Control"]
    pub g2chn_sel_mode_ctrl: G2CHNSELMODECTRL,
    #[doc = "0x19c - Event Group Current Count"]
    pub ev_curr_count: EVCURRCOUNT,
    #[doc = "0x1a0 - Group 1 Current Count"]
    pub g1curr_count: G1CURRCOUNT,
    #[doc = "0x1a4 - Group 2 Current Count"]
    pub g2curr_count: G2CURRCOUNT,
    #[doc = "0x1a8 - Event Group Max Count"]
    pub ev_max_count: EVMAXCOUNT,
    #[doc = "0x1ac - Group 1 Max Count"]
    pub g1max_count: G1MAXCOUNT,
    #[doc = "0x1b0 - Group 2 Max Count"]
    pub g2max_count: G2MAXCOUNT,
}
#[doc = "Reset Control Register"]
pub struct RSTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Control Register"]
pub mod rst_ctrl;
#[doc = "Operation Mode Control Register"]
pub struct OPMODECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operation Mode Control Register"]
pub mod op_mode_ctrl;
#[doc = "Clock Prescaler"]
pub struct CLCKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Prescaler"]
pub mod clck_ctrl;
#[doc = "Calibration Control Register"]
pub struct CALCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Control Register"]
pub mod cal_ctrl;
#[doc = "Ev Mode Control Register"]
pub struct EVMODECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ev Mode Control Register"]
pub mod ev_mode_ctrl;
#[doc = "G1 Mode Control Register"]
pub struct G1MODECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "G1 Mode Control Register"]
pub mod g1mode_ctrl;
#[doc = "G2 Mode Control Register"]
pub struct G2MODECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "G2 Mode Control Register"]
pub mod g2mode_ctrl;
#[doc = "Event Group Trigger Source Select"]
pub struct EVSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Trigger Source Select"]
pub mod ev_src;
#[doc = "Group 1 Trigger Source Select"]
pub struct G1SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Trigger Source Select"]
pub mod g1src;
#[doc = "Group 2 Trigger Source Select"]
pub struct G2SRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Trigger Source Select"]
pub mod g2src;
#[doc = "Event Group Interrupt Enable"]
pub struct EVINTENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Interrupt Enable"]
pub mod ev_int_ena;
#[doc = "Group 1 Interrupt Enable"]
pub struct G1INTENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Interrupt Enable"]
pub mod g1int_ena;
#[doc = "Group 2 Interrupt Enable"]
pub struct G2INTENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Interrupt Enable"]
pub mod g2int_ena;
#[doc = "Event Group Interrupt Flg"]
pub struct EVINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Interrupt Flg"]
pub mod ev_int_flg;
#[doc = "Group 1 Interrupt Flg"]
pub struct G1INTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Interrupt Flg"]
pub mod g1int_flg;
#[doc = "Group 2 Interrupt Flg"]
pub struct G2INTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Interrupt Flg"]
pub mod g2int_flg;
#[doc = "Event Group Interrupt Threshold Counter"]
pub struct EVINTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Interrupt Threshold Counter"]
pub mod ev_int_ctrl;
#[doc = "Group 1 Interrupt Threshold Counter"]
pub struct G1INTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Interrupt Threshold Counter"]
pub mod g1int_ctrl;
#[doc = "Group 2 Interrupt Threshold Counter"]
pub struct G2INTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Interrupt Threshold Counter"]
pub mod g2int_ctrl;
#[doc = "Event Group Dma Control Register"]
pub struct EVDMACTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Dma Control Register"]
pub mod ev_dma_ctrl;
#[doc = "Group 1 Dma Control Register"]
pub struct G1DMACTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Dma Control Register"]
pub mod g1dma_ctrl;
#[doc = "Group 2 Dma Control Register"]
pub struct G2DMACTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Dma Control Register"]
pub mod g2dma_ctrl;
#[doc = "Buffer Boundary Control Register"]
pub struct BNDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer Boundary Control Register"]
pub mod bnd_ctrl;
#[doc = "Buffer End Boundary"]
pub struct BNDEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer End Boundary"]
pub mod bnd_end;
#[doc = "Event Group Sample Window"]
pub struct EVSAMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Sample Window"]
pub mod ev_samp;
#[doc = "Group 1 Sample Window"]
pub struct G1SAMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Sample Window"]
pub mod g1samp;
#[doc = "Group 2 Sample Window"]
pub struct G2SAMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Sample Window"]
pub mod g2samp;
#[doc = "Event Group Status Register"]
pub struct EVSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Status Register"]
pub mod ev_sr;
#[doc = "Group 1 Status Register"]
pub struct G1SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Status Register"]
pub mod g1sr;
#[doc = "Group 2 Status Register"]
pub struct G2SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Status Register"]
pub mod g2sr;
#[doc = "Event Group select register"]
pub struct EVSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group select register"]
pub mod ev_sel;
#[doc = "Group 1 select register"]
pub struct G1SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 select register"]
pub mod g1sel;
#[doc = "Group 2 select register"]
pub struct G2SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 select register"]
pub mod g2sel;
#[doc = "Calibration Register"]
pub struct CALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Register"]
pub mod cal_r;
#[doc = "State Macine Current State"]
pub struct SMSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Macine Current State"]
pub mod sm_state;
#[doc = "Last Conversion"]
pub struct LASTCONV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last Conversion"]
pub mod last_conv;
#[doc = "Event Group Buffer"]
pub struct EVBUFFER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Buffer"]
pub mod ev_buffer1;
#[doc = "Event Group Buffer"]
pub struct EVBUFFER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Buffer"]
pub mod ev_buffer2;
#[doc = "Event Group Buffer"]
pub struct EVBUFFER3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Buffer"]
pub mod ev_buffer3;
#[doc = "Event Group Buffer"]
pub struct EVBUFFER4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Buffer"]
pub mod ev_buffer4;
#[doc = "Event Group Buffer"]
pub struct EVBUFFER5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Buffer"]
pub mod ev_buffer5;
#[doc = "Event Group Buffer"]
pub struct EVBUFFER6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Buffer"]
pub mod ev_buffer6;
#[doc = "Event Group Buffer"]
pub struct EVBUFFER7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Buffer"]
pub mod ev_buffer7;
#[doc = "Event Group Buffer"]
pub struct EVBUFFER8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Buffer"]
pub mod ev_buffer8;
#[doc = "Group 1 Buffer"]
pub struct G1BUFFER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Buffer"]
pub mod g1buffer1;
#[doc = "Group 1 Buffer"]
pub struct G1BUFFER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Buffer"]
pub mod g1buffer2;
#[doc = "Group 1 Buffer"]
pub struct G1BUFFER3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Buffer"]
pub mod g1buffer3;
#[doc = "Group 1 Buffer"]
pub struct G1BUFFER4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Buffer"]
pub mod g1buffer4;
#[doc = "Group 1 Buffer"]
pub struct G1BUFFER5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Buffer"]
pub mod g1buffer5;
#[doc = "Group 1 Buffer"]
pub struct G1BUFFER6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Buffer"]
pub mod g1buffer6;
#[doc = "Group 1 Buffer"]
pub struct G1BUFFER7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Buffer"]
pub mod g1buffer7;
#[doc = "Group 1 Buffer"]
pub struct G1BUFFER8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Buffer"]
pub mod g1buffer8;
#[doc = "Group 2 Buffer"]
pub struct G2BUFFER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Buffer"]
pub mod g2buffer1;
#[doc = "Group 2 Buffer"]
pub struct G2BUFFER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Buffer"]
pub mod g2buffer2;
#[doc = "Group 2 Buffer"]
pub struct G2BUFFER3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Buffer"]
pub mod g2buffer3;
#[doc = "Group 2 Buffer"]
pub struct G2BUFFER4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Buffer"]
pub mod g2buffer4;
#[doc = "Group 2 Buffer"]
pub struct G2BUFFER5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Buffer"]
pub mod g2buffer5;
#[doc = "Group 2 Buffer"]
pub struct G2BUFFER6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Buffer"]
pub mod g2buffer6;
#[doc = "Group 2 Buffer"]
pub struct G2BUFFER7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Buffer"]
pub mod g2buffer7;
#[doc = "Group 2 Buffer"]
pub struct G2BUFFER8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Buffer"]
pub mod g2buffer8;
#[doc = "Event Group Emu Buffer"]
pub struct EVEMUBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Emu Buffer"]
pub mod ev_emu_buffer;
#[doc = "Group 1 Emu Buffer"]
pub struct G1EMUBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Emu Buffer"]
pub mod g1emu_buffer;
#[doc = "Group 2 Emu Buffer"]
pub struct G2EMUBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Emu Buffer"]
pub mod g2emu_buffer;
#[doc = "Event Group pin direction selection"]
pub struct EVDIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group pin direction selection"]
pub mod ev_dir;
#[doc = "Event Group pin data output"]
pub struct EVDOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group pin data output"]
pub mod ev_dout;
#[doc = "Event Group pin input value"]
pub struct EVDIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group pin input value"]
pub mod ev_din;
#[doc = "Event Group pin set"]
pub struct EVDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group pin set"]
pub mod ev_dset;
#[doc = "Event Group pin clear"]
pub struct EVDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group pin clear"]
pub mod ev_dclr;
#[doc = "Event Group pin open-drain enable"]
pub struct EVPDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group pin open-drain enable"]
pub mod ev_pdr;
#[doc = "Event Group pin pull control enable"]
pub struct EVPDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group pin pull control enable"]
pub mod ev_pdis;
#[doc = "Event Group pull select"]
pub struct EVPSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group pull select"]
pub mod ev_psel;
#[doc = "Event Group Discharge Control"]
pub struct EVSAMPDISEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Discharge Control"]
pub mod ev_samp_dis_en;
#[doc = "Group 1 Discharge Control"]
pub struct G1SAMPDISEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Discharge Control"]
pub mod g1samp_dis_en;
#[doc = "Group 2 Discharge Control"]
pub struct G2SAMPDISEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Discharge Control"]
pub mod g2samp_dis_en;
#[doc = "Magnitude Interrupt Control"]
pub struct MAGINTCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Control"]
pub mod mag_int_ctrl1;
#[doc = "Magnitude Interrupt Mask"]
pub struct MAGINT1MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Mask"]
pub mod mag_int1msk;
#[doc = "Magnitude Interrupt Control"]
pub struct MAGINTCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Control"]
pub mod mag_int_ctrl2;
#[doc = "Magnitude Interrupt Mask"]
pub struct MAGINT2MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Mask"]
pub mod mag_int2msk;
#[doc = "Magnitude Interrupt Control"]
pub struct MAGINTCTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Control"]
pub mod mag_int_ctrl3;
#[doc = "Magnitude Interrupt Mask"]
pub struct MAGINT3MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Mask"]
pub mod mag_int3msk;
#[doc = "Magnitude Interrupt Control"]
pub struct MAGINTCTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Control"]
pub mod mag_int_ctrl4;
#[doc = "Magnitude Interrupt Mask"]
pub struct MAGINT4MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Mask"]
pub mod mag_int4msk;
#[doc = "Magnitude Interrupt Control"]
pub struct MAGINTCTRL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Control"]
pub mod mag_int_ctrl5;
#[doc = "Magnitude Interrupt Mask"]
pub struct MAGINT5MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Mask"]
pub mod mag_int5msk;
#[doc = "Magnitude Interrupt Control"]
pub struct MAGINTCTRL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Control"]
pub mod mag_int_ctrl6;
#[doc = "Magnitude Interrupt Mask"]
pub struct MAGINT6MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Mask"]
pub mod mag_int6msk;
#[doc = "Magnitude Interrupt Enable Set"]
pub struct MAGTHRINTENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Enable Set"]
pub mod mag_thr_int_ena_set;
#[doc = "Magnitude Interrupt Enable Clear"]
pub struct MAGTHRINTENACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Enable Clear"]
pub mod mag_thr_int_ena_clr;
#[doc = "Magnitude Interrupt Enable Flag"]
pub struct MAGTHRINTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Enable Flag"]
pub mod mag_thr_int_flg;
#[doc = "Magnitude Interrupt Offset"]
pub struct MAGTHRINTOFFST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Magnitude Interrupt Offset"]
pub mod mag_thr_int_offst;
#[doc = "Event Group FIFO Reset Control"]
pub struct EVFIFORSTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group FIFO Reset Control"]
pub mod ev_fifo_rst_ctrl;
#[doc = "Group 1 FIFO Reset Control"]
pub struct G1FIFORSTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 FIFO Reset Control"]
pub mod g1fifo_rst_ctrl;
#[doc = "Group 2 FIFO Reset Control"]
pub struct G2FIFORSTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 FIFO Reset Control"]
pub mod g2fifo_rst_ctrl;
#[doc = "Event Group RAM Pointer"]
pub struct EVRAMADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group RAM Pointer"]
pub mod ev_ram_addr;
#[doc = "Group 1 RAM Pointer"]
pub struct G1RAMADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 RAM Pointer"]
pub mod g1ram_addr;
#[doc = "Group 2 RAM Pointer"]
pub struct G2RAMADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 RAM Pointer"]
pub mod g2ram_addr;
#[doc = "Parity Control"]
pub struct PARCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Control"]
pub mod par_ctrl;
#[doc = "Parity Address"]
pub struct PARADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Address"]
pub mod par_addr;
#[doc = "Power up DLY Control"]
pub struct PWRUPDLYCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power up DLY Control"]
pub mod pwrup_dly_ctrl;
#[doc = "Event Group Channel Selection Mode Control"]
pub struct EVCHNSELMODECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Channel Selection Mode Control"]
pub mod ev_chn_sel_mode_ctrl;
#[doc = "Group1 Channel Selection Mode Control"]
pub struct G1CHNSELMODECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group1 Channel Selection Mode Control"]
pub mod g1chn_sel_mode_ctrl;
#[doc = "Group2 Channel Selection Mode Control"]
pub struct G2CHNSELMODECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group2 Channel Selection Mode Control"]
pub mod g2chn_sel_mode_ctrl;
#[doc = "Event Group Current Count"]
pub struct EVCURRCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Current Count"]
pub mod ev_curr_count;
#[doc = "Group 1 Current Count"]
pub struct G1CURRCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Current Count"]
pub mod g1curr_count;
#[doc = "Group 2 Current Count"]
pub struct G2CURRCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Current Count"]
pub mod g2curr_count;
#[doc = "Event Group Max Count"]
pub struct EVMAXCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Group Max Count"]
pub mod ev_max_count;
#[doc = "Group 1 Max Count"]
pub struct G1MAXCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 1 Max Count"]
pub mod g1max_count;
#[doc = "Group 2 Max Count"]
pub struct G2MAXCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Group 2 Max Count"]
pub mod g2max_count;
