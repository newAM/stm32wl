///Register `AHB3RSTR` reader
pub struct R(crate::R<AHB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB3RSTR` writer
pub struct W(crate::W<AHB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3RSTR_SPEC>;
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
impl From<crate::W<AHB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Flash interface reset
pub type FLASHRST_A = PKARST_A;
///Field `FLASHRST` reader - Flash interface reset
pub type FLASHRST_R = PKARST_R;
///Field `FLASHRST` writer - Flash interface reset
pub struct FLASHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FLASHRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(FLASHRST_A::NORESET)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FLASHRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///IPCCRST
pub type IPCCRST_A = PKARST_A;
///Field `IPCCRST` reader - IPCCRST
pub type IPCCRST_R = PKARST_R;
///Field `IPCCRST` writer - IPCCRST
pub struct IPCCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCCRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IPCCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(IPCCRST_A::NORESET)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IPCCRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///HSEMRST
pub type HSEMRST_A = PKARST_A;
///Field `HSEMRST` reader - HSEMRST
pub type HSEMRST_R = PKARST_R;
///Field `HSEMRST` writer - HSEMRST
pub struct HSEMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEMRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HSEMRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(HSEMRST_A::NORESET)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(HSEMRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///RNGRST
pub type RNGRST_A = PKARST_A;
///Field `RNGRST` reader - RNGRST
pub type RNGRST_R = PKARST_R;
///Field `RNGRST` writer - RNGRST
pub struct RNGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RNGRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(RNGRST_A::NORESET)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RNGRST_A::RESET)
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
///AESRST
pub type AESRST_A = PKARST_A;
///Field `AESRST` reader - AESRST
pub type AESRST_R = PKARST_R;
///Field `AESRST` writer - AESRST
pub struct AESRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AESRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AESRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(AESRST_A::NORESET)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(AESRST_A::RESET)
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
///PKARST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKARST_A {
    ///0: No effect
    NORESET = 0,
    ///1: Reset peripheral
    RESET = 1,
}
impl From<PKARST_A> for bool {
    #[inline(always)]
    fn from(variant: PKARST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PKARST` reader - PKARST
pub struct PKARST_R(crate::FieldReader<bool, PKARST_A>);
impl PKARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKARST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PKARST_A {
        match self.bits {
            false => PKARST_A::NORESET,
            true => PKARST_A::RESET,
        }
    }
    ///Checks if the value of the field is `NORESET`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == PKARST_A::NORESET
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == PKARST_A::RESET
    }
}
impl core::ops::Deref for PKARST_R {
    type Target = crate::FieldReader<bool, PKARST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PKARST` writer - PKARST
pub struct PKARST_W<'a> {
    w: &'a mut W,
}
impl<'a> PKARST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PKARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(PKARST_A::NORESET)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PKARST_A::RESET)
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
    ///Bit 25 - Flash interface reset
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 20 - IPCCRST
    #[inline(always)]
    pub fn ipccrst(&self) -> IPCCRST_R {
        IPCCRST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - HSEMRST
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - RNGRST
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - AESRST
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - PKARST
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 25 - Flash interface reset
    #[inline(always)]
    pub fn flashrst(&mut self) -> FLASHRST_W {
        FLASHRST_W { w: self }
    }
    ///Bit 20 - IPCCRST
    #[inline(always)]
    pub fn ipccrst(&mut self) -> IPCCRST_W {
        IPCCRST_W { w: self }
    }
    ///Bit 19 - HSEMRST
    #[inline(always)]
    pub fn hsemrst(&mut self) -> HSEMRST_W {
        HSEMRST_W { w: self }
    }
    ///Bit 18 - RNGRST
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W {
        RNGRST_W { w: self }
    }
    ///Bit 17 - AESRST
    #[inline(always)]
    pub fn aesrst(&mut self) -> AESRST_W {
        AESRST_W { w: self }
    }
    ///Bit 16 - PKARST
    #[inline(always)]
    pub fn pkarst(&mut self) -> PKARST_W {
        PKARST_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB3 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3rstr](index.html) module
pub struct AHB3RSTR_SPEC;
impl crate::RegisterSpec for AHB3RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb3rstr::R](R) reader structure
impl crate::Readable for AHB3RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb3rstr::W](W) writer structure
impl crate::Writable for AHB3RSTR_SPEC {
    type Writer = W;
}
///`reset()` method sets AHB3RSTR to value 0
impl crate::Resettable for AHB3RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
