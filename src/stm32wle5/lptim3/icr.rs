#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Repetition register update OK clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPOKCF_AW {
    #[doc = "1: Clear REPOK flag"]
    Clear = 1,
}
impl From<REPOKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: REPOKCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPOKCF` writer - Repetition register update OK clear flag"]
pub type REPOKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, REPOKCF_AW, O>;
impl<'a, const O: u8> REPOKCF_W<'a, O> {
    #[doc = "Clear REPOK flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REPOKCF_AW::Clear)
    }
}
#[doc = "Update event clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UECF_AW {
    #[doc = "1: Clear update event flag"]
    Clear = 1,
}
impl From<UECF_AW> for bool {
    #[inline(always)]
    fn from(variant: UECF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UECF` writer - Update event clear flag"]
pub type UECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, UECF_AW, O>;
impl<'a, const O: u8> UECF_W<'a, O> {
    #[doc = "Clear update event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UECF_AW::Clear)
    }
}
#[doc = "Direction change to down Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWNCF_AW {
    #[doc = "1: Direction change to down Clear Flag"]
    Clear = 1,
}
impl From<DOWNCF_AW> for bool {
    #[inline(always)]
    fn from(variant: DOWNCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWNCF` writer - Direction change to down Clear Flag"]
pub type DOWNCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, DOWNCF_AW, O>;
impl<'a, const O: u8> DOWNCF_W<'a, O> {
    #[doc = "Direction change to down Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DOWNCF_AW::Clear)
    }
}
#[doc = "Direction change to UP Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPCF_AW {
    #[doc = "1: Direction change to up Clear Flag"]
    Clear = 1,
}
impl From<UPCF_AW> for bool {
    #[inline(always)]
    fn from(variant: UPCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPCF` writer - Direction change to UP Clear Flag"]
pub type UPCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, UPCF_AW, O>;
impl<'a, const O: u8> UPCF_W<'a, O> {
    #[doc = "Direction change to up Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UPCF_AW::Clear)
    }
}
#[doc = "Autoreload register update OK Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARROKCF_AW {
    #[doc = "1: Autoreload register update OK Clear Flag"]
    Clear = 1,
}
impl From<ARROKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ARROKCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARROKCF` writer - Autoreload register update OK Clear Flag"]
pub type ARROKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ARROKCF_AW, O>;
impl<'a, const O: u8> ARROKCF_W<'a, O> {
    #[doc = "Autoreload register update OK Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARROKCF_AW::Clear)
    }
}
#[doc = "Compare register update OK Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPOKCF_AW {
    #[doc = "1: Compare register update OK Clear Flag"]
    Clear = 1,
}
impl From<CMPOKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPOKCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPOKCF` writer - Compare register update OK Clear Flag"]
pub type CMPOKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CMPOKCF_AW, O>;
impl<'a, const O: u8> CMPOKCF_W<'a, O> {
    #[doc = "Compare register update OK Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMPOKCF_AW::Clear)
    }
}
#[doc = "External trigger valid edge Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTTRIGCF_AW {
    #[doc = "1: External trigger valid edge Clear Flag"]
    Clear = 1,
}
impl From<EXTTRIGCF_AW> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge Clear Flag"]
pub type EXTTRIGCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, EXTTRIGCF_AW, O>;
impl<'a, const O: u8> EXTTRIGCF_W<'a, O> {
    #[doc = "External trigger valid edge Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EXTTRIGCF_AW::Clear)
    }
}
#[doc = "Autoreload match Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARRMCF_AW {
    #[doc = "1: Autoreload match Clear Flag"]
    Clear = 1,
}
impl From<ARRMCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ARRMCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARRMCF` writer - Autoreload match Clear Flag"]
pub type ARRMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ARRMCF_AW, O>;
impl<'a, const O: u8> ARRMCF_W<'a, O> {
    #[doc = "Autoreload match Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARRMCF_AW::Clear)
    }
}
#[doc = "compare match Clear Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMCF_AW {
    #[doc = "1: Compare match Clear Flag"]
    Clear = 1,
}
impl From<CMPMCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPMCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMCF` writer - compare match Clear Flag"]
pub type CMPMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CMPMCF_AW, O>;
impl<'a, const O: u8> CMPMCF_W<'a, O> {
    #[doc = "Compare match Clear Flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMPMCF_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 8 - Repetition register update OK clear flag"]
    #[inline(always)]
    pub fn repokcf(&mut self) -> REPOKCF_W<8> {
        REPOKCF_W::new(self)
    }
    #[doc = "Bit 7 - Update event clear flag"]
    #[inline(always)]
    pub fn uecf(&mut self) -> UECF_W<7> {
        UECF_W::new(self)
    }
    #[doc = "Bit 6 - Direction change to down Clear Flag"]
    #[inline(always)]
    pub fn downcf(&mut self) -> DOWNCF_W<6> {
        DOWNCF_W::new(self)
    }
    #[doc = "Bit 5 - Direction change to UP Clear Flag"]
    #[inline(always)]
    pub fn upcf(&mut self) -> UPCF_W<5> {
        UPCF_W::new(self)
    }
    #[doc = "Bit 4 - Autoreload register update OK Clear Flag"]
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ARROKCF_W<4> {
        ARROKCF_W::new(self)
    }
    #[doc = "Bit 3 - Compare register update OK Clear Flag"]
    #[inline(always)]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W<3> {
        CMPOKCF_W::new(self)
    }
    #[doc = "Bit 2 - External trigger valid edge Clear Flag"]
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<2> {
        EXTTRIGCF_W::new(self)
    }
    #[doc = "Bit 1 - Autoreload match Clear Flag"]
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ARRMCF_W<1> {
        ARRMCF_W::new(self)
    }
    #[doc = "Bit 0 - compare match Clear Flag"]
    #[inline(always)]
    pub fn cmpmcf(&mut self) -> CMPMCF_W<0> {
        CMPMCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
