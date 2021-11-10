///Register `BSRR` writer
pub struct W(crate::W<BSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSRR_SPEC>;
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
impl From<crate::W<BSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSRR_SPEC>) -> Self {
        W(writer)
    }
}
///Port x reset bit y (y = 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BR3_AW {
    ///1: Resets the corresponding ODRx bit
    RESET = 1,
}
impl From<BR3_AW> for bool {
    #[inline(always)]
    fn from(variant: BR3_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `BR3` writer - Port x reset bit y (y = 0..15)
pub struct BR3_W<'a> {
    w: &'a mut W,
}
impl<'a> BR3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR3_AW::RESET)
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
///Port x set bit y (y= 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS3_AW {
    ///1: Sets the corresponding ODRx bit
    SET = 1,
}
impl From<BS3_AW> for bool {
    #[inline(always)]
    fn from(variant: BS3_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `BS3` writer - Port x set bit y (y= 0..15)
pub struct BS3_W<'a> {
    w: &'a mut W,
}
impl<'a> BS3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BS3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BS3_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl W {
    ///Bit 19 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W {
        BR3_W { w: self }
    }
    ///Bit 3 - Port x set bit y (y= 0..15)
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W {
        BS3_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port bit set/reset register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsrr](index.html) module
pub struct BSRR_SPEC;
impl crate::RegisterSpec for BSRR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [bsrr::W](W) writer structure
impl crate::Writable for BSRR_SPEC {
    type Writer = W;
}
///`reset()` method sets BSRR to value 0
impl crate::Resettable for BSRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}