///Register `CCR3` reader
pub struct R(crate::R<CCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR3` writer
pub struct W(crate::W<CCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR3_SPEC>;
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
impl From<crate::W<CCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCR3` reader - Capture/Compare 3 value
pub struct CCR3_R(crate::FieldReader<u32, u32>);
impl CCR3_R {
    pub(crate) fn new(bits: u32) -> Self {
        CCR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCR3` writer - Capture/Compare 3 value
pub struct CCR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Capture/Compare 3 value
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Capture/Compare 3 value
    #[inline(always)]
    pub fn ccr3(&mut self) -> CCR3_W {
        CCR3_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr3](index.html) module
pub struct CCR3_SPEC;
impl crate::RegisterSpec for CCR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr3::R](R) reader structure
impl crate::Readable for CCR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr3::W](W) writer structure
impl crate::Writable for CCR3_SPEC {
    type Writer = W;
}
///`reset()` method sets CCR3 to value 0
impl crate::Resettable for CCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
