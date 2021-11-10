///Register `TISEL` reader
pub struct R(crate::R<TISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TISEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TISEL` writer
pub struct W(crate::W<TISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TISEL_SPEC>;
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
impl From<crate::W<TISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TISEL_SPEC>) -> Self {
        W(writer)
    }
}
///TISEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TISEL_A {
    ///0: TIM1_CH1 input selected
    SELECTED = 0,
}
impl From<TISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TISEL_A) -> Self {
        variant as _
    }
}
///Field `TISEL` reader - TISEL
pub struct TISEL_R(crate::FieldReader<u8, TISEL_A>);
impl TISEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TISEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TISEL_A> {
        match self.bits {
            0 => Some(TISEL_A::SELECTED),
            _ => None,
        }
    }
    ///Checks if the value of the field is `SELECTED`
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        **self == TISEL_A::SELECTED
    }
}
impl core::ops::Deref for TISEL_R {
    type Target = crate::FieldReader<u8, TISEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TISEL` writer - TISEL
pub struct TISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TISEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TISEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///TIM1_CH1 input selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(TISEL_A::SELECTED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - TISEL
    #[inline(always)]
    pub fn tisel(&self) -> TISEL_R {
        TISEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - TISEL
    #[inline(always)]
    pub fn tisel(&mut self) -> TISEL_W {
        TISEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM17 input selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tisel](index.html) module
pub struct TISEL_SPEC;
impl crate::RegisterSpec for TISEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [tisel::R](R) reader structure
impl crate::Readable for TISEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tisel::W](W) writer structure
impl crate::Writable for TISEL_SPEC {
    type Writer = W;
}
///`reset()` method sets TISEL to value 0
impl crate::Resettable for TISEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
