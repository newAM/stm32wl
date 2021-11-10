///Register `CCR4` reader
pub struct R(crate::R<CCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR4` writer
pub struct W(crate::W<CCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR4_SPEC>;
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
impl From<crate::W<CCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCR4` reader - Capture/Compare 4 value
pub struct CCR4_R(crate::FieldReader<u32, u32>);
impl CCR4_R {
    pub(crate) fn new(bits: u32) -> Self {
        CCR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCR4` writer - Capture/Compare 4 value
pub struct CCR4_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR4_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Capture/Compare 4 value
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Capture/Compare 4 value
    #[inline(always)]
    pub fn ccr4(&mut self) -> CCR4_W {
        CCR4_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr4](index.html) module
pub struct CCR4_SPEC;
impl crate::RegisterSpec for CCR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr4::R](R) reader structure
impl crate::Readable for CCR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr4::W](W) writer structure
impl crate::Writable for CCR4_SPEC {
    type Writer = W;
}
///`reset()` method sets CCR4 to value 0
impl crate::Resettable for CCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}