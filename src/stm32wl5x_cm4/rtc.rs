///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Time register
    pub tr: crate::Reg<tr::TR_SPEC>,
    ///0x04 - Date register
    pub dr: crate::Reg<dr::DR_SPEC>,
    ///0x08 - Sub second register
    pub ssr: crate::Reg<ssr::SSR_SPEC>,
    ///0x0c - Initialization control and status register
    pub icsr: crate::Reg<icsr::ICSR_SPEC>,
    ///0x10 - Pre-scaler register
    pub prer: crate::Reg<prer::PRER_SPEC>,
    ///0x14 - Wakeup timer register
    pub wutr: crate::Reg<wutr::WUTR_SPEC>,
    ///0x18 - Control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved7: [u8; 0x08],
    ///0x24 - Write protection register
    pub wpr: crate::Reg<wpr::WPR_SPEC>,
    ///0x28 - Calibration register
    pub calr: crate::Reg<calr::CALR_SPEC>,
    ///0x2c - Shift control register
    pub shiftr: crate::Reg<shiftr::SHIFTR_SPEC>,
    ///0x30 - Timestamp time register
    pub tstr: crate::Reg<tstr::TSTR_SPEC>,
    ///0x34 - Timestamp date register
    pub tsdr: crate::Reg<tsdr::TSDR_SPEC>,
    ///0x38 - Timestamp sub second register
    pub tsssr: crate::Reg<tsssr::TSSSR_SPEC>,
    _reserved13: [u8; 0x04],
    ///0x40 - Alarm A register
    pub alrmar: crate::Reg<alrmar::ALRMAR_SPEC>,
    ///0x44 - Alarm A sub second register
    pub alrmassr: crate::Reg<alrmassr::ALRMASSR_SPEC>,
    ///0x48 - Alarm B register
    pub alrmbr: crate::Reg<alrmbr::ALRMBR_SPEC>,
    ///0x4c - Alarm B sub second register
    pub alrmbssr: crate::Reg<alrmbssr::ALRMBSSR_SPEC>,
    ///0x50 - Status register (interrupts)
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x54 - Masked interrupt status register
    pub misr: crate::Reg<misr::MISR_SPEC>,
    _reserved19: [u8; 0x04],
    ///0x5c - Status clear register (interrupts)
    pub scr: crate::Reg<scr::SCR_SPEC>,
    _reserved20: [u8; 0x10],
    ///0x70 - RTC alarm A binary mode register
    pub alrabinr: crate::Reg<alrabinr::ALRABINR_SPEC>,
    ///0x74 - RTC alarm B binary mode register
    pub alrbbinr: crate::Reg<alrbbinr::ALRBBINR_SPEC>,
}
///TR register accessor: an alias for `Reg<TR_SPEC>`
pub type TR = crate::Reg<tr::TR_SPEC>;
///Time register
pub mod tr;
///DR register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///Date register
pub mod dr;
///SSR register accessor: an alias for `Reg<SSR_SPEC>`
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
///Sub second register
pub mod ssr;
///ICSR register accessor: an alias for `Reg<ICSR_SPEC>`
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
///Initialization control and status register
pub mod icsr;
///PRER register accessor: an alias for `Reg<PRER_SPEC>`
pub type PRER = crate::Reg<prer::PRER_SPEC>;
///Pre-scaler register
pub mod prer;
///WUTR register accessor: an alias for `Reg<WUTR_SPEC>`
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
///Wakeup timer register
pub mod wutr;
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register
pub mod cr;
///WPR register accessor: an alias for `Reg<WPR_SPEC>`
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
///Write protection register
pub mod wpr;
///CALR register accessor: an alias for `Reg<CALR_SPEC>`
pub type CALR = crate::Reg<calr::CALR_SPEC>;
///Calibration register
pub mod calr;
///SHIFTR register accessor: an alias for `Reg<SHIFTR_SPEC>`
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
///Shift control register
pub mod shiftr;
///TSTR register accessor: an alias for `Reg<TSTR_SPEC>`
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
///Timestamp time register
pub mod tstr;
///TSDR register accessor: an alias for `Reg<TSDR_SPEC>`
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
///Timestamp date register
pub mod tsdr;
///TSSSR register accessor: an alias for `Reg<TSSSR_SPEC>`
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
///Timestamp sub second register
pub mod tsssr;
///ALRMAR register accessor: an alias for `Reg<ALRMAR_SPEC>`
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
///Alarm A register
pub mod alrmar;
///ALRMASSR register accessor: an alias for `Reg<ALRMASSR_SPEC>`
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
///Alarm A sub second register
pub mod alrmassr;
///ALRMBR register accessor: an alias for `Reg<ALRMBR_SPEC>`
pub type ALRMBR = crate::Reg<alrmbr::ALRMBR_SPEC>;
///Alarm B register
pub mod alrmbr;
///ALRMBSSR register accessor: an alias for `Reg<ALRMBSSR_SPEC>`
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSR_SPEC>;
///Alarm B sub second register
pub mod alrmbssr;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register (interrupts)
pub mod sr;
///MISR register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///Masked interrupt status register
pub mod misr;
///SCR register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///Status clear register (interrupts)
pub mod scr;
///ALRABINR register accessor: an alias for `Reg<ALRABINR_SPEC>`
pub type ALRABINR = crate::Reg<alrabinr::ALRABINR_SPEC>;
///RTC alarm A binary mode register
pub mod alrabinr;
///ALRBBINR register accessor: an alias for `Reg<ALRBBINR_SPEC>`
pub type ALRBBINR = crate::Reg<alrbbinr::ALRBBINR_SPEC>;
///RTC alarm B binary mode register
pub mod alrbbinr;
