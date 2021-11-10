///Register `CIFR` reader
pub struct R(crate::R<CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIFR_SPEC>) -> Self {
        R(reader)
    }
}
///LSE Clock security system interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSECSSF_A {
    ///0: Not interrupted
    NOTINTERRUPTED = 0,
    ///1: Interrupted
    INTERRUPTED = 1,
}
impl From<LSECSSF_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSF` reader - LSE Clock security system interrupt flag
pub struct LSECSSF_R(crate::FieldReader<bool, LSECSSF_A>);
impl LSECSSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSECSSF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSECSSF_A {
        match self.bits {
            false => LSECSSF_A::NOTINTERRUPTED,
            true => LSECSSF_A::INTERRUPTED,
        }
    }
    ///Checks if the value of the field is `NOTINTERRUPTED`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        **self == LSECSSF_A::NOTINTERRUPTED
    }
    ///Checks if the value of the field is `INTERRUPTED`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        **self == LSECSSF_A::INTERRUPTED
    }
}
impl core::ops::Deref for LSECSSF_R {
    type Target = crate::FieldReader<bool, LSECSSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///HSE32 Clock security system interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSF_A {
    ///0: Not interrupted
    NOTINTERRUPTED = 0,
    ///1: Interrupted
    INTERRUPTED = 1,
}
impl From<CSSF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSF` reader - HSE32 Clock security system interrupt flag
pub struct CSSF_R(crate::FieldReader<bool, CSSF_A>);
impl CSSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSF_A {
        match self.bits {
            false => CSSF_A::NOTINTERRUPTED,
            true => CSSF_A::INTERRUPTED,
        }
    }
    ///Checks if the value of the field is `NOTINTERRUPTED`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        **self == CSSF_A::NOTINTERRUPTED
    }
    ///Checks if the value of the field is `INTERRUPTED`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        **self == CSSF_A::INTERRUPTED
    }
}
impl core::ops::Deref for CSSF_R {
    type Target = crate::FieldReader<bool, CSSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///PLL ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLRDYF_A {
    ///0: Not interrupted
    NOTINTERRUPTED = 0,
    ///1: Interrupted
    INTERRUPTED = 1,
}
impl From<PLLRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub struct PLLRDYF_R(crate::FieldReader<bool, PLLRDYF_A>);
impl PLLRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLRDYF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLRDYF_A {
        match self.bits {
            false => PLLRDYF_A::NOTINTERRUPTED,
            true => PLLRDYF_A::INTERRUPTED,
        }
    }
    ///Checks if the value of the field is `NOTINTERRUPTED`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        **self == PLLRDYF_A::NOTINTERRUPTED
    }
    ///Checks if the value of the field is `INTERRUPTED`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        **self == PLLRDYF_A::INTERRUPTED
    }
}
impl core::ops::Deref for PLLRDYF_R {
    type Target = crate::FieldReader<bool, PLLRDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///HSE32 ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSERDYF_A {
    ///0: Not interrupted
    NOTINTERRUPTED = 0,
    ///1: Interrupted
    INTERRUPTED = 1,
}
impl From<HSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYF` reader - HSE32 ready interrupt flag
pub struct HSERDYF_R(crate::FieldReader<bool, HSERDYF_A>);
impl HSERDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSERDYF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSERDYF_A {
        match self.bits {
            false => HSERDYF_A::NOTINTERRUPTED,
            true => HSERDYF_A::INTERRUPTED,
        }
    }
    ///Checks if the value of the field is `NOTINTERRUPTED`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        **self == HSERDYF_A::NOTINTERRUPTED
    }
    ///Checks if the value of the field is `INTERRUPTED`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        **self == HSERDYF_A::INTERRUPTED
    }
}
impl core::ops::Deref for HSERDYF_R {
    type Target = crate::FieldReader<bool, HSERDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///HSI16 ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDYF_A {
    ///0: Not interrupted
    NOTINTERRUPTED = 0,
    ///1: Interrupted
    INTERRUPTED = 1,
}
impl From<HSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYF` reader - HSI16 ready interrupt flag
pub struct HSIRDYF_R(crate::FieldReader<bool, HSIRDYF_A>);
impl HSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDYF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYF_A {
        match self.bits {
            false => HSIRDYF_A::NOTINTERRUPTED,
            true => HSIRDYF_A::INTERRUPTED,
        }
    }
    ///Checks if the value of the field is `NOTINTERRUPTED`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        **self == HSIRDYF_A::NOTINTERRUPTED
    }
    ///Checks if the value of the field is `INTERRUPTED`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        **self == HSIRDYF_A::INTERRUPTED
    }
}
impl core::ops::Deref for HSIRDYF_R {
    type Target = crate::FieldReader<bool, HSIRDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///MSI ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIRDYF_A {
    ///0: Not interrupted
    NOTINTERRUPTED = 0,
    ///1: Interrupted
    INTERRUPTED = 1,
}
impl From<MSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub struct MSIRDYF_R(crate::FieldReader<bool, MSIRDYF_A>);
impl MSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSIRDYF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSIRDYF_A {
        match self.bits {
            false => MSIRDYF_A::NOTINTERRUPTED,
            true => MSIRDYF_A::INTERRUPTED,
        }
    }
    ///Checks if the value of the field is `NOTINTERRUPTED`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        **self == MSIRDYF_A::NOTINTERRUPTED
    }
    ///Checks if the value of the field is `INTERRUPTED`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        **self == MSIRDYF_A::INTERRUPTED
    }
}
impl core::ops::Deref for MSIRDYF_R {
    type Target = crate::FieldReader<bool, MSIRDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///LSE ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDYF_A {
    ///0: Not interrupted
    NOTINTERRUPTED = 0,
    ///1: Interrupted
    INTERRUPTED = 1,
}
impl From<LSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDYF` reader - LSE ready interrupt flag
pub struct LSERDYF_R(crate::FieldReader<bool, LSERDYF_A>);
impl LSERDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSERDYF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSERDYF_A {
        match self.bits {
            false => LSERDYF_A::NOTINTERRUPTED,
            true => LSERDYF_A::INTERRUPTED,
        }
    }
    ///Checks if the value of the field is `NOTINTERRUPTED`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        **self == LSERDYF_A::NOTINTERRUPTED
    }
    ///Checks if the value of the field is `INTERRUPTED`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        **self == LSERDYF_A::INTERRUPTED
    }
}
impl core::ops::Deref for LSERDYF_R {
    type Target = crate::FieldReader<bool, LSERDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///LSI ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYF_A {
    ///0: Not interrupted
    NOTINTERRUPTED = 0,
    ///1: Interrupted
    INTERRUPTED = 1,
}
impl From<LSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYF` reader - LSI ready interrupt flag
pub struct LSIRDYF_R(crate::FieldReader<bool, LSIRDYF_A>);
impl LSIRDYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYF_A {
        match self.bits {
            false => LSIRDYF_A::NOTINTERRUPTED,
            true => LSIRDYF_A::INTERRUPTED,
        }
    }
    ///Checks if the value of the field is `NOTINTERRUPTED`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        **self == LSIRDYF_A::NOTINTERRUPTED
    }
    ///Checks if the value of the field is `INTERRUPTED`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        **self == LSIRDYF_A::INTERRUPTED
    }
}
impl core::ops::Deref for LSIRDYF_R {
    type Target = crate::FieldReader<bool, LSIRDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 9 - LSE Clock security system interrupt flag
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - HSE32 Clock security system interrupt flag
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 5 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - HSE32 ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - HSI16 ready interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
}
///Clock interrupt flag register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cifr](index.html) module
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cifr::R](R) reader structure
impl crate::Readable for CIFR_SPEC {
    type Reader = R;
}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
