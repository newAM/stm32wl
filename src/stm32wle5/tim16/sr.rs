///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Capture/Compare 1 overcapture flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1OF_A {
    ///0: No overcapture has been detected
    NOOVERCAPTURE = 0,
    ///1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
    OVERCAPTURE = 1,
}
impl From<CC1OF_A> for bool {
    #[inline(always)]
    fn from(variant: CC1OF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1OF` reader - Capture/Compare 1 overcapture flag
pub struct CC1OF_R(crate::FieldReader<bool, CC1OF_A>);
impl CC1OF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1OF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1OF_A {
        match self.bits {
            false => CC1OF_A::NOOVERCAPTURE,
            true => CC1OF_A::OVERCAPTURE,
        }
    }
    ///Checks if the value of the field is `NOOVERCAPTURE`
    #[inline(always)]
    pub fn is_no_overcapture(&self) -> bool {
        **self == CC1OF_A::NOOVERCAPTURE
    }
    ///Checks if the value of the field is `OVERCAPTURE`
    #[inline(always)]
    pub fn is_overcapture(&self) -> bool {
        **self == CC1OF_A::OVERCAPTURE
    }
}
impl core::ops::Deref for CC1OF_R {
    type Target = crate::FieldReader<bool, CC1OF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Capture/Compare 1 overcapture flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1OF_AW {
    ///0: Clear flag
    CLEAR = 0,
}
impl From<CC1OF_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1OF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1OF` writer - Capture/Compare 1 overcapture flag
pub struct CC1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1OF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1OF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC1OF_AW::CLEAR)
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
///Break interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIF_A {
    ///0: No break event occurred
    NOBREAK = 0,
    ///1: Break interrupt pending
    BREAK = 1,
}
impl From<BIF_A> for bool {
    #[inline(always)]
    fn from(variant: BIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BIF` reader - Break interrupt flag
pub struct BIF_R(crate::FieldReader<bool, BIF_A>);
impl BIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BIF_A {
        match self.bits {
            false => BIF_A::NOBREAK,
            true => BIF_A::BREAK,
        }
    }
    ///Checks if the value of the field is `NOBREAK`
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        **self == BIF_A::NOBREAK
    }
    ///Checks if the value of the field is `BREAK`
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        **self == BIF_A::BREAK
    }
}
impl core::ops::Deref for BIF_R {
    type Target = crate::FieldReader<bool, BIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Break interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIF_AW {
    ///0: Clear flag
    CLEAR = 0,
}
impl From<BIF_AW> for bool {
    #[inline(always)]
    fn from(variant: BIF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `BIF` writer - Break interrupt flag
pub struct BIF_W<'a> {
    w: &'a mut W,
}
impl<'a> BIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BIF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BIF_AW::CLEAR)
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
///COM interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMIF_A {
    ///0: No COM event occurred
    NOCOM = 0,
    ///1: COM interrupt pending
    COM = 1,
}
impl From<COMIF_A> for bool {
    #[inline(always)]
    fn from(variant: COMIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COMIF` reader - COM interrupt flag
pub struct COMIF_R(crate::FieldReader<bool, COMIF_A>);
impl COMIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMIF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COMIF_A {
        match self.bits {
            false => COMIF_A::NOCOM,
            true => COMIF_A::COM,
        }
    }
    ///Checks if the value of the field is `NOCOM`
    #[inline(always)]
    pub fn is_no_com(&self) -> bool {
        **self == COMIF_A::NOCOM
    }
    ///Checks if the value of the field is `COM`
    #[inline(always)]
    pub fn is_com(&self) -> bool {
        **self == COMIF_A::COM
    }
}
impl core::ops::Deref for COMIF_R {
    type Target = crate::FieldReader<bool, COMIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///COM interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMIF_AW {
    ///0: Clear flag
    CLEAR = 0,
}
impl From<COMIF_AW> for bool {
    #[inline(always)]
    fn from(variant: COMIF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `COMIF` writer - COM interrupt flag
pub struct COMIF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COMIF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMIF_AW::CLEAR)
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
///Capture/Compare 1 interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IF_A {
    ///0: No campture/compare has been detected
    NOMATCH = 0,
    ///1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register
    MATCH = 1,
}
impl From<CC1IF_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1IF` reader - Capture/Compare 1 interrupt flag
pub struct CC1IF_R(crate::FieldReader<bool, CC1IF_A>);
impl CC1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1IF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1IF_A {
        match self.bits {
            false => CC1IF_A::NOMATCH,
            true => CC1IF_A::MATCH,
        }
    }
    ///Checks if the value of the field is `NOMATCH`
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        **self == CC1IF_A::NOMATCH
    }
    ///Checks if the value of the field is `MATCH`
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        **self == CC1IF_A::MATCH
    }
}
impl core::ops::Deref for CC1IF_R {
    type Target = crate::FieldReader<bool, CC1IF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Capture/Compare 1 interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IF_AW {
    ///0: Clear flag
    CLEAR = 0,
}
impl From<CC1IF_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1IF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1IF` writer - Capture/Compare 1 interrupt flag
pub struct CC1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1IF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1IF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC1IF_AW::CLEAR)
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
///Update interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIF_A {
    ///0: No update occurred
    CLEAR = 0,
    ///1: Update interrupt pending.
    UPDATEPENDING = 1,
}
impl From<UIF_A> for bool {
    #[inline(always)]
    fn from(variant: UIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UIF` reader - Update interrupt flag
pub struct UIF_R(crate::FieldReader<bool, UIF_A>);
impl UIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UIF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UIF_A {
        match self.bits {
            false => UIF_A::CLEAR,
            true => UIF_A::UPDATEPENDING,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == UIF_A::CLEAR
    }
    ///Checks if the value of the field is `UPDATEPENDING`
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        **self == UIF_A::UPDATEPENDING
    }
}
impl core::ops::Deref for UIF_R {
    type Target = crate::FieldReader<bool, UIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UIF` writer - Update interrupt flag
pub struct UIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No update occurred
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UIF_A::CLEAR)
    }
    ///Update interrupt pending.
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut W {
        self.variant(UIF_A::UPDATEPENDING)
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
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W {
        CC1OF_W { w: self }
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W {
        BIF_W { w: self }
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W {
        COMIF_W { w: self }
    }
    ///Bit 1 - Capture/Compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W {
        CC1IF_W { w: self }
    }
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W {
        UIF_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM16/TIM17 status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
