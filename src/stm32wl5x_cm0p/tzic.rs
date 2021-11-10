///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TZIC interrupt enable register 1
    pub ier1: crate::Reg<ier1::IER1_SPEC>,
    _reserved1: [u8; 0x0c],
    ///0x10 - TZIC status register 1
    pub misr1: crate::Reg<misr1::MISR1_SPEC>,
    _reserved2: [u8; 0x0c],
    ///0x20 - TZIC interrupt status clear register 1
    pub icr1: crate::Reg<icr1::ICR1_SPEC>,
}
///IER1 register accessor: an alias for `Reg<IER1_SPEC>`
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
///TZIC interrupt enable register 1
pub mod ier1;
///MISR1 register accessor: an alias for `Reg<MISR1_SPEC>`
pub type MISR1 = crate::Reg<misr1::MISR1_SPEC>;
///TZIC status register 1
pub mod misr1;
///ICR1 register accessor: an alias for `Reg<ICR1_SPEC>`
pub type ICR1 = crate::Reg<icr1::ICR1_SPEC>;
///TZIC interrupt status clear register 1
pub mod icr1;
