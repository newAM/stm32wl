///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - request line multiplexer channel x configuration register
    pub c0cr: crate::Reg<c0cr::C0CR_SPEC>,
    ///0x04 - request line multiplexer channel x configuration register
    pub c1cr: crate::Reg<c1cr::C1CR_SPEC>,
    ///0x08 - request line multiplexer channel x configuration register
    pub c2cr: crate::Reg<c2cr::C2CR_SPEC>,
    ///0x0c - request line multiplexer channel x configuration register
    pub c3cr: crate::Reg<c3cr::C3CR_SPEC>,
    ///0x10 - request line multiplexer channel x configuration register
    pub c4cr: crate::Reg<c4cr::C4CR_SPEC>,
    ///0x14 - request line multiplexer channel x configuration register
    pub c5cr: crate::Reg<c5cr::C5CR_SPEC>,
    ///0x18 - request line multiplexer channel x configuration register
    pub c6cr: crate::Reg<c6cr::C6CR_SPEC>,
    ///0x1c - request line multiplexer channel x configuration register
    pub c7cr: crate::Reg<c7cr::C7CR_SPEC>,
    ///0x20 - request line multiplexer channel x configuration register
    pub c8cr: crate::Reg<c8cr::C8CR_SPEC>,
    ///0x24 - request line multiplexer channel x configuration register
    pub c9cr: crate::Reg<c9cr::C9CR_SPEC>,
    ///0x28 - C10CR
    pub c10cr: crate::Reg<c10cr::C10CR_SPEC>,
    ///0x2c - C11CR
    pub c11cr: crate::Reg<c11cr::C11CR_SPEC>,
    ///0x30 - C12CR
    pub c12cr: crate::Reg<c12cr::C12CR_SPEC>,
    ///0x34 - C13CR
    pub c13cr: crate::Reg<c13cr::C13CR_SPEC>,
    _reserved14: [u8; 0x48],
    ///0x80 - request line multiplexer interrupt channel status register
    pub csr: crate::Reg<csr::CSR_SPEC>,
    ///0x84 - request line multiplexer interrupt channel clear flag register
    pub ccfr: crate::Reg<ccfr::CCFR_SPEC>,
    _reserved16: [u8; 0x78],
    ///0x100 - request generator channel x configuration register
    pub rg0cr: crate::Reg<rg0cr::RG0CR_SPEC>,
    ///0x104 - request generator channel x configuration register
    pub rg1cr: crate::Reg<rg1cr::RG1CR_SPEC>,
    ///0x108 - request generator channel x configuration register
    pub rg2cr: crate::Reg<rg2cr::RG2CR_SPEC>,
    ///0x10c - request generator channel x configuration register
    pub rg3cr: crate::Reg<rg3cr::RG3CR_SPEC>,
    _reserved20: [u8; 0x30],
    ///0x140 - request generator interrupt status register
    pub rgsr: crate::Reg<rgsr::RGSR_SPEC>,
    ///0x144 - request generator interrupt clear flag register
    pub rgcfr: crate::Reg<rgcfr::RGCFR_SPEC>,
}
///C0CR register accessor: an alias for `Reg<C0CR_SPEC>`
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
///request line multiplexer channel x configuration register
pub mod c0cr;
///C1CR register accessor: an alias for `Reg<C1CR_SPEC>`
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
///request line multiplexer channel x configuration register
pub mod c1cr;
///C2CR register accessor: an alias for `Reg<C2CR_SPEC>`
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
///request line multiplexer channel x configuration register
pub mod c2cr;
///C3CR register accessor: an alias for `Reg<C3CR_SPEC>`
pub type C3CR = crate::Reg<c3cr::C3CR_SPEC>;
///request line multiplexer channel x configuration register
pub mod c3cr;
///C4CR register accessor: an alias for `Reg<C4CR_SPEC>`
pub type C4CR = crate::Reg<c4cr::C4CR_SPEC>;
///request line multiplexer channel x configuration register
pub mod c4cr;
///C5CR register accessor: an alias for `Reg<C5CR_SPEC>`
pub type C5CR = crate::Reg<c5cr::C5CR_SPEC>;
///request line multiplexer channel x configuration register
pub mod c5cr;
///C6CR register accessor: an alias for `Reg<C6CR_SPEC>`
pub type C6CR = crate::Reg<c6cr::C6CR_SPEC>;
///request line multiplexer channel x configuration register
pub mod c6cr;
///C7CR register accessor: an alias for `Reg<C7CR_SPEC>`
pub type C7CR = crate::Reg<c7cr::C7CR_SPEC>;
///request line multiplexer channel x configuration register
pub mod c7cr;
///C8CR register accessor: an alias for `Reg<C8CR_SPEC>`
pub type C8CR = crate::Reg<c8cr::C8CR_SPEC>;
///request line multiplexer channel x configuration register
pub mod c8cr;
///C9CR register accessor: an alias for `Reg<C9CR_SPEC>`
pub type C9CR = crate::Reg<c9cr::C9CR_SPEC>;
///request line multiplexer channel x configuration register
pub mod c9cr;
///C10CR register accessor: an alias for `Reg<C10CR_SPEC>`
pub type C10CR = crate::Reg<c10cr::C10CR_SPEC>;
///C10CR
pub mod c10cr;
///C11CR register accessor: an alias for `Reg<C11CR_SPEC>`
pub type C11CR = crate::Reg<c11cr::C11CR_SPEC>;
///C11CR
pub mod c11cr;
///C12CR register accessor: an alias for `Reg<C12CR_SPEC>`
pub type C12CR = crate::Reg<c12cr::C12CR_SPEC>;
///C12CR
pub mod c12cr;
///C13CR register accessor: an alias for `Reg<C13CR_SPEC>`
pub type C13CR = crate::Reg<c13cr::C13CR_SPEC>;
///C13CR
pub mod c13cr;
///CSR register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///request line multiplexer interrupt channel status register
pub mod csr;
///CCFR register accessor: an alias for `Reg<CCFR_SPEC>`
pub type CCFR = crate::Reg<ccfr::CCFR_SPEC>;
///request line multiplexer interrupt channel clear flag register
pub mod ccfr;
///RG0CR register accessor: an alias for `Reg<RG0CR_SPEC>`
pub type RG0CR = crate::Reg<rg0cr::RG0CR_SPEC>;
///request generator channel x configuration register
pub mod rg0cr;
///RG1CR register accessor: an alias for `Reg<RG1CR_SPEC>`
pub type RG1CR = crate::Reg<rg1cr::RG1CR_SPEC>;
///request generator channel x configuration register
pub mod rg1cr;
///RG2CR register accessor: an alias for `Reg<RG2CR_SPEC>`
pub type RG2CR = crate::Reg<rg2cr::RG2CR_SPEC>;
///request generator channel x configuration register
pub mod rg2cr;
///RG3CR register accessor: an alias for `Reg<RG3CR_SPEC>`
pub type RG3CR = crate::Reg<rg3cr::RG3CR_SPEC>;
///request generator channel x configuration register
pub mod rg3cr;
///RGSR register accessor: an alias for `Reg<RGSR_SPEC>`
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
///request generator interrupt status register
pub mod rgsr;
///RGCFR register accessor: an alias for `Reg<RGCFR_SPEC>`
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
///request generator interrupt clear flag register
pub mod rgcfr;