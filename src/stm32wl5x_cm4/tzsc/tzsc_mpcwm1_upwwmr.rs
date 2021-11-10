///Register `TZSC_MPCWM1_UPWWMR` reader
pub struct R(crate::R<TZSC_MPCWM1_UPWWMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_MPCWM1_UPWWMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_MPCWM1_UPWWMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_MPCWM1_UPWWMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_MPCWM1_UPWWMR` writer
pub struct W(crate::W<TZSC_MPCWM1_UPWWMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_MPCWM1_UPWWMR_SPEC>;
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
impl From<crate::W<TZSC_MPCWM1_UPWWMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_MPCWM1_UPWWMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LGTH` reader - Define the length of Flash Unprivileged Writable area, in 2
pub struct LGTH_R(crate::FieldReader<u16, u16>);
impl LGTH_R {
    pub(crate) fn new(bits: u16) -> Self {
        LGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LGTH` writer - Define the length of Flash Unprivileged Writable area, in 2
pub struct LGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LGTH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    ///Bits 16:27 - Define the length of Flash Unprivileged Writable area, in 2
    #[inline(always)]
    pub fn lgth(&self) -> LGTH_R {
        LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 16:27 - Define the length of Flash Unprivileged Writable area, in 2
    #[inline(always)]
    pub fn lgth(&mut self) -> LGTH_W {
        LGTH_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Unprivileged Writable Water Mark 1 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_mpcwm1_upwwmr](index.html) module
pub struct TZSC_MPCWM1_UPWWMR_SPEC;
impl crate::RegisterSpec for TZSC_MPCWM1_UPWWMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_mpcwm1_upwwmr::R](R) reader structure
impl crate::Readable for TZSC_MPCWM1_UPWWMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_mpcwm1_upwwmr::W](W) writer structure
impl crate::Writable for TZSC_MPCWM1_UPWWMR_SPEC {
    type Writer = W;
}
///`reset()` method sets TZSC_MPCWM1_UPWWMR to value 0x0fff_0000
impl crate::Resettable for TZSC_MPCWM1_UPWWMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}
