///Register `ALRABINR` reader
pub struct R(crate::R<ALRABINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRABINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRABINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRABINR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ALRABINR` writer
pub struct W(crate::W<ALRABINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRABINR_SPEC>;
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
impl From<crate::W<ALRABINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRABINR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SS` reader - Synchronous counter alarm value in Binary mode
pub struct SS_R(crate::FieldReader<u32, u32>);
impl SS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SS` writer - Synchronous counter alarm value in Binary mode
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Synchronous counter alarm value in Binary mode
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Synchronous counter alarm value in Binary mode
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
///RTC alarm A binary mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrabinr](index.html) module
pub struct ALRABINR_SPEC;
impl crate::RegisterSpec for ALRABINR_SPEC {
    type Ux = u32;
}
///`read()` method returns [alrabinr::R](R) reader structure
impl crate::Readable for ALRABINR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [alrabinr::W](W) writer structure
impl crate::Writable for ALRABINR_SPEC {
    type Writer = W;
}
///`reset()` method sets ALRABINR to value 0
impl crate::Resettable for ALRABINR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}