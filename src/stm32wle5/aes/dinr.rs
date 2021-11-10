///Register `DINR` reader
pub struct R(crate::R<DINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DINR` writer
pub struct W(crate::W<DINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DINR_SPEC>;
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
impl From<crate::W<DINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DINR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIN` reader - Data Input Register
pub struct DIN_R(crate::FieldReader<u32, u32>);
impl DIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        DIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DIN` writer - Data Input Register
pub struct DIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Data Input Register
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Data Input Register
    #[inline(always)]
    pub fn din(&mut self) -> DIN_W {
        DIN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr](index.html) module
pub struct DINR_SPEC;
impl crate::RegisterSpec for DINR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr::R](R) reader structure
impl crate::Readable for DINR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dinr::W](W) writer structure
impl crate::Writable for DINR_SPEC {
    type Writer = W;
}
///`reset()` method sets DINR to value 0
impl crate::Resettable for DINR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
