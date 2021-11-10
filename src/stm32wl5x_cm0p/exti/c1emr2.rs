///Register `C1EMR2` reader
pub struct R(crate::R<C1EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1EMR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C1EMR2` writer
pub struct W(crate::W<C1EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1EMR2_SPEC>;
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
impl From<crate::W<C1EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1EMR2_SPEC>) -> Self {
        W(writer)
    }
}
///Wakeup with event generation Mask on Event input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM40_A {
    ///0: Interrupt request line is masked
    MASKED = 0,
    ///1: Interrupt request line is unmasked
    UNMASKED = 1,
}
impl From<EM40_A> for bool {
    #[inline(always)]
    fn from(variant: EM40_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EM40` reader - Wakeup with event generation Mask on Event input
pub struct EM40_R(crate::FieldReader<bool, EM40_A>);
impl EM40_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM40_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EM40_A {
        match self.bits {
            false => EM40_A::MASKED,
            true => EM40_A::UNMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == EM40_A::MASKED
    }
    ///Checks if the value of the field is `UNMASKED`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == EM40_A::UNMASKED
    }
}
impl core::ops::Deref for EM40_R {
    type Target = crate::FieldReader<bool, EM40_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EM40` writer - Wakeup with event generation Mask on Event input
pub struct EM40_W<'a> {
    w: &'a mut W,
}
impl<'a> EM40_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM40_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM40_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM40_A::UNMASKED)
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
///Wakeup with event generation Mask on Event input
pub type EM41_A = EM40_A;
///Field `EM41` reader - Wakeup with event generation Mask on Event input
pub type EM41_R = EM40_R;
///Field `EM41` writer - Wakeup with event generation Mask on Event input
pub struct EM41_W<'a> {
    w: &'a mut W,
}
impl<'a> EM41_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM41_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM41_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM41_A::UNMASKED)
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
    ///Bit 8 - Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    ///Bit 8 - Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em40(&mut self) -> EM40_W {
        EM40_W { w: self }
    }
    ///Bit 9 - Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em41(&mut self) -> EM41_W {
        EM41_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///wakeup with event mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1emr2](index.html) module
pub struct C1EMR2_SPEC;
impl crate::RegisterSpec for C1EMR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1emr2::R](R) reader structure
impl crate::Readable for C1EMR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c1emr2::W](W) writer structure
impl crate::Writable for C1EMR2_SPEC {
    type Writer = W;
}
///`reset()` method sets C1EMR2 to value 0
impl crate::Resettable for C1EMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
