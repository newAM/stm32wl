///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    ///0x04 - Control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    ///0x08 - Control register 3
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    ///0x0c - Baud rate register
    pub brr: crate::Reg<brr::BRR_SPEC>,
    _reserved4: [u8; 0x08],
    ///0x18 - Request register
    pub rqr: crate::Reg<rqr::RQR_SPEC>,
    ///0x1c - Interrupt and status register
    pub isr: crate::Reg<isr::ISR_SPEC>,
    ///0x20 - Interrupt flag clear register
    pub icr: crate::Reg<icr::ICR_SPEC>,
    ///0x24 - Receive data register
    pub rdr: crate::Reg<rdr::RDR_SPEC>,
    ///0x28 - Transmit data register
    pub tdr: crate::Reg<tdr::TDR_SPEC>,
    ///0x2c - Prescaler register
    pub presc: crate::Reg<presc::PRESC_SPEC>,
}
///CR1 register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///Control register 1
pub mod cr1;
///CR2 register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///Control register 2
pub mod cr2;
///CR3 register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///Control register 3
pub mod cr3;
///BRR register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///Baud rate register
pub mod brr;
///RQR register accessor: an alias for `Reg<RQR_SPEC>`
pub type RQR = crate::Reg<rqr::RQR_SPEC>;
///Request register
pub mod rqr;
///ISR register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt and status register
pub mod isr;
///ICR register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///Interrupt flag clear register
pub mod icr;
///RDR register accessor: an alias for `Reg<RDR_SPEC>`
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
///Receive data register
pub mod rdr;
///TDR register accessor: an alias for `Reg<TDR_SPEC>`
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
///Transmit data register
pub mod tdr;
///PRESC register accessor: an alias for `Reg<PRESC_SPEC>`
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
///Prescaler register
pub mod presc;
