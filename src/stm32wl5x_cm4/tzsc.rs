///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TZSC control register
    pub tzsc_cr: crate::Reg<tzsc_cr::TZSC_CR_SPEC>,
    _reserved1: [u8; 0x0c],
    ///0x10 - TZSC security configuration register
    pub tzsc_seccfgr1: crate::Reg<tzsc_seccfgr1::TZSC_SECCFGR1_SPEC>,
    _reserved2: [u8; 0x0c],
    ///0x20 - TZSC privilege configuration register 1
    pub tzsc_privcfgr1: crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1_SPEC>,
    _reserved3: [u8; 0x010c],
    ///0x130 - Unprivileged Water Mark 1 register
    pub tzsc_mpcwm1_upwmr: crate::Reg<tzsc_mpcwm1_upwmr::TZSC_MPCWM1_UPWMR_SPEC>,
    ///0x134 - Unprivileged Writable Water Mark 1 register
    pub tzsc_mpcwm1_upwwmr: crate::Reg<tzsc_mpcwm1_upwwmr::TZSC_MPCWM1_UPWWMR_SPEC>,
    ///0x138 - Unprivileged Water Mark 2 register
    pub tzsc_mpcwm2_upwmr: crate::Reg<tzsc_mpcwm2_upwmr::TZSC_MPCWM2_UPWMR_SPEC>,
    _reserved6: [u8; 0x04],
    ///0x140 - Unprivileged Water Mark 3 register
    pub tzsc_mpcwm3_upwmr: crate::Reg<tzsc_mpcwm3_upwmr::TZSC_MPCWM3_UPWMR_SPEC>,
}
///TZSC_CR register accessor: an alias for `Reg<TZSC_CR_SPEC>`
pub type TZSC_CR = crate::Reg<tzsc_cr::TZSC_CR_SPEC>;
///TZSC control register
pub mod tzsc_cr;
///TZSC_SECCFGR1 register accessor: an alias for `Reg<TZSC_SECCFGR1_SPEC>`
pub type TZSC_SECCFGR1 = crate::Reg<tzsc_seccfgr1::TZSC_SECCFGR1_SPEC>;
///TZSC security configuration register
pub mod tzsc_seccfgr1;
///TZSC_PRIVCFGR1 register accessor: an alias for `Reg<TZSC_PRIVCFGR1_SPEC>`
pub type TZSC_PRIVCFGR1 = crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1_SPEC>;
///TZSC privilege configuration register 1
pub mod tzsc_privcfgr1;
///TZSC_MPCWM1_UPWMR register accessor: an alias for `Reg<TZSC_MPCWM1_UPWMR_SPEC>`
pub type TZSC_MPCWM1_UPWMR = crate::Reg<tzsc_mpcwm1_upwmr::TZSC_MPCWM1_UPWMR_SPEC>;
///Unprivileged Water Mark 1 register
pub mod tzsc_mpcwm1_upwmr;
///TZSC_MPCWM1_UPWWMR register accessor: an alias for `Reg<TZSC_MPCWM1_UPWWMR_SPEC>`
pub type TZSC_MPCWM1_UPWWMR = crate::Reg<tzsc_mpcwm1_upwwmr::TZSC_MPCWM1_UPWWMR_SPEC>;
///Unprivileged Writable Water Mark 1 register
pub mod tzsc_mpcwm1_upwwmr;
///TZSC_MPCWM2_UPWMR register accessor: an alias for `Reg<TZSC_MPCWM2_UPWMR_SPEC>`
pub type TZSC_MPCWM2_UPWMR = crate::Reg<tzsc_mpcwm2_upwmr::TZSC_MPCWM2_UPWMR_SPEC>;
///Unprivileged Water Mark 2 register
pub mod tzsc_mpcwm2_upwmr;
///TZSC_MPCWM3_UPWMR register accessor: an alias for `Reg<TZSC_MPCWM3_UPWMR_SPEC>`
pub type TZSC_MPCWM3_UPWMR = crate::Reg<tzsc_mpcwm3_upwmr::TZSC_MPCWM3_UPWMR_SPEC>;
///Unprivileged Water Mark 3 register
pub mod tzsc_mpcwm3_upwmr;