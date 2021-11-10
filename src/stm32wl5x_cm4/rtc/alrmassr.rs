///Register `ALRMASSR` reader
pub struct R(crate::R<ALRMASSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRMASSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRMASSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRMASSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ALRMASSR` writer
pub struct W(crate::W<ALRMASSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMASSR_SPEC>;
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
impl From<crate::W<ALRMASSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMASSR_SPEC>) -> Self {
        W(writer)
    }
}
///Clear synchronous counter on alarm (Binary mode only)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSCLR_A {
    ///0: The synchronous binary counter (SS\[31:0\]
    ///in RTC_SSR) is free-running
    FREERUNNING = 0,
    ///1: The synchronous binary counter (SS\[31:0\]
    ///in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR → SS\[31:0\]
    ///value and is automatically reloaded with 0xFFFF FFFF when reaching RTC_ALRMABINR → SS\[31:0\]
    ALRMBINR = 1,
}
impl From<SSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: SSCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCLR` reader - Clear synchronous counter on alarm (Binary mode only)
pub struct SSCLR_R(crate::FieldReader<bool, SSCLR_A>);
impl SSCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSCLR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSCLR_A {
        match self.bits {
            false => SSCLR_A::FREERUNNING,
            true => SSCLR_A::ALRMBINR,
        }
    }
    ///Checks if the value of the field is `FREERUNNING`
    #[inline(always)]
    pub fn is_free_running(&self) -> bool {
        **self == SSCLR_A::FREERUNNING
    }
    ///Checks if the value of the field is `ALRMBINR`
    #[inline(always)]
    pub fn is_alrmbinr(&self) -> bool {
        **self == SSCLR_A::ALRMBINR
    }
}
impl core::ops::Deref for SSCLR_R {
    type Target = crate::FieldReader<bool, SSCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SSCLR` writer - Clear synchronous counter on alarm (Binary mode only)
pub struct SSCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSCLR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SSCLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The synchronous binary counter (SS\[31:0\]
    ///in RTC_SSR) is free-running
    #[inline(always)]
    pub fn free_running(self) -> &'a mut W {
        self.variant(SSCLR_A::FREERUNNING)
    }
    ///The synchronous binary counter (SS\[31:0\]
    ///in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR → SS\[31:0\]
    ///value and is automatically reloaded with 0xFFFF FFFF when reaching RTC_ALRMABINR → SS\[31:0\]
    #[inline(always)]
    pub fn alrmbinr(self) -> &'a mut W {
        self.variant(SSCLR_A::ALRMBINR)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
///Field `MASKSS` reader - Mask the most-significant bits starting at this bit
pub struct MASKSS_R(crate::FieldReader<u8, u8>);
impl MASKSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASKSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASKSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MASKSS` writer - Mask the most-significant bits starting at this bit
pub struct MASKSS_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKSS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
///Field `SS` reader - Sub seconds value
pub struct SS_R(crate::FieldReader<u16, u16>);
impl SS_R {
    pub(crate) fn new(bits: u16) -> Self {
        SS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SS` writer - Sub seconds value
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    ///Bit 31 - Clear synchronous counter on alarm (Binary mode only)
    #[inline(always)]
    pub fn ssclr(&self) -> SSCLR_R {
        SSCLR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bits 24:29 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bit 31 - Clear synchronous counter on alarm (Binary mode only)
    #[inline(always)]
    pub fn ssclr(&mut self) -> SSCLR_W {
        SSCLR_W { w: self }
    }
    ///Bits 24:29 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&mut self) -> MASKSS_W {
        MASKSS_W { w: self }
    }
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Alarm A sub second register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrmassr](index.html) module
pub struct ALRMASSR_SPEC;
impl crate::RegisterSpec for ALRMASSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [alrmassr::R](R) reader structure
impl crate::Readable for ALRMASSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [alrmassr::W](W) writer structure
impl crate::Writable for ALRMASSR_SPEC {
    type Writer = W;
}
///`reset()` method sets ALRMASSR to value 0
impl crate::Resettable for ALRMASSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
