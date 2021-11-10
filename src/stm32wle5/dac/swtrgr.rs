///Register `SWTRGR` writer
pub struct W(crate::W<SWTRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRGR_SPEC>;
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
impl From<crate::W<SWTRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRGR_SPEC>) -> Self {
        W(writer)
    }
}
///DAC channel1 software trigger
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIG1_AW {
    ///0: No trigger
    NOTRIGGER = 0,
    ///1: Trigger
    TRIGGER = 1,
}
impl From<SWTRIG1_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SWTRIG1` writer - DAC channel1 software trigger
pub struct SWTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWTRIG1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No trigger
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(SWTRIG1_AW::NOTRIGGER)
    }
    ///Trigger
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(SWTRIG1_AW::TRIGGER)
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
impl W {
    ///Bit 0 - DAC channel1 software trigger
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W {
        SWTRIG1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///software trigger register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swtrgr](index.html) module
pub struct SWTRGR_SPEC;
impl crate::RegisterSpec for SWTRGR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [swtrgr::W](W) writer structure
impl crate::Writable for SWTRGR_SPEC {
    type Writer = W;
}
///`reset()` method sets SWTRGR to value 0
impl crate::Resettable for SWTRGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
