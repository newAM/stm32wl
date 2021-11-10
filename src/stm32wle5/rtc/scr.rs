///Register `SCR` writer
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
///Clear SSR underflow flag
pub type CSSRUF_AW = CALRAF_AW;
///Field `CSSRUF` writer - Clear SSR underflow flag
pub struct CSSRUF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSRUF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CSSRUF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSSRUF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Clear internal timestamp flag
pub type CITSF_AW = CALRAF_AW;
///Field `CITSF` writer - Clear internal timestamp flag
pub struct CITSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CITSF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CITSF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CITSF_AW::CLEAR)
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
///Clear timestamp overflow flag
pub type CTSOVF_AW = CALRAF_AW;
///Field `CTSOVF` writer - Clear timestamp overflow flag
pub struct CTSOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSOVF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTSOVF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSOVF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Clear timestamp flag
pub type CTSF_AW = CALRAF_AW;
///Field `CTSF` writer - Clear timestamp flag
pub struct CTSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTSF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSF_AW::CLEAR)
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
///Clear wakeup timer flag
pub type CWUTF_AW = CALRAF_AW;
///Field `CWUTF` writer - Clear wakeup timer flag
pub struct CWUTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CWUTF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUTF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Clear alarm B flag
pub type CALRBF_AW = CALRAF_AW;
///Field `CALRBF` writer - Clear alarm B flag
pub struct CALRBF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRBF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CALRBF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CALRBF_AW::CLEAR)
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
///Clear alarm A flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALRAF_AW {
    ///1: Clear interrupt flag by writing 1
    CLEAR = 1,
}
impl From<CALRAF_AW> for bool {
    #[inline(always)]
    fn from(variant: CALRAF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CALRAF` writer - Clear alarm A flag
pub struct CALRAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRAF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CALRAF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag by writing 1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CALRAF_AW::CLEAR)
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
    ///Bit 6 - Clear SSR underflow flag
    #[inline(always)]
    pub fn cssruf(&mut self) -> CSSRUF_W {
        CSSRUF_W { w: self }
    }
    ///Bit 5 - Clear internal timestamp flag
    #[inline(always)]
    pub fn citsf(&mut self) -> CITSF_W {
        CITSF_W { w: self }
    }
    ///Bit 4 - Clear timestamp overflow flag
    #[inline(always)]
    pub fn ctsovf(&mut self) -> CTSOVF_W {
        CTSOVF_W { w: self }
    }
    ///Bit 3 - Clear timestamp flag
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W {
        CTSF_W { w: self }
    }
    ///Bit 2 - Clear wakeup timer flag
    #[inline(always)]
    pub fn cwutf(&mut self) -> CWUTF_W {
        CWUTF_W { w: self }
    }
    ///Bit 1 - Clear alarm B flag
    #[inline(always)]
    pub fn calrbf(&mut self) -> CALRBF_W {
        CALRBF_W { w: self }
    }
    ///Bit 0 - Clear alarm A flag
    #[inline(always)]
    pub fn calraf(&mut self) -> CALRAF_W {
        CALRAF_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Status clear register (interrupts)
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [scr](index.html) module
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [scr::W](W) writer structure
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}