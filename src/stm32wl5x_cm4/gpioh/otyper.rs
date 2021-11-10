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
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OT3_A {
    ///0: Output push-pull (reset state)
    PUSHPULL = 0,
    ///1: Output open-drain
    OPENDRAIN = 1,
}
impl From<OT3_A> for bool {
    #[inline(always)]
    fn from(variant: OT3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OT3` reader - Port x configuration bits (y = 0..15)
pub struct OT3_R(crate::FieldReader<bool, OT3_A>);
impl OT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OT3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OT3_A {
        match self.bits {
            false => OT3_A::PUSHPULL,
            true => OT3_A::OPENDRAIN,
        }
    }
    ///Checks if the value of the field is `PUSHPULL`
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        **self == OT3_A::PUSHPULL
    }
    ///Checks if the value of the field is `OPENDRAIN`
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == OT3_A::OPENDRAIN
    }
}
impl core::ops::Deref for OT3_R {
    type Target = crate::FieldReader<bool, OT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
impl R {
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot3(&mut self) -> OT3_W {
        OT3_W { w: self }
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
