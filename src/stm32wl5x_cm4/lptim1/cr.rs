///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///RSTARE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTARE_A {
    ///0: CNT Register reads do not trigger reset
    DISABLED = 0,
    ///1: CNT Register reads trigger reset of LPTIM
    ENABLED = 1,
}
impl From<RSTARE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTARE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSTARE` reader - RSTARE
pub struct RSTARE_R(crate::FieldReader<bool, RSTARE_A>);
impl RSTARE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTARE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSTARE_A {
        match self.bits {
            false => RSTARE_A::DISABLED,
            true => RSTARE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RSTARE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RSTARE_A::ENABLED
    }
}
impl core::ops::Deref for RSTARE_R {
    type Target = crate::FieldReader<bool, RSTARE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RSTARE` writer - RSTARE
pub struct RSTARE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTARE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RSTARE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CNT Register reads do not trigger reset
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTARE_A::DISABLED)
    }
    ///CNT Register reads trigger reset of LPTIM
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTARE_A::ENABLED)
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
///COUNTRST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUNTRST_A {
    ///0: Triggering of reset is possible
    IDLE = 0,
    ///1: Reset in progress, do not write 1 to this field
    BUSY = 1,
}
impl From<COUNTRST_A> for bool {
    #[inline(always)]
    fn from(variant: COUNTRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COUNTRST` reader - COUNTRST
pub struct COUNTRST_R(crate::FieldReader<bool, COUNTRST_A>);
impl COUNTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        COUNTRST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COUNTRST_A {
        match self.bits {
            false => COUNTRST_A::IDLE,
            true => COUNTRST_A::BUSY,
        }
    }
    ///Checks if the value of the field is `IDLE`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == COUNTRST_A::IDLE
    }
    ///Checks if the value of the field is `BUSY`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == COUNTRST_A::BUSY
    }
}
impl core::ops::Deref for COUNTRST_R {
    type Target = crate::FieldReader<bool, COUNTRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///COUNTRST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUNTRST_AW {
    ///1: Trigger synchronous reset of CNT (3 LPTimer core clock cycles)
    RESET = 1,
}
impl From<COUNTRST_AW> for bool {
    #[inline(always)]
    fn from(variant: COUNTRST_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `COUNTRST` writer - COUNTRST
pub struct COUNTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COUNTRST_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Trigger synchronous reset of CNT (3 LPTimer core clock cycles)
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(COUNTRST_AW::RESET)
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
///CNTSTRT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTSTRT_A {
    ///1: Timer start in Continuous mode
    START = 1,
}
impl From<CNTSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: CNTSTRT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CNTSTRT` reader - CNTSTRT
pub struct CNTSTRT_R(crate::FieldReader<bool, CNTSTRT_A>);
impl CNTSTRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTSTRT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTSTRT_A> {
        match self.bits {
            true => Some(CNTSTRT_A::START),
            _ => None,
        }
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == CNTSTRT_A::START
    }
}
impl core::ops::Deref for CNTSTRT_R {
    type Target = crate::FieldReader<bool, CNTSTRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CNTSTRT` writer - CNTSTRT
pub struct CNTSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSTRT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CNTSTRT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Timer start in Continuous mode
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CNTSTRT_A::START)
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
///SNGSTRT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNGSTRT_A {
    ///1: LPTIM start in Single mode
    START = 1,
}
impl From<SNGSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: SNGSTRT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNGSTRT` reader - SNGSTRT
pub struct SNGSTRT_R(crate::FieldReader<bool, SNGSTRT_A>);
impl SNGSTRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNGSTRT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SNGSTRT_A> {
        match self.bits {
            true => Some(SNGSTRT_A::START),
            _ => None,
        }
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SNGSTRT_A::START
    }
}
impl core::ops::Deref for SNGSTRT_R {
    type Target = crate::FieldReader<bool, SNGSTRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SNGSTRT` writer - SNGSTRT
pub struct SNGSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SNGSTRT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SNGSTRT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///LPTIM start in Single mode
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SNGSTRT_A::START)
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
///ENABLE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    ///0: LPTIM is disabled
    DISABLED = 0,
    ///1: LPTIM is enabled
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - ENABLE
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENABLE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENABLE_A::ENABLED
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ENABLE` writer - ENABLE
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///LPTIM is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    ///LPTIM is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
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
impl R {
    ///Bit 4 - RSTARE
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - COUNTRST
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - CNTSTRT
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - SNGSTRT
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - ENABLE
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 4 - RSTARE
    #[inline(always)]
    pub fn rstare(&mut self) -> RSTARE_W {
        RSTARE_W { w: self }
    }
    ///Bit 3 - COUNTRST
    #[inline(always)]
    pub fn countrst(&mut self) -> COUNTRST_W {
        COUNTRST_W { w: self }
    }
    ///Bit 2 - CNTSTRT
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CNTSTRT_W {
        CNTSTRT_W { w: self }
    }
    ///Bit 1 - SNGSTRT
    #[inline(always)]
    pub fn sngstrt(&mut self) -> SNGSTRT_W {
        SNGSTRT_W { w: self }
    }
    ///Bit 0 - ENABLE
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
