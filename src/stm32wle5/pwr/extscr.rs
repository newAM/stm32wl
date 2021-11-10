///Register `EXTSCR` reader
pub struct R(crate::R<EXTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTSCR` writer
pub struct W(crate::W<EXTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EXTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTSCR_SPEC>) -> Self {
        W(writer)
    }
}
///CPU1 deepsleep mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1DS_A {
    ///0: CPU is running or in sleep
    RUNNINGORSLEEP = 0,
    ///1: CPU is in Deep-Sleep
    DEEPSLEEP = 1,
}
impl From<C1DS_A> for bool {
    #[inline(always)]
    fn from(variant: C1DS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1DS` reader - CPU1 deepsleep mode
pub struct C1DS_R(crate::FieldReader<bool, C1DS_A>);
impl C1DS_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1DS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> C1DS_A {
        match self.bits {
            false => C1DS_A::RUNNINGORSLEEP,
            true => C1DS_A::DEEPSLEEP,
        }
    }
    ///Checks if the value of the field is `RUNNINGORSLEEP`
    #[inline(always)]
    pub fn is_running_or_sleep(&self) -> bool {
        **self == C1DS_A::RUNNINGORSLEEP
    }
    ///Checks if the value of the field is `DEEPSLEEP`
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        **self == C1DS_A::DEEPSLEEP
    }
}
impl core::ops::Deref for C1DS_R {
    type Target = crate::FieldReader<bool, C1DS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///System Stop0, 1 flag for CPU1. (All core states retained)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1STOPF_A {
    ///0: System has not been in Stop 0 or 1 mode
    NOSTOP = 0,
    ///1: System has been in Stop 0 or 1 mode
    STOP = 1,
}
impl From<C1STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: C1STOPF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1STOPF` reader - System Stop0, 1 flag for CPU1. (All core states retained)
pub struct C1STOPF_R(crate::FieldReader<bool, C1STOPF_A>);
impl C1STOPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1STOPF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> C1STOPF_A {
        match self.bits {
            false => C1STOPF_A::NOSTOP,
            true => C1STOPF_A::STOP,
        }
    }
    ///Checks if the value of the field is `NOSTOP`
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        **self == C1STOPF_A::NOSTOP
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == C1STOPF_A::STOP
    }
}
impl core::ops::Deref for C1STOPF_R {
    type Target = crate::FieldReader<bool, C1STOPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///System Stop2 flag for CPU1. (partial core states retained)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1STOP2F_A {
    ///0: System has not been in Stop 2 mode
    NOSTOP = 0,
    ///1: System has been in Stop 2 mode
    STOP = 1,
}
impl From<C1STOP2F_A> for bool {
    #[inline(always)]
    fn from(variant: C1STOP2F_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1STOP2F` reader - System Stop2 flag for CPU1. (partial core states retained)
pub struct C1STOP2F_R(crate::FieldReader<bool, C1STOP2F_A>);
impl C1STOP2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1STOP2F_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> C1STOP2F_A {
        match self.bits {
            false => C1STOP2F_A::NOSTOP,
            true => C1STOP2F_A::STOP,
        }
    }
    ///Checks if the value of the field is `NOSTOP`
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        **self == C1STOP2F_A::NOSTOP
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == C1STOP2F_A::STOP
    }
}
impl core::ops::Deref for C1STOP2F_R {
    type Target = crate::FieldReader<bool, C1STOP2F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///System Standby flag for CPU1. (no core states retained)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1SBF_A {
    ///0: System has not been in Standby mode
    NOSTANDBY = 0,
    ///1: System has been in Standby mode
    STANDBY = 1,
}
impl From<C1SBF_A> for bool {
    #[inline(always)]
    fn from(variant: C1SBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1SBF` reader - System Standby flag for CPU1. (no core states retained)
pub struct C1SBF_R(crate::FieldReader<bool, C1SBF_A>);
impl C1SBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1SBF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> C1SBF_A {
        match self.bits {
            false => C1SBF_A::NOSTANDBY,
            true => C1SBF_A::STANDBY,
        }
    }
    ///Checks if the value of the field is `NOSTANDBY`
    #[inline(always)]
    pub fn is_no_standby(&self) -> bool {
        **self == C1SBF_A::NOSTANDBY
    }
    ///Checks if the value of the field is `STANDBY`
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        **self == C1SBF_A::STANDBY
    }
}
impl core::ops::Deref for C1SBF_R {
    type Target = crate::FieldReader<bool, C1SBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Clear CPU1 Stop Standby flags
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1CSSF_AW {
    ///1: Setting this bit clears the C1STOPF and C1SBF bits
    CLEAR = 1,
}
impl From<C1CSSF_AW> for bool {
    #[inline(always)]
    fn from(variant: C1CSSF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `C1CSSF` writer - Clear CPU1 Stop Standby flags
pub struct C1CSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> C1CSSF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: C1CSSF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Setting this bit clears the C1STOPF and C1SBF bits
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(C1CSSF_AW::CLEAR)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    ///Bit 14 - CPU1 deepsleep mode
    #[inline(always)]
    pub fn c1ds(&self) -> C1DS_R {
        C1DS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 10 - System Stop0, 1 flag for CPU1. (All core states retained)
    #[inline(always)]
    pub fn c1stopf(&self) -> C1STOPF_R {
        C1STOPF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - System Stop2 flag for CPU1. (partial core states retained)
    #[inline(always)]
    pub fn c1stop2f(&self) -> C1STOP2F_R {
        C1STOP2F_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - System Standby flag for CPU1. (no core states retained)
    #[inline(always)]
    pub fn c1sbf(&self) -> C1SBF_R {
        C1SBF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Clear CPU1 Stop Standby flags
    #[inline(always)]
    pub fn c1cssf(&mut self) -> C1CSSF_W {
        C1CSSF_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power extended status and status clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [extscr](index.html) module
pub struct EXTSCR_SPEC;
impl crate::RegisterSpec for EXTSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [extscr::R](R) reader structure
impl crate::Readable for EXTSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [extscr::W](W) writer structure
impl crate::Writable for EXTSCR_SPEC {
    type Writer = W;
}
///`reset()` method sets EXTSCR to value 0
impl crate::Resettable for EXTSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}