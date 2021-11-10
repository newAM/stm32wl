///Register `CALR` reader
pub struct R(crate::R<CALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CALR` writer
pub struct W(crate::W<CALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALR_SPEC>;
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
impl From<crate::W<CALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALR_SPEC>) -> Self {
        W(writer)
    }
}
///Use an 8-second calibration cycle period
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALP_A {
    ///0: No RTCCLK pulses are added
    NOCHANGE = 0,
    ///1: One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)
    INCREASEFREQ = 1,
}
impl From<CALP_A> for bool {
    #[inline(always)]
    fn from(variant: CALP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CALP` reader - Use an 8-second calibration cycle period
pub struct CALP_R(crate::FieldReader<bool, CALP_A>);
impl CALP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CALP_A {
        match self.bits {
            false => CALP_A::NOCHANGE,
            true => CALP_A::INCREASEFREQ,
        }
    }
    ///Checks if the value of the field is `NOCHANGE`
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == CALP_A::NOCHANGE
    }
    ///Checks if the value of the field is `INCREASEFREQ`
    #[inline(always)]
    pub fn is_increase_freq(&self) -> bool {
        **self == CALP_A::INCREASEFREQ
    }
}
impl core::ops::Deref for CALP_R {
    type Target = crate::FieldReader<bool, CALP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CALP` writer - Use an 8-second calibration cycle period
pub struct CALP_W<'a> {
    w: &'a mut W,
}
impl<'a> CALP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CALP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No RTCCLK pulses are added
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CALP_A::NOCHANGE)
    }
    ///One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)
    #[inline(always)]
    pub fn increase_freq(self) -> &'a mut W {
        self.variant(CALP_A::INCREASEFREQ)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///Use a 16-second calibration cycle period
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALW8_A {
    ///1: When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected
    EIGHTSECONDS = 1,
}
impl From<CALW8_A> for bool {
    #[inline(always)]
    fn from(variant: CALW8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CALW8` reader - Use a 16-second calibration cycle period
pub struct CALW8_R(crate::FieldReader<bool, CALW8_A>);
impl CALW8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALW8_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CALW8_A> {
        match self.bits {
            true => Some(CALW8_A::EIGHTSECONDS),
            _ => None,
        }
    }
    ///Checks if the value of the field is `EIGHTSECONDS`
    #[inline(always)]
    pub fn is_eight_seconds(&self) -> bool {
        **self == CALW8_A::EIGHTSECONDS
    }
}
impl core::ops::Deref for CALW8_R {
    type Target = crate::FieldReader<bool, CALW8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CALW8` writer - Use a 16-second calibration cycle period
pub struct CALW8_W<'a> {
    w: &'a mut W,
}
impl<'a> CALW8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CALW8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected
    #[inline(always)]
    pub fn eight_seconds(self) -> &'a mut W {
        self.variant(CALW8_A::EIGHTSECONDS)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///CALW16
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALW16_A {
    ///1: When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1
    SIXTEENSECONDS = 1,
}
impl From<CALW16_A> for bool {
    #[inline(always)]
    fn from(variant: CALW16_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CALW16` reader - CALW16
pub struct CALW16_R(crate::FieldReader<bool, CALW16_A>);
impl CALW16_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALW16_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CALW16_A> {
        match self.bits {
            true => Some(CALW16_A::SIXTEENSECONDS),
            _ => None,
        }
    }
    ///Checks if the value of the field is `SIXTEENSECONDS`
    #[inline(always)]
    pub fn is_sixteen_seconds(&self) -> bool {
        **self == CALW16_A::SIXTEENSECONDS
    }
}
impl core::ops::Deref for CALW16_R {
    type Target = crate::FieldReader<bool, CALW16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CALW16` writer - CALW16
pub struct CALW16_W<'a> {
    w: &'a mut W,
}
impl<'a> CALW16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CALW16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1
    #[inline(always)]
    pub fn sixteen_seconds(self) -> &'a mut W {
        self.variant(CALW16_A::SIXTEENSECONDS)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Calibration low-power mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCAL_A {
    ///0: Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode should be set only when less than 32s calibration window is required
    RTCCLK = 0,
    ///1: Calibration window is 220 ck_apre, which is the required configuration for ultra-low consumption mode
    CKAPRE = 1,
}
impl From<LPCAL_A> for bool {
    #[inline(always)]
    fn from(variant: LPCAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LPCAL` reader - Calibration low-power mode
pub struct LPCAL_R(crate::FieldReader<bool, LPCAL_A>);
impl LPCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPCAL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPCAL_A {
        match self.bits {
            false => LPCAL_A::RTCCLK,
            true => LPCAL_A::CKAPRE,
        }
    }
    ///Checks if the value of the field is `RTCCLK`
    #[inline(always)]
    pub fn is_rtcclk(&self) -> bool {
        **self == LPCAL_A::RTCCLK
    }
    ///Checks if the value of the field is `CKAPRE`
    #[inline(always)]
    pub fn is_ck_apre(&self) -> bool {
        **self == LPCAL_A::CKAPRE
    }
}
impl core::ops::Deref for LPCAL_R {
    type Target = crate::FieldReader<bool, LPCAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPCAL` writer - Calibration low-power mode
pub struct LPCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCAL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPCAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode should be set only when less than 32s calibration window is required
    #[inline(always)]
    pub fn rtcclk(self) -> &'a mut W {
        self.variant(LPCAL_A::RTCCLK)
    }
    ///Calibration window is 220 ck_apre, which is the required configuration for ultra-low consumption mode
    #[inline(always)]
    pub fn ck_apre(self) -> &'a mut W {
        self.variant(LPCAL_A::CKAPRE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///Field `CALM` reader - Calibration minus
pub struct CALM_R(crate::FieldReader<u16, u16>);
impl CALM_R {
    pub(crate) fn new(bits: u16) -> Self {
        CALM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CALM` writer - Calibration minus
pub struct CALM_W<'a> {
    w: &'a mut W,
}
impl<'a> CALM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    ///Bit 15 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - CALW16
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Calibration low-power mode
    #[inline(always)]
    pub fn lpcal(&self) -> LPCAL_R {
        LPCAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    ///Bit 15 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calp(&mut self) -> CALP_W {
        CALP_W { w: self }
    }
    ///Bit 14 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&mut self) -> CALW8_W {
        CALW8_W { w: self }
    }
    ///Bit 13 - CALW16
    #[inline(always)]
    pub fn calw16(&mut self) -> CALW16_W {
        CALW16_W { w: self }
    }
    ///Bit 12 - Calibration low-power mode
    #[inline(always)]
    pub fn lpcal(&mut self) -> LPCAL_W {
        LPCAL_W { w: self }
    }
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&mut self) -> CALM_W {
        CALM_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Calibration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [calr](index.html) module
pub struct CALR_SPEC;
impl crate::RegisterSpec for CALR_SPEC {
    type Ux = u32;
}
///`read()` method returns [calr::R](R) reader structure
impl crate::Readable for CALR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [calr::W](W) writer structure
impl crate::Writable for CALR_SPEC {
    type Writer = W;
}
///`reset()` method sets CALR to value 0
impl crate::Resettable for CALR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
