///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    ///0x04 - control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    ///0x08 - TAMP control register 3
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    ///0x0c - TAMP filter control register
    pub fltcr: crate::Reg<fltcr::FLTCR_SPEC>,
    _reserved4: [u8; 0x1c],
    ///0x2c - TAMP interrupt enable register
    pub ier: crate::Reg<ier::IER_SPEC>,
    ///0x30 - TAMP status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x34 - TAMP masked interrupt status register
    pub misr: crate::Reg<misr::MISR_SPEC>,
    _reserved7: [u8; 0x04],
    ///0x3c - TAMP status clear register
    pub scr: crate::Reg<scr::SCR_SPEC>,
    ///0x40 - monotonic counter register
    pub countr: crate::Reg<countr::COUNTR_SPEC>,
    _reserved9: [u8; 0xbc],
    ///0x100 - TAMP backup register
    pub bkp0r: crate::Reg<bkp0r::BKP0R_SPEC>,
    ///0x104 - TAMP backup register
    pub bkp1r: crate::Reg<bkp1r::BKP1R_SPEC>,
    ///0x108 - TAMP backup register
    pub bkp2r: crate::Reg<bkp2r::BKP2R_SPEC>,
    ///0x10c - TAMP backup register
    pub bkp3r: crate::Reg<bkp3r::BKP3R_SPEC>,
    ///0x110 - TAMP backup register
    pub bkp4r: crate::Reg<bkp4r::BKP4R_SPEC>,
    ///0x114 - TAMP backup register
    pub bkp5r: crate::Reg<bkp5r::BKP5R_SPEC>,
    ///0x118 - TAMP backup register
    pub bkp6r: crate::Reg<bkp6r::BKP6R_SPEC>,
    ///0x11c - TAMP backup register
    pub bkp7r: crate::Reg<bkp7r::BKP7R_SPEC>,
    ///0x120 - TAMP backup register
    pub bkp8r: crate::Reg<bkp8r::BKP8R_SPEC>,
    ///0x124 - TAMP backup register
    pub bkp9r: crate::Reg<bkp9r::BKP9R_SPEC>,
    _reserved19: [u8; 0x18],
    ///0x140 - TAMP backup register
    pub bkp10r: crate::Reg<bkp10r::BKP10R_SPEC>,
    ///0x144 - TAMP backup register
    pub bkp11r: crate::Reg<bkp11r::BKP11R_SPEC>,
    ///0x148 - TAMP backup register
    pub bkp12r: crate::Reg<bkp12r::BKP12R_SPEC>,
    ///0x14c - TAMP backup register
    pub bkp13r: crate::Reg<bkp13r::BKP13R_SPEC>,
    ///0x150 - TAMP backup register
    pub bkp14r: crate::Reg<bkp14r::BKP14R_SPEC>,
    ///0x154 - TAMP backup register
    pub bkp15r: crate::Reg<bkp15r::BKP15R_SPEC>,
    ///0x158 - TAMP backup register
    pub bkp16r: crate::Reg<bkp16r::BKP16R_SPEC>,
    ///0x15c - TAMP backup register
    pub bkp17r: crate::Reg<bkp17r::BKP17R_SPEC>,
    ///0x160 - TAMP backup register
    pub bkp18r: crate::Reg<bkp18r::BKP18R_SPEC>,
    ///0x164 - TAMP backup register
    pub bkp19r: crate::Reg<bkp19r::BKP19R_SPEC>,
}
///CR1 register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///CR3 register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///TAMP control register 3
pub mod cr3;
///FLTCR register accessor: an alias for `Reg<FLTCR_SPEC>`
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
///TAMP filter control register
pub mod fltcr;
///IER register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///TAMP interrupt enable register
pub mod ier;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///TAMP status register
pub mod sr;
///MISR register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///TAMP masked interrupt status register
pub mod misr;
///SCR register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///TAMP status clear register
pub mod scr;
///COUNTR register accessor: an alias for `Reg<COUNTR_SPEC>`
pub type COUNTR = crate::Reg<countr::COUNTR_SPEC>;
///monotonic counter register
pub mod countr;
///BKP0R register accessor: an alias for `Reg<BKP0R_SPEC>`
pub type BKP0R = crate::Reg<bkp0r::BKP0R_SPEC>;
///TAMP backup register
pub mod bkp0r;
///BKP1R register accessor: an alias for `Reg<BKP1R_SPEC>`
pub type BKP1R = crate::Reg<bkp1r::BKP1R_SPEC>;
///TAMP backup register
pub mod bkp1r;
///BKP2R register accessor: an alias for `Reg<BKP2R_SPEC>`
pub type BKP2R = crate::Reg<bkp2r::BKP2R_SPEC>;
///TAMP backup register
pub mod bkp2r;
///BKP3R register accessor: an alias for `Reg<BKP3R_SPEC>`
pub type BKP3R = crate::Reg<bkp3r::BKP3R_SPEC>;
///TAMP backup register
pub mod bkp3r;
///BKP4R register accessor: an alias for `Reg<BKP4R_SPEC>`
pub type BKP4R = crate::Reg<bkp4r::BKP4R_SPEC>;
///TAMP backup register
pub mod bkp4r;
///BKP5R register accessor: an alias for `Reg<BKP5R_SPEC>`
pub type BKP5R = crate::Reg<bkp5r::BKP5R_SPEC>;
///TAMP backup register
pub mod bkp5r;
///BKP6R register accessor: an alias for `Reg<BKP6R_SPEC>`
pub type BKP6R = crate::Reg<bkp6r::BKP6R_SPEC>;
///TAMP backup register
pub mod bkp6r;
///BKP7R register accessor: an alias for `Reg<BKP7R_SPEC>`
pub type BKP7R = crate::Reg<bkp7r::BKP7R_SPEC>;
///TAMP backup register
pub mod bkp7r;
///BKP8R register accessor: an alias for `Reg<BKP8R_SPEC>`
pub type BKP8R = crate::Reg<bkp8r::BKP8R_SPEC>;
///TAMP backup register
pub mod bkp8r;
///BKP9R register accessor: an alias for `Reg<BKP9R_SPEC>`
pub type BKP9R = crate::Reg<bkp9r::BKP9R_SPEC>;
///TAMP backup register
pub mod bkp9r;
///BKP10R register accessor: an alias for `Reg<BKP10R_SPEC>`
pub type BKP10R = crate::Reg<bkp10r::BKP10R_SPEC>;
///TAMP backup register
pub mod bkp10r;
///BKP11R register accessor: an alias for `Reg<BKP11R_SPEC>`
pub type BKP11R = crate::Reg<bkp11r::BKP11R_SPEC>;
///TAMP backup register
pub mod bkp11r;
///BKP12R register accessor: an alias for `Reg<BKP12R_SPEC>`
pub type BKP12R = crate::Reg<bkp12r::BKP12R_SPEC>;
///TAMP backup register
pub mod bkp12r;
///BKP13R register accessor: an alias for `Reg<BKP13R_SPEC>`
pub type BKP13R = crate::Reg<bkp13r::BKP13R_SPEC>;
///TAMP backup register
pub mod bkp13r;
///BKP14R register accessor: an alias for `Reg<BKP14R_SPEC>`
pub type BKP14R = crate::Reg<bkp14r::BKP14R_SPEC>;
///TAMP backup register
pub mod bkp14r;
///BKP15R register accessor: an alias for `Reg<BKP15R_SPEC>`
pub type BKP15R = crate::Reg<bkp15r::BKP15R_SPEC>;
///TAMP backup register
pub mod bkp15r;
///BKP16R register accessor: an alias for `Reg<BKP16R_SPEC>`
pub type BKP16R = crate::Reg<bkp16r::BKP16R_SPEC>;
///TAMP backup register
pub mod bkp16r;
///BKP17R register accessor: an alias for `Reg<BKP17R_SPEC>`
pub type BKP17R = crate::Reg<bkp17r::BKP17R_SPEC>;
///TAMP backup register
pub mod bkp17r;
///BKP18R register accessor: an alias for `Reg<BKP18R_SPEC>`
pub type BKP18R = crate::Reg<bkp18r::BKP18R_SPEC>;
///TAMP backup register
pub mod bkp18r;
///BKP19R register accessor: an alias for `Reg<BKP19R_SPEC>`
pub type BKP19R = crate::Reg<bkp19r::BKP19R_SPEC>;
///TAMP backup register
pub mod bkp19r;
