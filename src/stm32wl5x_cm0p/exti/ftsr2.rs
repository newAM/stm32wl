///Register `FTSR2` reader
pub struct R(crate::R<FTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FTSR2` writer
pub struct W(crate::W<FTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR2_SPEC>;
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
impl From<crate::W<FTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR2_SPEC>) -> Self {
        W(writer)
    }
}
///Falling trigger event configuration bit of Configurable Event input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FT34_A {
    ///0: Falling edge trigger is disabled
    DISABLED = 0,
    ///1: Falling edge trigger is enabled
    ENABLED = 1,
}
impl From<FT34_A> for bool {
    #[inline(always)]
    fn from(variant: FT34_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FT34` reader - Falling trigger event configuration bit of Configurable Event input
pub struct FT34_R(crate::FieldReader<bool, FT34_A>);
impl FT34_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT34_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FT34_A {
        match self.bits {
            false => FT34_A::DISABLED,
            true => FT34_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FT34_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FT34_A::ENABLED
    }
}
impl core::ops::Deref for FT34_R {
    type Target = crate::FieldReader<bool, FT34_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FT34` writer - Falling trigger event configuration bit of Configurable Event input
pub struct FT34_W<'a> {
    w: &'a mut W,
}
impl<'a> FT34_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT34_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT34_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT34_A::ENABLED)
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
///Falling trigger event configuration bit of Configurable Event input
pub type FT40_A = FT34_A;
///Field `FT40` reader - Falling trigger event configuration bit of Configurable Event input
pub type FT40_R = FT34_R;
///Field `FT40` writer - Falling trigger event configuration bit of Configurable Event input
pub struct FT40_W<'a> {
    w: &'a mut W,
}
impl<'a> FT40_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT40_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT40_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT40_A::ENABLED)
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
///Falling trigger event configuration bit of Configurable Event input
pub type FT41_A = FT34_A;
///Field `FT41` reader - Falling trigger event configuration bit of Configurable Event input
pub type FT41_R = FT34_R;
///Field `FT41` writer - Falling trigger event configuration bit of Configurable Event input
pub struct FT41_W<'a> {
    w: &'a mut W,
}
impl<'a> FT41_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT41_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT41_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT41_A::ENABLED)
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
///Falling trigger event configuration bit of Configurable Event input
pub type FT45_A = FT34_A;
///Field `FT45` reader - Falling trigger event configuration bit of Configurable Event input
pub type FT45_R = FT34_R;
///Field `FT45` writer - Falling trigger event configuration bit of Configurable Event input
pub struct FT45_W<'a> {
    w: &'a mut W,
}
impl<'a> FT45_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT45_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT45_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT45_A::ENABLED)
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
    ///Bit 2 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft34(&self) -> FT34_R {
        FT34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 8 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft40(&self) -> FT40_R {
        FT40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft41(&self) -> FT41_R {
        FT41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 13 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft45(&self) -> FT45_R {
        FT45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft34(&mut self) -> FT34_W {
        FT34_W { w: self }
    }
    ///Bit 8 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft40(&mut self) -> FT40_W {
        FT40_W { w: self }
    }
    ///Bit 9 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft41(&mut self) -> FT41_W {
        FT41_W { w: self }
    }
    ///Bit 13 - Falling trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn ft45(&mut self) -> FT45_W {
        FT45_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///falling trigger selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ftsr2](index.html) module
pub struct FTSR2_SPEC;
impl crate::RegisterSpec for FTSR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ftsr2::R](R) reader structure
impl crate::Readable for FTSR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ftsr2::W](W) writer structure
impl crate::Writable for FTSR2_SPEC {
    type Writer = W;
}
///`reset()` method sets FTSR2 to value 0
impl crate::Resettable for FTSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
