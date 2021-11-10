///Register `CMAR1` reader
pub struct R(crate::R<CMAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMAR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CMAR1` writer
pub struct W(crate::W<CMAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMAR1_SPEC>;
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
impl From<crate::W<CMAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMAR1_SPEC>) -> Self {
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
///For information about available fields see [cmar1](index.html) module
pub struct CMAR1_SPEC;
impl crate::RegisterSpec for CMAR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cmar1::R](R) reader structure
impl crate::Readable for CMAR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cmar1::W](W) writer structure
impl crate::Writable for CMAR1_SPEC {
    type Writer = W;
}
///`reset()` method sets CMAR1 to value 0
impl crate::Resettable for CMAR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}