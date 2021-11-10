///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - HSEM register HSEM_R0 HSEM_R31
    pub r0: crate::Reg<r0::R0_SPEC>,
    ///0x04 - HSEM register HSEM_R0 HSEM_R31
    pub r1: crate::Reg<r1::R1_SPEC>,
    ///0x08 - HSEM register HSEM_R0 HSEM_R31
    pub r2: crate::Reg<r2::R2_SPEC>,
    ///0x0c - HSEM register HSEM_R0 HSEM_R31
    pub r3: crate::Reg<r3::R3_SPEC>,
    ///0x10 - HSEM register HSEM_R0 HSEM_R31
    pub r4: crate::Reg<r4::R4_SPEC>,
    ///0x14 - HSEM register HSEM_R0 HSEM_R31
    pub r5: crate::Reg<r5::R5_SPEC>,
    ///0x18 - HSEM register HSEM_R0 HSEM_R31
    pub r6: crate::Reg<r6::R6_SPEC>,
    ///0x1c - HSEM register HSEM_R0 HSEM_R31
    pub r7: crate::Reg<r7::R7_SPEC>,
    ///0x20 - HSEM register HSEM_R0 HSEM_R31
    pub r8: crate::Reg<r8::R8_SPEC>,
    ///0x24 - HSEM register HSEM_R0 HSEM_R31
    pub r9: crate::Reg<r9::R9_SPEC>,
    ///0x28 - HSEM register HSEM_R0 HSEM_R31
    pub r10: crate::Reg<r10::R10_SPEC>,
    ///0x2c - HSEM register HSEM_R0 HSEM_R31
    pub r11: crate::Reg<r11::R11_SPEC>,
    ///0x30 - HSEM register HSEM_R0 HSEM_R31
    pub r12: crate::Reg<r12::R12_SPEC>,
    ///0x34 - HSEM register HSEM_R0 HSEM_R31
    pub r13: crate::Reg<r13::R13_SPEC>,
    ///0x38 - HSEM register HSEM_R0 HSEM_R31
    pub r14: crate::Reg<r14::R14_SPEC>,
    ///0x3c - HSEM register HSEM_R0 HSEM_R31
    pub r15: crate::Reg<r15::R15_SPEC>,
    _reserved16: [u8; 0x40],
    ///0x80 - HSEM Read lock register
    pub rlr0: crate::Reg<rlr0::RLR0_SPEC>,
    ///0x84 - HSEM Read lock register
    pub rlr1: crate::Reg<rlr1::RLR1_SPEC>,
    ///0x88 - HSEM Read lock register
    pub rlr2: crate::Reg<rlr2::RLR2_SPEC>,
    ///0x8c - HSEM Read lock register
    pub rlr3: crate::Reg<rlr3::RLR3_SPEC>,
    ///0x90 - HSEM Read lock register
    pub rlr4: crate::Reg<rlr4::RLR4_SPEC>,
    ///0x94 - HSEM Read lock register
    pub rlr5: crate::Reg<rlr5::RLR5_SPEC>,
    ///0x98 - HSEM Read lock register
    pub rlr6: crate::Reg<rlr6::RLR6_SPEC>,
    ///0x9c - HSEM Read lock register
    pub rlr7: crate::Reg<rlr7::RLR7_SPEC>,
    ///0xa0 - HSEM Read lock register
    pub rlr8: crate::Reg<rlr8::RLR8_SPEC>,
    ///0xa4 - HSEM Read lock register
    pub rlr9: crate::Reg<rlr9::RLR9_SPEC>,
    ///0xa8 - HSEM Read lock register
    pub rlr10: crate::Reg<rlr10::RLR10_SPEC>,
    ///0xac - HSEM Read lock register
    pub rlr11: crate::Reg<rlr11::RLR11_SPEC>,
    ///0xb0 - HSEM Read lock register
    pub rlr12: crate::Reg<rlr12::RLR12_SPEC>,
    ///0xb4 - HSEM Read lock register
    pub rlr13: crate::Reg<rlr13::RLR13_SPEC>,
    ///0xb8 - HSEM Read lock register
    pub rlr14: crate::Reg<rlr14::RLR14_SPEC>,
    ///0xbc - HSEM Read lock register
    pub rlr15: crate::Reg<rlr15::RLR15_SPEC>,
    _reserved32: [u8; 0x40],
    ///0x100 - HSEM Interrupt enable register
    pub c1ier: crate::Reg<c1ier::C1IER_SPEC>,
    ///0x104 - HSEM Interrupt clear register
    pub c1icr: crate::Reg<c1icr::C1ICR_SPEC>,
    ///0x108 - HSEM Interrupt status register
    pub c1isr: crate::Reg<c1isr::C1ISR_SPEC>,
    ///0x10c - HSEM Masked interrupt status register
    pub c1misr: crate::Reg<c1misr::C1MISR_SPEC>,
    _reserved36: [u8; 0x30],
    ///0x140 - HSEM Clear register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x144 - HSEM Interrupt clear register
    pub keyr: crate::Reg<keyr::KEYR_SPEC>,
}
///R0 register accessor: an alias for `Reg<R0_SPEC>`
pub type R0 = crate::Reg<r0::R0_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r0;
///R1 register accessor: an alias for `Reg<R1_SPEC>`
pub type R1 = crate::Reg<r1::R1_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r1;
///R2 register accessor: an alias for `Reg<R2_SPEC>`
pub type R2 = crate::Reg<r2::R2_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r2;
///R3 register accessor: an alias for `Reg<R3_SPEC>`
pub type R3 = crate::Reg<r3::R3_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r3;
///R4 register accessor: an alias for `Reg<R4_SPEC>`
pub type R4 = crate::Reg<r4::R4_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r4;
///R5 register accessor: an alias for `Reg<R5_SPEC>`
pub type R5 = crate::Reg<r5::R5_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r5;
///R6 register accessor: an alias for `Reg<R6_SPEC>`
pub type R6 = crate::Reg<r6::R6_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r6;
///R7 register accessor: an alias for `Reg<R7_SPEC>`
pub type R7 = crate::Reg<r7::R7_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r7;
///R8 register accessor: an alias for `Reg<R8_SPEC>`
pub type R8 = crate::Reg<r8::R8_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r8;
///R9 register accessor: an alias for `Reg<R9_SPEC>`
pub type R9 = crate::Reg<r9::R9_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r9;
///R10 register accessor: an alias for `Reg<R10_SPEC>`
pub type R10 = crate::Reg<r10::R10_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r10;
///R11 register accessor: an alias for `Reg<R11_SPEC>`
pub type R11 = crate::Reg<r11::R11_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r11;
///R12 register accessor: an alias for `Reg<R12_SPEC>`
pub type R12 = crate::Reg<r12::R12_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r12;
///R13 register accessor: an alias for `Reg<R13_SPEC>`
pub type R13 = crate::Reg<r13::R13_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r13;
///R14 register accessor: an alias for `Reg<R14_SPEC>`
pub type R14 = crate::Reg<r14::R14_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r14;
///R15 register accessor: an alias for `Reg<R15_SPEC>`
pub type R15 = crate::Reg<r15::R15_SPEC>;
///HSEM register HSEM_R0 HSEM_R31
pub mod r15;
///RLR0 register accessor: an alias for `Reg<RLR0_SPEC>`
pub type RLR0 = crate::Reg<rlr0::RLR0_SPEC>;
///HSEM Read lock register
pub mod rlr0;
///RLR1 register accessor: an alias for `Reg<RLR1_SPEC>`
pub type RLR1 = crate::Reg<rlr1::RLR1_SPEC>;
///HSEM Read lock register
pub mod rlr1;
///RLR2 register accessor: an alias for `Reg<RLR2_SPEC>`
pub type RLR2 = crate::Reg<rlr2::RLR2_SPEC>;
///HSEM Read lock register
pub mod rlr2;
///RLR3 register accessor: an alias for `Reg<RLR3_SPEC>`
pub type RLR3 = crate::Reg<rlr3::RLR3_SPEC>;
///HSEM Read lock register
pub mod rlr3;
///RLR4 register accessor: an alias for `Reg<RLR4_SPEC>`
pub type RLR4 = crate::Reg<rlr4::RLR4_SPEC>;
///HSEM Read lock register
pub mod rlr4;
///RLR5 register accessor: an alias for `Reg<RLR5_SPEC>`
pub type RLR5 = crate::Reg<rlr5::RLR5_SPEC>;
///HSEM Read lock register
pub mod rlr5;
///RLR6 register accessor: an alias for `Reg<RLR6_SPEC>`
pub type RLR6 = crate::Reg<rlr6::RLR6_SPEC>;
///HSEM Read lock register
pub mod rlr6;
///RLR7 register accessor: an alias for `Reg<RLR7_SPEC>`
pub type RLR7 = crate::Reg<rlr7::RLR7_SPEC>;
///HSEM Read lock register
pub mod rlr7;
///RLR8 register accessor: an alias for `Reg<RLR8_SPEC>`
pub type RLR8 = crate::Reg<rlr8::RLR8_SPEC>;
///HSEM Read lock register
pub mod rlr8;
///RLR9 register accessor: an alias for `Reg<RLR9_SPEC>`
pub type RLR9 = crate::Reg<rlr9::RLR9_SPEC>;
///HSEM Read lock register
pub mod rlr9;
///RLR10 register accessor: an alias for `Reg<RLR10_SPEC>`
pub type RLR10 = crate::Reg<rlr10::RLR10_SPEC>;
///HSEM Read lock register
pub mod rlr10;
///RLR11 register accessor: an alias for `Reg<RLR11_SPEC>`
pub type RLR11 = crate::Reg<rlr11::RLR11_SPEC>;
///HSEM Read lock register
pub mod rlr11;
///RLR12 register accessor: an alias for `Reg<RLR12_SPEC>`
pub type RLR12 = crate::Reg<rlr12::RLR12_SPEC>;
///HSEM Read lock register
pub mod rlr12;
///RLR13 register accessor: an alias for `Reg<RLR13_SPEC>`
pub type RLR13 = crate::Reg<rlr13::RLR13_SPEC>;
///HSEM Read lock register
pub mod rlr13;
///RLR14 register accessor: an alias for `Reg<RLR14_SPEC>`
pub type RLR14 = crate::Reg<rlr14::RLR14_SPEC>;
///HSEM Read lock register
pub mod rlr14;
///RLR15 register accessor: an alias for `Reg<RLR15_SPEC>`
pub type RLR15 = crate::Reg<rlr15::RLR15_SPEC>;
///HSEM Read lock register
pub mod rlr15;
///C1IER register accessor: an alias for `Reg<C1IER_SPEC>`
pub type C1IER = crate::Reg<c1ier::C1IER_SPEC>;
///HSEM Interrupt enable register
pub mod c1ier;
///C1ICR register accessor: an alias for `Reg<C1ICR_SPEC>`
pub type C1ICR = crate::Reg<c1icr::C1ICR_SPEC>;
///HSEM Interrupt clear register
pub mod c1icr;
///C1ISR register accessor: an alias for `Reg<C1ISR_SPEC>`
pub type C1ISR = crate::Reg<c1isr::C1ISR_SPEC>;
///HSEM Interrupt status register
pub mod c1isr;
///C1MISR register accessor: an alias for `Reg<C1MISR_SPEC>`
pub type C1MISR = crate::Reg<c1misr::C1MISR_SPEC>;
///HSEM Masked interrupt status register
pub mod c1misr;
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///HSEM Clear register
pub mod cr;
///KEYR register accessor: an alias for `Reg<KEYR_SPEC>`
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
///HSEM Interrupt clear register
pub mod keyr;
