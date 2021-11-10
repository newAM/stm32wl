///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - interrupt status register
    pub isr: crate::Reg<isr::ISR_SPEC>,
    ///0x04 - interrupt flag clear register
    pub ifcr: crate::Reg<ifcr::IFCR_SPEC>,
    ///0x08 - channel x configuration register
    pub ccr1: crate::Reg<ccr1::CCR1_SPEC>,
    ///0x0c - channel x number of data to transfer register
    pub cndtr1: crate::Reg<cndtr1::CNDTR1_SPEC>,
    ///0x10 - channel x peripheral address register
    pub cpar1: crate::Reg<cpar1::CPAR1_SPEC>,
    ///0x14 - channel x memory address register
    pub cmar1: crate::Reg<cmar1::CMAR1_SPEC>,
    _reserved6: [u8; 0x04],
    ///0x1c - channel x configuration register
    pub ccr2: crate::Reg<ccr2::CCR2_SPEC>,
    ///0x20 - channel x number of data to transfer register
    pub cndtr2: crate::Reg<cndtr2::CNDTR2_SPEC>,
    ///0x24 - channel x peripheral address register
    pub cpar2: crate::Reg<cpar2::CPAR2_SPEC>,
    ///0x28 - channel x memory address register
    pub cmar2: crate::Reg<cmar2::CMAR2_SPEC>,
    _reserved10: [u8; 0x04],
    ///0x30 - channel x configuration register
    pub ccr3: crate::Reg<ccr3::CCR3_SPEC>,
    ///0x34 - channel x number of data to transfer register
    pub cndtr3: crate::Reg<cndtr3::CNDTR3_SPEC>,
    ///0x38 - channel x peripheral address register
    pub cpar3: crate::Reg<cpar3::CPAR3_SPEC>,
    ///0x3c - channel x memory address register
    pub cmar3: crate::Reg<cmar3::CMAR3_SPEC>,
    _reserved14: [u8; 0x04],
    ///0x44 - channel x configuration register
    pub ccr4: crate::Reg<ccr4::CCR4_SPEC>,
    ///0x48 - channel x number of data to transfer register
    pub cndtr4: crate::Reg<cndtr4::CNDTR4_SPEC>,
    ///0x4c - channel x peripheral address register
    pub cpar4: crate::Reg<cpar4::CPAR4_SPEC>,
    ///0x50 - channel x memory address register
    pub cmar4: crate::Reg<cmar4::CMAR4_SPEC>,
    _reserved18: [u8; 0x04],
    ///0x58 - channel x configuration register
    pub ccr5: crate::Reg<ccr5::CCR5_SPEC>,
    ///0x5c - channel x number of data to transfer register
    pub cndtr5: crate::Reg<cndtr5::CNDTR5_SPEC>,
    ///0x60 - channel x peripheral address register
    pub cpar5: crate::Reg<cpar5::CPAR5_SPEC>,
    ///0x64 - channel x memory address register
    pub cmar5: crate::Reg<cmar5::CMAR5_SPEC>,
    _reserved22: [u8; 0x04],
    ///0x6c - channel x configuration register
    pub ccr6: crate::Reg<ccr6::CCR6_SPEC>,
    ///0x70 - channel x number of data to transfer register
    pub cndtr6: crate::Reg<cndtr6::CNDTR6_SPEC>,
    ///0x74 - channel x peripheral address register
    pub cpar6: crate::Reg<cpar6::CPAR6_SPEC>,
    ///0x78 - channel x memory address register
    pub cmar6: crate::Reg<cmar6::CMAR6_SPEC>,
    _reserved26: [u8; 0x04],
    ///0x80 - channel x configuration register
    pub ccr7: crate::Reg<ccr7::CCR7_SPEC>,
    ///0x84 - channel x number of data to transfer register
    pub cndtr7: crate::Reg<cndtr7::CNDTR7_SPEC>,
    ///0x88 - channel x peripheral address register
    pub cpar7: crate::Reg<cpar7::CPAR7_SPEC>,
    ///0x8c - channel x memory address register
    pub cmar7: crate::Reg<cmar7::CMAR7_SPEC>,
}
///ISR register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt status register
pub mod isr;
///IFCR register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///interrupt flag clear register
pub mod ifcr;
///CCR1 register accessor: an alias for `Reg<CCR1_SPEC>`
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
///channel x configuration register
pub mod ccr1;
///CNDTR1 register accessor: an alias for `Reg<CNDTR1_SPEC>`
pub type CNDTR1 = crate::Reg<cndtr1::CNDTR1_SPEC>;
///channel x number of data to transfer register
pub mod cndtr1;
///CPAR1 register accessor: an alias for `Reg<CPAR1_SPEC>`
pub type CPAR1 = crate::Reg<cpar1::CPAR1_SPEC>;
///channel x peripheral address register
pub mod cpar1;
///CMAR1 register accessor: an alias for `Reg<CMAR1_SPEC>`
pub type CMAR1 = crate::Reg<cmar1::CMAR1_SPEC>;
///channel x memory address register
pub mod cmar1;
///CCR2 register accessor: an alias for `Reg<CCR2_SPEC>`
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
///channel x configuration register
pub mod ccr2;
///CNDTR2 register accessor: an alias for `Reg<CNDTR2_SPEC>`
pub type CNDTR2 = crate::Reg<cndtr2::CNDTR2_SPEC>;
///channel x number of data to transfer register
pub mod cndtr2;
///CPAR2 register accessor: an alias for `Reg<CPAR2_SPEC>`
pub type CPAR2 = crate::Reg<cpar2::CPAR2_SPEC>;
///channel x peripheral address register
pub mod cpar2;
///CMAR2 register accessor: an alias for `Reg<CMAR2_SPEC>`
pub type CMAR2 = crate::Reg<cmar2::CMAR2_SPEC>;
///channel x memory address register
pub mod cmar2;
///CCR3 register accessor: an alias for `Reg<CCR3_SPEC>`
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
///channel x configuration register
pub mod ccr3;
///CNDTR3 register accessor: an alias for `Reg<CNDTR3_SPEC>`
pub type CNDTR3 = crate::Reg<cndtr3::CNDTR3_SPEC>;
///channel x number of data to transfer register
pub mod cndtr3;
///CPAR3 register accessor: an alias for `Reg<CPAR3_SPEC>`
pub type CPAR3 = crate::Reg<cpar3::CPAR3_SPEC>;
///channel x peripheral address register
pub mod cpar3;
///CMAR3 register accessor: an alias for `Reg<CMAR3_SPEC>`
pub type CMAR3 = crate::Reg<cmar3::CMAR3_SPEC>;
///channel x memory address register
pub mod cmar3;
///CCR4 register accessor: an alias for `Reg<CCR4_SPEC>`
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
///channel x configuration register
pub mod ccr4;
///CNDTR4 register accessor: an alias for `Reg<CNDTR4_SPEC>`
pub type CNDTR4 = crate::Reg<cndtr4::CNDTR4_SPEC>;
///channel x number of data to transfer register
pub mod cndtr4;
///CPAR4 register accessor: an alias for `Reg<CPAR4_SPEC>`
pub type CPAR4 = crate::Reg<cpar4::CPAR4_SPEC>;
///channel x peripheral address register
pub mod cpar4;
///CMAR4 register accessor: an alias for `Reg<CMAR4_SPEC>`
pub type CMAR4 = crate::Reg<cmar4::CMAR4_SPEC>;
///channel x memory address register
pub mod cmar4;
///CCR5 register accessor: an alias for `Reg<CCR5_SPEC>`
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
///channel x configuration register
pub mod ccr5;
///CNDTR5 register accessor: an alias for `Reg<CNDTR5_SPEC>`
pub type CNDTR5 = crate::Reg<cndtr5::CNDTR5_SPEC>;
///channel x number of data to transfer register
pub mod cndtr5;
///CPAR5 register accessor: an alias for `Reg<CPAR5_SPEC>`
pub type CPAR5 = crate::Reg<cpar5::CPAR5_SPEC>;
///channel x peripheral address register
pub mod cpar5;
///CMAR5 register accessor: an alias for `Reg<CMAR5_SPEC>`
pub type CMAR5 = crate::Reg<cmar5::CMAR5_SPEC>;
///channel x memory address register
pub mod cmar5;
///CCR6 register accessor: an alias for `Reg<CCR6_SPEC>`
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
///channel x configuration register
pub mod ccr6;
///CNDTR6 register accessor: an alias for `Reg<CNDTR6_SPEC>`
pub type CNDTR6 = crate::Reg<cndtr6::CNDTR6_SPEC>;
///channel x number of data to transfer register
pub mod cndtr6;
///CPAR6 register accessor: an alias for `Reg<CPAR6_SPEC>`
pub type CPAR6 = crate::Reg<cpar6::CPAR6_SPEC>;
///channel x peripheral address register
pub mod cpar6;
///CMAR6 register accessor: an alias for `Reg<CMAR6_SPEC>`
pub type CMAR6 = crate::Reg<cmar6::CMAR6_SPEC>;
///channel x memory address register
pub mod cmar6;
///CCR7 register accessor: an alias for `Reg<CCR7_SPEC>`
pub type CCR7 = crate::Reg<ccr7::CCR7_SPEC>;
///channel x configuration register
pub mod ccr7;
///CNDTR7 register accessor: an alias for `Reg<CNDTR7_SPEC>`
pub type CNDTR7 = crate::Reg<cndtr7::CNDTR7_SPEC>;
///channel x number of data to transfer register
pub mod cndtr7;
///CPAR7 register accessor: an alias for `Reg<CPAR7_SPEC>`
pub type CPAR7 = crate::Reg<cpar7::CPAR7_SPEC>;
///channel x peripheral address register
pub mod cpar7;
///CMAR7 register accessor: an alias for `Reg<CMAR7_SPEC>`
pub type CMAR7 = crate::Reg<cmar7::CMAR7_SPEC>;
///channel x memory address register
pub mod cmar7;
