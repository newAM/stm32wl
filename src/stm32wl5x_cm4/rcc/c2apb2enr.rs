///Register `C2APB2ENR` reader
pub struct R(crate::R<C2APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2APB2ENR` writer
pub struct W(crate::W<C2APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB2ENR_SPEC>;
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
impl From<crate::W<C2APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
///CPU2 TIM17 timer clock enable
pub type TIM17EN_A = ADCEN_A;
///Field `TIM17EN` reader - CPU2 TIM17 timer clock enable
pub type TIM17EN_R = ADCEN_R;
///Field `TIM17EN` writer - CPU2 TIM17 timer clock enable
pub struct TIM17EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM17EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM17EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM17EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///CPU2 TIM16 timer clock enable
pub type TIM16EN_A = ADCEN_A;
///Field `TIM16EN` reader - CPU2 TIM16 timer clock enable
pub type TIM16EN_R = ADCEN_R;
///Field `TIM16EN` writer - CPU2 TIM16 timer clock enable
pub struct TIM16EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM16EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM16EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM16EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///CPU2 USART1clocks enable
pub type USART1EN_A = ADCEN_A;
///Field `USART1EN` reader - CPU2 USART1clocks enable
pub type USART1EN_R = ADCEN_R;
///Field `USART1EN` writer - CPU2 USART1clocks enable
pub struct USART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART1EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART1EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///CPU2 SPI1 clock enable
pub type SPI1EN_A = ADCEN_A;
///Field `SPI1EN` reader - CPU2 SPI1 clock enable
pub type SPI1EN_R = ADCEN_R;
///Field `SPI1EN` writer - CPU2 SPI1 clock enable
pub struct SPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI1EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI1EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///CPU2 TIM1 timer clock enable
pub type TIM1EN_A = ADCEN_A;
///Field `TIM1EN` reader - CPU2 TIM1 timer clock enable
pub type TIM1EN_R = ADCEN_R;
///Field `TIM1EN` writer - CPU2 TIM1 timer clock enable
pub struct TIM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///ADC clocks enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCEN_A {
    ///0: Clock disabled
    DISABLED = 0,
    ///1: Clock enabled
    ENABLED = 1,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCEN` reader - ADC clocks enable
pub struct ADCEN_R(crate::FieldReader<bool, ADCEN_A>);
impl ADCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::DISABLED,
            true => ADCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADCEN_A::ENABLED
    }
}
impl core::ops::Deref for ADCEN_R {
    type Target = crate::FieldReader<bool, ADCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ADCEN` writer - ADC clocks enable
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCEN_A::ENABLED)
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
impl R {
    ///Bit 18 - CPU2 TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - CPU2 TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 14 - CPU2 USART1clocks enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 12 - CPU2 SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - CPU2 TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 9 - ADC clocks enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    ///Bit 18 - CPU2 TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W {
        TIM17EN_W { w: self }
    }
    ///Bit 17 - CPU2 TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W {
        TIM16EN_W { w: self }
    }
    ///Bit 14 - CPU2 USART1clocks enable
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W { w: self }
    }
    ///Bit 12 - CPU2 SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W { w: self }
    }
    ///Bit 11 - CPU2 TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W {
        TIM1EN_W { w: self }
    }
    ///Bit 9 - ADC clocks enable
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 APB2 peripheral clock enable register \[dual core device only\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2apb2enr](index.html) module
pub struct C2APB2ENR_SPEC;
impl crate::RegisterSpec for C2APB2ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2apb2enr::R](R) reader structure
impl crate::Readable for C2APB2ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2apb2enr::W](W) writer structure
impl crate::Writable for C2APB2ENR_SPEC {
    type Writer = W;
}
///`reset()` method sets C2APB2ENR to value 0
impl crate::Resettable for C2APB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
