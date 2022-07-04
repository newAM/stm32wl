#[doc = "Register `IVR3` reader"]
pub struct R(crate::R<IVR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IVR3` writer"]
pub struct W(crate::W<IVR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR3_SPEC>;
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
impl From<crate::W<IVR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IVI` reader - Initialization Vector Register (MSB IVR \\[127:96\\])"]
pub type IVI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IVI` writer - Initialization Vector Register (MSB IVR \\[127:96\\])"]
pub type IVI_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IVR3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register (MSB IVR \\[127:96\\])"]
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register (MSB IVR \\[127:96\\])"]
    #[inline(always)]
    pub fn ivi(&mut self) -> IVI_W<0> {
        IVI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "initialization vector register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr3](index.html) module"]
pub struct IVR3_SPEC;
impl crate::RegisterSpec for IVR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ivr3::R](R) reader structure"]
impl crate::Readable for IVR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ivr3::W](W) writer structure"]
impl crate::Writable for IVR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IVR3 to value 0"]
impl crate::Resettable for IVR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
