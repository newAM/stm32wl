///Register `C2APB1ENR2` reader
pub struct R(crate::R<C2APB1ENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB1ENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB1ENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB1ENR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2APB1ENR2` writer
pub struct W(crate::W<C2APB1ENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB1ENR2_SPEC>;
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
impl From<crate::W<C2APB1ENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB1ENR2_SPEC>) -> Self {
        W(writer)
    }
}
///CPU2 Low power timer 3 clocks enable
pub type LPTIM3EN_A = LPUART1EN_A;
///Field `LPTIM3EN` reader - CPU2 Low power timer 3 clocks enable
pub type LPTIM3EN_R = LPUART1EN_R;
///Field `LPTIM3EN` writer - CPU2 Low power timer 3 clocks enable
pub struct LPTIM3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM3EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM3EN_A::ENABLED)
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
///CPU2 Low power timer 2 clocks enable
pub type LPTIM2EN_A = LPUART1EN_A;
///Field `LPTIM2EN` reader - CPU2 Low power timer 2 clocks enable
pub type LPTIM2EN_R = LPUART1EN_R;
///Field `LPTIM2EN` writer - CPU2 Low power timer 2 clocks enable
pub struct LPTIM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM2EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM2EN_A::ENABLED)
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
///CPU2 Low power UART 1 clocks enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1EN_A {
    ///0: Clock disabled
    DISABLED = 0,
    ///1: Clock enabled
    ENABLED = 1,
}
impl From<LPUART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LPUART1EN` reader - CPU2 Low power UART 1 clocks enable
pub struct LPUART1EN_R(crate::FieldReader<bool, LPUART1EN_A>);
impl LPUART1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1EN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPUART1EN_A {
        match self.bits {
            false => LPUART1EN_A::DISABLED,
            true => LPUART1EN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LPUART1EN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LPUART1EN_A::ENABLED
    }
}
impl core::ops::Deref for LPUART1EN_R {
    type Target = crate::FieldReader<bool, LPUART1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPUART1EN` writer - CPU2 Low power UART 1 clocks enable
pub struct LPUART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPUART1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::ENABLED)
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
    ///Bit 6 - CPU2 Low power timer 3 clocks enable
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - CPU2 Low power timer 2 clocks enable
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 0 - CPU2 Low power UART 1 clocks enable
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 6 - CPU2 Low power timer 3 clocks enable
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W {
        LPTIM3EN_W { w: self }
    }
    ///Bit 5 - CPU2 Low power timer 2 clocks enable
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W {
        LPTIM2EN_W { w: self }
    }
    ///Bit 0 - CPU2 Low power UART 1 clocks enable
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W {
        LPUART1EN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 APB1 peripheral clock enable register 2 \[dual core device only\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2apb1enr2](index.html) module
pub struct C2APB1ENR2_SPEC;
impl crate::RegisterSpec for C2APB1ENR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2apb1enr2::R](R) reader structure
impl crate::Readable for C2APB1ENR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2apb1enr2::W](W) writer structure
impl crate::Writable for C2APB1ENR2_SPEC {
    type Writer = W;
}
///`reset()` method sets C2APB1ENR2 to value 0
impl crate::Resettable for C2APB1ENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}