///Register `CNDTR2` reader
pub struct R(crate::R<CNDTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDTR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CNDTR2` writer
pub struct W(crate::W<CNDTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDTR2_SPEC>;
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
impl From<crate::W<CNDTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDTR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NDT` reader - number of data to transfer (0 to 218 - 1)
pub struct NDT_R(crate::FieldReader<u32, u32>);
impl NDT_R {
    pub(crate) fn new(bits: u32) -> Self {
        NDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NDT` writer - number of data to transfer (0 to 218 - 1)
pub struct NDT_W<'a> {
    w: &'a mut W,
}
impl<'a> NDT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:17 - number of data to transfer (0 to 218 - 1)
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    ///Bits 0:17 - number of data to transfer (0 to 218 - 1)
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W {
        NDT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel x number of data to transfer register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cndtr2](index.html) module
pub struct CNDTR2_SPEC;
impl crate::RegisterSpec for CNDTR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cndtr2::R](R) reader structure
impl crate::Readable for CNDTR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cndtr2::W](W) writer structure
impl crate::Writable for CNDTR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CNDTR2 to value 0
impl crate::Resettable for CNDTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
