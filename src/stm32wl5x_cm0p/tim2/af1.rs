///Register `AF1` reader
pub struct R(crate::R<AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AF1` writer
pub struct W(crate::W<AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF1_SPEC>;
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
impl From<crate::W<AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF1_SPEC>) -> Self {
        W(writer)
    }
}
///External trigger source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETRSEL_A {
    ///0: ETR legacy mode
    LEGACY = 0,
    ///1: COMP1 output
    COMP1 = 1,
    ///2: COMP2 output
    COMP2 = 2,
}
impl From<ETRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL_A) -> Self {
        variant as _
    }
}
///Field `ETRSEL` reader - External trigger source selection
pub struct ETRSEL_R(crate::FieldReader<u8, ETRSEL_A>);
impl ETRSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETRSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ETRSEL_A> {
        match self.bits {
            0 => Some(ETRSEL_A::LEGACY),
            1 => Some(ETRSEL_A::COMP1),
            2 => Some(ETRSEL_A::COMP2),
            _ => None,
        }
    }
    ///Checks if the value of the field is `LEGACY`
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        **self == ETRSEL_A::LEGACY
    }
    ///Checks if the value of the field is `COMP1`
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        **self == ETRSEL_A::COMP1
    }
    ///Checks if the value of the field is `COMP2`
    #[inline(always)]
    pub fn is_comp2(&self) -> bool {
        **self == ETRSEL_A::COMP2
    }
}
impl core::ops::Deref for ETRSEL_R {
    type Target = crate::FieldReader<u8, ETRSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ETRSEL` writer - External trigger source selection
pub struct ETRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ETRSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///ETR legacy mode
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(ETRSEL_A::LEGACY)
    }
    ///COMP1 output
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(ETRSEL_A::COMP1)
    }
    ///COMP2 output
    #[inline(always)]
    pub fn comp2(self) -> &'a mut W {
        self.variant(ETRSEL_A::COMP2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
impl R {
    ///Bits 14:17 - External trigger source selection
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 14:17 - External trigger source selection
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W {
        ETRSEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM2 alternate function option register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [af1](index.html) module
pub struct AF1_SPEC;
impl crate::RegisterSpec for AF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [af1::R](R) reader structure
impl crate::Readable for AF1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [af1::W](W) writer structure
impl crate::Writable for AF1_SPEC {
    type Writer = W;
}
///`reset()` method sets AF1 to value 0
impl crate::Resettable for AF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
