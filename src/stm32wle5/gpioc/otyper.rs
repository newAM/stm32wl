///Register `OTYPER` reader
pub struct R(crate::R<OTYPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTYPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTYPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTYPER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTYPER` writer
pub struct W(crate::W<OTYPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTYPER_SPEC>;
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
impl From<crate::W<OTYPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTYPER_SPEC>) -> Self {
        W(writer)
    }
}
///Port x configuration bits (y = 0..15)
pub type OT15_A = OT0_A;
///Field `OT15` reader - Port x configuration bits (y = 0..15)
pub type OT15_R = OT0_R;
///Field `OT15` writer - Port x configuration bits (y = 0..15)
pub struct OT15_W<'a> {
    w: &'a mut W,
}
impl<'a> OT15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OT15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT15_A::PUSHPULL)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT15_A::OPENDRAIN)
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
///Port x configuration bits (y = 0..15)
pub type OT14_A = OT0_A;
///Field `OT14` reader - Port x configuration bits (y = 0..15)
pub type OT14_R = OT0_R;
///Field `OT14` writer - Port x configuration bits (y = 0..15)
pub struct OT14_W<'a> {
    w: &'a mut W,
}
impl<'a> OT14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OT14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT14_A::PUSHPULL)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT14_A::OPENDRAIN)
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
///Port x configuration bits (y = 0..15)
pub type OT13_A = OT0_A;
///Field `OT13` reader - Port x configuration bits (y = 0..15)
pub type OT13_R = OT0_R;
///Field `OT13` writer - Port x configuration bits (y = 0..15)
pub struct OT13_W<'a> {
    w: &'a mut W,
}
impl<'a> OT13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OT13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT13_A::PUSHPULL)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT13_A::OPENDRAIN)
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
///Port x configuration bits (y = 0..15)
pub type OT6_A = OT0_A;
///Field `OT6` reader - Port x configuration bits (y = 0..15)
pub type OT6_R = OT0_R;
///Field `OT6` writer - Port x configuration bits (y = 0..15)
pub struct OT6_W<'a> {
    w: &'a mut W,
}
impl<'a> OT6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT6_A::PUSHPULL)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT6_A::OPENDRAIN)
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
///Port x configuration bits (y = 0..15)
pub type OT5_A = OT0_A;
///Field `OT5` reader - Port x configuration bits (y = 0..15)
pub type OT5_R = OT0_R;
///Field `OT5` writer - Port x configuration bits (y = 0..15)
pub struct OT5_W<'a> {
    w: &'a mut W,
}
impl<'a> OT5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT5_A::PUSHPULL)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT5_A::OPENDRAIN)
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
///Port x configuration bits (y = 0..15)
pub type OT4_A = OT0_A;
///Field `OT4` reader - Port x configuration bits (y = 0..15)
pub type OT4_R = OT0_R;
///Field `OT4` writer - Port x configuration bits (y = 0..15)
pub struct OT4_W<'a> {
    w: &'a mut W,
}
impl<'a> OT4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT4_A::PUSHPULL)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT4_A::OPENDRAIN)
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
///Port x configuration bits (y = 0..15)
pub type OT3_A = OT0_A;
///Field `OT3` reader - Port x configuration bits (y = 0..15)
pub type OT3_R = OT0_R;
///Field `OT3` writer - Port x configuration bits (y = 0..15)
pub struct OT3_W<'a> {
    w: &'a mut W,
}
impl<'a> OT3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT3_A::PUSHPULL)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT3_A::OPENDRAIN)
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
///Port x configuration bits (y = 0..15)
pub type OT2_A = OT0_A;
///Field `OT2` reader - Port x configuration bits (y = 0..15)
pub type OT2_R = OT0_R;
///Field `OT2` writer - Port x configuration bits (y = 0..15)
pub struct OT2_W<'a> {
    w: &'a mut W,
}
impl<'a> OT2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT2_A::PUSHPULL)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT2_A::OPENDRAIN)
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
///Port x configuration bits (y = 0..15)
pub type OT1_A = OT0_A;
///Field `OT1` reader - Port x configuration bits (y = 0..15)
pub type OT1_R = OT0_R;
///Field `OT1` writer - Port x configuration bits (y = 0..15)
pub struct OT1_W<'a> {
    w: &'a mut W,
}
impl<'a> OT1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT1_A::PUSHPULL)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT1_A::OPENDRAIN)
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
///Port x configuration bits (y = 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OT0_A {
    ///0: Output push-pull (reset state)
    PUSHPULL = 0,
    ///1: Output open-drain
    OPENDRAIN = 1,
}
impl From<OT0_A> for bool {
    #[inline(always)]
    fn from(variant: OT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OT0` reader - Port x configuration bits (y = 0..15)
pub struct OT0_R(crate::FieldReader<bool, OT0_A>);
impl OT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OT0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OT0_A {
        match self.bits {
            false => OT0_A::PUSHPULL,
            true => OT0_A::OPENDRAIN,
        }
    }
    ///Checks if the value of the field is `PUSHPULL`
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        **self == OT0_A::PUSHPULL
    }
    ///Checks if the value of the field is `OPENDRAIN`
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == OT0_A::OPENDRAIN
    }
}
impl core::ops::Deref for OT0_R {
    type Target = crate::FieldReader<bool, OT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OT0` writer - Port x configuration bits (y = 0..15)
pub struct OT0_W<'a> {
    w: &'a mut W,
}
impl<'a> OT0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT0_A::PUSHPULL)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT0_A::OPENDRAIN)
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
    ///Bit 15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot15(&self) -> OT15_R {
        OT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot14(&self) -> OT14_R {
        OT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot13(&self) -> OT13_R {
        OT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 6 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot6(&self) -> OT6_R {
        OT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot5(&self) -> OT5_R {
        OT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot4(&self) -> OT4_R {
        OT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot2(&self) -> OT2_R {
        OT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot1(&self) -> OT1_R {
        OT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot0(&self) -> OT0_R {
        OT0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot15(&mut self) -> OT15_W {
        OT15_W { w: self }
    }
    ///Bit 14 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot14(&mut self) -> OT14_W {
        OT14_W { w: self }
    }
    ///Bit 13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot13(&mut self) -> OT13_W {
        OT13_W { w: self }
    }
    ///Bit 6 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot6(&mut self) -> OT6_W {
        OT6_W { w: self }
    }
    ///Bit 5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot5(&mut self) -> OT5_W {
        OT5_W { w: self }
    }
    ///Bit 4 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot4(&mut self) -> OT4_W {
        OT4_W { w: self }
    }
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot3(&mut self) -> OT3_W {
        OT3_W { w: self }
    }
    ///Bit 2 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot2(&mut self) -> OT2_W {
        OT2_W { w: self }
    }
    ///Bit 1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot1(&mut self) -> OT1_W {
        OT1_W { w: self }
    }
    ///Bit 0 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot0(&mut self) -> OT0_W {
        OT0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port output type register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otyper](index.html) module
pub struct OTYPER_SPEC;
impl crate::RegisterSpec for OTYPER_SPEC {
    type Ux = u32;
}
///`read()` method returns [otyper::R](R) reader structure
impl crate::Readable for OTYPER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otyper::W](W) writer structure
impl crate::Writable for OTYPER_SPEC {
    type Writer = W;
}
///`reset()` method sets OTYPER to value 0
impl crate::Resettable for OTYPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
