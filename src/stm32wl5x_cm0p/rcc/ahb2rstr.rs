///Register `AHB2RSTR` reader
pub struct R(crate::R<AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2RSTR` writer
pub struct W(crate::W<AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RSTR_SPEC>;
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
impl From<crate::W<AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///IO port H reset
pub type GPIOHRST_A = GPIOARST_A;
///Field `GPIOHRST` reader - IO port H reset
pub type GPIOHRST_R = GPIOARST_R;
///Field `GPIOHRST` writer - IO port H reset
pub struct GPIOHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOHRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(GPIOHRST_A::NORESET)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOHRST_A::RESET)
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
///IO port C reset
pub type GPIOCRST_A = GPIOARST_A;
///Field `GPIOCRST` reader - IO port C reset
pub type GPIOCRST_R = GPIOARST_R;
///Field `GPIOCRST` writer - IO port C reset
pub struct GPIOCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(GPIOCRST_A::NORESET)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOCRST_A::RESET)
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
///IO port B reset
pub type GPIOBRST_A = GPIOARST_A;
///Field `GPIOBRST` reader - IO port B reset
pub type GPIOBRST_R = GPIOARST_R;
///Field `GPIOBRST` writer - IO port B reset
pub struct GPIOBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOBRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(GPIOBRST_A::NORESET)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOBRST_A::RESET)
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
///IO port A reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOARST_A {
    ///0: No effect
    NORESET = 0,
    ///1: Reset peripheral
    RESET = 1,
}
impl From<GPIOARST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOARST` reader - IO port A reset
pub struct GPIOARST_R(crate::FieldReader<bool, GPIOARST_A>);
impl GPIOARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOARST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GPIOARST_A {
        match self.bits {
            false => GPIOARST_A::NORESET,
            true => GPIOARST_A::RESET,
        }
    }
    ///Checks if the value of the field is `NORESET`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == GPIOARST_A::NORESET
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == GPIOARST_A::RESET
    }
}
impl core::ops::Deref for GPIOARST_R {
    type Target = crate::FieldReader<bool, GPIOARST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `GPIOARST` writer - IO port A reset
pub struct GPIOARST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOARST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GPIOARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(GPIOARST_A::NORESET)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOARST_A::RESET)
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
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W {
        GPIOHRST_W { w: self }
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W {
        GPIOCRST_W { w: self }
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W {
        GPIOBRST_W { w: self }
    }
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W {
        GPIOARST_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB2 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2rstr](index.html) module
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2rstr::R](R) reader structure
impl crate::Readable for AHB2RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2rstr::W](W) writer structure
impl crate::Writable for AHB2RSTR_SPEC {
    type Writer = W;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}