///Register `CICR` writer
pub struct W(crate::W<CICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CICR_SPEC>;
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
impl From<crate::W<CICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CICR_SPEC>) -> Self {
        W(writer)
    }
}
///LSE Clock security system interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSECSSC_AW {
    ///1: Clear interrupt flag
    CLEAR = 1,
}
impl From<LSECSSC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSECSSC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSC` writer - LSE Clock security system interrupt clear
pub struct LSECSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSECSSC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSECSSC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///HSE32 Clock security system interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSC_AW {
    ///1: Clear interrupt flag
    CLEAR = 1,
}
impl From<CSSC_AW> for bool {
    #[inline(always)]
    fn from(variant: CSSC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSC` writer - HSE32 Clock security system interrupt clear
pub struct CSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CSSC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSSC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///PLL ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLRDYC_AW {
    ///1: Clear interrupt flag
    CLEAR = 1,
}
impl From<PLLRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRDYC` writer - PLL ready interrupt clear
pub struct PLLRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDYC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLRDYC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLRDYC_AW::CLEAR)
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
///HSE32 ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSERDYC_AW {
    ///1: Clear interrupt flag
    CLEAR = 1,
}
impl From<HSERDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSERDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYC` writer - HSE32 ready interrupt clear
pub struct HSERDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HSERDYC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSERDYC_AW::CLEAR)
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
///HSI16 ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDYC_AW {
    ///1: Clear interrupt flag
    CLEAR = 1,
}
impl From<HSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYC` writer - HSI16 ready interrupt clear
pub struct HSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HSIRDYC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSIRDYC_AW::CLEAR)
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
///MSI ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIRDYC_AW {
    ///1: Clear interrupt flag
    CLEAR = 1,
}
impl From<MSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRDYC` writer - MSI ready interrupt clear
pub struct MSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRDYC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSIRDYC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MSIRDYC_AW::CLEAR)
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
///LSE ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDYC_AW {
    ///1: Clear interrupt flag
    CLEAR = 1,
}
impl From<LSERDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSERDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDYC` writer - LSE ready interrupt clear
pub struct LSERDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSERDYC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSERDYC_AW::CLEAR)
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
///LSI ready interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYC_AW {
    ///1: Clear interrupt flag
    CLEAR = 1,
}
impl From<LSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` writer - LSI ready interrupt clear
pub struct LSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSIRDYC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYC_AW::CLEAR)
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
    ///Bit 9 - LSE Clock security system interrupt clear
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LSECSSC_W {
        LSECSSC_W { w: self }
    }
    ///Bit 8 - HSE32 Clock security system interrupt clear
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W {
        CSSC_W { w: self }
    }
    ///Bit 5 - PLL ready interrupt clear
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W {
        PLLRDYC_W { w: self }
    }
    ///Bit 4 - HSE32 ready interrupt clear
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W {
        HSERDYC_W { w: self }
    }
    ///Bit 3 - HSI16 ready interrupt clear
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W {
        HSIRDYC_W { w: self }
    }
    ///Bit 2 - MSI ready interrupt clear
    #[inline(always)]
    pub fn msirdyc(&mut self) -> MSIRDYC_W {
        MSIRDYC_W { w: self }
    }
    ///Bit 1 - LSE ready interrupt clear
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W {
        LSERDYC_W { w: self }
    }
    ///Bit 0 - LSI ready interrupt clear
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W {
        LSIRDYC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock interrupt clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cicr](index.html) module
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [cicr::W](W) writer structure
impl crate::Writable for CICR_SPEC {
    type Writer = W;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
