use core::convert::TryFrom;
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Interrupt 1"]
    INT1,
    #[doc = "1 - Interrupt 2"]
    INT2,
    #[doc = "2 - Interrupt 3"]
    INT3,
    #[doc = "3 - Interrupt 4"]
    INT4,
    #[doc = "4 - Interrupt 5"]
    INT5,
    #[doc = "5 - Interrupt 6"]
    INT6,
    #[doc = "6 - Interrupt 7"]
    INT7,
    #[doc = "7 - Interrupt 8"]
    INT8,
    #[doc = "8 - Interrupt 9"]
    INT9,
    #[doc = "9 - Interrupt 10"]
    INT10,
    #[doc = "10 - Interrupt 11"]
    INT11,
    #[doc = "11 - Interrupt 12"]
    INT12,
    #[doc = "12 - Interrupt 13"]
    INT13,
    #[doc = "13 - Interrupt 14"]
    INT14,
    #[doc = "14 - Interrupt 15"]
    INT15,
    #[doc = "15 - Interrupt 16"]
    INT16,
    #[doc = "16 - Interrupt 17"]
    INT17,
    #[doc = "17 - Interrupt 18"]
    INT18,
    #[doc = "18 - Interrupt 19"]
    INT19,
    #[doc = "19 - Interrupt 20"]
    INT20,
    #[doc = "20 - Interrupt 21"]
    INT21,
    #[doc = "21 - Interrupt 22"]
    INT22,
    #[doc = "22 - Interrupt 23"]
    INT23,
    #[doc = "23 - Interrupt 24"]
    INT24,
    #[doc = "24 - Interrupt 25"]
    INT25,
    #[doc = "25 - Interrupt 26"]
    INT26,
    #[doc = "26 - Interrupt 27"]
    INT27,
    #[doc = "27 - Interrupt 28"]
    INT28,
    #[doc = "28 - Interrupt 29"]
    INT29,
    #[doc = "29 - Interrupt 30"]
    INT30,
    #[doc = "30 - Interrupt 31"]
    INT31,
    #[doc = "31 - Interrupt 32"]
    INT32,
    #[doc = "32 - Interrupt 33"]
    INT33,
    #[doc = "33 - Interrupt 34"]
    INT34,
    #[doc = "34 - Interrupt 35"]
    INT35,
    #[doc = "35 - Interrupt 36"]
    INT36,
    #[doc = "36 - Interrupt 37"]
    INT37,
    #[doc = "37 - Interrupt 38"]
    INT38,
    #[doc = "38 - Interrupt 39"]
    INT39,
    #[doc = "39 - Interrupt 40"]
    INT40,
    #[doc = "40 - Interrupt 41"]
    INT41,
    #[doc = "41 - Interrupt 42"]
    INT42,
    #[doc = "42 - Interrupt 43"]
    INT43,
    #[doc = "43 - Interrupt 44"]
    INT44,
    #[doc = "44 - Interrupt 45"]
    INT45,
    #[doc = "45 - Interrupt 46"]
    INT46,
    #[doc = "46 - Interrupt 47"]
    INT47,
    #[doc = "47 - Interrupt 48"]
    INT48,
    #[doc = "48 - Interrupt 49"]
    INT49,
    #[doc = "49 - Interrupt 50"]
    INT50,
    #[doc = "50 - Interrupt 51"]
    INT51,
    #[doc = "51 - Interrupt 52"]
    INT52,
    #[doc = "52 - Interrupt 53"]
    INT53,
    #[doc = "53 - Interrupt 54"]
    INT54,
    #[doc = "54 - Interrupt 55"]
    INT55,
    #[doc = "55 - Interrupt 56"]
    INT56,
    #[doc = "56 - Interrupt 57"]
    INT57,
    #[doc = "57 - Interrupt 58"]
    INT58,
    #[doc = "58 - Interrupt 59"]
    INT59,
    #[doc = "59 - Interrupt 60"]
    INT60,
    #[doc = "60 - Interrupt 61"]
    INT61,
    #[doc = "61 - Interrupt 62"]
    INT62,
    #[doc = "62 - Interrupt 63"]
    INT63,
    #[doc = "63 - Interrupt 64"]
    INT64,
    #[doc = "64 - Interrupt 65"]
    INT65,
    #[doc = "65 - Interrupt 66"]
    INT66,
    #[doc = "66 - Interrupt 67"]
    INT67,
    #[doc = "67 - Interrupt 68"]
    INT68,
    #[doc = "68 - Interrupt 69"]
    INT69,
    #[doc = "69 - Interrupt 70"]
    INT70,
    #[doc = "70 - Interrupt 71"]
    INT71,
    #[doc = "71 - Interrupt 72"]
    INT72,
    #[doc = "72 - Interrupt 73"]
    INT73,
    #[doc = "73 - Interrupt 74"]
    INT74,
    #[doc = "74 - Interrupt 75"]
    INT75,
    #[doc = "75 - Interrupt 76"]
    INT76,
    #[doc = "76 - Interrupt 77"]
    INT77,
    #[doc = "77 - Interrupt 78"]
    INT78,
    #[doc = "78 - Interrupt 79"]
    INT79,
    #[doc = "79 - Interrupt 80"]
    INT80,
    #[doc = "80 - Interrupt 81"]
    INT81,
    #[doc = "81 - Interrupt 82"]
    INT82,
    #[doc = "82 - Interrupt 83"]
    INT83,
    #[doc = "83 - Interrupt 84"]
    INT84,
    #[doc = "84 - Interrupt 85"]
    INT85,
    #[doc = "85 - Interrupt 86"]
    INT86,
    #[doc = "86 - Interrupt 87"]
    INT87,
    #[doc = "87 - Interrupt 88"]
    INT88,
    #[doc = "88 - Interrupt 89"]
    INT89,
    #[doc = "89 - Interrupt 90"]
    INT90,
    #[doc = "90 - Interrupt 91"]
    INT91,
    #[doc = "91 - Interrupt 92"]
    INT92,
    #[doc = "92 - Interrupt 93"]
    INT93,
    #[doc = "93 - Interrupt 94"]
    INT94,
    #[doc = "94 - Interrupt 95"]
    INT95,
    #[doc = "95 - Interrupt 96"]
    INT96,
    #[doc = "96 - Interrupt 97"]
    INT97,
    #[doc = "97 - Interrupt 98"]
    INT98,
    #[doc = "98 - Interrupt 99"]
    INT99,
    #[doc = "99 - Interrupt 100"]
    INT100,
    #[doc = "100 - Interrupt 101"]
    INT101,
    #[doc = "101 - Interrupt 102"]
    INT102,
    #[doc = "102 - Interrupt 103"]
    INT103,
    #[doc = "103 - Interrupt 104"]
    INT104,
    #[doc = "104 - Interrupt 105"]
    INT105,
    #[doc = "105 - Interrupt 106"]
    INT106,
    #[doc = "106 - Interrupt 107"]
    INT107,
    #[doc = "107 - Interrupt 108"]
    INT108,
    #[doc = "108 - Interrupt 109"]
    INT109,
    #[doc = "109 - Interrupt 110"]
    INT110,
    #[doc = "110 - Interrupt 111"]
    INT111,
    #[doc = "111 - Interrupt 112"]
    INT112,
    #[doc = "112 - Interrupt 113"]
    INT113,
    #[doc = "113 - Interrupt 114"]
    INT114,
    #[doc = "114 - Interrupt 115"]
    INT115,
    #[doc = "115 - Interrupt 116"]
    INT116,
    #[doc = "116 - Interrupt 117"]
    INT117,
    #[doc = "117 - Interrupt 118"]
    INT118,
    #[doc = "118 - Interrupt 119"]
    INT119,
    #[doc = "119 - Interrupt 120"]
    INT120,
    #[doc = "120 - Interrupt 121"]
    INT121,
    #[doc = "121 - Interrupt 122"]
    INT122,
    #[doc = "122 - Interrupt 123"]
    INT123,
    #[doc = "123 - Interrupt 124"]
    INT124,
    #[doc = "124 - Interrupt 125"]
    INT125,
    #[doc = "125 - Interrupt 126"]
    INT126,
    #[doc = "126 - Interrupt 127"]
    INT127,
    #[doc = "127 - Interrupt 128"]
    INT128,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::INT1 => 0,
            Interrupt::INT2 => 1,
            Interrupt::INT3 => 2,
            Interrupt::INT4 => 3,
            Interrupt::INT5 => 4,
            Interrupt::INT6 => 5,
            Interrupt::INT7 => 6,
            Interrupt::INT8 => 7,
            Interrupt::INT9 => 8,
            Interrupt::INT10 => 9,
            Interrupt::INT11 => 10,
            Interrupt::INT12 => 11,
            Interrupt::INT13 => 12,
            Interrupt::INT14 => 13,
            Interrupt::INT15 => 14,
            Interrupt::INT16 => 15,
            Interrupt::INT17 => 16,
            Interrupt::INT18 => 17,
            Interrupt::INT19 => 18,
            Interrupt::INT20 => 19,
            Interrupt::INT21 => 20,
            Interrupt::INT22 => 21,
            Interrupt::INT23 => 22,
            Interrupt::INT24 => 23,
            Interrupt::INT25 => 24,
            Interrupt::INT26 => 25,
            Interrupt::INT27 => 26,
            Interrupt::INT28 => 27,
            Interrupt::INT29 => 28,
            Interrupt::INT30 => 29,
            Interrupt::INT31 => 30,
            Interrupt::INT32 => 31,
            Interrupt::INT33 => 32,
            Interrupt::INT34 => 33,
            Interrupt::INT35 => 34,
            Interrupt::INT36 => 35,
            Interrupt::INT37 => 36,
            Interrupt::INT38 => 37,
            Interrupt::INT39 => 38,
            Interrupt::INT40 => 39,
            Interrupt::INT41 => 40,
            Interrupt::INT42 => 41,
            Interrupt::INT43 => 42,
            Interrupt::INT44 => 43,
            Interrupt::INT45 => 44,
            Interrupt::INT46 => 45,
            Interrupt::INT47 => 46,
            Interrupt::INT48 => 47,
            Interrupt::INT49 => 48,
            Interrupt::INT50 => 49,
            Interrupt::INT51 => 50,
            Interrupt::INT52 => 51,
            Interrupt::INT53 => 52,
            Interrupt::INT54 => 53,
            Interrupt::INT55 => 54,
            Interrupt::INT56 => 55,
            Interrupt::INT57 => 56,
            Interrupt::INT58 => 57,
            Interrupt::INT59 => 58,
            Interrupt::INT60 => 59,
            Interrupt::INT61 => 60,
            Interrupt::INT62 => 61,
            Interrupt::INT63 => 62,
            Interrupt::INT64 => 63,
            Interrupt::INT65 => 64,
            Interrupt::INT66 => 65,
            Interrupt::INT67 => 66,
            Interrupt::INT68 => 67,
            Interrupt::INT69 => 68,
            Interrupt::INT70 => 69,
            Interrupt::INT71 => 70,
            Interrupt::INT72 => 71,
            Interrupt::INT73 => 72,
            Interrupt::INT74 => 73,
            Interrupt::INT75 => 74,
            Interrupt::INT76 => 75,
            Interrupt::INT77 => 76,
            Interrupt::INT78 => 77,
            Interrupt::INT79 => 78,
            Interrupt::INT80 => 79,
            Interrupt::INT81 => 80,
            Interrupt::INT82 => 81,
            Interrupt::INT83 => 82,
            Interrupt::INT84 => 83,
            Interrupt::INT85 => 84,
            Interrupt::INT86 => 85,
            Interrupt::INT87 => 86,
            Interrupt::INT88 => 87,
            Interrupt::INT89 => 88,
            Interrupt::INT90 => 89,
            Interrupt::INT91 => 90,
            Interrupt::INT92 => 91,
            Interrupt::INT93 => 92,
            Interrupt::INT94 => 93,
            Interrupt::INT95 => 94,
            Interrupt::INT96 => 95,
            Interrupt::INT97 => 96,
            Interrupt::INT98 => 97,
            Interrupt::INT99 => 98,
            Interrupt::INT100 => 99,
            Interrupt::INT101 => 100,
            Interrupt::INT102 => 101,
            Interrupt::INT103 => 102,
            Interrupt::INT104 => 103,
            Interrupt::INT105 => 104,
            Interrupt::INT106 => 105,
            Interrupt::INT107 => 106,
            Interrupt::INT108 => 107,
            Interrupt::INT109 => 108,
            Interrupt::INT110 => 109,
            Interrupt::INT111 => 110,
            Interrupt::INT112 => 111,
            Interrupt::INT113 => 112,
            Interrupt::INT114 => 113,
            Interrupt::INT115 => 114,
            Interrupt::INT116 => 115,
            Interrupt::INT117 => 116,
            Interrupt::INT118 => 117,
            Interrupt::INT119 => 118,
            Interrupt::INT120 => 119,
            Interrupt::INT121 => 120,
            Interrupt::INT122 => 121,
            Interrupt::INT123 => 122,
            Interrupt::INT124 => 123,
            Interrupt::INT125 => 124,
            Interrupt::INT126 => 125,
            Interrupt::INT127 => 126,
            Interrupt::INT128 => 127,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl TryFrom<u8> for Interrupt {
    type Error = TryFromInterruptError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Interrupt::INT1),
            1 => Ok(Interrupt::INT2),
            2 => Ok(Interrupt::INT3),
            3 => Ok(Interrupt::INT4),
            4 => Ok(Interrupt::INT5),
            5 => Ok(Interrupt::INT6),
            6 => Ok(Interrupt::INT7),
            7 => Ok(Interrupt::INT8),
            8 => Ok(Interrupt::INT9),
            9 => Ok(Interrupt::INT10),
            10 => Ok(Interrupt::INT11),
            11 => Ok(Interrupt::INT12),
            12 => Ok(Interrupt::INT13),
            13 => Ok(Interrupt::INT14),
            14 => Ok(Interrupt::INT15),
            15 => Ok(Interrupt::INT16),
            16 => Ok(Interrupt::INT17),
            17 => Ok(Interrupt::INT18),
            18 => Ok(Interrupt::INT19),
            19 => Ok(Interrupt::INT20),
            20 => Ok(Interrupt::INT21),
            21 => Ok(Interrupt::INT22),
            22 => Ok(Interrupt::INT23),
            23 => Ok(Interrupt::INT24),
            24 => Ok(Interrupt::INT25),
            25 => Ok(Interrupt::INT26),
            26 => Ok(Interrupt::INT27),
            27 => Ok(Interrupt::INT28),
            28 => Ok(Interrupt::INT29),
            29 => Ok(Interrupt::INT30),
            30 => Ok(Interrupt::INT31),
            31 => Ok(Interrupt::INT32),
            32 => Ok(Interrupt::INT33),
            33 => Ok(Interrupt::INT34),
            34 => Ok(Interrupt::INT35),
            35 => Ok(Interrupt::INT36),
            36 => Ok(Interrupt::INT37),
            37 => Ok(Interrupt::INT38),
            38 => Ok(Interrupt::INT39),
            39 => Ok(Interrupt::INT40),
            40 => Ok(Interrupt::INT41),
            41 => Ok(Interrupt::INT42),
            42 => Ok(Interrupt::INT43),
            43 => Ok(Interrupt::INT44),
            44 => Ok(Interrupt::INT45),
            45 => Ok(Interrupt::INT46),
            46 => Ok(Interrupt::INT47),
            47 => Ok(Interrupt::INT48),
            48 => Ok(Interrupt::INT49),
            49 => Ok(Interrupt::INT50),
            50 => Ok(Interrupt::INT51),
            51 => Ok(Interrupt::INT52),
            52 => Ok(Interrupt::INT53),
            53 => Ok(Interrupt::INT54),
            54 => Ok(Interrupt::INT55),
            55 => Ok(Interrupt::INT56),
            56 => Ok(Interrupt::INT57),
            57 => Ok(Interrupt::INT58),
            58 => Ok(Interrupt::INT59),
            59 => Ok(Interrupt::INT60),
            60 => Ok(Interrupt::INT61),
            61 => Ok(Interrupt::INT62),
            62 => Ok(Interrupt::INT63),
            63 => Ok(Interrupt::INT64),
            64 => Ok(Interrupt::INT65),
            65 => Ok(Interrupt::INT66),
            66 => Ok(Interrupt::INT67),
            67 => Ok(Interrupt::INT68),
            68 => Ok(Interrupt::INT69),
            69 => Ok(Interrupt::INT70),
            70 => Ok(Interrupt::INT71),
            71 => Ok(Interrupt::INT72),
            72 => Ok(Interrupt::INT73),
            73 => Ok(Interrupt::INT74),
            74 => Ok(Interrupt::INT75),
            75 => Ok(Interrupt::INT76),
            76 => Ok(Interrupt::INT77),
            77 => Ok(Interrupt::INT78),
            78 => Ok(Interrupt::INT79),
            79 => Ok(Interrupt::INT80),
            80 => Ok(Interrupt::INT81),
            81 => Ok(Interrupt::INT82),
            82 => Ok(Interrupt::INT83),
            83 => Ok(Interrupt::INT84),
            84 => Ok(Interrupt::INT85),
            85 => Ok(Interrupt::INT86),
            86 => Ok(Interrupt::INT87),
            87 => Ok(Interrupt::INT88),
            88 => Ok(Interrupt::INT89),
            89 => Ok(Interrupt::INT90),
            90 => Ok(Interrupt::INT91),
            91 => Ok(Interrupt::INT92),
            92 => Ok(Interrupt::INT93),
            93 => Ok(Interrupt::INT94),
            94 => Ok(Interrupt::INT95),
            95 => Ok(Interrupt::INT96),
            96 => Ok(Interrupt::INT97),
            97 => Ok(Interrupt::INT98),
            98 => Ok(Interrupt::INT99),
            99 => Ok(Interrupt::INT100),
            100 => Ok(Interrupt::INT101),
            101 => Ok(Interrupt::INT102),
            102 => Ok(Interrupt::INT103),
            103 => Ok(Interrupt::INT104),
            104 => Ok(Interrupt::INT105),
            105 => Ok(Interrupt::INT106),
            106 => Ok(Interrupt::INT107),
            107 => Ok(Interrupt::INT108),
            108 => Ok(Interrupt::INT109),
            109 => Ok(Interrupt::INT110),
            110 => Ok(Interrupt::INT111),
            111 => Ok(Interrupt::INT112),
            112 => Ok(Interrupt::INT113),
            113 => Ok(Interrupt::INT114),
            114 => Ok(Interrupt::INT115),
            115 => Ok(Interrupt::INT116),
            116 => Ok(Interrupt::INT117),
            117 => Ok(Interrupt::INT118),
            118 => Ok(Interrupt::INT119),
            119 => Ok(Interrupt::INT120),
            120 => Ok(Interrupt::INT121),
            121 => Ok(Interrupt::INT122),
            122 => Ok(Interrupt::INT123),
            123 => Ok(Interrupt::INT124),
            124 => Ok(Interrupt::INT125),
            125 => Ok(Interrupt::INT126),
            126 => Ok(Interrupt::INT127),
            127 => Ok(Interrupt::INT128),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
