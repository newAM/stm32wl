///Register `WPR` writer
pub struct W(crate::W<WPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPR_SPEC>;
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
impl From<crate::W<WPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPR_SPEC>) -> Self {
        W(writer)
    }
}
///Write protection key
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_AW {
    ///202: Key 1
    DEACTIVATE1 = 202,
    ///83: Key 2
    DEACTIVATE2 = 83,
    ///0: Activate write protection (any value that is not the keys)
    ACTIVATE = 0,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
///Field `KEY` writer - Write protection key
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: KEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Key 1
    #[inline(always)]
    pub fn deactivate1(self) -> &'a mut W {
        self.variant(KEY_AW::DEACTIVATE1)
    }
    ///Key 2
    #[inline(always)]
    pub fn deactivate2(self) -> &'a mut W {
        self.variant(KEY_AW::DEACTIVATE2)
    }
    ///Activate write protection (any value that is not the keys)
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(KEY_AW::ACTIVATE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    ///Bits 0:7 - Write protection key
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Write protection register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wpr](index.html) module
pub struct WPR_SPEC;
impl crate::RegisterSpec for WPR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [wpr::W](W) writer structure
impl crate::Writable for WPR_SPEC {
    type Writer = W;
}
///`reset()` method sets WPR to value 0
impl crate::Resettable for WPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
