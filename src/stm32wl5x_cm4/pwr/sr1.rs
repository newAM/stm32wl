///Register `SR1` reader
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
///Internal wakeup interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUFI_A {
    ///0: All internal wakeup sources are cleared
    CLEAR = 0,
    ///1: wakeup is detected on the internal wakeup line
    WAKEUP = 1,
}
impl From<WUFI_A> for bool {
    #[inline(always)]
    fn from(variant: WUFI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUFI` reader - Internal wakeup interrupt flag
pub struct WUFI_R(crate::FieldReader<bool, WUFI_A>);
impl WUFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUFI_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUFI_A {
        match self.bits {
            false => WUFI_A::CLEAR,
            true => WUFI_A::WAKEUP,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == WUFI_A::CLEAR
    }
    ///Checks if the value of the field is `WAKEUP`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        **self == WUFI_A::WAKEUP
    }
}
impl core::ops::Deref for WUFI_R {
    type Target = crate::FieldReader<bool, WUFI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `C2HF` reader - PU2 Hold interrupt flag
pub struct C2HF_R(crate::FieldReader<bool, bool>);
impl C2HF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2HF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2HF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Radio BUSY wakeup flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRFBUSYF_A {
    ///0: No wakeup event detected on radio busy
    CLEAR = 0,
    ///1: Wakeup event detected on radio busy
    WAKEUP = 1,
}
impl From<WRFBUSYF_A> for bool {
    #[inline(always)]
    fn from(variant: WRFBUSYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WRFBUSYF` reader - Radio BUSY wakeup flag
pub struct WRFBUSYF_R(crate::FieldReader<bool, WRFBUSYF_A>);
impl WRFBUSYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRFBUSYF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WRFBUSYF_A {
        match self.bits {
            false => WRFBUSYF_A::CLEAR,
            true => WRFBUSYF_A::WAKEUP,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == WRFBUSYF_A::CLEAR
    }
    ///Checks if the value of the field is `WAKEUP`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        **self == WRFBUSYF_A::WAKEUP
    }
}
impl core::ops::Deref for WRFBUSYF_R {
    type Target = crate::FieldReader<bool, WRFBUSYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Wakeup PVD flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPVDF_A {
    ///0: No wakeup event detected on PVD
    CLEAR = 0,
    ///1: Wakeup event detected on PVD
    WAKEUP = 1,
}
impl From<WPVDF_A> for bool {
    #[inline(always)]
    fn from(variant: WPVDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WPVDF` reader - Wakeup PVD flag
pub struct WPVDF_R(crate::FieldReader<bool, WPVDF_A>);
impl WPVDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPVDF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WPVDF_A {
        match self.bits {
            false => WPVDF_A::CLEAR,
            true => WPVDF_A::WAKEUP,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == WPVDF_A::CLEAR
    }
    ///Checks if the value of the field is `WAKEUP`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        **self == WPVDF_A::WAKEUP
    }
}
impl core::ops::Deref for WPVDF_R {
    type Target = crate::FieldReader<bool, WPVDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Wakeup flag 3
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF3_A {
    ///0: No wakeup event detected on WKUP3
    CLEAR = 0,
    ///1: Wakeup event detected on WKUP3
    WAKEUP = 1,
}
impl From<WUF3_A> for bool {
    #[inline(always)]
    fn from(variant: WUF3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF3` reader - Wakeup flag 3
pub struct WUF3_R(crate::FieldReader<bool, WUF3_A>);
impl WUF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUF3_A {
        match self.bits {
            false => WUF3_A::CLEAR,
            true => WUF3_A::WAKEUP,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == WUF3_A::CLEAR
    }
    ///Checks if the value of the field is `WAKEUP`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        **self == WUF3_A::WAKEUP
    }
}
impl core::ops::Deref for WUF3_R {
    type Target = crate::FieldReader<bool, WUF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Wakeup flag 2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF2_A {
    ///0: No wakeup event detected on WKUP2
    CLEAR = 0,
    ///1: Wakeup event detected on WKUP2
    WAKEUP = 1,
}
impl From<WUF2_A> for bool {
    #[inline(always)]
    fn from(variant: WUF2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF2` reader - Wakeup flag 2
pub struct WUF2_R(crate::FieldReader<bool, WUF2_A>);
impl WUF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUF2_A {
        match self.bits {
            false => WUF2_A::CLEAR,
            true => WUF2_A::WAKEUP,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == WUF2_A::CLEAR
    }
    ///Checks if the value of the field is `WAKEUP`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        **self == WUF2_A::WAKEUP
    }
}
impl core::ops::Deref for WUF2_R {
    type Target = crate::FieldReader<bool, WUF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Wakeup flag 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF1_A {
    ///0: No wakeup event detected on WKUP1
    CLEAR = 0,
    ///1: Wakeup event detected on WKUP1
    WAKEUP = 1,
}
impl From<WUF1_A> for bool {
    #[inline(always)]
    fn from(variant: WUF1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF1` reader - Wakeup flag 1
pub struct WUF1_R(crate::FieldReader<bool, WUF1_A>);
impl WUF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUF1_A {
        match self.bits {
            false => WUF1_A::CLEAR,
            true => WUF1_A::WAKEUP,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == WUF1_A::CLEAR
    }
    ///Checks if the value of the field is `WAKEUP`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        **self == WUF1_A::WAKEUP
    }
}
impl core::ops::Deref for WUF1_R {
    type Target = crate::FieldReader<bool, WUF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 15 - Internal wakeup interrupt flag
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - PU2 Hold interrupt flag
    #[inline(always)]
    pub fn c2hf(&self) -> C2HF_R {
        C2HF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 11 - Radio BUSY wakeup flag
    #[inline(always)]
    pub fn wrfbusyf(&self) -> WRFBUSYF_R {
        WRFBUSYF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 8 - Wakeup PVD flag
    #[inline(always)]
    pub fn wpvdf(&self) -> WPVDF_R {
        WPVDF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 2 - Wakeup flag 3
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Wakeup flag 2
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Wakeup flag 1
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 0x01) != 0)
    }
}
///Power status register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr1](index.html) module
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr1::R](R) reader structure
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
