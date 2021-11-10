///Register `PCROP1BSR` reader
pub struct R(crate::R<PCROP1BSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1BSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1BSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1BSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCROP1BSR` writer
pub struct W(crate::W<PCROP1BSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1BSR_SPEC>;
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
impl From<crate::W<PCROP1BSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1BSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCROP1B_STRT` reader - Bank 1 WRP second area B end offset
pub struct PCROP1B_STRT_R(crate::FieldReader<u8, u8>);
impl PCROP1B_STRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCROP1B_STRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP1B_STRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PCROP1B_STRT` writer - Bank 1 WRP second area B end offset
pub struct PCROP1B_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1B_STRT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Bank 1 WRP second area B end offset
    #[inline(always)]
    pub fn pcrop1b_strt(&self) -> PCROP1B_STRT_R {
        PCROP1B_STRT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Bank 1 WRP second area B end offset
    #[inline(always)]
    pub fn pcrop1b_strt(&mut self) -> PCROP1B_STRT_W {
        PCROP1B_STRT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash PCROP zone B Start address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop1bsr](index.html) module
pub struct PCROP1BSR_SPEC;
impl crate::RegisterSpec for PCROP1BSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcrop1bsr::R](R) reader structure
impl crate::Readable for PCROP1BSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcrop1bsr::W](W) writer structure
impl crate::Writable for PCROP1BSR_SPEC {
    type Writer = W;
}
///`reset()` method sets PCROP1BSR to value 0xffff_ffff
impl crate::Resettable for PCROP1BSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}