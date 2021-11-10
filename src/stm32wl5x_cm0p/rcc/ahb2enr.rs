///Register `AHB2ENR` reader
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2ENR` writer
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
///CPU1 IO port H clock enable
pub type GPIOHEN_A = GPIOAEN_A;
///Field `GPIOHEN` reader - CPU1 IO port H clock enable
pub type GPIOHEN_R = GPIOAEN_R;
///Field `GPIOHEN` writer - CPU1 IO port H clock enable
pub struct GPIOHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOHEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOHEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///CPU1 IO port C clock enable
pub type GPIOCEN_A = GPIOAEN_A;
///Field `GPIOCEN` reader - CPU1 IO port C clock enable
pub type GPIOCEN_R = GPIOAEN_R;
///Field `GPIOCEN` writer - CPU1 IO port C clock enable
pub struct GPIOCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOCEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOCEN_A::ENABLED)
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
///CPU1 IO port B clock enable
pub type GPIOBEN_A = GPIOAEN_A;
///Field `GPIOBEN` reader - CPU1 IO port B clock enable
pub type GPIOBEN_R = GPIOAEN_R;
///Field `GPIOBEN` writer - CPU1 IO port B clock enable
pub struct GPIOBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOBEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOBEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///CPU1 IO port A clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOAEN_A {
    ///0: Clock disabled
    DISABLED = 0,
    ///1: Clock enabled
    ENABLED = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - CPU1 IO port A clock enable
pub struct GPIOAEN_R(crate::FieldReader<bool, GPIOAEN_A>);
impl GPIOAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::DISABLED,
            true => GPIOAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GPIOAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GPIOAEN_A::ENABLED
    }
}
impl core::ops::Deref for GPIOAEN_R {
    type Target = crate::FieldReader<bool, GPIOAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `GPIOAEN` writer - CPU1 IO port A clock enable
pub struct GPIOAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    ///Bit 7 - CPU1 IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 2 - CPU1 IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - CPU1 IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - CPU1 IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 7 - CPU1 IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W {
        GPIOHEN_W { w: self }
    }
    ///Bit 2 - CPU1 IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W {
        GPIOCEN_W { w: self }
    }
    ///Bit 1 - CPU1 IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W {
        GPIOBEN_W { w: self }
    }
    ///Bit 0 - CPU1 IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W {
        GPIOAEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB2 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2enr](index.html) module
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2enr::R](R) reader structure
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2enr::W](W) writer structure
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
}
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}