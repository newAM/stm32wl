///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///TAMP1E
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1E_A {
    ///0: Tamper detection on TAMP_INx is disabled
    DISABLED = 0,
    ///1: Tamper detection on TAMP_IN3 is enabled
    ENABLED = 1,
}
impl From<TAMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1E` reader - TAMP1E
pub struct TAMP1E_R(crate::FieldReader<bool, TAMP1E_A>);
impl TAMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP1E_A {
        match self.bits {
            false => TAMP1E_A::DISABLED,
            true => TAMP1E_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TAMP1E_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TAMP1E_A::ENABLED
    }
}
impl core::ops::Deref for TAMP1E_R {
    type Target = crate::FieldReader<bool, TAMP1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP1E` writer - TAMP1E
pub struct TAMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper detection on TAMP_INx is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP1E_A::DISABLED)
    }
    ///Tamper detection on TAMP_IN3 is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP1E_A::ENABLED)
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
///TAMP2E
pub type TAMP2E_A = TAMP1E_A;
///Field `TAMP2E` reader - TAMP2E
pub type TAMP2E_R = TAMP1E_R;
///Field `TAMP2E` writer - TAMP2E
pub struct TAMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP2E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper detection on TAMP_INx is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP2E_A::DISABLED)
    }
    ///Tamper detection on TAMP_IN3 is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP2E_A::ENABLED)
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
///TAMP2E
pub type TAMP3E_A = TAMP1E_A;
///Field `TAMP3E` reader - TAMP2E
pub type TAMP3E_R = TAMP1E_R;
///Field `TAMP3E` writer - TAMP2E
pub struct TAMP3E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP3E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper detection on TAMP_INx is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP3E_A::DISABLED)
    }
    ///Tamper detection on TAMP_IN3 is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP3E_A::ENABLED)
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
///ITAMP3E
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP3E_A {
    ///0: Internal tamper x disabled
    DISABLED = 0,
    ///1: Internal tamper x enabled
    ENABLED = 1,
}
impl From<ITAMP3E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP3E` reader - ITAMP3E
pub struct ITAMP3E_R(crate::FieldReader<bool, ITAMP3E_A>);
impl ITAMP3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3E_A {
        match self.bits {
            false => ITAMP3E_A::DISABLED,
            true => ITAMP3E_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ITAMP3E_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ITAMP3E_A::ENABLED
    }
}
impl core::ops::Deref for ITAMP3E_R {
    type Target = crate::FieldReader<bool, ITAMP3E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITAMP3E` writer - ITAMP3E
pub struct ITAMP3E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP3E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP3E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper x disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITAMP3E_A::DISABLED)
    }
    ///Internal tamper x enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITAMP3E_A::ENABLED)
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
///ITAMP5E
pub type ITAMP5E_A = ITAMP3E_A;
///Field `ITAMP5E` reader - ITAMP5E
pub type ITAMP5E_R = ITAMP3E_R;
///Field `ITAMP5E` writer - ITAMP5E
pub struct ITAMP5E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP5E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP5E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper x disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITAMP5E_A::DISABLED)
    }
    ///Internal tamper x enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITAMP5E_A::ENABLED)
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
///ITAMP6E
pub type ITAMP6E_A = ITAMP3E_A;
///Field `ITAMP6E` reader - ITAMP6E
pub type ITAMP6E_R = ITAMP3E_R;
///Field `ITAMP6E` writer - ITAMP6E
pub struct ITAMP6E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP6E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP6E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper x disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITAMP6E_A::DISABLED)
    }
    ///Internal tamper x enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITAMP6E_A::ENABLED)
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
///ITAMP8E
pub type ITAMP8E_A = ITAMP3E_A;
///Field `ITAMP8E` reader - ITAMP8E
pub type ITAMP8E_R = ITAMP3E_R;
///Field `ITAMP8E` writer - ITAMP8E
pub struct ITAMP8E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP8E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP8E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper x disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITAMP8E_A::DISABLED)
    }
    ///Internal tamper x enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITAMP8E_A::ENABLED)
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
impl R {
    ///Bit 0 - TAMP1E
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TAMP2E
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - TAMP2E
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 18 - ITAMP3E
    #[inline(always)]
    pub fn itamp3e(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 20 - ITAMP5E
    #[inline(always)]
    pub fn itamp5e(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - ITAMP6E
    #[inline(always)]
    pub fn itamp6e(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 23 - ITAMP8E
    #[inline(always)]
    pub fn itamp8e(&self) -> ITAMP8E_R {
        ITAMP8E_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TAMP1E
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W {
        TAMP1E_W { w: self }
    }
    ///Bit 1 - TAMP2E
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W {
        TAMP2E_W { w: self }
    }
    ///Bit 2 - TAMP2E
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W {
        TAMP3E_W { w: self }
    }
    ///Bit 18 - ITAMP3E
    #[inline(always)]
    pub fn itamp3e(&mut self) -> ITAMP3E_W {
        ITAMP3E_W { w: self }
    }
    ///Bit 20 - ITAMP5E
    #[inline(always)]
    pub fn itamp5e(&mut self) -> ITAMP5E_W {
        ITAMP5E_W { w: self }
    }
    ///Bit 21 - ITAMP6E
    #[inline(always)]
    pub fn itamp6e(&mut self) -> ITAMP6E_W {
        ITAMP6E_W { w: self }
    }
    ///Bit 23 - ITAMP8E
    #[inline(always)]
    pub fn itamp8e(&mut self) -> ITAMP8E_W {
        ITAMP8E_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
///`reset()` method sets CR1 to value 0xffff_0000
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
