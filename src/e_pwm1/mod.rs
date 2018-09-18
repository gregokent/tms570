#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Time Base Status Register"]
    pub tbsts: TBSTS,
    #[doc = "0x02 - Time Base Control Register"]
    pub tbctl: TBCTL,
    #[doc = "0x04 - Time Base Phase Register"]
    pub tbphs: TBPHS,
    _reserved0: [u8; 2usize],
    #[doc = "0x08 - Time Base Period Register"]
    pub tbprd: TBPRD,
    #[doc = "0x0a - Time Base Counter Register"]
    pub tbctr: TBCTR,
    #[doc = "0x0c - Counter Compare Control Register"]
    pub cmpctl: CMPCTL,
    _reserved1: [u8; 2usize],
    #[doc = "0x10 - Counter Compare A Register"]
    pub cmpa: CMPA,
    _reserved2: [u8; 2usize],
    #[doc = "0x14 - Action Qualifier Control Register For Output A"]
    pub aqctla: AQCTLA,
    #[doc = "0x16 - Counter Compare B Register"]
    pub cmpb: CMPB,
    #[doc = "0x18 - Action Qualifier Software Force Register"]
    pub aqsfrc: AQSFRC,
    #[doc = "0x1a - Action Qualifier Control Register For Output B"]
    pub aqctlb: AQCTLB,
    #[doc = "0x1c - Dead-Band Generator Control Register"]
    pub dbctl: DBCTL,
    #[doc = "0x1e - Action Qualifier Continuous S/W Force Register"]
    pub aqcsfrc: AQCSFRC,
    #[doc = "0x20 - Dead-Band Generator Falling Edge Delay Count Register"]
    pub dbfed: DBFED,
    #[doc = "0x22 - Dead-Band Generator Rising Edge Delay Count Register"]
    pub dbred: DBRED,
    #[doc = "0x24 - Trip Zone Digital Comparator Select Register"]
    pub tzdcsel: TZDCSEL,
    #[doc = "0x26 - Trip Zone Select Register"]
    pub tzsel: TZSEL,
    #[doc = "0x28 - Trip Zone Enable Interrupt Register"]
    pub tzeint: TZEINT,
    #[doc = "0x2a - Trip Zone Control Register"]
    pub tzctl: TZCTL,
    #[doc = "0x2c - Trip Zone Clear Register"]
    pub tzclr: TZCLR,
    #[doc = "0x2e - Trip Zone Flag Register"]
    pub tzflg: TZFLG,
    #[doc = "0x30 - Event Trigger Selection Register"]
    pub etsel: ETSEL,
    #[doc = "0x32 - Trip Zone Force Register"]
    pub tzfrc: TZFRC,
    #[doc = "0x34 - Event Trigger Flag Register"]
    pub etflg: ETFLG,
    #[doc = "0x36 - Event Trigger Pre-Scale Register"]
    pub etps: ETPS,
    #[doc = "0x38 - Event Trigger Force Register"]
    pub etfrc: ETFRC,
    #[doc = "0x3a - Event Trigger Clear Register"]
    pub etclr: ETCLR,
    _reserved3: [u8; 2usize],
    #[doc = "0x3e - PWM Chopper Control Register"]
    pub pcctl: PCCTL,
    _reserved4: [u8; 32usize],
    #[doc = "0x60 - Digital Compare A Control Register"]
    pub dcactl: DCACTL,
    #[doc = "0x62 - Digital Compare Trip Select Register"]
    pub dctripsel: DCTRIPSEL,
    #[doc = "0x64 - Digital Compare Filter Control Register"]
    pub dcfctl: DCFCTL,
    #[doc = "0x66 - Digital Compare B Control Register"]
    pub dcbctl: DCBCTL,
    #[doc = "0x68 - Digital Compare Filter Offset Register"]
    pub dcfoffset: DCFOFFSET,
    #[doc = "0x6a - Digital Compare Capture Control Register"]
    pub dccapctl: DCCAPCTL,
    #[doc = "0x6c - Digital Compare Filter Window Register"]
    pub dcfwindow: DCFWINDOW,
    #[doc = "0x6e - Digital Compare Filter Offset Counter Register"]
    pub dcfoffsetcnt: DCFOFFSETCNT,
    #[doc = "0x70 - Digital Compare Counter Capture Register"]
    pub dccap: DCCAP,
    #[doc = "0x72 - Digital Compare Filter Window Counter Register"]
    pub dcfwindowcnt: DCFWINDOWCNT,
}
#[doc = "Time Base Status Register"]
pub struct TBSTS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Time Base Status Register"]
pub mod tbsts;
#[doc = "Time Base Control Register"]
pub struct TBCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Time Base Control Register"]
pub mod tbctl;
#[doc = "Time Base Phase Register"]
pub struct TBPHS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Time Base Phase Register"]
pub mod tbphs;
#[doc = "Time Base Period Register"]
pub struct TBPRD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Time Base Period Register"]
pub mod tbprd;
#[doc = "Time Base Counter Register"]
pub struct TBCTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Time Base Counter Register"]
pub mod tbctr;
#[doc = "Counter Compare Control Register"]
pub struct CMPCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter Compare Control Register"]
pub mod cmpctl;
#[doc = "Counter Compare A Register"]
pub struct CMPA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter Compare A Register"]
pub mod cmpa;
#[doc = "Action Qualifier Control Register For Output A"]
pub struct AQCTLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Action Qualifier Control Register For Output A"]
pub mod aqctla;
#[doc = "Counter Compare B Register"]
pub struct CMPB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter Compare B Register"]
pub mod cmpb;
#[doc = "Action Qualifier Software Force Register"]
pub struct AQSFRC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Action Qualifier Software Force Register"]
pub mod aqsfrc;
#[doc = "Action Qualifier Control Register For Output B"]
pub struct AQCTLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Action Qualifier Control Register For Output B"]
pub mod aqctlb;
#[doc = "Dead-Band Generator Control Register"]
pub struct DBCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Dead-Band Generator Control Register"]
pub mod dbctl;
#[doc = "Action Qualifier Continuous S/W Force Register"]
pub struct AQCSFRC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Action Qualifier Continuous S/W Force Register"]
pub mod aqcsfrc;
#[doc = "Dead-Band Generator Falling Edge Delay Count Register"]
pub struct DBFED {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Dead-Band Generator Falling Edge Delay Count Register"]
pub mod dbfed;
#[doc = "Dead-Band Generator Rising Edge Delay Count Register"]
pub struct DBRED {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Dead-Band Generator Rising Edge Delay Count Register"]
pub mod dbred;
#[doc = "Trip Zone Digital Comparator Select Register"]
pub struct TZDCSEL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Trip Zone Digital Comparator Select Register"]
pub mod tzdcsel;
#[doc = "Trip Zone Select Register"]
pub struct TZSEL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Trip Zone Select Register"]
pub mod tzsel;
#[doc = "Trip Zone Enable Interrupt Register"]
pub struct TZEINT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Trip Zone Enable Interrupt Register"]
pub mod tzeint;
#[doc = "Trip Zone Control Register"]
pub struct TZCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Trip Zone Control Register"]
pub mod tzctl;
#[doc = "Trip Zone Clear Register"]
pub struct TZCLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Trip Zone Clear Register"]
pub mod tzclr;
#[doc = "Trip Zone Flag Register"]
pub struct TZFLG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Trip Zone Flag Register"]
pub mod tzflg;
#[doc = "Event Trigger Selection Register"]
pub struct ETSEL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Event Trigger Selection Register"]
pub mod etsel;
#[doc = "Trip Zone Force Register"]
pub struct TZFRC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Trip Zone Force Register"]
pub mod tzfrc;
#[doc = "Event Trigger Flag Register"]
pub struct ETFLG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Event Trigger Flag Register"]
pub mod etflg;
#[doc = "Event Trigger Pre-Scale Register"]
pub struct ETPS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Event Trigger Pre-Scale Register"]
pub mod etps;
#[doc = "Event Trigger Force Register"]
pub struct ETFRC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Event Trigger Force Register"]
pub mod etfrc;
#[doc = "Event Trigger Clear Register"]
pub struct ETCLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Event Trigger Clear Register"]
pub mod etclr;
#[doc = "PWM Chopper Control Register"]
pub struct PCCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PWM Chopper Control Register"]
pub mod pcctl;
#[doc = "Digital Compare A Control Register"]
pub struct DCACTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Digital Compare A Control Register"]
pub mod dcactl;
#[doc = "Digital Compare Trip Select Register"]
pub struct DCTRIPSEL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Digital Compare Trip Select Register"]
pub mod dctripsel;
#[doc = "Digital Compare Filter Control Register"]
pub struct DCFCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Digital Compare Filter Control Register"]
pub mod dcfctl;
#[doc = "Digital Compare B Control Register"]
pub struct DCBCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Digital Compare B Control Register"]
pub mod dcbctl;
#[doc = "Digital Compare Filter Offset Register"]
pub struct DCFOFFSET {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Digital Compare Filter Offset Register"]
pub mod dcfoffset;
#[doc = "Digital Compare Capture Control Register"]
pub struct DCCAPCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Digital Compare Capture Control Register"]
pub mod dccapctl;
#[doc = "Digital Compare Filter Window Register"]
pub struct DCFWINDOW {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Digital Compare Filter Window Register"]
pub mod dcfwindow;
#[doc = "Digital Compare Filter Offset Counter Register"]
pub struct DCFOFFSETCNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Digital Compare Filter Offset Counter Register"]
pub mod dcfoffsetcnt;
#[doc = "Digital Compare Counter Capture Register"]
pub struct DCCAP {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Digital Compare Counter Capture Register"]
pub mod dccap;
#[doc = "Digital Compare Filter Window Counter Register"]
pub struct DCFWINDOWCNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Digital Compare Filter Window Counter Register"]
pub mod dcfwindowcnt;
