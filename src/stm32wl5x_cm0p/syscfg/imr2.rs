///Register `IMR2` reader
pub struct R(crate::R<IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IMR2` writer
pub struct W(crate::W<IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR2_SPEC>;
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
impl From<crate::W<IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PVM3IM` reader - PVM3IM
pub struct PVM3IM_R(crate::FieldReader<bool, bool>);
impl PVM3IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVM3IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVM3IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PVM3IM` writer - PVM3IM
pub struct PVM3IM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVM3IM_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///Field `PVDIM` reader - PVDIM
pub struct PVDIM_R(crate::FieldReader<bool, bool>);
impl PVDIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PVDIM` writer - PVDIM
pub struct PVDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDIM_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    ///Bit 18 - PVM3IM
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 20 - PVDIM
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    ///Bit 18 - PVM3IM
    #[inline(always)]
    pub fn pvm3im(&mut self) -> PVM3IM_W {
        PVM3IM_W { w: self }
    }
    ///Bit 20 - PVDIM
    #[inline(always)]
    pub fn pvdim(&mut self) -> PVDIM_W {
        PVDIM_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SYSCFG CPU1 interrupt mask register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imr2](index.html) module
pub struct IMR2_SPEC;
impl crate::RegisterSpec for IMR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [imr2::R](R) reader structure
impl crate::Readable for IMR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [imr2::W](W) writer structure
impl crate::Writable for IMR2_SPEC {
    type Writer = W;
}
///`reset()` method sets IMR2 to value 0
impl crate::Resettable for IMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}