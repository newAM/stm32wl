#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - HSEM register HSEM_R%s HSEM_R31"]
    pub r: [crate::Reg<r::R_SPEC>; 16],
    _reserved1: [u8; 0x40],
    #[doc = "0x80..0xc0 - HSEM Read lock register"]
    pub rlr: [crate::Reg<rlr::RLR_SPEC>; 16],
    _reserved2: [u8; 0x40],
    #[doc = "0x100 - HSEM Interrupt enable register"]
    pub c1ier: crate::Reg<c1ier::C1IER_SPEC>,
    #[doc = "0x104 - HSEM Interrupt clear register"]
    pub c1icr: crate::Reg<c1icr::C1ICR_SPEC>,
    #[doc = "0x108 - HSEM Interrupt status register"]
    pub c1isr: crate::Reg<c1isr::C1ISR_SPEC>,
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    pub c1misr: crate::Reg<c1misr::C1MISR_SPEC>,
    _reserved6: [u8; 0x30],
    #[doc = "0x140 - HSEM Clear register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x144 - HSEM Interrupt clear register"]
    pub keyr: crate::Reg<keyr::KEYR_SPEC>,
}
#[doc = "R register accessor: an alias for `Reg<R_SPEC>`"]
pub type R = crate::Reg<r::R_SPEC>;
#[doc = "HSEM register HSEM_R%s HSEM_R31"]
pub mod r;
#[doc = "RLR register accessor: an alias for `Reg<RLR_SPEC>`"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr;
#[doc = "C1IER register accessor: an alias for `Reg<C1IER_SPEC>`"]
pub type C1IER = crate::Reg<c1ier::C1IER_SPEC>;
#[doc = "HSEM Interrupt enable register"]
pub mod c1ier;
#[doc = "C1ICR register accessor: an alias for `Reg<C1ICR_SPEC>`"]
pub type C1ICR = crate::Reg<c1icr::C1ICR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod c1icr;
#[doc = "C1ISR register accessor: an alias for `Reg<C1ISR_SPEC>`"]
pub type C1ISR = crate::Reg<c1isr::C1ISR_SPEC>;
#[doc = "HSEM Interrupt status register"]
pub mod c1isr;
#[doc = "C1MISR register accessor: an alias for `Reg<C1MISR_SPEC>`"]
pub type C1MISR = crate::Reg<c1misr::C1MISR_SPEC>;
#[doc = "HSEM Masked interrupt status register"]
pub mod c1misr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "HSEM Clear register"]
pub mod cr;
#[doc = "KEYR register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod keyr;
