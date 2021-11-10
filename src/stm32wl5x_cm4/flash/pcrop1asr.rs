///Register `PCROP1ASR` reader
pub struct R(crate::R<PCROP1ASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1ASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1ASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1ASR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCROP1ASR` writer
pub struct W(crate::W<PCROP1ASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1ASR_SPEC>;
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
impl From<crate::W<PCROP1ASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1ASR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCROP1A_STRT` reader - PCROP1A area start offset
pub struct PCROP1A_STRT_R(crate::FieldReader<u8, u8>);
impl PCROP1A_STRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCROP1A_STRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP1A_STRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PCROP1A_STRT` writer - PCROP1A area start offset
pub struct PCROP1A_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1A_STRT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - PCROP1A area start offset
    #[inline(always)]
    pub fn pcrop1a_strt(&self) -> PCROP1A_STRT_R {
        PCROP1A_STRT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - PCROP1A area start offset
    #[inline(always)]
    pub fn pcrop1a_strt(&mut self) -> PCROP1A_STRT_W {
        PCROP1A_STRT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash PCROP zone A Start address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop1asr](index.html) module
pub struct PCROP1ASR_SPEC;
impl crate::RegisterSpec for PCROP1ASR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcrop1asr::R](R) reader structure
impl crate::Readable for PCROP1ASR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcrop1asr::W](W) writer structure
impl crate::Writable for PCROP1ASR_SPEC {
    type Writer = W;
}
///`reset()` method sets PCROP1ASR to value 0xffff_ffff
impl crate::Resettable for PCROP1ASR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}