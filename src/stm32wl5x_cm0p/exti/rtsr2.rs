///Register `RTSR2` reader
pub struct R(crate::R<RTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RTSR2` writer
pub struct W(crate::W<RTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR2_SPEC>;
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
impl From<crate::W<RTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR2_SPEC>) -> Self {
        W(writer)
    }
}
///Rising trigger event configuration bit of Configurable Event input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT34_A {
    ///0: Rising edge trigger is disabled
    DISABLED = 0,
    ///1: Rising edge trigger is enabled
    ENABLED = 1,
}
impl From<RT34_A> for bool {
    #[inline(always)]
    fn from(variant: RT34_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RT34` reader - Rising trigger event configuration bit of Configurable Event input
pub struct RT34_R(crate::FieldReader<bool, RT34_A>);
impl RT34_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT34_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RT34_A {
        match self.bits {
            false => RT34_A::DISABLED,
            true => RT34_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RT34_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RT34_A::ENABLED
    }
}
impl core::ops::Deref for RT34_R {
    type Target = crate::FieldReader<bool, RT34_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RT34` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT34_W<'a> {
    w: &'a mut W,
}
impl<'a> RT34_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT34_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT34_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT34_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Rising trigger event configuration bit of Configurable Event input
pub type RT40_A = RT34_A;
///Field `RT40` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT40_R = RT34_R;
///Field `RT40` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT40_W<'a> {
    w: &'a mut W,
}
impl<'a> RT40_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT40_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT40_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT40_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Rising trigger event configuration bit of Configurable Event input
pub type RT41_A = RT34_A;
///Field `RT41` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT41_R = RT34_R;
///Field `RT41` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT41_W<'a> {
    w: &'a mut W,
}
impl<'a> RT41_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT41_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT41_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT41_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Rising trigger event configuration bit of Configurable Event input
pub type RT45_A = RT34_A;
///Field `RT45` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT45_R = RT34_R;
///Field `RT45` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT45_W<'a> {
    w: &'a mut W,
}
impl<'a> RT45_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT45_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT45_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT45_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt34(&self) -> RT34_R {
        RT34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt40(&self) -> RT40_R {
        RT40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt41(&self) -> RT41_R {
        RT41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt45(&self) -> RT45_R {
        RT45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt34(&mut self) -> RT34_W {
        RT34_W { w: self }
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt40(&mut self) -> RT40_W {
        RT40_W { w: self }
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt41(&mut self) -> RT41_W {
        RT41_W { w: self }
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt45(&mut self) -> RT45_W {
        RT45_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///rising trigger selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtsr2](index.html) module
pub struct RTSR2_SPEC;
impl crate::RegisterSpec for RTSR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rtsr2::R](R) reader structure
impl crate::Readable for RTSR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rtsr2::W](W) writer structure
impl crate::Writable for RTSR2_SPEC {
    type Writer = W;
}
///`reset()` method sets RTSR2 to value 0
impl crate::Resettable for RTSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}