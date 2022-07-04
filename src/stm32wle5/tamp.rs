#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x08 - TAMP control register 3"]
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    #[doc = "0x0c - TAMP filter control register"]
    pub fltcr: crate::Reg<fltcr::FLTCR_SPEC>,
    _reserved4: [u8; 0x1c],
    #[doc = "0x2c - TAMP interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x30 - TAMP status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x34 - TAMP masked interrupt status register"]
    pub misr: crate::Reg<misr::MISR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x3c - TAMP status clear register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x40 - monotonic counter register"]
    pub countr: crate::Reg<countr::COUNTR_SPEC>,
    _reserved9: [u8; 0xbc],
    #[doc = "0x100..0x150 - TAMP backup register"]
    pub bkpr: [crate::Reg<bkpr::BKPR_SPEC>; 20],
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "CR3 register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "TAMP control register 3"]
pub mod cr3;
#[doc = "FLTCR register accessor: an alias for `Reg<FLTCR_SPEC>`"]
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
#[doc = "TAMP filter control register"]
pub mod fltcr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "TAMP interrupt enable register"]
pub mod ier;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TAMP status register"]
pub mod sr;
#[doc = "MISR register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "TAMP masked interrupt status register"]
pub mod misr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "TAMP status clear register"]
pub mod scr;
#[doc = "COUNTR register accessor: an alias for `Reg<COUNTR_SPEC>`"]
pub type COUNTR = crate::Reg<countr::COUNTR_SPEC>;
#[doc = "monotonic counter register"]
pub mod countr;
#[doc = "BKPR register accessor: an alias for `Reg<BKPR_SPEC>`"]
pub type BKPR = crate::Reg<bkpr::BKPR_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkpr;
