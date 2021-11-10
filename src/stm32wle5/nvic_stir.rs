///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Software trigger interrupt register
    pub stir: crate::Reg<stir::STIR_SPEC>,
}
///STIR register accessor: an alias for `Reg<STIR_SPEC>`
pub type STIR = crate::Reg<stir::STIR_SPEC>;
///Software trigger interrupt register
pub mod stir;
