#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - global control register"]
    pub glb_ctrl: GLBCTRL,
    #[doc = "0x04 - prescaler factor register"]
    pub pfr: PFR,
    #[doc = "0x08 - Current Address Register"]
    pub addr: ADDR,
    #[doc = "0x0c - Offset Level 1 Register"]
    pub offst1: OFFST1,
    #[doc = "0x10 - Offset Level 2 Register"]
    pub offst2: OFFST2,
    #[doc = "0x14 - Interrupt Enable Set Register"]
    pub int_ena_set: INTENASET,
    #[doc = "0x18 - Interrupt Enable Clear Register"]
    pub int_ena_clr: INTENACLR,
    #[doc = "0x1c - Exception Control Register 1"]
    pub exc1: EXC1,
    #[doc = "0x20 - Exception Control Register 2"]
    pub exc2: EXC2,
    #[doc = "0x24 - Interrupt Priority Register"]
    pub int_prio: INTPRIO,
    #[doc = "0x28 - Interrupt Flag Register"]
    pub int_flg: INTFLG,
    _reserved0: [u8; 8usize],
    #[doc = "0x34 - HR Share Control Register"]
    pub hr_sh: HRSH,
    #[doc = "0x38 - HR Xor control register"]
    pub xor: XOR,
    #[doc = "0x3c - Request Enable Set Register"]
    pub req_ena_set: REQENASET,
    #[doc = "0x40 - Request Enable Clear Register"]
    pub req_ena_clr: REQENACLR,
    #[doc = "0x44 - Request Destination Select Register"]
    pub req_dst: REQDST,
    _reserved1: [u8; 4usize],
    #[doc = "0x4c - Direction Register"]
    pub dir: DIR,
    #[doc = "0x50 - Input Data Register"]
    pub din: DIN,
    #[doc = "0x54 - Output Data Register"]
    pub dout: DOUT,
    #[doc = "0x58 - Set Data Register"]
    pub dset: DSET,
    #[doc = "0x5c - Clear Data Register"]
    pub dclr: DCLR,
    #[doc = "0x60 - Open Drain Register"]
    pub pdr: PDR,
    #[doc = "0x64 - Pull Disable Register"]
    pub pdis: PDIS,
    #[doc = "0x68 - Pull Select Register"]
    pub psel: PSEL,
    _reserved2: [u8; 8usize],
    #[doc = "0x74 - Parity Control Register"]
    pub par_ctrl: PARCTRL,
    #[doc = "0x78 - Parity Address Register"]
    pub par_addr: PARADDR,
    #[doc = "0x7c - Parity Pin Register"]
    pub par_pin_reg: PARPINREG,
    #[doc = "0x80 - Suppresion Filter Preload Register"]
    pub sf_prld: SFPRLD,
    #[doc = "0x84 - Suppresion Filter Enable Register"]
    pub sf_ena: SFENA,
    _reserved3: [u8; 4usize],
    #[doc = "0x8c - Loop Back Pair Select Register"]
    pub lb_pair_sel: LBPAIRSEL,
    #[doc = "0x90 - Loop Back Pair Direction Register"]
    pub lb_pair_dir: LBPAIRDIR,
    #[doc = "0x94 - Pin Disable Register"]
    pub pin_dis: PINDIS,
    _reserved4: [u8; 4usize],
    #[doc = "0x9c - HWAG Pin Select Register"]
    pub hwapin_sel: HWAPINSEL,
    #[doc = "0xa0 - HWAG Control Register 0"]
    pub hwagcr0: HWAGCR0,
    #[doc = "0xa4 - HWAG Control Register 1"]
    pub hwagcr1: HWAGCR1,
    #[doc = "0xa8 - HWAG Control Register 2"]
    pub hwagcr2: HWAGCR2,
    #[doc = "0xac - HWAG Interrupt Enable Set Register"]
    pub hwaenaset: HWAENASET,
    #[doc = "0xb0 - HWAG Interrupt Enable Clear Register"]
    pub hwaenaclr: HWAENACLR,
    #[doc = "0xb4 - HWAG Interrupt Priority Set Register"]
    pub hwalvlset: HWALVLSET,
    #[doc = "0xb8 - HWAG Interrupt Priority Clear Register"]
    pub hwalvlclr: HWALVLCLR,
    #[doc = "0xbc - HWAG Interrupt Flags Register"]
    pub hwaflg: HWAFLG,
    #[doc = "0xc0 - HWAG Interrupt Offset Register 1, HWAG Low Priority Interrupt Offset"]
    pub hwaoff0: HWAOFF0,
    #[doc = "0xc4 - HWAG Interrupt Offset Register 2, HWAG High Priority Interrupt Offset"]
    pub hwaoff1: HWAOFF1,
    #[doc = "0xc8 - HWAG ACNT Register, HWAG Angle Value"]
    pub hwaacnt: HWAACNT,
    #[doc = "0xcc - HWAG PCNT (n-1) Register, HWAG Previous Tooth Period"]
    pub hwapcnt1: HWAPCNT1,
    #[doc = "0xd0 - HWAG PCNT (n) Register, HWAG Current Tooth Period"]
    pub hwapcnt: HWAPCNT,
    #[doc = "0xd4 - HWAG Step Register"]
    pub hwastwd: HWASTWD,
    #[doc = "0xd8 - HWAG Teeth Number Register"]
    pub hwathnb: HWATHNB,
    #[doc = "0xdc - HHWAG Current Teeth Number Register"]
    pub hwathvl: HWATHVL,
    #[doc = "0xe0 - HWAG Filter Register, HWAG Tick Counter Compare Value"]
    pub hwafil: HWAFIL,
    _reserved5: [u8; 4usize],
    #[doc = "0xe8 - HWAG Filter Register 2, HWAG Tick Counter Compare Value During Singularity Tooth"]
    pub hwafil2: HWAFIL2,
    _reserved6: [u8; 4usize],
    #[doc = "0xf0 - HWAG Angle Increment Register"]
    pub hwaangi: HWAANGI,
}
#[doc = "global control register"]
pub struct GLBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "global control register"]
pub mod glb_ctrl;
#[doc = "prescaler factor register"]
pub struct PFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "prescaler factor register"]
pub mod pfr;
#[doc = "Current Address Register"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Address Register"]
pub mod addr;
#[doc = "Offset Level 1 Register"]
pub struct OFFST1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Offset Level 1 Register"]
pub mod offst1;
#[doc = "Offset Level 2 Register"]
pub struct OFFST2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Offset Level 2 Register"]
pub mod offst2;
#[doc = "Interrupt Enable Set Register"]
pub struct INTENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set Register"]
pub mod int_ena_set;
#[doc = "Interrupt Enable Clear Register"]
pub struct INTENACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear Register"]
pub mod int_ena_clr;
#[doc = "Exception Control Register 1"]
pub struct EXC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exception Control Register 1"]
pub mod exc1;
#[doc = "Exception Control Register 2"]
pub struct EXC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exception Control Register 2"]
pub mod exc2;
#[doc = "Interrupt Priority Register"]
pub struct INTPRIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Priority Register"]
pub mod int_prio;
#[doc = "Interrupt Flag Register"]
pub struct INTFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Register"]
pub mod int_flg;
#[doc = "HR Share Control Register"]
pub struct HRSH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HR Share Control Register"]
pub mod hr_sh;
#[doc = "HR Xor control register"]
pub struct XOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HR Xor control register"]
pub mod xor;
#[doc = "Request Enable Set Register"]
pub struct REQENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Enable Set Register"]
pub mod req_ena_set;
#[doc = "Request Enable Clear Register"]
pub struct REQENACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Enable Clear Register"]
pub mod req_ena_clr;
#[doc = "Request Destination Select Register"]
pub struct REQDST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Request Destination Select Register"]
pub mod req_dst;
#[doc = "Direction Register"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direction Register"]
pub mod dir;
#[doc = "Input Data Register"]
pub struct DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Data Register"]
pub mod din;
#[doc = "Output Data Register"]
pub struct DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Data Register"]
pub mod dout;
#[doc = "Set Data Register"]
pub struct DSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Data Register"]
pub mod dset;
#[doc = "Clear Data Register"]
pub struct DCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Data Register"]
pub mod dclr;
#[doc = "Open Drain Register"]
pub struct PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open Drain Register"]
pub mod pdr;
#[doc = "Pull Disable Register"]
pub struct PDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull Disable Register"]
pub mod pdis;
#[doc = "Pull Select Register"]
pub struct PSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pull Select Register"]
pub mod psel;
#[doc = "Parity Control Register"]
pub struct PARCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Control Register"]
pub mod par_ctrl;
#[doc = "Parity Address Register"]
pub struct PARADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Address Register"]
pub mod par_addr;
#[doc = "Parity Pin Register"]
pub struct PARPINREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parity Pin Register"]
pub mod par_pin_reg;
#[doc = "Suppresion Filter Preload Register"]
pub struct SFPRLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suppresion Filter Preload Register"]
pub mod sf_prld;
#[doc = "Suppresion Filter Enable Register"]
pub struct SFENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suppresion Filter Enable Register"]
pub mod sf_ena;
#[doc = "Loop Back Pair Select Register"]
pub struct LBPAIRSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Loop Back Pair Select Register"]
pub mod lb_pair_sel;
#[doc = "Loop Back Pair Direction Register"]
pub struct LBPAIRDIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Loop Back Pair Direction Register"]
pub mod lb_pair_dir;
#[doc = "Pin Disable Register"]
pub struct PINDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Disable Register"]
pub mod pin_dis;
#[doc = "HWAG Pin Select Register"]
pub struct HWAPINSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Pin Select Register"]
pub mod hwapin_sel;
#[doc = "HWAG Control Register 0"]
pub struct HWAGCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Control Register 0"]
pub mod hwagcr0;
#[doc = "HWAG Control Register 1"]
pub struct HWAGCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Control Register 1"]
pub mod hwagcr1;
#[doc = "HWAG Control Register 2"]
pub struct HWAGCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Control Register 2"]
pub mod hwagcr2;
#[doc = "HWAG Interrupt Enable Set Register"]
pub struct HWAENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Interrupt Enable Set Register"]
pub mod hwaenaset;
#[doc = "HWAG Interrupt Enable Clear Register"]
pub struct HWAENACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Interrupt Enable Clear Register"]
pub mod hwaenaclr;
#[doc = "HWAG Interrupt Priority Set Register"]
pub struct HWALVLSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Interrupt Priority Set Register"]
pub mod hwalvlset;
#[doc = "HWAG Interrupt Priority Clear Register"]
pub struct HWALVLCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Interrupt Priority Clear Register"]
pub mod hwalvlclr;
#[doc = "HWAG Interrupt Flags Register"]
pub struct HWAFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Interrupt Flags Register"]
pub mod hwaflg;
#[doc = "HWAG Interrupt Offset Register 1, HWAG Low Priority Interrupt Offset"]
pub struct HWAOFF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Interrupt Offset Register 1, HWAG Low Priority Interrupt Offset"]
pub mod hwaoff0;
#[doc = "HWAG Interrupt Offset Register 2, HWAG High Priority Interrupt Offset"]
pub struct HWAOFF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Interrupt Offset Register 2, HWAG High Priority Interrupt Offset"]
pub mod hwaoff1;
#[doc = "HWAG ACNT Register, HWAG Angle Value"]
pub struct HWAACNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG ACNT Register, HWAG Angle Value"]
pub mod hwaacnt;
#[doc = "HWAG PCNT (n-1) Register, HWAG Previous Tooth Period"]
pub struct HWAPCNT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG PCNT (n-1) Register, HWAG Previous Tooth Period"]
pub mod hwapcnt1;
#[doc = "HWAG PCNT (n) Register, HWAG Current Tooth Period"]
pub struct HWAPCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG PCNT (n) Register, HWAG Current Tooth Period"]
pub mod hwapcnt;
#[doc = "HWAG Step Register"]
pub struct HWASTWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Step Register"]
pub mod hwastwd;
#[doc = "HWAG Teeth Number Register"]
pub struct HWATHNB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Teeth Number Register"]
pub mod hwathnb;
#[doc = "HHWAG Current Teeth Number Register"]
pub struct HWATHVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HHWAG Current Teeth Number Register"]
pub mod hwathvl;
#[doc = "HWAG Filter Register, HWAG Tick Counter Compare Value"]
pub struct HWAFIL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Filter Register, HWAG Tick Counter Compare Value"]
pub mod hwafil;
#[doc = "HWAG Filter Register 2, HWAG Tick Counter Compare Value During Singularity Tooth"]
pub struct HWAFIL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Filter Register 2, HWAG Tick Counter Compare Value During Singularity Tooth"]
pub mod hwafil2;
#[doc = "HWAG Angle Increment Register"]
pub struct HWAANGI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HWAG Angle Increment Register"]
pub mod hwaangi;
