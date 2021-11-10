///Register `EGR` writer
pub struct W(crate::W<EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EGR_SPEC>;
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
impl From<crate::W<EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EGR_SPEC>) -> Self {
        W(writer)
    }
}
///Break generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BG_AW {
    ///1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled
    TRIGGER = 1,
}
impl From<BG_AW> for bool {
    #[inline(always)]
    fn from(variant: BG_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `BG` writer - Break generation
pub struct BG_W<'a> {
    w: &'a mut W,
}
impl<'a> BG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(BG_AW::TRIGGER)
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
///Capture/Compare control update generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMG_AW {
    ///1: When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated
    TRIGGER = 1,
}
impl From<COMG_AW> for bool {
    #[inline(always)]
    fn from(variant: COMG_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `COMG` writer - Capture/Compare control update generation
pub struct COMG_W<'a> {
    w: &'a mut W,
}
impl<'a> COMG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COMG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(COMG_AW::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Capture/Compare 1 generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1G_AW {
    ///1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register
    TRIGGER = 1,
}
impl From<CC1G_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1G_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1G` writer - Capture/Compare 1 generation
pub struct CC1G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1G_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1G_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC1G_AW::TRIGGER)
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
///Update generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UG_AW {
    ///1: Re-initializes the timer counter and generates an update of the registers.
    UPDATE = 1,
}
impl From<UG_AW> for bool {
    #[inline(always)]
    fn from(variant: UG_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `UG` writer - Update generation
pub struct UG_W<'a> {
    w: &'a mut W,
}
impl<'a> UG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Re-initializes the timer counter and generates an update of the registers.
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UG_AW::UPDATE)
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
    ///Bit 7 - Break generation
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W {
        BG_W { w: self }
    }
    ///Bit 5 - Capture/Compare control update generation
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W {
        COMG_W { w: self }
    }
    ///Bit 1 - Capture/Compare 1 generation
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W {
        CC1G_W { w: self }
    }
    ///Bit 0 - Update generation
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W {
        UG_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM16/TIM17 event generation register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [egr](index.html) module
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [egr::W](W) writer structure
impl crate::Writable for EGR_SPEC {
    type Writer = W;
}
///`reset()` method sets EGR to value 0
impl crate::Resettable for EGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
