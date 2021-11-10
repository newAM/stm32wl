///Register `C1CR` reader
pub struct R(crate::R<C1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C1CR` writer
pub struct W(crate::W<C1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1CR_SPEC>;
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
impl From<crate::W<C1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1CR_SPEC>) -> Self {
        W(writer)
    }
}
///RXOIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOIE_A {
    ///1: Enable an unmasked processor receive channel occupied to generate an RX occupied interrupt
    ENABLED = 1,
    ///0: Processor RX occupied interrupt disabled
    DISABLED = 0,
}
impl From<RXOIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXOIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXOIE` reader - RXOIE
pub struct RXOIE_R(crate::FieldReader<bool, RXOIE_A>);
impl RXOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXOIE_A {
        match self.bits {
            true => RXOIE_A::ENABLED,
            false => RXOIE_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXOIE_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXOIE_A::DISABLED
    }
}
impl core::ops::Deref for RXOIE_R {
    type Target = crate::FieldReader<bool, RXOIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXOIE` writer - RXOIE
pub struct RXOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXOIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable an unmasked processor receive channel occupied to generate an RX occupied interrupt
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXOIE_A::ENABLED)
    }
    ///Processor RX occupied interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXOIE_A::DISABLED)
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
///TXFIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIE_A {
    ///1: Enable an unmasked processor transmit channel free to generate a TX free interrupt
    ENABLED = 1,
    ///0: Processor TX free interrupt disabled
    DISABLED = 0,
}
impl From<TXFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFIE` reader - TXFIE
pub struct TXFIE_R(crate::FieldReader<bool, TXFIE_A>);
impl TXFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXFIE_A {
        match self.bits {
            true => TXFIE_A::ENABLED,
            false => TXFIE_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXFIE_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXFIE_A::DISABLED
    }
}
impl core::ops::Deref for TXFIE_R {
    type Target = crate::FieldReader<bool, TXFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXFIE` writer - TXFIE
pub struct TXFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable an unmasked processor transmit channel free to generate a TX free interrupt
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFIE_A::ENABLED)
    }
    ///Processor TX free interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFIE_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - RXOIE
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 16 - TXFIE
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - RXOIE
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W {
        RXOIE_W { w: self }
    }
    ///Bit 16 - TXFIE
    #[inline(always)]
    pub fn txfie(&mut self) -> TXFIE_W {
        TXFIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///IPCC Processor 1 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1cr](index.html) module
pub struct C1CR_SPEC;
impl crate::RegisterSpec for C1CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1cr::R](R) reader structure
impl crate::Readable for C1CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c1cr::W](W) writer structure
impl crate::Writable for C1CR_SPEC {
    type Writer = W;
}
///`reset()` method sets C1CR to value 0
impl crate::Resettable for C1CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
