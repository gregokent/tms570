#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Transmit Identification and Version Register"]
    pub txrev: TXREV,
    #[doc = "0x04 - Transmit Control Register"]
    pub txcontrol: TXCONTROL,
    #[doc = "0x08 - Transmit Teardown Register"]
    pub txteardown: TXTEARDOWN,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - RX Identification and Version Register"]
    pub rxrev: RXREV,
    #[doc = "0x14 - RX Control Register"]
    pub rxcontrol: RXCONTROL,
    #[doc = "0x18 - RX Teardown Register"]
    pub rxteardown: RXTEARDOWN,
    _reserved1: [u8; 100usize],
    #[doc = "0x80 - Transmit Interrupt Status Register"]
    pub txintstatraw: TXINTSTATRAW,
    #[doc = "0x84 - Transmit Interrupt Status Register Masked"]
    pub txintstatmasked: TXINTSTATMASKED,
    #[doc = "0x88 - Transmit Interrupt Mask Set Register"]
    pub txintmaskset: TXINTMASKSET,
    #[doc = "0x8c - Transmit Interrupt Clear Register"]
    pub txintmaskclear: TXINTMASKCLEAR,
    #[doc = "0x90 - MAC Input Vector Register"]
    pub macinvector: MACINVECTOR,
    #[doc = "0x94 - MAC End of Interrupt Vector"]
    pub maceoivector: MACEOIVECTOR,
    _reserved2: [u8; 8usize],
    #[doc = "0xa0 - Receive Interrupt Status Register Raw"]
    pub rxintstatraw: RXINTSTATRAW,
    #[doc = "0xa4 - Receive Interrupt Status Register Masked"]
    pub rxintstatmasked: RXINTSTATMASKED,
    #[doc = "0xa8 - Receive Interrupt Mask Set Register"]
    pub rxintmaskset: RXINTMASKSET,
    #[doc = "0xac - Receive Interrupt Mask Clear Register"]
    pub rxintmaskclear: RXINTMASKCLEAR,
    #[doc = "0xb0 - MAC Interrupt Status Register Raw (Unmasked)"]
    pub macintstatraw: MACINTSTATRAW,
    #[doc = "0xb4 - MAC Interrupt Status Register Masked"]
    pub macintstatmasked: MACINTSTATMASKED,
    #[doc = "0xb8 - MAC Interrupt Mask Set Register"]
    pub macintmaskset: MACINTMASKSET,
    #[doc = "0xbc - MAC Interrupt Mask Clear Register"]
    pub macintmaskclear: MACINTMASKCLEAR,
    _reserved3: [u8; 64usize],
    #[doc = "0x100 - Receive Multicast/Broadcast/Promiscuous Channel Enable Register"]
    pub rxmbpenable: RXMBPENABLE,
    #[doc = "0x104 - Receive Unicast Enable Set Register"]
    pub rxunicastset: RXUNICASTSET,
    #[doc = "0x108 - Receive Unicast Clear Register"]
    pub rxunicastclear: RXUNICASTCLEAR,
    #[doc = "0x10c - Receive Maximum Length Register"]
    pub rxmaxlen: RXMAXLEN,
    #[doc = "0x110 - Receive Buffer Offset Register"]
    pub rxbufferoffset: RXBUFFEROFFSET,
    #[doc = "0x114 - Receive Filter Low Priority Frame Threshold Register"]
    pub rxfilterlowthresh: RXFILTERLOWTHRESH,
    _reserved4: [u8; 8usize],
    #[doc = "0x120 - Receive Channel 0 Flow Control Threshold Register"]
    pub rx0flowthresh: RX0FLOWTHRESH,
    #[doc = "0x124 - Receive Channel 1 Flow Control Threshold Register"]
    pub rx1flowthresh: RX1FLOWTHRESH,
    #[doc = "0x128 - Receive Channel 2 Flow Control Threshold Register"]
    pub rx2flowthresh: RX2FLOWTHRESH,
    #[doc = "0x12c - Receive Channel 3 Flow Control Threshold Register"]
    pub rx3flowthresh: RX3FLOWTHRESH,
    #[doc = "0x130 - Receive Channel 4 Flow Control Threshold Register"]
    pub rx4flowthresh: RX4FLOWTHRESH,
    #[doc = "0x134 - Receive Channel 5 Flow Control Threshold Register"]
    pub rx5flowthresh: RX5FLOWTHRESH,
    #[doc = "0x138 - Receive Channel 6 Flow Control Threshold Register"]
    pub rx6flowthresh: RX6FLOWTHRESH,
    #[doc = "0x13c - Receive Channel 7 Flow Control Threshold Register"]
    pub rx7flowthresh: RX7FLOWTHRESH,
    #[doc = "0x140 - Receive Channel 0 Free Buffer Count Register"]
    pub rx0freebuffer: RX0FREEBUFFER,
    #[doc = "0x144 - Receive Channel 1 Free Buffer Count Register"]
    pub rx1freebuffer: RX1FREEBUFFER,
    #[doc = "0x148 - Receive Channel 2 Free Buffer Count Register"]
    pub rx2freebuffer: RX2FREEBUFFER,
    #[doc = "0x14c - Receive Channel 3 Free Buffer Count Register"]
    pub rx3freebuffer: RX3FREEBUFFER,
    #[doc = "0x150 - Receive Channel 4 Free Buffer Count Register"]
    pub rx4freebuffer: RX4FREEBUFFER,
    #[doc = "0x154 - Receive Channel 5 Free Buffer Count Register"]
    pub rx5freebuffer: RX5FREEBUFFER,
    #[doc = "0x158 - Receive Channel 6 Free Buffer Count Register"]
    pub rx6freebuffer: RX6FREEBUFFER,
    #[doc = "0x15c - Receive Channel 7 Free Buffer Count Register"]
    pub rx7freebuffer: RX7FREEBUFFER,
    #[doc = "0x160 - MAC Control Register"]
    pub maccontrol: MACCONTROL,
    #[doc = "0x164 - MAC Status Register"]
    pub macstatus: MACSTATUS,
    #[doc = "0x168 - Emulation Control Register"]
    pub emcontrol: EMCONTROL,
    #[doc = "0x16c - FIFO Control Register"]
    pub fifocontrol: FIFOCONTROL,
    #[doc = "0x170 - MAC Configuration Register"]
    pub macconfig: MACCONFIG,
    #[doc = "0x174 - Soft Reset Register"]
    pub softreset: SOFTRESET,
    _reserved5: [u8; 88usize],
    #[doc = "0x1d0 - MAC Source Address Low"]
    pub macsrcaddrlo: MACSRCADDRLO,
    #[doc = "0x1d4 - MAC Source Address High"]
    pub macsrcaddrhi: MACSRCADDRHI,
    #[doc = "0x1d8 - MAC Hash Address Register 1"]
    pub machash1: MACHASH1,
    #[doc = "0x1dc - MAC Hash Address Register 2"]
    pub machash2: MACHASH2,
    #[doc = "0x1e0 - Back Off Test Register"]
    pub bofftest: BOFFTEST,
    #[doc = "0x1e4 - Transmit Pacing Algorithm Test Register"]
    pub tpacetest: TPACETEST,
    #[doc = "0x1e8 - Receive Pause Timer Register"]
    pub rxpause: RXPAUSE,
    #[doc = "0x1ec - Transmit Pause Timer Register"]
    pub txpause: TXPAUSE,
    _reserved6: [u8; 16usize],
    #[doc = "0x200 - Good RX Frames"]
    pub rxgoodframes: RXGOODFRAMES,
    #[doc = "0x204 - Total number of good broadcast frames received"]
    pub rxbcastframes: RXBCASTFRAMES,
    #[doc = "0x208 - Total number of good multicast frames received"]
    pub rxmcastframes: RXMCASTFRAMES,
    #[doc = "0x20c - Pause RX Frames Register"]
    pub rxpauseframes: RXPAUSEFRAMES,
    #[doc = "0x210 - Total number of frames received with CRC errors"]
    pub rxcrcerrors: RXCRCERRORS,
    #[doc = "0x214 - Total number of frames received with alignment/code errors"]
    pub rxaligncodeerrors: RXALIGNCODEERRORS,
    #[doc = "0x218 - Total number of oversized frames received"]
    pub rxoversized: RXOVERSIZED,
    #[doc = "0x21c - Total number of jabber frames received"]
    pub rxjabber: RXJABBER,
    #[doc = "0x220 - Total number of undersized frames received"]
    pub rxundersized: RXUNDERSIZED,
    #[doc = "0x224 - RX Frame Fragments Register"]
    pub rxfragments: RXFRAGMENTS,
    #[doc = "0x228 - Filtered Receive Frames"]
    pub rxfiltered: RXFILTERED,
    #[doc = "0x22c - Received Frames Filtered by QOS"]
    pub rxqosfiltered: RXQOSFILTERED,
    #[doc = "0x230 - Total number of received bytes in good frames"]
    pub rxoctets: RXOCTETS,
    #[doc = "0x234 - Total number of good frames transmitted"]
    pub txgoodframes: TXGOODFRAMES,
    #[doc = "0x238 - Broadcast TX Frames Register"]
    pub txbcastframes: TXBCASTFRAMES,
    #[doc = "0x23c - Multicast TX Frames Register"]
    pub txmcastframes: TXMCASTFRAMES,
    #[doc = "0x240 - Pause TX Frames Register"]
    pub txpauseframes: TXPAUSEFRAMES,
    #[doc = "0x244 - Deferred TX Frames Register"]
    pub txdeferred: TXDEFERRED,
    #[doc = "0x248 - TX Collision Frames Register"]
    pub txcollision: TXCOLLISION,
    #[doc = "0x24c - TX Single Collision Frames Register"]
    pub txsinglecoll: TXSINGLECOLL,
    #[doc = "0x250 - TX Multiple Collision Frames Register"]
    pub txmulticoll: TXMULTICOLL,
    #[doc = "0x254 - TX Excessive Collision Frames Register"]
    pub txexcessivecoll: TXEXCESSIVECOLL,
    #[doc = "0x258 - TX Late Collision Frames Register"]
    pub txlatecoll: TXLATECOLL,
    #[doc = "0x25c - TX Underrun Error Register"]
    pub txunderrun: TXUNDERRUN,
    #[doc = "0x260 - TX Carrier Sense Errors Register"]
    pub txcarriersense: TXCARRIERSENSE,
    #[doc = "0x264 - TX Octet Frames Register"]
    pub txoctets: TXOCTETS,
    #[doc = "0x268 - Transmit and Receive 64 Octet Frames Register"]
    pub frame64: FRAME64,
    #[doc = "0x26c - Transmit and Receive 65 to 127 Octet Frames Register"]
    pub frame65t127: FRAME65T127,
    #[doc = "0x270 - Transmit and Receive 128 to 255 Octet Frames Register"]
    pub frame128t255: FRAME128T255,
    #[doc = "0x274 - Transmit and Receive 256 to 511 Octet Frames Register"]
    pub frame256t511: FRAME256T511,
    #[doc = "0x278 - Transmit and Receive 512 to 1023 Octet Frames Register"]
    pub frame512t1023: FRAME512T1023,
    #[doc = "0x27c - Transmit and Receive 1024 to 1518 Octet Frames Register"]
    pub frame1024tup: FRAME1024TUP,
    #[doc = "0x280 - Network Octet Frames Register"]
    pub netoctets: NETOCTETS,
    #[doc = "0x284 - Receive FIFO or DMA Start of Frame Overruns Register"]
    pub rxsofoverruns: RXSOFOVERRUNS,
    #[doc = "0x288 - Receive FIFO or DMA Middle of Frame Overruns Register"]
    pub rxmofoverruns: RXMOFOVERRUNS,
    #[doc = "0x28c - Receive DMA Start of Frame and Middle of Frame Overruns Register"]
    pub rxdmaoverruns: RXDMAOVERRUNS,
    _reserved7: [u8; 624usize],
    #[doc = "0x500 - MAC Address Low - From Receive Address Matching Memory Map"]
    pub macaddrlo: MACADDRLO,
    #[doc = "0x504 - MAC Address High - Receive Address Matching"]
    pub macaddrhi: MACADDRHI,
    #[doc = "0x508 - MAC Index Register"]
    pub macindex: MACINDEX,
    _reserved8: [u8; 244usize],
    #[doc = "0x600 - Transmit Channel 0 DMA Head Descriptor Pointer Register"]
    pub tx0hdp: TX0HDP,
    #[doc = "0x604 - Transmit Channel 1 DMA Head Descriptor Pointer Register"]
    pub tx1hdp: TX1HDP,
    #[doc = "0x608 - Transmit Channel 2 DMA Head Descriptor Pointer Register"]
    pub tx2hdp: TX2HDP,
    #[doc = "0x60c - Transmit Channel 3 DMA Head Descriptor Pointer Register"]
    pub tx3hdp: TX3HDP,
    #[doc = "0x610 - Transmit Channel 4 DMA Head Descriptor Pointer Register"]
    pub tx4hdp: TX4HDP,
    #[doc = "0x614 - Transmit Channel 5 DMA Head Descriptor Pointer Register"]
    pub tx5hdp: TX5HDP,
    #[doc = "0x618 - Transmit Channel 6 DMA Head Descriptor Pointer Register"]
    pub tx6hdp: TX6HDP,
    #[doc = "0x61c - Transmit Channel 7 DMA Head Descriptor Pointer Register"]
    pub tx7hdp: TX7HDP,
    #[doc = "0x620 - Receive Channel 0 DMA Head Descriptor Pointer Register"]
    pub rx0hdp: RX0HDP,
    #[doc = "0x624 - Receive Channel 1 DMA Head Descriptor Pointer Register"]
    pub rx1hdp: RX1HDP,
    #[doc = "0x628 - Receive Channel 2 DMA Head Descriptor Pointer Register"]
    pub rx2hdp: RX2HDP,
    #[doc = "0x62c - Receive Channel 3 DMA Head Descriptor Pointer Register"]
    pub rx3hdp: RX3HDP,
    #[doc = "0x630 - Receive Channel 4 DMA Head Descriptor Pointer Register"]
    pub rx4hdp: RX4HDP,
    #[doc = "0x634 - Receive Channel 5 DMA Head Descriptor Pointer Register"]
    pub rx5hdp: RX5HDP,
    #[doc = "0x638 - Receive Channel 6 DMA Head Descriptor Pointer Register"]
    pub rx6hdp: RX6HDP,
    #[doc = "0x63c - Receive Channel 7 DMA Head Descriptor Pointer Register"]
    pub rx7hdp: RX7HDP,
    #[doc = "0x640 - Transmit Channel 0 Completion Pointer (Interrupt Ack) Register"]
    pub tx0cp: TX0CP,
    #[doc = "0x644 - Transmit Channel 1 Completion Pointer Register"]
    pub tx1cp: TX1CP,
    #[doc = "0x648 - Transmit Channel 2 Completion Pointer Register"]
    pub tx2cp: TX2CP,
    #[doc = "0x64c - Transmit Channel 3 Completion Pointer Register"]
    pub tx3cp: TX3CP,
    #[doc = "0x650 - Transmit Channel 4 Completion Pointer Register"]
    pub tx4cp: TX4CP,
    #[doc = "0x654 - Transmit Channel 5 Completion Pointer Register"]
    pub tx5cp: TX5CP,
    #[doc = "0x658 - Transmit Channel 6 Completion Pointer Register"]
    pub tx6cp: TX6CP,
    #[doc = "0x65c - Transmit Channel 7 Completion Pointer Register"]
    pub tx7cp: TX7CP,
    #[doc = "0x660 - Receive Channel 0 Completion Pointer (Interrupt Ack) Register"]
    pub rx0cp: RX0CP,
    #[doc = "0x664 - Receive Channel 1 Completion Pointer (Interrupt Ack) Register"]
    pub rx1cp: RX1CP,
    #[doc = "0x668 - Receive Channel 2 Completion Pointer (Interrupt Ack) Register"]
    pub rx2cp: RX2CP,
    #[doc = "0x66c - Receive Channel 3 Completion Pointer (Interrupt Ack) Register"]
    pub rx3cp: RX3CP,
    #[doc = "0x670 - Receive Channel 4 Completion Pointer (Interrupt Ack) Register"]
    pub rx4cp: RX4CP,
    #[doc = "0x674 - Receive Channel 5 Completion Pointer (Interrupt Ack) Register"]
    pub rx5cp: RX5CP,
    #[doc = "0x678 - Receive Channel 6 Completion Pointer (Interrupt Ack) Register"]
    pub rx6cp: RX6CP,
    #[doc = "0x67c - Receive Channel 7 Completion Pointer (Interrupt Ack) Register"]
    pub rx7cp: RX7CP,
}
#[doc = "Transmit Identification and Version Register"]
pub struct TXREV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Identification and Version Register"]
pub mod txrev;
#[doc = "Transmit Control Register"]
pub struct TXCONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Control Register"]
pub mod txcontrol;
#[doc = "Transmit Teardown Register"]
pub struct TXTEARDOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Teardown Register"]
pub mod txteardown;
#[doc = "RX Identification and Version Register"]
pub struct RXREV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Identification and Version Register"]
pub mod rxrev;
#[doc = "RX Control Register"]
pub struct RXCONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Control Register"]
pub mod rxcontrol;
#[doc = "RX Teardown Register"]
pub struct RXTEARDOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Teardown Register"]
pub mod rxteardown;
#[doc = "Transmit Interrupt Status Register"]
pub struct TXINTSTATRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Interrupt Status Register"]
pub mod txintstatraw;
#[doc = "Transmit Interrupt Status Register Masked"]
pub struct TXINTSTATMASKED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Interrupt Status Register Masked"]
pub mod txintstatmasked;
#[doc = "Transmit Interrupt Mask Set Register"]
pub struct TXINTMASKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Interrupt Mask Set Register"]
pub mod txintmaskset;
#[doc = "Transmit Interrupt Clear Register"]
pub struct TXINTMASKCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Interrupt Clear Register"]
pub mod txintmaskclear;
#[doc = "MAC Input Vector Register"]
pub struct MACINVECTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Input Vector Register"]
pub mod macinvector;
#[doc = "MAC End of Interrupt Vector"]
pub struct MACEOIVECTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC End of Interrupt Vector"]
pub mod maceoivector;
#[doc = "Receive Interrupt Status Register Raw"]
pub struct RXINTSTATRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Interrupt Status Register Raw"]
pub mod rxintstatraw;
#[doc = "Receive Interrupt Status Register Masked"]
pub struct RXINTSTATMASKED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Interrupt Status Register Masked"]
pub mod rxintstatmasked;
#[doc = "Receive Interrupt Mask Set Register"]
pub struct RXINTMASKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Interrupt Mask Set Register"]
pub mod rxintmaskset;
#[doc = "Receive Interrupt Mask Clear Register"]
pub struct RXINTMASKCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Interrupt Mask Clear Register"]
pub mod rxintmaskclear;
#[doc = "MAC Interrupt Status Register Raw (Unmasked)"]
pub struct MACINTSTATRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Interrupt Status Register Raw (Unmasked)"]
pub mod macintstatraw;
#[doc = "MAC Interrupt Status Register Masked"]
pub struct MACINTSTATMASKED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Interrupt Status Register Masked"]
pub mod macintstatmasked;
#[doc = "MAC Interrupt Mask Set Register"]
pub struct MACINTMASKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Interrupt Mask Set Register"]
pub mod macintmaskset;
#[doc = "MAC Interrupt Mask Clear Register"]
pub struct MACINTMASKCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Interrupt Mask Clear Register"]
pub mod macintmaskclear;
#[doc = "Receive Multicast/Broadcast/Promiscuous Channel Enable Register"]
pub struct RXMBPENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Multicast/Broadcast/Promiscuous Channel Enable Register"]
pub mod rxmbpenable;
#[doc = "Receive Unicast Enable Set Register"]
pub struct RXUNICASTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Unicast Enable Set Register"]
pub mod rxunicastset;
#[doc = "Receive Unicast Clear Register"]
pub struct RXUNICASTCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Unicast Clear Register"]
pub mod rxunicastclear;
#[doc = "Receive Maximum Length Register"]
pub struct RXMAXLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Maximum Length Register"]
pub mod rxmaxlen;
#[doc = "Receive Buffer Offset Register"]
pub struct RXBUFFEROFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Offset Register"]
pub mod rxbufferoffset;
#[doc = "Receive Filter Low Priority Frame Threshold Register"]
pub struct RXFILTERLOWTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Filter Low Priority Frame Threshold Register"]
pub mod rxfilterlowthresh;
#[doc = "Receive Channel 0 Flow Control Threshold Register"]
pub struct RX0FLOWTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 0 Flow Control Threshold Register"]
pub mod rx0flowthresh;
#[doc = "Receive Channel 1 Flow Control Threshold Register"]
pub struct RX1FLOWTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 1 Flow Control Threshold Register"]
pub mod rx1flowthresh;
#[doc = "Receive Channel 2 Flow Control Threshold Register"]
pub struct RX2FLOWTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 2 Flow Control Threshold Register"]
pub mod rx2flowthresh;
#[doc = "Receive Channel 3 Flow Control Threshold Register"]
pub struct RX3FLOWTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 3 Flow Control Threshold Register"]
pub mod rx3flowthresh;
#[doc = "Receive Channel 4 Flow Control Threshold Register"]
pub struct RX4FLOWTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 4 Flow Control Threshold Register"]
pub mod rx4flowthresh;
#[doc = "Receive Channel 5 Flow Control Threshold Register"]
pub struct RX5FLOWTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 5 Flow Control Threshold Register"]
pub mod rx5flowthresh;
#[doc = "Receive Channel 6 Flow Control Threshold Register"]
pub struct RX6FLOWTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 6 Flow Control Threshold Register"]
pub mod rx6flowthresh;
#[doc = "Receive Channel 7 Flow Control Threshold Register"]
pub struct RX7FLOWTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 7 Flow Control Threshold Register"]
pub mod rx7flowthresh;
#[doc = "Receive Channel 0 Free Buffer Count Register"]
pub struct RX0FREEBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 0 Free Buffer Count Register"]
pub mod rx0freebuffer;
#[doc = "Receive Channel 1 Free Buffer Count Register"]
pub struct RX1FREEBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 1 Free Buffer Count Register"]
pub mod rx1freebuffer;
#[doc = "Receive Channel 2 Free Buffer Count Register"]
pub struct RX2FREEBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 2 Free Buffer Count Register"]
pub mod rx2freebuffer;
#[doc = "Receive Channel 3 Free Buffer Count Register"]
pub struct RX3FREEBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 3 Free Buffer Count Register"]
pub mod rx3freebuffer;
#[doc = "Receive Channel 4 Free Buffer Count Register"]
pub struct RX4FREEBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 4 Free Buffer Count Register"]
pub mod rx4freebuffer;
#[doc = "Receive Channel 5 Free Buffer Count Register"]
pub struct RX5FREEBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 5 Free Buffer Count Register"]
pub mod rx5freebuffer;
#[doc = "Receive Channel 6 Free Buffer Count Register"]
pub struct RX6FREEBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 6 Free Buffer Count Register"]
pub mod rx6freebuffer;
#[doc = "Receive Channel 7 Free Buffer Count Register"]
pub struct RX7FREEBUFFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 7 Free Buffer Count Register"]
pub mod rx7freebuffer;
#[doc = "MAC Control Register"]
pub struct MACCONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Control Register"]
pub mod maccontrol;
#[doc = "MAC Status Register"]
pub struct MACSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Status Register"]
pub mod macstatus;
#[doc = "Emulation Control Register"]
pub struct EMCONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Emulation Control Register"]
pub mod emcontrol;
#[doc = "FIFO Control Register"]
pub struct FIFOCONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Control Register"]
pub mod fifocontrol;
#[doc = "MAC Configuration Register"]
pub struct MACCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Configuration Register"]
pub mod macconfig;
#[doc = "Soft Reset Register"]
pub struct SOFTRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Soft Reset Register"]
pub mod softreset;
#[doc = "MAC Source Address Low"]
pub struct MACSRCADDRLO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Source Address Low"]
pub mod macsrcaddrlo;
#[doc = "MAC Source Address High"]
pub struct MACSRCADDRHI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Source Address High"]
pub mod macsrcaddrhi;
#[doc = "MAC Hash Address Register 1"]
pub struct MACHASH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Hash Address Register 1"]
pub mod machash1;
#[doc = "MAC Hash Address Register 2"]
pub struct MACHASH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Hash Address Register 2"]
pub mod machash2;
#[doc = "Back Off Test Register"]
pub struct BOFFTEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Back Off Test Register"]
pub mod bofftest;
#[doc = "Transmit Pacing Algorithm Test Register"]
pub struct TPACETEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Pacing Algorithm Test Register"]
pub mod tpacetest;
#[doc = "Receive Pause Timer Register"]
pub struct RXPAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Pause Timer Register"]
pub mod rxpause;
#[doc = "Transmit Pause Timer Register"]
pub struct TXPAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Pause Timer Register"]
pub mod txpause;
#[doc = "MAC Address Low - From Receive Address Matching Memory Map"]
pub struct MACADDRLO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Address Low - From Receive Address Matching Memory Map"]
pub mod macaddrlo;
#[doc = "MAC Address High - Receive Address Matching"]
pub struct MACADDRHI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Address High - Receive Address Matching"]
pub mod macaddrhi;
#[doc = "MAC Index Register"]
pub struct MACINDEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC Index Register"]
pub mod macindex;
#[doc = "Transmit Channel 0 DMA Head Descriptor Pointer Register"]
pub struct TX0HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 0 DMA Head Descriptor Pointer Register"]
pub mod tx0hdp;
#[doc = "Transmit Channel 1 DMA Head Descriptor Pointer Register"]
pub struct TX1HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 1 DMA Head Descriptor Pointer Register"]
pub mod tx1hdp;
#[doc = "Transmit Channel 2 DMA Head Descriptor Pointer Register"]
pub struct TX2HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 2 DMA Head Descriptor Pointer Register"]
pub mod tx2hdp;
#[doc = "Transmit Channel 3 DMA Head Descriptor Pointer Register"]
pub struct TX3HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 3 DMA Head Descriptor Pointer Register"]
pub mod tx3hdp;
#[doc = "Transmit Channel 4 DMA Head Descriptor Pointer Register"]
pub struct TX4HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 4 DMA Head Descriptor Pointer Register"]
pub mod tx4hdp;
#[doc = "Transmit Channel 5 DMA Head Descriptor Pointer Register"]
pub struct TX5HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 5 DMA Head Descriptor Pointer Register"]
pub mod tx5hdp;
#[doc = "Transmit Channel 6 DMA Head Descriptor Pointer Register"]
pub struct TX6HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 6 DMA Head Descriptor Pointer Register"]
pub mod tx6hdp;
#[doc = "Transmit Channel 7 DMA Head Descriptor Pointer Register"]
pub struct TX7HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 7 DMA Head Descriptor Pointer Register"]
pub mod tx7hdp;
#[doc = "Receive Channel 0 DMA Head Descriptor Pointer Register"]
pub struct RX0HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 0 DMA Head Descriptor Pointer Register"]
pub mod rx0hdp;
#[doc = "Receive Channel 1 DMA Head Descriptor Pointer Register"]
pub struct RX1HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 1 DMA Head Descriptor Pointer Register"]
pub mod rx1hdp;
#[doc = "Receive Channel 2 DMA Head Descriptor Pointer Register"]
pub struct RX2HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 2 DMA Head Descriptor Pointer Register"]
pub mod rx2hdp;
#[doc = "Receive Channel 3 DMA Head Descriptor Pointer Register"]
pub struct RX3HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 3 DMA Head Descriptor Pointer Register"]
pub mod rx3hdp;
#[doc = "Receive Channel 4 DMA Head Descriptor Pointer Register"]
pub struct RX4HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 4 DMA Head Descriptor Pointer Register"]
pub mod rx4hdp;
#[doc = "Receive Channel 5 DMA Head Descriptor Pointer Register"]
pub struct RX5HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 5 DMA Head Descriptor Pointer Register"]
pub mod rx5hdp;
#[doc = "Receive Channel 6 DMA Head Descriptor Pointer Register"]
pub struct RX6HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 6 DMA Head Descriptor Pointer Register"]
pub mod rx6hdp;
#[doc = "Receive Channel 7 DMA Head Descriptor Pointer Register"]
pub struct RX7HDP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 7 DMA Head Descriptor Pointer Register"]
pub mod rx7hdp;
#[doc = "Transmit Channel 0 Completion Pointer (Interrupt Ack) Register"]
pub struct TX0CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 0 Completion Pointer (Interrupt Ack) Register"]
pub mod tx0cp;
#[doc = "Transmit Channel 1 Completion Pointer Register"]
pub struct TX1CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 1 Completion Pointer Register"]
pub mod tx1cp;
#[doc = "Transmit Channel 2 Completion Pointer Register"]
pub struct TX2CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 2 Completion Pointer Register"]
pub mod tx2cp;
#[doc = "Transmit Channel 3 Completion Pointer Register"]
pub struct TX3CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 3 Completion Pointer Register"]
pub mod tx3cp;
#[doc = "Transmit Channel 4 Completion Pointer Register"]
pub struct TX4CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 4 Completion Pointer Register"]
pub mod tx4cp;
#[doc = "Transmit Channel 5 Completion Pointer Register"]
pub struct TX5CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 5 Completion Pointer Register"]
pub mod tx5cp;
#[doc = "Transmit Channel 6 Completion Pointer Register"]
pub struct TX6CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 6 Completion Pointer Register"]
pub mod tx6cp;
#[doc = "Transmit Channel 7 Completion Pointer Register"]
pub struct TX7CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Channel 7 Completion Pointer Register"]
pub mod tx7cp;
#[doc = "Receive Channel 0 Completion Pointer (Interrupt Ack) Register"]
pub struct RX0CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 0 Completion Pointer (Interrupt Ack) Register"]
pub mod rx0cp;
#[doc = "Receive Channel 1 Completion Pointer (Interrupt Ack) Register"]
pub struct RX1CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 1 Completion Pointer (Interrupt Ack) Register"]
pub mod rx1cp;
#[doc = "Receive Channel 2 Completion Pointer (Interrupt Ack) Register"]
pub struct RX2CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 2 Completion Pointer (Interrupt Ack) Register"]
pub mod rx2cp;
#[doc = "Receive Channel 3 Completion Pointer (Interrupt Ack) Register"]
pub struct RX3CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 3 Completion Pointer (Interrupt Ack) Register"]
pub mod rx3cp;
#[doc = "Receive Channel 4 Completion Pointer (Interrupt Ack) Register"]
pub struct RX4CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 4 Completion Pointer (Interrupt Ack) Register"]
pub mod rx4cp;
#[doc = "Receive Channel 5 Completion Pointer (Interrupt Ack) Register"]
pub struct RX5CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 5 Completion Pointer (Interrupt Ack) Register"]
pub mod rx5cp;
#[doc = "Receive Channel 6 Completion Pointer (Interrupt Ack) Register"]
pub struct RX6CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 6 Completion Pointer (Interrupt Ack) Register"]
pub mod rx6cp;
#[doc = "Receive Channel 7 Completion Pointer (Interrupt Ack) Register"]
pub struct RX7CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Channel 7 Completion Pointer (Interrupt Ack) Register"]
pub mod rx7cp;
#[doc = "Good RX Frames"]
pub struct RXGOODFRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Good RX Frames"]
pub mod rxgoodframes;
#[doc = "Total number of good broadcast frames received"]
pub struct RXBCASTFRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total number of good broadcast frames received"]
pub mod rxbcastframes;
#[doc = "Total number of good multicast frames received"]
pub struct RXMCASTFRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total number of good multicast frames received"]
pub mod rxmcastframes;
#[doc = "Pause RX Frames Register"]
pub struct RXPAUSEFRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pause RX Frames Register"]
pub mod rxpauseframes;
#[doc = "Total number of frames received with CRC errors"]
pub struct RXCRCERRORS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total number of frames received with CRC errors"]
pub mod rxcrcerrors;
#[doc = "Total number of frames received with alignment/code errors"]
pub struct RXALIGNCODEERRORS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total number of frames received with alignment/code errors"]
pub mod rxaligncodeerrors;
#[doc = "Total number of oversized frames received"]
pub struct RXOVERSIZED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total number of oversized frames received"]
pub mod rxoversized;
#[doc = "Total number of jabber frames received"]
pub struct RXJABBER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total number of jabber frames received"]
pub mod rxjabber;
#[doc = "Total number of undersized frames received"]
pub struct RXUNDERSIZED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total number of undersized frames received"]
pub mod rxundersized;
#[doc = "RX Frame Fragments Register"]
pub struct RXFRAGMENTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Frame Fragments Register"]
pub mod rxfragments;
#[doc = "Filtered Receive Frames"]
pub struct RXFILTERED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filtered Receive Frames"]
pub mod rxfiltered;
#[doc = "Received Frames Filtered by QOS"]
pub struct RXQOSFILTERED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received Frames Filtered by QOS"]
pub mod rxqosfiltered;
#[doc = "Total number of received bytes in good frames"]
pub struct RXOCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total number of received bytes in good frames"]
pub mod rxoctets;
#[doc = "Total number of good frames transmitted"]
pub struct TXGOODFRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total number of good frames transmitted"]
pub mod txgoodframes;
#[doc = "Broadcast TX Frames Register"]
pub struct TXBCASTFRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Broadcast TX Frames Register"]
pub mod txbcastframes;
#[doc = "Multicast TX Frames Register"]
pub struct TXMCASTFRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multicast TX Frames Register"]
pub mod txmcastframes;
#[doc = "Pause TX Frames Register"]
pub struct TXPAUSEFRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pause TX Frames Register"]
pub mod txpauseframes;
#[doc = "Deferred TX Frames Register"]
pub struct TXDEFERRED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deferred TX Frames Register"]
pub mod txdeferred;
#[doc = "TX Collision Frames Register"]
pub struct TXCOLLISION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Collision Frames Register"]
pub mod txcollision;
#[doc = "TX Single Collision Frames Register"]
pub struct TXSINGLECOLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Single Collision Frames Register"]
pub mod txsinglecoll;
#[doc = "TX Multiple Collision Frames Register"]
pub struct TXMULTICOLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Multiple Collision Frames Register"]
pub mod txmulticoll;
#[doc = "TX Excessive Collision Frames Register"]
pub struct TXEXCESSIVECOLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Excessive Collision Frames Register"]
pub mod txexcessivecoll;
#[doc = "TX Late Collision Frames Register"]
pub struct TXLATECOLL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Late Collision Frames Register"]
pub mod txlatecoll;
#[doc = "TX Underrun Error Register"]
pub struct TXUNDERRUN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Underrun Error Register"]
pub mod txunderrun;
#[doc = "TX Carrier Sense Errors Register"]
pub struct TXCARRIERSENSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Carrier Sense Errors Register"]
pub mod txcarriersense;
#[doc = "TX Octet Frames Register"]
pub struct TXOCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Octet Frames Register"]
pub mod txoctets;
#[doc = "Transmit and Receive 64 Octet Frames Register"]
pub struct FRAME64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit and Receive 64 Octet Frames Register"]
pub mod frame64;
#[doc = "Transmit and Receive 65 to 127 Octet Frames Register"]
pub struct FRAME65T127 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit and Receive 65 to 127 Octet Frames Register"]
pub mod frame65t127;
#[doc = "Transmit and Receive 128 to 255 Octet Frames Register"]
pub struct FRAME128T255 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit and Receive 128 to 255 Octet Frames Register"]
pub mod frame128t255;
#[doc = "Transmit and Receive 256 to 511 Octet Frames Register"]
pub struct FRAME256T511 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit and Receive 256 to 511 Octet Frames Register"]
pub mod frame256t511;
#[doc = "Transmit and Receive 512 to 1023 Octet Frames Register"]
pub struct FRAME512T1023 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit and Receive 512 to 1023 Octet Frames Register"]
pub mod frame512t1023;
#[doc = "Transmit and Receive 1024 to 1518 Octet Frames Register"]
pub struct FRAME1024TUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit and Receive 1024 to 1518 Octet Frames Register"]
pub mod frame1024tup;
#[doc = "Network Octet Frames Register"]
pub struct NETOCTETS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Network Octet Frames Register"]
pub mod netoctets;
#[doc = "Receive FIFO or DMA Start of Frame Overruns Register"]
pub struct RXSOFOVERRUNS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO or DMA Start of Frame Overruns Register"]
pub mod rxsofoverruns;
#[doc = "Receive FIFO or DMA Middle of Frame Overruns Register"]
pub struct RXMOFOVERRUNS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO or DMA Middle of Frame Overruns Register"]
pub mod rxmofoverruns;
#[doc = "Receive DMA Start of Frame and Middle of Frame Overruns Register"]
pub struct RXDMAOVERRUNS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive DMA Start of Frame and Middle of Frame Overruns Register"]
pub mod rxdmaoverruns;
