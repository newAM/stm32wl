///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - interrupt and status register
    pub isr: crate::Reg<isr::ISR_SPEC>,
    ///0x04 - interrupt clear register
    pub icr: crate::Reg<icr::ICR_SPEC>,
    ///0x08 - interrupt enable register
    pub ier: crate::Reg<ier::IER_SPEC>,
    ///0x0c - configuration register
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    ///0x10 - control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x14 - compare register
    pub cmp: crate::Reg<cmp::CMP_SPEC>,
    ///0x18 - autoreload register
    pub arr: crate::Reg<arr::ARR_SPEC>,
    ///0x1c - counter register
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    ///0x20 - option register
    pub or: crate::Reg<or::OR_SPEC>,
    _reserved9: [u8; 0x04],
    ///0x28 - repetition register
    pub rcr: crate::Reg<rcr::RCR_SPEC>,
}
///ISR register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt and status register
pub mod isr;
///ICR register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///interrupt clear register
pub mod icr;
///IER register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///interrupt enable register
pub mod ier;
///CFGR register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///configuration register
pub mod cfgr;
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///CMP register accessor: an alias for `Reg<CMP_SPEC>`
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
///compare register
pub mod cmp;
///ARR register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///autoreload register
pub mod arr;
///CNT register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///counter register
pub mod cnt;
///OR register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///option register
pub mod or;
///RCR register accessor: an alias for `Reg<RCR_SPEC>`
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
///repetition register
pub mod rcr;
