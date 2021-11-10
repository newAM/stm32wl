///Register `CMAR2` reader
pub struct R(crate::R<CMAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMAR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CMAR2` writer
pub struct W(crate::W<CMAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMAR2_SPEC>;
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
impl From<crate::W<CMAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMAR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MA` reader - peripheral address
pub struct MA_R(crate::FieldReader<u32, u32>);
impl MA_R {
    pub(crate) fn new(bits: u32) -> Self {
        MA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MA` writer - peripheral address
pub struct MA_W<'a> {
    w: &'a mut W,
}
impl<'a> MA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W {
        MA_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmar2](index.html) module
pub struct CMAR2_SPEC;
impl crate::RegisterSpec for CMAR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cmar2::R](R) reader structure
impl crate::Readable for CMAR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cmar2::W](W) writer structure
impl crate::Writable for CMAR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CMAR2 to value 0
impl crate::Resettable for CMAR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}