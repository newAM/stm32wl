///Register `RCR` reader
pub struct R(crate::R<RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCR` writer
pub struct W(crate::W<RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR_SPEC>;
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
impl From<crate::W<RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REP` reader - Repetition counter value
pub struct REP_R(crate::FieldReader<u16, u16>);
impl REP_R {
    pub(crate) fn new(bits: u16) -> Self {
        REP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REP` writer - Repetition counter value
pub struct REP_W<'a> {
    w: &'a mut W,
}
impl<'a> REP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Repetition counter value
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Repetition counter value
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W {
        REP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///repetition counter register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcr](index.html) module
pub struct RCR_SPEC;
impl crate::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcr::R](R) reader structure
impl crate::Readable for RCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcr::W](W) writer structure
impl crate::Writable for RCR_SPEC {
    type Writer = W;
}
///`reset()` method sets RCR to value 0
impl crate::Resettable for RCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
