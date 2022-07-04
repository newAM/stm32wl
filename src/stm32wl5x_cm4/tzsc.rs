#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TZSC control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - TZSC security configuration register"]
    pub seccfgr1: crate::Reg<seccfgr1::SECCFGR1_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - TZSC privilege configuration register 1"]
    pub privcfgr1: crate::Reg<privcfgr1::PRIVCFGR1_SPEC>,
    _reserved3: [u8; 0x010c],
    #[doc = "0x130 - Unprivileged Water Mark 1 register"]
    pub mpcwm1_upwmr: crate::Reg<mpcwm1_upwmr::MPCWM1_UPWMR_SPEC>,
    #[doc = "0x134 - Unprivileged Writable Water Mark 1 register"]
    pub mpcwm1_upwwmr: crate::Reg<mpcwm1_upwwmr::MPCWM1_UPWWMR_SPEC>,
    #[doc = "0x138 - Unprivileged Water Mark 2 register"]
    pub mpcwm2_upwmr: crate::Reg<mpcwm2_upwmr::MPCWM2_UPWMR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x140 - Unprivileged Water Mark 3 register"]
    pub mpcwm3_upwmr: crate::Reg<mpcwm3_upwmr::MPCWM3_UPWMR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "TZSC control register"]
pub mod cr;
#[doc = "SECCFGR1 register accessor: an alias for `Reg<SECCFGR1_SPEC>`"]
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1_SPEC>;
#[doc = "TZSC security configuration register"]
pub mod seccfgr1;
#[doc = "PRIVCFGR1 register accessor: an alias for `Reg<PRIVCFGR1_SPEC>`"]
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1_SPEC>;
#[doc = "TZSC privilege configuration register 1"]
pub mod privcfgr1;
#[doc = "MPCWM1_UPWMR register accessor: an alias for `Reg<MPCWM1_UPWMR_SPEC>`"]
pub type MPCWM1_UPWMR = crate::Reg<mpcwm1_upwmr::MPCWM1_UPWMR_SPEC>;
#[doc = "Unprivileged Water Mark 1 register"]
pub mod mpcwm1_upwmr;
#[doc = "MPCWM1_UPWWMR register accessor: an alias for `Reg<MPCWM1_UPWWMR_SPEC>`"]
pub type MPCWM1_UPWWMR = crate::Reg<mpcwm1_upwwmr::MPCWM1_UPWWMR_SPEC>;
#[doc = "Unprivileged Writable Water Mark 1 register"]
pub mod mpcwm1_upwwmr;
#[doc = "MPCWM2_UPWMR register accessor: an alias for `Reg<MPCWM2_UPWMR_SPEC>`"]
pub type MPCWM2_UPWMR = crate::Reg<mpcwm2_upwmr::MPCWM2_UPWMR_SPEC>;
#[doc = "Unprivileged Water Mark 2 register"]
pub mod mpcwm2_upwmr;
#[doc = "MPCWM3_UPWMR register accessor: an alias for `Reg<MPCWM3_UPWMR_SPEC>`"]
pub type MPCWM3_UPWMR = crate::Reg<mpcwm3_upwmr::MPCWM3_UPWMR_SPEC>;
#[doc = "Unprivileged Water Mark 3 register"]
pub mod mpcwm3_upwmr;
