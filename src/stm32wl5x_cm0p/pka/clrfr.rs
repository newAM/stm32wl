///Register `CLRFR` writer
pub struct W(crate::W<CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRFR_SPEC>;
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
impl From<crate::W<CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
///Clear Address error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRERRFC_AW {
    ///1: Clear ADDRERRF flag
    CLEAR = 1,
}
impl From<ADDRERRFC_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRFC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDRERRFC` writer - Clear Address error flag
pub struct ADDRERRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRERRFC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADDRERRFC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear ADDRERRF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADDRERRFC_AW::CLEAR)
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
///Clear PKA RAM error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMERRFC_AW {
    ///1: Clear RAMERRF flag
    CLEAR = 1,
}
impl From<RAMERRFC_AW> for bool {
    #[inline(always)]
    fn from(variant: RAMERRFC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RAMERRFC` writer - Clear PKA RAM error flag
pub struct RAMERRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMERRFC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RAMERRFC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear RAMERRF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RAMERRFC_AW::CLEAR)
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
///Clear PKA End of Operation flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROCENDFC_AW {
    ///1: Clear PROCENDF flag
    CLEAR = 1,
}
impl From<PROCENDFC_AW> for bool {
    #[inline(always)]
    fn from(variant: PROCENDFC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PROCENDFC` writer - Clear PKA End of Operation flag
pub struct PROCENDFC_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCENDFC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PROCENDFC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear PROCENDF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PROCENDFC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl W {
    ///Bit 20 - Clear Address error flag
    #[inline(always)]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W {
        ADDRERRFC_W { w: self }
    }
    ///Bit 19 - Clear PKA RAM error flag
    #[inline(always)]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W {
        RAMERRFC_W { w: self }
    }
    ///Bit 17 - Clear PKA End of Operation flag
    #[inline(always)]
    pub fn procendfc(&mut self) -> PROCENDFC_W {
        PROCENDFC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///clear flag register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clrfr](index.html) module
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [clrfr::W](W) writer structure
impl crate::Writable for CLRFR_SPEC {
    type Writer = W;
}
///`reset()` method sets CLRFR to value 0
impl crate::Resettable for CLRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
