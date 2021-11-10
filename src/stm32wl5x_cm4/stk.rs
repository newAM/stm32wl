///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SysTick control and status register
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    ///0x04 - SysTick reload value register
    pub load: crate::Reg<load::LOAD_SPEC>,
    ///0x08 - SysTick current value register
    pub val: crate::Reg<val::VAL_SPEC>,
    ///0x0c - SysTick calibration value register
    pub calib: crate::Reg<calib::CALIB_SPEC>,
}
///CTRL register accessor: an alias for `Reg<CTRL_SPEC>`
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///SysTick control and status register
pub mod ctrl;
///LOAD register accessor: an alias for `Reg<LOAD_SPEC>`
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
///SysTick reload value register
pub mod load;
///VAL register accessor: an alias for `Reg<VAL_SPEC>`
pub type VAL = crate::Reg<val::VAL_SPEC>;
///SysTick current value register
pub mod val;
///CALIB register accessor: an alias for `Reg<CALIB_SPEC>`
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
///SysTick calibration value register
pub mod calib;
