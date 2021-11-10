///Register `SHHR` reader
pub struct R(crate::R<SHHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHHR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SHHR` writer
pub struct W(crate::W<SHHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHHR_SPEC>;
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
impl From<crate::W<SHHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHHR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `THOLD1` reader - DAC Channel 1 hold Time (only valid in Sample and Hold mode)
pub struct THOLD1_R(crate::FieldReader<u16, u16>);
impl THOLD1_R {
    pub(crate) fn new(bits: u16) -> Self {
        THOLD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THOLD1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `THOLD1` writer - DAC Channel 1 hold Time (only valid in Sample and Hold mode)
pub struct THOLD1_W<'a> {
    w: &'a mut W,
}
impl<'a> THOLD1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    ///Bits 0:9 - DAC Channel 1 hold Time (only valid in Sample and Hold mode)
    #[inline(always)]
    pub fn thold1(&self) -> THOLD1_R {
        THOLD1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - DAC Channel 1 hold Time (only valid in Sample and Hold mode)
    #[inline(always)]
    pub fn thold1(&mut self) -> THOLD1_W {
        THOLD1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Sample and Hold hold time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [shhr](index.html) module
pub struct SHHR_SPEC;
impl crate::RegisterSpec for SHHR_SPEC {
    type Ux = u32;
}
///`read()` method returns [shhr::R](R) reader structure
impl crate::Readable for SHHR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [shhr::W](W) writer structure
impl crate::Writable for SHHR_SPEC {
    type Writer = W;
}
///`reset()` method sets SHHR to value 0x0001_0001
impl crate::Resettable for SHHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0001
    }
}
