///Register `KEYR0` writer
pub struct W(crate::W<KEYR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR0_SPEC>;
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
impl From<crate::W<KEYR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `KEY` writer - Data Output Register (LSB key \[31:0\])
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    ///Bits 0:31 - Data Output Register (LSB key \[31:0\])
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
///key register 0
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [keyr0](index.html) module
pub struct KEYR0_SPEC;
impl crate::RegisterSpec for KEYR0_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [keyr0::W](W) writer structure
impl crate::Writable for KEYR0_SPEC {
    type Writer = W;
}
///`reset()` method sets KEYR0 to value 0
impl crate::Resettable for KEYR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
