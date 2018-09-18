#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Set-only register to Protect PCS frames 0 to 31"]
    pub pmprot_set0: PMPROTSET0,
    #[doc = "0x04 - Set-only register to Protect PCS frames 32 to 64"]
    pub pmprot_set1: PMPROTSET1,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Clear-only register to Protect PCS frames 0 to 31"]
    pub pmprot_clr0: PMPROTCLR0,
    #[doc = "0x14 - Clear-only register to Protect PCS frames 32 to 64"]
    pub pmprot_clr1: PMPROTCLR1,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - Set-only register to Protect the 32 quadrants of PS0 to PS7"]
    pub pprot_set0: PPROTSET0,
    #[doc = "0x24 - Set-only register to Protect the 32 quadrants of PS8 to PS15"]
    pub pprot_set1: PPROTSET1,
    #[doc = "0x28 - Set-only register to Protect the 32 quadrants of PS16 to PS23"]
    pub pprot_set2: PPROTSET2,
    #[doc = "0x2c - Set-only register to Protect the 32 quadrants of PS24 to PS31"]
    pub pprot_set3: PPROTSET3,
    _reserved2: [u8; 16usize],
    #[doc = "0x40 - clear-only register to Protect the quadrants of PS0 to PS7"]
    pub pprot_clr0: PPROTCLR0,
    #[doc = "0x44 - clear-only register to Protect the quadrants of PS8 to PS15"]
    pub pprot_clr1: PPROTCLR1,
    #[doc = "0x48 - clear-only register to Protect the quadrants of PS16 to PS23"]
    pub pprot_clr2: PPROTCLR2,
    #[doc = "0x4c - clear-only register to Protect the quadrants of PS24 to PS31"]
    pub pprot_clr3: PPROTCLR3,
    _reserved3: [u8; 16usize],
    #[doc = "0x60 - no description"]
    pub pcspwr_dwn_set0: PCSPWRDWNSET0,
    #[doc = "0x64 - no description"]
    pub pcspwr_dwn_set1: PCSPWRDWNSET1,
    _reserved4: [u8; 8usize],
    #[doc = "0x70 - no description"]
    pub pcspwr_dwn_clr0: PCSPWRDWNCLR0,
    #[doc = "0x74 - no description"]
    pub pcspwr_dwn_clr1: PCSPWRDWNCLR1,
    _reserved5: [u8; 8usize],
    #[doc = "0x80 - no description"]
    pub pspwr_dwn_set0: PSPWRDWNSET0,
    #[doc = "0x84 - no description"]
    pub pspwr_dwn_set1: PSPWRDWNSET1,
    #[doc = "0x88 - no description"]
    pub pspwr_dwn_set2: PSPWRDWNSET2,
    #[doc = "0x8c - no description"]
    pub pspwr_dwn_set3: PSPWRDWNSET3,
    _reserved6: [u8; 16usize],
    #[doc = "0xa0 - no description"]
    pub pspwr_dwn_clr0: PSPWRDWNCLR0,
    #[doc = "0xa4 - no description"]
    pub pspwr_dwn_clr1: PSPWRDWNCLR1,
    #[doc = "0xa8 - no description"]
    pub pspwr_dwn_clr2: PSPWRDWNCLR2,
    #[doc = "0xac - no description"]
    pub pspwr_dwn_clr3: PSPWRDWNCLR3,
    _reserved7: [u8; 16usize],
    #[doc = "0xc0 - Debug Frame Power Down Set register"]
    pub dbg_pwr_dwn_set: DBGPWRDWNSET,
    _reserved8: [u8; 316usize],
    #[doc = "0x200 - MasterID Protection Write Enable Register"]
    pub mst_id_wr_ena: MSTIDWRENA,
    #[doc = "0x204 - MasterID Check Enable Register"]
    pub mst_id_ena: MSTIDENA,
    #[doc = "0x208 - MasterID Diagnostic Control Register"]
    pub mst_id_diag_ctrl: MSTIDDIAGCTRL,
    _reserved9: [u8; 244usize],
    #[doc = "0x300 - Peripheral Frame 0 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps0mst_id_l: PS0MSTID_L,
    #[doc = "0x304 - Peripheral Frame 0 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps0mst_id_h: PS0MSTID_H,
    #[doc = "0x308 - Peripheral Frame 1 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps1mst_id_l: PS1MSTID_L,
    #[doc = "0x30c - Peripheral Frame 1 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps1mst_id_h: PS1MSTID_H,
    #[doc = "0x310 - Peripheral Frame 2 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps2mst_id_l: PS2MSTID_L,
    #[doc = "0x314 - Peripheral Frame 2 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps2mst_id_h: PS2MSTID_H,
    #[doc = "0x318 - Peripheral Frame 3 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps3mst_id_l: PS3MSTID_L,
    #[doc = "0x31c - Peripheral Frame 3 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps3mst_id_h: PS3MSTID_H,
    #[doc = "0x320 - Peripheral Frame 4 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps4mst_id_l: PS4MSTID_L,
    #[doc = "0x324 - Peripheral Frame 4 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps4mst_id_h: PS4MSTID_H,
    #[doc = "0x328 - Peripheral Frame 5 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps5mst_id_l: PS5MSTID_L,
    #[doc = "0x32c - Peripheral Frame 5 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps5mst_id_h: PS5MSTID_H,
    #[doc = "0x330 - Peripheral Frame 6 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps6mst_id_l: PS6MSTID_L,
    #[doc = "0x334 - Peripheral Frame 6 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps6mst_id_h: PS6MSTID_H,
    #[doc = "0x338 - Peripheral Frame 7 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps7mst_id_l: PS7MSTID_L,
    #[doc = "0x33c - Peripheral Frame 7 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps7mst_id_h: PS7MSTID_H,
    #[doc = "0x340 - Peripheral Frame 8 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps8mst_id_l: PS8MSTID_L,
    #[doc = "0x344 - Peripheral Frame 8 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps8mst_id_h: PS8MSTID_H,
    #[doc = "0x348 - Peripheral Frame 9 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps9mst_id_l: PS9MSTID_L,
    #[doc = "0x34c - Peripheral Frame 9 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps9mst_id_h: PS9MSTID_H,
    #[doc = "0x350 - Peripheral Frame 10 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps10mst_id_l: PS10MSTID_L,
    #[doc = "0x354 - Peripheral Frame 10(Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps10mst_id_h: PS10MSTID_H,
    #[doc = "0x358 - Peripheral Frame 11 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps11mst_id_l: PS11MSTID_L,
    #[doc = "0x35c - Peripheral Frame 11 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps11mst_id_h: PS11MSTID_H,
    #[doc = "0x360 - Peripheral Frame 12 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps12mst_id_l: PS12MSTID_L,
    #[doc = "0x364 - Peripheral Frame 12 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps12mst_id_h: PS12MSTID_H,
    #[doc = "0x368 - Peripheral Frame 13 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps13mst_id_l: PS13MSTID_L,
    #[doc = "0x36c - Peripheral Frame 13 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps13mst_id_h: PS13MSTID_H,
    #[doc = "0x370 - Peripheral Frame 14 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps14mst_id_l: PS14MSTID_L,
    #[doc = "0x374 - Peripheral Frame 14 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps14mst_id_h: PS14MSTID_H,
    #[doc = "0x378 - Peripheral Frame 15 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps15mst_id_l: PS15MSTID_L,
    #[doc = "0x37c - Peripheral Frame 15 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps15mst_id_h: PS15MSTID_H,
    #[doc = "0x380 - Peripheral Frame 16 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps16mst_id_l: PS16MSTID_L,
    #[doc = "0x384 - Peripheral Frame 16 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps16mst_id_h: PS16MSTID_H,
    #[doc = "0x388 - Peripheral Frame 17 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps17mst_id_l: PS17MSTID_L,
    #[doc = "0x38c - Peripheral Frame 17 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps17mst_id_h: PS17MSTID_H,
    #[doc = "0x390 - Peripheral Frame 18 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps18mst_id_l: PS18MSTID_L,
    #[doc = "0x394 - Peripheral Frame 18 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps18mst_id_h: PS18MSTID_H,
    #[doc = "0x398 - Peripheral Frame 19 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps19mst_id_l: PS19MSTID_L,
    #[doc = "0x39c - Peripheral Frame 19 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps19mst_id_h: PS19MSTID_H,
    #[doc = "0x3a0 - Peripheral Frame 20 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps20mst_id_l: PS20MSTID_L,
    #[doc = "0x3a4 - Peripheral Frame 20 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps20mst_id_h: PS20MSTID_H,
    #[doc = "0x3a8 - Peripheral Frame 21 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps21mst_id_l: PS21MSTID_L,
    #[doc = "0x3ac - Peripheral Frame 21 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps21mst_id_h: PS21MSTID_H,
    #[doc = "0x3b0 - Peripheral Frame 22 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps22mst_id_l: PS22MSTID_L,
    #[doc = "0x3b4 - Peripheral Frame 22 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps22mst_id_h: PS22MSTID_H,
    #[doc = "0x3b8 - Peripheral Frame 23 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps23mst_id_l: PS23MSTID_L,
    #[doc = "0x3bc - Peripheral Frame 23 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps23mst_id_h: PS23MSTID_H,
    #[doc = "0x3c0 - Peripheral Frame 24 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps24mst_id_l: PS24MSTID_L,
    #[doc = "0x3c4 - Peripheral Frame 24 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps24mst_id_h: PS24MSTID_H,
    #[doc = "0x3c8 - Peripheral Frame 25 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps25mst_id_l: PS25MSTID_L,
    #[doc = "0x3cc - Peripheral Frame 25 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps25mst_id_h: PS25MSTID_H,
    #[doc = "0x3d0 - Peripheral Frame 26 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps26mst_id_l: PS26MSTID_L,
    #[doc = "0x3d4 - Peripheral Frame 26 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps26mst_id_h: PS26MSTID_H,
    #[doc = "0x3d8 - Peripheral Frame 27 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps27mst_id_l: PS27MSTID_L,
    #[doc = "0x3dc - Peripheral Frame 27 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps27mst_id_h: PS27MSTID_H,
    #[doc = "0x3e0 - Peripheral Frame 28 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps28mst_id_l: PS28MSTID_L,
    #[doc = "0x3e4 - Peripheral Frame 28 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps28mst_id_h: PS28MSTID_H,
    #[doc = "0x3e8 - Peripheral Frame 29 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps29mst_id_l: PS29MSTID_L,
    #[doc = "0x3ec - Peripheral Frame 29 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps29mst_id_h: PS29MSTID_H,
    #[doc = "0x3f0 - Peripheral Frame 30 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps30mst_id_l: PS30MSTID_L,
    #[doc = "0x3f4 - Peripheral Frame 30 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps30mst_id_h: PS30MSTID_H,
    #[doc = "0x3f8 - Peripheral Frame 31 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ps31mst_id_l: PS31MSTID_L,
    #[doc = "0x3fc - Peripheral Frame 31 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ps31mst_id_h: PS31MSTID_H,
    #[doc = "0x400 - Privileged Peripheral Frame 0 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub pps0mst_id_l: PPS0MSTID_L,
    #[doc = "0x404 - Privileged Peripheral Frame 0 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub pps0mst_id_h: PPS0MSTID_H,
    #[doc = "0x408 - Privileged Peripheral Frame 1 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub pps1mst_id_l: PPS1MSTID_L,
    #[doc = "0x40c - Privileged Peripheral Frame 1 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub pps1mst_id_h: PPS1MSTID_H,
    #[doc = "0x410 - Privileged Peripheral Frame 2 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub pps2mst_id_l: PPS2MSTID_L,
    #[doc = "0x414 - Privileged Peripheral Frame 2 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub pps2mst_id_h: PPS2MSTID_H,
    #[doc = "0x418 - Privileged Peripheral Frame 3 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub pps3mst_id_l: PPS3MSTID_L,
    #[doc = "0x41c - Privileged Peripheral Frame 3 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub pps3mst_id_h: PPS3MSTID_H,
    #[doc = "0x420 - Privileged Peripheral Frame 4 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub pps4mst_id_l: PPS4MSTID_L,
    #[doc = "0x424 - Privileged Peripheral Frame 4 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub pps4mst_id_h: PPS4MSTID_H,
    #[doc = "0x428 - Privileged Peripheral Frame 5 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub pps5mst_id_l: PPS5MSTID_L,
    #[doc = "0x42c - Privileged Peripheral Frame 5 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub pps5mst_id_h: PPS5MSTID_H,
    #[doc = "0x430 - Privileged Peripheral Frame 6 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub pps6mst_id_l: PPS6MSTID_L,
    #[doc = "0x434 - Privileged Peripheral Frame 6 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub pps6mst_id_h: PPS6MSTID_H,
    #[doc = "0x438 - Privileged Peripheral Frame 7 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub pps7mst_id_l: PPS7MSTID_L,
    #[doc = "0x43c - Privileged Peripheral Frame 7 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub pps7mst_id_h: PPS7MSTID_H,
    #[doc = "0x440 - Privileged Peripheral Extended Frame 0 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse0mst_id_l: PPSE0MSTID_L,
    #[doc = "0x444 - Privileged Peripheral Extended Frame 0 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse0mst_id_h: PPSE0MSTID_H,
    #[doc = "0x448 - Privileged Peripheral Extended Frame 1 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse1mst_id_l: PPSE1MSTID_L,
    #[doc = "0x44c - Privileged Peripheral Extended Frame 1 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse1mst_id_h: PPSE1MSTID_H,
    #[doc = "0x450 - Privileged Peripheral Extended Frame 2 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse2mst_id_l: PPSE2MSTID_L,
    #[doc = "0x454 - Privileged Peripheral Extended Frame 2 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse2mst_id_h: PPSE2MSTID_H,
    #[doc = "0x458 - Privileged Peripheral Extended Frame 3 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse3mst_id_l: PPSE3MSTID_L,
    #[doc = "0x45c - Privileged Peripheral Extended Frame 3 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse3mst_id_h: PPSE3MSTID_H,
    #[doc = "0x460 - Privileged Peripheral Extended Frame 4 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse4mst_id_l: PPSE4MSTID_L,
    #[doc = "0x464 - Privileged Peripheral Extended Frame 4 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse4mst_id_h: PPSE4MSTID_H,
    #[doc = "0x468 - Privileged Peripheral Extended Frame 5 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse5mst_id_l: PPSE5MSTID_L,
    #[doc = "0x46c - Privileged Peripheral Extended Frame 5 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse5mst_id_h: PPSE5MSTID_H,
    #[doc = "0x470 - Privileged Peripheral Extended Frame 6 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse6mst_id_l: PPSE6MSTID_L,
    #[doc = "0x474 - Privileged Peripheral Extended Frame 6 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse6mst_id_h: PPSE6MSTID_H,
    #[doc = "0x478 - Privileged Peripheral Extended Frame 7 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse7mst_id_l: PPSE7MSTID_L,
    #[doc = "0x47c - Privileged Peripheral Extended Frame 7 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse7mst_id_h: PPSE7MSTID_H,
    #[doc = "0x480 - Privileged Peripheral Extended Frame 8 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse8mst_id_l: PPSE8MSTID_L,
    #[doc = "0x484 - Privileged Peripheral Extended Frame 8 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse8mst_id_h: PPSE8MSTID_H,
    #[doc = "0x488 - Privileged Peripheral Extended Frame 9 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse9mst_id_l: PPSE9MSTID_L,
    #[doc = "0x48c - Privileged Peripheral Extended Frame 9 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse9mst_id_h: PPSE9MSTID_H,
    #[doc = "0x490 - Privileged Peripheral Extended Frame 10 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse10mst_id_l: PPSE10MSTID_L,
    #[doc = "0x494 - Privileged Peripheral Extended Frame 10 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse10mst_id_h: PPSE10MSTID_H,
    #[doc = "0x498 - Privileged Peripheral Extended Frame 11 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse11mst_id_l: PPSE11MSTID_L,
    #[doc = "0x49c - Privileged Peripheral Extended Frame 11 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse11mst_id_h: PPSE11MSTID_H,
    #[doc = "0x4a0 - Privileged Peripheral Extended Frame 12 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse12mst_id_l: PPSE12MSTID_L,
    #[doc = "0x4a4 - Privileged Peripheral Extended Frame 12 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse12mst_id_h: PPSE12MSTID_H,
    #[doc = "0x4a8 - Privileged Peripheral Extended Frame 13 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse13mst_id_l: PPSE13MSTID_L,
    #[doc = "0x4ac - Privileged Peripheral Extended Frame 13 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse13mst_id_h: PPSE13MSTID_H,
    #[doc = "0x4b0 - Privileged Peripheral Extended Frame 14 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse14mst_id_l: PPSE14MSTID_L,
    #[doc = "0x4b4 - Privileged Peripheral Extended Frame 14 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse14mst_id_h: PPSE14MSTID_H,
    #[doc = "0x4b8 - Privileged Peripheral Extended Frame 15 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse15mst_id_l: PPSE15MSTID_L,
    #[doc = "0x4bc - Privileged Peripheral Extended Frame 15 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse15mst_id_h: PPSE15MSTID_H,
    #[doc = "0x4c0 - Privileged Peripheral Extended Frame 16 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse16mst_id_l: PPSE16MSTID_L,
    #[doc = "0x4c4 - Privileged Peripheral Extended Frame 16 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse16mst_id_h: PPSE16MSTID_H,
    #[doc = "0x4c8 - Privileged Peripheral Extended Frame 17 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse17mst_id_l: PPSE17MSTID_L,
    #[doc = "0x4cc - Privileged Peripheral Extended Frame 17 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse17mst_id_h: PPSE17MSTID_H,
    #[doc = "0x4d0 - Privileged Peripheral Extended Frame 18 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse18mst_id_l: PPSE18MSTID_L,
    #[doc = "0x4d4 - Privileged Peripheral Extended Frame 18 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse18mst_id_h: PPSE18MSTID_H,
    #[doc = "0x4d8 - Privileged Peripheral Extended Frame 19 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse19mst_id_l: PPSE19MSTID_L,
    #[doc = "0x4dc - Privileged Peripheral Extended Frame 19 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse19mst_id_h: PPSE19MSTID_H,
    #[doc = "0x4e0 - Privileged Peripheral Extended Frame 20 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse20mst_id_l: PPSE20MSTID_L,
    #[doc = "0x4e4 - Privileged Peripheral Extended Frame 20 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse20mst_id_h: PPSE20MSTID_H,
    #[doc = "0x4e8 - Privileged Peripheral Extended Frame 21 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse21mst_id_l: PPSE21MSTID_L,
    _reserved10: [u8; 4usize],
    #[doc = "0x4f0 - Privileged Peripheral Extended Frame 22 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse22mst_id_l: PPSE22MSTID_L,
    #[doc = "0x4f4 - Privileged Peripheral Extended Frame 22 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse22mst_id_h: PPSE22MSTID_H,
    #[doc = "0x4f8 - Privileged Peripheral Extended Frame 23 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse23mst_id_l: PPSE23MSTID_L,
    #[doc = "0x4fc - Privileged Peripheral Extended Frame 21 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse21mst_id_h: PPSE21MSTID_H,
    #[doc = "0x500 - Privileged Peripheral Extended Frame 24 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse24mst_id_l: PPSE24MSTID_L,
    #[doc = "0x504 - Privileged Peripheral Extended Frame 24 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse24mst_id_h: PPSE24MSTID_H,
    #[doc = "0x508 - Privileged Peripheral Extended Frame 25 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse25mst_id_l: PPSE25MSTID_L,
    #[doc = "0x50c - Privileged Peripheral Extended Frame 25 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse25mst_id_h: PPSE25MSTID_H,
    #[doc = "0x510 - Privileged Peripheral Extended Frame 26 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse26mst_id_l: PPSE26MSTID_L,
    #[doc = "0x514 - Privileged Peripheral Extended Frame 26 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse26mst_id_h: PPSE26MSTID_H,
    #[doc = "0x518 - Privileged Peripheral Extended Frame 27 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse27mst_id_l: PPSE27MSTID_L,
    #[doc = "0x51c - Privileged Peripheral Extended Frame 27 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse27mst_id_h: PPSE27MSTID_H,
    #[doc = "0x520 - Privileged Peripheral Extended Frame 28 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse28mst_id_l: PPSE28MSTID_L,
    #[doc = "0x524 - Privileged Peripheral Extended Frame 28 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse28mst_id_h: PPSE28MSTID_H,
    #[doc = "0x528 - Privileged Peripheral Extended Frame 29 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse29mst_id_l: PPSE29MSTID_L,
    #[doc = "0x52c - Privileged Peripheral Extended Frame 29 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse29mst_id_h: PPSE29MSTID_H,
    #[doc = "0x530 - Privileged Peripheral Extended Frame 30 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse30mst_id_l: PPSE30MSTID_L,
    #[doc = "0x534 - Privileged Peripheral Extended Frame 30 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse30mst_id_h: PPSE30MSTID_H,
    #[doc = "0x538 - Privileged Peripheral Extended Frame 31 (Quadrant 0 and 1) MasteriD Protection Register"]
    pub ppse31mst_id_l: PPSE31MSTID_L,
    #[doc = "0x53c - Privileged Peripheral Extended Frame 31 (Quadrant 2 and 3) MasteriD Protection Register"]
    pub ppse31mst_id_h: PPSE31MSTID_H,
    #[doc = "0x540 - Memory Frame 0 MasteriD Protection Register"]
    pub pcs0mst_id: PCS0MSTID,
    #[doc = "0x544 - Memory Frame 1 MasteriD Protection Register"]
    pub pcs2mst_id: PCS2MSTID,
    #[doc = "0x548 - Memory Frame 2 MasteriD Protection Register"]
    pub pcs4mst_id: PCS4MSTID,
    #[doc = "0x54c - Memory Frame 3 MasteriD Protection Register"]
    pub pcs6mst_id: PCS6MSTID,
    #[doc = "0x550 - Memory Frame 4 MasteriD Protection Register"]
    pub pcs8mst_id: PCS8MSTID,
    #[doc = "0x554 - Memory Frame 5 MasteriD Protection Register"]
    pub pcs10mst_id: PCS10MSTID,
    #[doc = "0x558 - Memory Frame 6 MasteriD Protection Register"]
    pub pcs12mst_id: PCS12MSTID,
    #[doc = "0x55c - Memory Frame 7 MasteriD Protection Register"]
    pub pcs14mst_id: PCS14MSTID,
    #[doc = "0x560 - Memory Frame 8 MasteriD Protection Register"]
    pub pcs16mst_id: PCS16MSTID,
    #[doc = "0x564 - Memory Frame 9 MasteriD Protection Register"]
    pub pcs18mst_id: PCS18MSTID,
    #[doc = "0x568 - Memory Frame 10 MasteriD Protection Register"]
    pub pcs20mst_id: PCS20MSTID,
    #[doc = "0x56c - Memory Frame 11 MasteriD Protection Register"]
    pub pcs22mst_id: PCS22MSTID,
    #[doc = "0x570 - Memory Frame 12 MasteriD Protection Register"]
    pub pcs24mst_id: PCS24MSTID,
    #[doc = "0x574 - Memory Frame 13 MasteriD Protection Register"]
    pub pcs26mst_id: PCS26MSTID,
    #[doc = "0x578 - Memory Frame 14 MasteriD Protection Register"]
    pub pcs28mst_id: PCS28MSTID,
    #[doc = "0x57c - Memory Frame 15 MasteriD Protection Register"]
    pub pcs30mst_id: PCS30MSTID,
    #[doc = "0x580 - Memory Frame 16 MasteriD Protection Register"]
    pub pcs32mst_id: PCS32MSTID,
    #[doc = "0x584 - Memory Frame 17 MasteriD Protection Register"]
    pub pcs34mst_id: PCS34MSTID,
    #[doc = "0x588 - Memory Frame 18 MasteriD Protection Register"]
    pub pcs36mst_id: PCS36MSTID,
    #[doc = "0x58c - Memory Frame 19 MasteriD Protection Register"]
    pub pcs38mst_id: PCS38MSTID,
    #[doc = "0x590 - Memory Frame 20 MasteriD Protection Register"]
    pub pcs40mst_id: PCS40MSTID,
    #[doc = "0x594 - Memory Frame 21 MasteriD Protection Register"]
    pub pcs42mst_id: PCS42MSTID,
    #[doc = "0x598 - Memory Frame 22 MasteriD Protection Register"]
    pub pcs44mst_id: PCS44MSTID,
    #[doc = "0x59c - Memory Frame 23 MasteriD Protection Register"]
    pub pcs46mst_id: PCS46MSTID,
    #[doc = "0x5a0 - Memory Frame 24 MasteriD Protection Register"]
    pub pcs48mst_id: PCS48MSTID,
    #[doc = "0x5a4 - Memory Frame 25 MasteriD Protection Register"]
    pub pcs50mst_id: PCS50MSTID,
    #[doc = "0x5a8 - Memory Frame 26 MasteriD Protection Register"]
    pub pcs52mst_id: PCS52MSTID,
    #[doc = "0x5ac - Memory Frame 27 MasteriD Protection Register"]
    pub pcs54mst_id: PCS54MSTID,
    #[doc = "0x5b0 - Memory Frame 28 MasteriD Protection Register"]
    pub pcs56mst_id: PCS56MSTID,
    #[doc = "0x5b4 - Memory Frame 29 MasteriD Protection Register"]
    pub pcs58mst_id: PCS58MSTID,
    #[doc = "0x5b8 - Memory Frame 30 MasteriD Protection Register"]
    pub pcs60mst_id: PCS60MSTID,
    #[doc = "0x5bc - Memory Frame 31 MasteriD Protection Register"]
    pub pcs62mst_id: PCS62MSTID,
    #[doc = "0x5c0 - Priviledged Memory Frame 0 MasteriD Protection Register"]
    pub ppcs0mst_id: PPCS0MSTID,
    #[doc = "0x5c4 - Priviledged Memory Frame 1 MasteriD Protection Register"]
    pub ppcs2mst_id: PPCS2MSTID,
    #[doc = "0x5c8 - Priviledged Memory Frame 2 MasteriD Protection Register"]
    pub ppcs4mst_id: PPCS4MSTID,
    #[doc = "0x5cc - Priviledged Memory Frame 3 MasteriD Protection Register"]
    pub ppcs6mst_id: PPCS6MSTID,
    #[doc = "0x5d0 - Priviledged Memory Frame 4 MasteriD Protection Register"]
    pub ppcs8mst_id: PPCS8MSTID,
    #[doc = "0x5d4 - Priviledged Memory Frame 5 MasteriD Protection Register"]
    pub ppcs10mst_id: PPCS10MSTID,
    #[doc = "0x5d8 - Priviledged Memory Frame 6 MasteriD Protection Register"]
    pub ppcs12mst_id: PPCS12MSTID,
    #[doc = "0x5dc - Priviledged Memory Frame 7 MasteriD Protection Register"]
    pub ppcs14mst_id: PPCS14MSTID,
    #[doc = "0x5e0 - Master-ID Protection Register for External PCR"]
    pub pcrextms_tid: PCREXTMSTID,
}
#[doc = "Set-only register to Protect PCS frames 0 to 31"]
pub struct PMPROTSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set-only register to Protect PCS frames 0 to 31"]
pub mod pmprot_set0;
#[doc = "Set-only register to Protect PCS frames 32 to 64"]
pub struct PMPROTSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set-only register to Protect PCS frames 32 to 64"]
pub mod pmprot_set1;
#[doc = "Clear-only register to Protect PCS frames 0 to 31"]
pub struct PMPROTCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear-only register to Protect PCS frames 0 to 31"]
pub mod pmprot_clr0;
#[doc = "Clear-only register to Protect PCS frames 32 to 64"]
pub struct PMPROTCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear-only register to Protect PCS frames 32 to 64"]
pub mod pmprot_clr1;
#[doc = "Set-only register to Protect the 32 quadrants of PS0 to PS7"]
pub struct PPROTSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set-only register to Protect the 32 quadrants of PS0 to PS7"]
pub mod pprot_set0;
#[doc = "Set-only register to Protect the 32 quadrants of PS8 to PS15"]
pub struct PPROTSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set-only register to Protect the 32 quadrants of PS8 to PS15"]
pub mod pprot_set1;
#[doc = "Set-only register to Protect the 32 quadrants of PS16 to PS23"]
pub struct PPROTSET2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set-only register to Protect the 32 quadrants of PS16 to PS23"]
pub mod pprot_set2;
#[doc = "Set-only register to Protect the 32 quadrants of PS24 to PS31"]
pub struct PPROTSET3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set-only register to Protect the 32 quadrants of PS24 to PS31"]
pub mod pprot_set3;
#[doc = "clear-only register to Protect the quadrants of PS0 to PS7"]
pub struct PPROTCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clear-only register to Protect the quadrants of PS0 to PS7"]
pub mod pprot_clr0;
#[doc = "clear-only register to Protect the quadrants of PS8 to PS15"]
pub struct PPROTCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clear-only register to Protect the quadrants of PS8 to PS15"]
pub mod pprot_clr1;
#[doc = "clear-only register to Protect the quadrants of PS16 to PS23"]
pub struct PPROTCLR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clear-only register to Protect the quadrants of PS16 to PS23"]
pub mod pprot_clr2;
#[doc = "clear-only register to Protect the quadrants of PS24 to PS31"]
pub struct PPROTCLR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clear-only register to Protect the quadrants of PS24 to PS31"]
pub mod pprot_clr3;
#[doc = "no description"]
pub struct PCSPWRDWNSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pcspwr_dwn_set0;
#[doc = "no description"]
pub struct PCSPWRDWNSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pcspwr_dwn_set1;
#[doc = "no description"]
pub struct PCSPWRDWNCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pcspwr_dwn_clr0;
#[doc = "no description"]
pub struct PCSPWRDWNCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pcspwr_dwn_clr1;
#[doc = "no description"]
pub struct PSPWRDWNSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pspwr_dwn_set0;
#[doc = "no description"]
pub struct PSPWRDWNSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pspwr_dwn_set1;
#[doc = "no description"]
pub struct PSPWRDWNSET2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pspwr_dwn_set2;
#[doc = "no description"]
pub struct PSPWRDWNSET3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pspwr_dwn_set3;
#[doc = "no description"]
pub struct PSPWRDWNCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pspwr_dwn_clr0;
#[doc = "no description"]
pub struct PSPWRDWNCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pspwr_dwn_clr1;
#[doc = "no description"]
pub struct PSPWRDWNCLR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pspwr_dwn_clr2;
#[doc = "no description"]
pub struct PSPWRDWNCLR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description"]
pub mod pspwr_dwn_clr3;
#[doc = "Debug Frame Power Down Set register"]
pub struct DBGPWRDWNSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Frame Power Down Set register"]
pub mod dbg_pwr_dwn_set;
#[doc = "MasterID Protection Write Enable Register"]
pub struct MSTIDWRENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MasterID Protection Write Enable Register"]
pub mod mst_id_wr_ena;
#[doc = "MasterID Check Enable Register"]
pub struct MSTIDENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MasterID Check Enable Register"]
pub mod mst_id_ena;
#[doc = "MasterID Diagnostic Control Register"]
pub struct MSTIDDIAGCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MasterID Diagnostic Control Register"]
pub mod mst_id_diag_ctrl;
#[doc = "Peripheral Frame 0 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS0MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 0 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps0mst_id_l;
#[doc = "Peripheral Frame 0 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS0MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 0 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps0mst_id_h;
#[doc = "Peripheral Frame 1 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS1MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 1 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps1mst_id_l;
#[doc = "Peripheral Frame 1 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS1MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 1 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps1mst_id_h;
#[doc = "Peripheral Frame 2 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS2MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 2 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps2mst_id_l;
#[doc = "Peripheral Frame 2 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS2MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 2 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps2mst_id_h;
#[doc = "Peripheral Frame 3 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS3MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 3 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps3mst_id_l;
#[doc = "Peripheral Frame 3 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS3MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 3 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps3mst_id_h;
#[doc = "Peripheral Frame 4 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS4MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 4 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps4mst_id_l;
#[doc = "Peripheral Frame 4 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS4MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 4 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps4mst_id_h;
#[doc = "Peripheral Frame 5 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS5MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 5 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps5mst_id_l;
#[doc = "Peripheral Frame 5 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS5MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 5 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps5mst_id_h;
#[doc = "Peripheral Frame 6 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS6MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 6 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps6mst_id_l;
#[doc = "Peripheral Frame 6 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS6MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 6 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps6mst_id_h;
#[doc = "Peripheral Frame 7 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS7MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 7 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps7mst_id_l;
#[doc = "Peripheral Frame 7 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS7MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 7 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps7mst_id_h;
#[doc = "Peripheral Frame 8 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS8MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 8 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps8mst_id_l;
#[doc = "Peripheral Frame 8 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS8MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 8 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps8mst_id_h;
#[doc = "Peripheral Frame 9 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS9MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 9 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps9mst_id_l;
#[doc = "Peripheral Frame 9 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS9MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 9 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps9mst_id_h;
#[doc = "Peripheral Frame 10 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS10MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 10 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps10mst_id_l;
#[doc = "Peripheral Frame 10(Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS10MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 10(Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps10mst_id_h;
#[doc = "Peripheral Frame 11 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS11MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 11 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps11mst_id_l;
#[doc = "Peripheral Frame 11 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS11MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 11 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps11mst_id_h;
#[doc = "Peripheral Frame 12 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS12MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 12 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps12mst_id_l;
#[doc = "Peripheral Frame 12 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS12MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 12 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps12mst_id_h;
#[doc = "Peripheral Frame 13 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS13MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 13 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps13mst_id_l;
#[doc = "Peripheral Frame 13 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS13MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 13 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps13mst_id_h;
#[doc = "Peripheral Frame 14 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS14MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 14 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps14mst_id_l;
#[doc = "Peripheral Frame 14 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS14MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 14 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps14mst_id_h;
#[doc = "Peripheral Frame 15 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS15MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 15 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps15mst_id_l;
#[doc = "Peripheral Frame 15 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS15MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 15 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps15mst_id_h;
#[doc = "Peripheral Frame 16 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS16MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 16 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps16mst_id_l;
#[doc = "Peripheral Frame 16 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS16MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 16 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps16mst_id_h;
#[doc = "Peripheral Frame 17 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS17MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 17 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps17mst_id_l;
#[doc = "Peripheral Frame 17 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS17MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 17 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps17mst_id_h;
#[doc = "Peripheral Frame 18 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS18MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 18 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps18mst_id_l;
#[doc = "Peripheral Frame 18 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS18MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 18 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps18mst_id_h;
#[doc = "Peripheral Frame 19 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS19MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 19 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps19mst_id_l;
#[doc = "Peripheral Frame 19 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS19MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 19 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps19mst_id_h;
#[doc = "Peripheral Frame 20 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS20MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 20 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps20mst_id_l;
#[doc = "Peripheral Frame 20 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS20MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 20 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps20mst_id_h;
#[doc = "Peripheral Frame 21 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS21MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 21 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps21mst_id_l;
#[doc = "Peripheral Frame 21 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS21MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 21 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps21mst_id_h;
#[doc = "Peripheral Frame 22 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS22MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 22 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps22mst_id_l;
#[doc = "Peripheral Frame 22 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS22MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 22 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps22mst_id_h;
#[doc = "Peripheral Frame 23 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS23MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 23 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps23mst_id_l;
#[doc = "Peripheral Frame 23 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS23MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 23 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps23mst_id_h;
#[doc = "Peripheral Frame 24 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS24MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 24 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps24mst_id_l;
#[doc = "Peripheral Frame 24 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS24MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 24 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps24mst_id_h;
#[doc = "Peripheral Frame 25 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS25MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 25 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps25mst_id_l;
#[doc = "Peripheral Frame 25 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS25MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 25 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps25mst_id_h;
#[doc = "Peripheral Frame 26 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS26MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 26 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps26mst_id_l;
#[doc = "Peripheral Frame 26 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS26MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 26 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps26mst_id_h;
#[doc = "Peripheral Frame 27 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS27MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 27 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps27mst_id_l;
#[doc = "Peripheral Frame 27 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS27MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 27 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps27mst_id_h;
#[doc = "Peripheral Frame 28 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS28MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 28 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps28mst_id_l;
#[doc = "Peripheral Frame 28 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS28MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 28 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps28mst_id_h;
#[doc = "Peripheral Frame 29 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS29MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 29 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps29mst_id_l;
#[doc = "Peripheral Frame 29 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS29MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 29 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps29mst_id_h;
#[doc = "Peripheral Frame 30 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS30MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 30 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps30mst_id_l;
#[doc = "Peripheral Frame 30 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS30MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 30 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps30mst_id_h;
#[doc = "Peripheral Frame 31 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PS31MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 31 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ps31mst_id_l;
#[doc = "Peripheral Frame 31 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PS31MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Frame 31 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ps31mst_id_h;
#[doc = "Privileged Peripheral Frame 0 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPS0MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 0 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod pps0mst_id_l;
#[doc = "Privileged Peripheral Frame 0 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPS0MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 0 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod pps0mst_id_h;
#[doc = "Privileged Peripheral Frame 1 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPS1MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 1 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod pps1mst_id_l;
#[doc = "Privileged Peripheral Frame 1 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPS1MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 1 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod pps1mst_id_h;
#[doc = "Privileged Peripheral Frame 2 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPS2MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 2 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod pps2mst_id_l;
#[doc = "Privileged Peripheral Frame 2 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPS2MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 2 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod pps2mst_id_h;
#[doc = "Privileged Peripheral Frame 3 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPS3MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 3 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod pps3mst_id_l;
#[doc = "Privileged Peripheral Frame 3 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPS3MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 3 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod pps3mst_id_h;
#[doc = "Privileged Peripheral Frame 4 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPS4MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 4 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod pps4mst_id_l;
#[doc = "Privileged Peripheral Frame 4 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPS4MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 4 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod pps4mst_id_h;
#[doc = "Privileged Peripheral Frame 5 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPS5MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 5 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod pps5mst_id_l;
#[doc = "Privileged Peripheral Frame 5 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPS5MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 5 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod pps5mst_id_h;
#[doc = "Privileged Peripheral Frame 6 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPS6MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 6 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod pps6mst_id_l;
#[doc = "Privileged Peripheral Frame 6 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPS6MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 6 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod pps6mst_id_h;
#[doc = "Privileged Peripheral Frame 7 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPS7MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 7 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod pps7mst_id_l;
#[doc = "Privileged Peripheral Frame 7 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPS7MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Frame 7 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod pps7mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 0 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE0MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 0 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse0mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 0 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE0MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 0 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse0mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 1 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE1MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 1 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse1mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 1 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE1MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 1 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse1mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 2 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE2MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 2 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse2mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 2 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE2MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 2 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse2mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 3 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE3MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 3 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse3mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 3 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE3MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 3 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse3mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 4 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE4MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 4 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse4mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 4 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE4MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 4 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse4mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 5 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE5MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 5 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse5mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 5 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE5MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 5 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse5mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 6 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE6MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 6 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse6mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 6 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE6MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 6 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse6mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 7 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE7MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 7 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse7mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 7 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE7MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 7 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse7mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 8 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE8MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 8 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse8mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 8 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE8MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 8 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse8mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 9 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE9MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 9 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse9mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 9 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE9MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 9 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse9mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 10 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE10MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 10 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse10mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 10 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE10MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 10 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse10mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 11 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE11MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 11 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse11mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 11 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE11MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 11 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse11mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 12 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE12MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 12 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse12mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 12 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE12MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 12 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse12mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 13 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE13MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 13 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse13mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 13 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE13MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 13 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse13mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 14 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE14MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 14 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse14mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 14 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE14MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 14 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse14mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 15 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE15MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 15 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse15mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 15 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE15MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 15 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse15mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 16 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE16MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 16 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse16mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 16 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE16MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 16 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse16mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 17 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE17MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 17 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse17mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 17 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE17MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 17 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse17mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 18 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE18MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 18 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse18mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 18 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE18MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 18 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse18mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 19 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE19MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 19 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse19mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 19 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE19MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 19 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse19mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 20 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE20MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 20 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse20mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 20 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE20MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 20 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse20mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 21 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE21MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 21 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse21mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 21 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE21MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 21 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse21mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 22 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE22MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 22 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse22mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 22 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE22MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 22 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse22mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 23 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE23MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 23 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse23mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 23 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE23MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 23 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse23mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 24 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE24MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 24 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse24mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 24 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE24MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 24 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse24mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 25 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE25MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 25 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse25mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 25 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE25MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 25 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse25mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 26 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE26MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 26 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse26mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 26 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE26MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 26 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse26mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 27 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE27MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 27 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse27mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 27 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE27MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 27 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse27mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 28 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE28MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 28 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse28mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 28 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE28MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 28 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse28mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 29 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE29MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 29 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse29mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 29 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE29MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 29 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse29mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 30 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE30MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 30 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse30mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 30 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE30MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 30 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse30mst_id_h;
#[doc = "Privileged Peripheral Extended Frame 31 (Quadrant 0 and 1) MasteriD Protection Register"]
pub struct PPSE31MSTID_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 31 (Quadrant 0 and 1) MasteriD Protection Register"]
pub mod ppse31mst_id_l;
#[doc = "Privileged Peripheral Extended Frame 31 (Quadrant 2 and 3) MasteriD Protection Register"]
pub struct PPSE31MSTID_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Privileged Peripheral Extended Frame 31 (Quadrant 2 and 3) MasteriD Protection Register"]
pub mod ppse31mst_id_h;
#[doc = "Memory Frame 0 MasteriD Protection Register"]
pub struct PCS0MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 0 MasteriD Protection Register"]
pub mod pcs0mst_id;
#[doc = "Memory Frame 1 MasteriD Protection Register"]
pub struct PCS2MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 1 MasteriD Protection Register"]
pub mod pcs2mst_id;
#[doc = "Memory Frame 2 MasteriD Protection Register"]
pub struct PCS4MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 2 MasteriD Protection Register"]
pub mod pcs4mst_id;
#[doc = "Memory Frame 3 MasteriD Protection Register"]
pub struct PCS6MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 3 MasteriD Protection Register"]
pub mod pcs6mst_id;
#[doc = "Memory Frame 4 MasteriD Protection Register"]
pub struct PCS8MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 4 MasteriD Protection Register"]
pub mod pcs8mst_id;
#[doc = "Memory Frame 5 MasteriD Protection Register"]
pub struct PCS10MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 5 MasteriD Protection Register"]
pub mod pcs10mst_id;
#[doc = "Memory Frame 6 MasteriD Protection Register"]
pub struct PCS12MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 6 MasteriD Protection Register"]
pub mod pcs12mst_id;
#[doc = "Memory Frame 7 MasteriD Protection Register"]
pub struct PCS14MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 7 MasteriD Protection Register"]
pub mod pcs14mst_id;
#[doc = "Memory Frame 8 MasteriD Protection Register"]
pub struct PCS16MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 8 MasteriD Protection Register"]
pub mod pcs16mst_id;
#[doc = "Memory Frame 9 MasteriD Protection Register"]
pub struct PCS18MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 9 MasteriD Protection Register"]
pub mod pcs18mst_id;
#[doc = "Memory Frame 10 MasteriD Protection Register"]
pub struct PCS20MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 10 MasteriD Protection Register"]
pub mod pcs20mst_id;
#[doc = "Memory Frame 11 MasteriD Protection Register"]
pub struct PCS22MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 11 MasteriD Protection Register"]
pub mod pcs22mst_id;
#[doc = "Memory Frame 12 MasteriD Protection Register"]
pub struct PCS24MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 12 MasteriD Protection Register"]
pub mod pcs24mst_id;
#[doc = "Memory Frame 13 MasteriD Protection Register"]
pub struct PCS26MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 13 MasteriD Protection Register"]
pub mod pcs26mst_id;
#[doc = "Memory Frame 14 MasteriD Protection Register"]
pub struct PCS28MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 14 MasteriD Protection Register"]
pub mod pcs28mst_id;
#[doc = "Memory Frame 15 MasteriD Protection Register"]
pub struct PCS30MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 15 MasteriD Protection Register"]
pub mod pcs30mst_id;
#[doc = "Memory Frame 16 MasteriD Protection Register"]
pub struct PCS32MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 16 MasteriD Protection Register"]
pub mod pcs32mst_id;
#[doc = "Memory Frame 17 MasteriD Protection Register"]
pub struct PCS34MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 17 MasteriD Protection Register"]
pub mod pcs34mst_id;
#[doc = "Memory Frame 18 MasteriD Protection Register"]
pub struct PCS36MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 18 MasteriD Protection Register"]
pub mod pcs36mst_id;
#[doc = "Memory Frame 19 MasteriD Protection Register"]
pub struct PCS38MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 19 MasteriD Protection Register"]
pub mod pcs38mst_id;
#[doc = "Memory Frame 20 MasteriD Protection Register"]
pub struct PCS40MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 20 MasteriD Protection Register"]
pub mod pcs40mst_id;
#[doc = "Memory Frame 21 MasteriD Protection Register"]
pub struct PCS42MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 21 MasteriD Protection Register"]
pub mod pcs42mst_id;
#[doc = "Memory Frame 22 MasteriD Protection Register"]
pub struct PCS44MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 22 MasteriD Protection Register"]
pub mod pcs44mst_id;
#[doc = "Memory Frame 23 MasteriD Protection Register"]
pub struct PCS46MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 23 MasteriD Protection Register"]
pub mod pcs46mst_id;
#[doc = "Memory Frame 24 MasteriD Protection Register"]
pub struct PCS48MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 24 MasteriD Protection Register"]
pub mod pcs48mst_id;
#[doc = "Memory Frame 25 MasteriD Protection Register"]
pub struct PCS50MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 25 MasteriD Protection Register"]
pub mod pcs50mst_id;
#[doc = "Memory Frame 26 MasteriD Protection Register"]
pub struct PCS52MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 26 MasteriD Protection Register"]
pub mod pcs52mst_id;
#[doc = "Memory Frame 27 MasteriD Protection Register"]
pub struct PCS54MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 27 MasteriD Protection Register"]
pub mod pcs54mst_id;
#[doc = "Memory Frame 28 MasteriD Protection Register"]
pub struct PCS56MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 28 MasteriD Protection Register"]
pub mod pcs56mst_id;
#[doc = "Memory Frame 29 MasteriD Protection Register"]
pub struct PCS58MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 29 MasteriD Protection Register"]
pub mod pcs58mst_id;
#[doc = "Memory Frame 30 MasteriD Protection Register"]
pub struct PCS60MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 30 MasteriD Protection Register"]
pub mod pcs60mst_id;
#[doc = "Memory Frame 31 MasteriD Protection Register"]
pub struct PCS62MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Frame 31 MasteriD Protection Register"]
pub mod pcs62mst_id;
#[doc = "Priviledged Memory Frame 0 MasteriD Protection Register"]
pub struct PPCS0MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priviledged Memory Frame 0 MasteriD Protection Register"]
pub mod ppcs0mst_id;
#[doc = "Priviledged Memory Frame 1 MasteriD Protection Register"]
pub struct PPCS2MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priviledged Memory Frame 1 MasteriD Protection Register"]
pub mod ppcs2mst_id;
#[doc = "Priviledged Memory Frame 2 MasteriD Protection Register"]
pub struct PPCS4MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priviledged Memory Frame 2 MasteriD Protection Register"]
pub mod ppcs4mst_id;
#[doc = "Priviledged Memory Frame 3 MasteriD Protection Register"]
pub struct PPCS6MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priviledged Memory Frame 3 MasteriD Protection Register"]
pub mod ppcs6mst_id;
#[doc = "Priviledged Memory Frame 4 MasteriD Protection Register"]
pub struct PPCS8MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priviledged Memory Frame 4 MasteriD Protection Register"]
pub mod ppcs8mst_id;
#[doc = "Priviledged Memory Frame 5 MasteriD Protection Register"]
pub struct PPCS10MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priviledged Memory Frame 5 MasteriD Protection Register"]
pub mod ppcs10mst_id;
#[doc = "Priviledged Memory Frame 6 MasteriD Protection Register"]
pub struct PPCS12MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priviledged Memory Frame 6 MasteriD Protection Register"]
pub mod ppcs12mst_id;
#[doc = "Priviledged Memory Frame 7 MasteriD Protection Register"]
pub struct PPCS14MSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priviledged Memory Frame 7 MasteriD Protection Register"]
pub mod ppcs14mst_id;
#[doc = "Master-ID Protection Register for External PCR"]
pub struct PCREXTMSTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master-ID Protection Register for External PCR"]
pub mod pcrextms_tid;
