///Register `C2APB1SMENR1` reader
pub struct R(crate::R<C2APB1SMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB1SMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB1SMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB1SMENR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2APB1SMENR1` writer
pub struct W(crate::W<C2APB1SMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB1SMENR1_SPEC>;
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
impl From<crate::W<C2APB1SMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB1SMENR1_SPEC>) -> Self {
        W(writer)
    }
}
///Low power timer 1 clock enable during CPU2 CSleep and CStop mode
pub type LPTIM1SMEN_A = TIM2SMEN_A;
///Field `LPTIM1SMEN` reader - Low power timer 1 clock enable during CPU2 CSleep and CStop mode
pub type LPTIM1SMEN_R = TIM2SMEN_R;
///Field `LPTIM1SMEN` writer - Low power timer 1 clock enable during CPU2 CSleep and CStop mode
pub struct LPTIM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM1SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM1SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
///DAC1 clock enable during CPU2 CSleep mode.
pub type DAC1SMEN_A = TIM2SMEN_A;
///Field `DAC1SMEN` reader - DAC1 clock enable during CPU2 CSleep mode.
pub type DAC1SMEN_R = TIM2SMEN_R;
///Field `DAC1SMEN` writer - DAC1 clock enable during CPU2 CSleep mode.
pub struct DAC1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DAC1SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC1SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC1SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
///I2C3 clock enable during CPU2 CSleep and CStop modes
pub type I2C3SMEN_A = TIM2SMEN_A;
///Field `I2C3SMEN` reader - I2C3 clock enable during CPU2 CSleep and CStop modes
pub type I2C3SMEN_R = TIM2SMEN_R;
///Field `I2C3SMEN` writer - I2C3 clock enable during CPU2 CSleep and CStop modes
pub struct I2C3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C3SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C3SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C3SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///I2C2 clock enable during CPU2 CSleep and CStop modes
pub type I2C2SMEN_A = TIM2SMEN_A;
///Field `I2C2SMEN` reader - I2C2 clock enable during CPU2 CSleep and CStop modes
pub type I2C2SMEN_R = TIM2SMEN_R;
///Field `I2C2SMEN` writer - I2C2 clock enable during CPU2 CSleep and CStop modes
pub struct I2C2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C2SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C2SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C2SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///I2C1 clock enable during CPU2 CSleep and CStop modes
pub type I2C1SMEN_A = TIM2SMEN_A;
///Field `I2C1SMEN` reader - I2C1 clock enable during CPU2 CSleep and CStop modes
pub type I2C1SMEN_R = TIM2SMEN_R;
///Field `I2C1SMEN` writer - I2C1 clock enable during CPU2 CSleep and CStop modes
pub struct I2C1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C1SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C1SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C1SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///USART2 clock enable during CPU2 CSleep mode.
pub type USART2SMEN_A = TIM2SMEN_A;
///Field `USART2SMEN` reader - USART2 clock enable during CPU2 CSleep mode.
pub type USART2SMEN_R = TIM2SMEN_R;
///Field `USART2SMEN` writer - USART2 clock enable during CPU2 CSleep mode.
pub struct USART2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART2SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART2SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART2SMEN_A::ENABLED)
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
///SPI2S2 clock enable during CPU2 CSleep mode.
pub type SPI2S2SMEN_A = TIM2SMEN_A;
///Field `SPI2S2SMEN` reader - SPI2S2 clock enable during CPU2 CSleep mode.
pub type SPI2S2SMEN_R = TIM2SMEN_R;
///Field `SPI2S2SMEN` writer - SPI2S2 clock enable during CPU2 CSleep mode.
pub struct SPI2S2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2S2SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI2S2SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI2S2SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI2S2SMEN_A::ENABLED)
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
///RTC bus clock enable during CPU2 CSleep mode.
pub type RTCAPBSMEN_A = TIM2SMEN_A;
///Field `RTCAPBSMEN` reader - RTC bus clock enable during CPU2 CSleep mode.
pub type RTCAPBSMEN_R = TIM2SMEN_R;
///Field `RTCAPBSMEN` writer - RTC bus clock enable during CPU2 CSleep mode.
pub struct RTCAPBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBSMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTCAPBSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCAPBSMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCAPBSMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///TIM2 timer clock enable during CPU2 CSleep mode.
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2SMEN_A {
    ///0: Clock disabled
    DISABLED = 0,
    ///1: Clock enabled
    ENABLED = 1,
}
impl From<TIM2SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2SMEN` reader - TIM2 timer clock enable during CPU2 CSleep mode.
pub struct TIM2SMEN_R(crate::FieldReader<bool, TIM2SMEN_A>);
impl TIM2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2SMEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM2SMEN_A {
        match self.bits {
            false => TIM2SMEN_A::DISABLED,
            true => TIM2SMEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TIM2SMEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TIM2SMEN_A::ENABLED
    }
}
impl core::ops::Deref for TIM2SMEN_R {
    type Target = crate::FieldReader<bool, TIM2SMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM2SMEN` writer - TIM2 timer clock enable during CPU2 CSleep mode.
pub struct TIM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM2SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2SMEN_A::ENABLED)
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
    ///Bit 31 - Low power timer 1 clock enable during CPU2 CSleep and CStop mode
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 29 - DAC1 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 23 - I2C3 clock enable during CPU2 CSleep and CStop modes
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - I2C2 clock enable during CPU2 CSleep and CStop modes
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 21 - I2C1 clock enable during CPU2 CSleep and CStop modes
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 17 - USART2 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 14 - SPI2S2 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn spi2s2smen(&self) -> SPI2S2SMEN_R {
        SPI2S2SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 10 - RTC bus clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 0 - TIM2 timer clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 31 - Low power timer 1 clock enable during CPU2 CSleep and CStop mode
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W {
        LPTIM1SMEN_W { w: self }
    }
    ///Bit 29 - DAC1 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W {
        DAC1SMEN_W { w: self }
    }
    ///Bit 23 - I2C3 clock enable during CPU2 CSleep and CStop modes
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W {
        I2C3SMEN_W { w: self }
    }
    ///Bit 22 - I2C2 clock enable during CPU2 CSleep and CStop modes
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W {
        I2C2SMEN_W { w: self }
    }
    ///Bit 21 - I2C1 clock enable during CPU2 CSleep and CStop modes
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W {
        I2C1SMEN_W { w: self }
    }
    ///Bit 17 - USART2 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W {
        USART2SMEN_W { w: self }
    }
    ///Bit 14 - SPI2S2 clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn spi2s2smen(&mut self) -> SPI2S2SMEN_W {
        SPI2S2SMEN_W { w: self }
    }
    ///Bit 10 - RTC bus clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W {
        RTCAPBSMEN_W { w: self }
    }
    ///Bit 0 - TIM2 timer clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W {
        TIM2SMEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \[dual core device only\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2apb1smenr1](index.html) module
pub struct C2APB1SMENR1_SPEC;
impl crate::RegisterSpec for C2APB1SMENR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2apb1smenr1::R](R) reader structure
impl crate::Readable for C2APB1SMENR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2apb1smenr1::W](W) writer structure
impl crate::Writable for C2APB1SMENR1_SPEC {
    type Writer = W;
}
///`reset()` method sets C2APB1SMENR1 to value 0xa0e2_4401
impl crate::Resettable for C2APB1SMENR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa0e2_4401
    }
}