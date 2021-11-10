///Register `APB1SMENR2` reader
pub struct R(crate::R<APB1SMENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1SMENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1SMENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1SMENR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1SMENR2` writer
pub struct W(crate::W<APB1SMENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1SMENR2_SPEC>;
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
impl From<crate::W<APB1SMENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1SMENR2_SPEC>) -> Self {
        W(writer)
    }
}
///Low power timer 3 clock enable during CPU1 Csleep and CStop modes
pub type LPTIM3SMEN_A = LPUART1SMEN_A;
///Field `LPTIM3SMEN` reader - Low power timer 3 clock enable during CPU1 Csleep and CStop modes
pub type LPTIM3SMEN_R = LPUART1SMEN_R;
///Field `LPTIM3SMEN` writer - Low power timer 3 clock enable during CPU1 Csleep and CStop modes
pub struct LPTIM3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM3SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM3SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Low power timer 2 clock enable during CPU1 Csleep and CStop modes
pub type LPTIM2SMEN_A = LPUART1SMEN_A;
///Field `LPTIM2SMEN` reader - Low power timer 2 clock enable during CPU1 Csleep and CStop modes
pub type LPTIM2SMEN_R = LPUART1SMEN_R;
///Field `LPTIM2SMEN` writer - Low power timer 2 clock enable during CPU1 Csleep and CStop modes
pub struct LPTIM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM2SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM2SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Low power UART 1 clock enable during CPU1 Csleep and CStop modes.
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1SMEN_A {
    ///0: Clock disabled
    DISABLED = 0,
    ///1: Clock enabled
    ENABLED = 1,
}
impl From<LPUART1SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1SMEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LPUART1SMEN` reader - Low power UART 1 clock enable during CPU1 Csleep and CStop modes.
pub struct LPUART1SMEN_R(crate::FieldReader<bool, LPUART1SMEN_A>);
impl LPUART1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1SMEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPUART1SMEN_A {
        match self.bits {
            false => LPUART1SMEN_A::DISABLED,
            true => LPUART1SMEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LPUART1SMEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LPUART1SMEN_A::ENABLED
    }
}
impl core::ops::Deref for LPUART1SMEN_R {
    type Target = crate::FieldReader<bool, LPUART1SMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPUART1SMEN` writer - Low power UART 1 clock enable during CPU1 Csleep and CStop modes.
pub struct LPUART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPUART1SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1SMEN_A::ENABLED)
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
    ///Bit 6 - Low power timer 3 clock enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Low power timer 2 clock enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 0 - Low power UART 1 clock enable during CPU1 Csleep and CStop modes.
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 6 - Low power timer 3 clock enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W {
        LPTIM3SMEN_W { w: self }
    }
    ///Bit 5 - Low power timer 2 clock enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W {
        LPTIM2SMEN_W { w: self }
    }
    ///Bit 0 - Low power UART 1 clock enable during CPU1 Csleep and CStop modes.
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W {
        LPUART1SMEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB1 peripheral clocks enable in Sleep mode register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1smenr2](index.html) module
pub struct APB1SMENR2_SPEC;
impl crate::RegisterSpec for APB1SMENR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1smenr2::R](R) reader structure
impl crate::Readable for APB1SMENR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1smenr2::W](W) writer structure
impl crate::Writable for APB1SMENR2_SPEC {
    type Writer = W;
}
///`reset()` method sets APB1SMENR2 to value 0x61
impl crate::Resettable for APB1SMENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x61
    }
}