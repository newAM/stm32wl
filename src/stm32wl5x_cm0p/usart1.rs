///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    ///0x04 - control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    ///0x08 - control register 3
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    ///0x0c - baud rate register
    pub brr: crate::Reg<brr::BRR_SPEC>,
    ///0x10 - guard time and prescaler register
    pub gtpr: crate::Reg<gtpr::GTPR_SPEC>,
    ///0x14 - receiver timeout register
    pub rtor: crate::Reg<rtor::RTOR_SPEC>,
    ///0x18 - request register
    pub rqr: crate::Reg<rqr::RQR_SPEC>,
    ///0x1c - interrupt and status register
    pub isr: crate::Reg<isr::ISR_SPEC>,
    ///0x20 - interrupt flag clear register
    pub icr: crate::Reg<icr::ICR_SPEC>,
    ///0x24 - receive data register
    pub rdr: crate::Reg<rdr::RDR_SPEC>,
    ///0x28 - transmit data register
    pub tdr: crate::Reg<tdr::TDR_SPEC>,
    ///0x2c - prescaler register
    pub presc: crate::Reg<presc::PRESC_SPEC>,
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
///control register 3
pub mod cr3;
///BRR register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///baud rate register
pub mod brr;
///GTPR register accessor: an alias for `Reg<GTPR_SPEC>`
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
///guard time and prescaler register
pub mod gtpr;
///RTOR register accessor: an alias for `Reg<RTOR_SPEC>`
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
///receiver timeout register
pub mod rtor;
///RQR register accessor: an alias for `Reg<RQR_SPEC>`
pub type RQR = crate::Reg<rqr::RQR_SPEC>;
///request register
pub mod rqr;
///ISR register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt and status register
pub mod isr;
///ICR register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///interrupt flag clear register
pub mod icr;
///RDR register accessor: an alias for `Reg<RDR_SPEC>`
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
///receive data register
pub mod rdr;
///TDR register accessor: an alias for `Reg<TDR_SPEC>`
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
///transmit data register
pub mod tdr;
///PRESC register accessor: an alias for `Reg<PRESC_SPEC>`
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
///prescaler register
pub mod presc;
