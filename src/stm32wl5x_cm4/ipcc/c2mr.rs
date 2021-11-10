///Register `C2MR` reader
pub struct R(crate::R<C2MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2MR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2MR` writer
pub struct W(crate::W<C2MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2MR_SPEC>;
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
impl From<crate::W<C2MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2MR_SPEC>) -> Self {
        W(writer)
    }
}
///CH1OM
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OM_A {
    ///1: Receive channel n occupied interrupt masked
    MASKED = 1,
    ///0: Receive channel n occupied interrupt not masked
    UNMASKED = 0,
}
impl From<CH1OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1OM` reader - CH1OM
pub struct CH1OM_R(crate::FieldReader<bool, CH1OM_A>);
impl CH1OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1OM_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CH1OM_A {
        match self.bits {
            true => CH1OM_A::MASKED,
            false => CH1OM_A::UNMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == CH1OM_A::MASKED
    }
    ///Checks if the value of the field is `UNMASKED`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == CH1OM_A::UNMASKED
    }
}
impl core::ops::Deref for CH1OM_R {
    type Target = crate::FieldReader<bool, CH1OM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CH1OM` writer - CH1OM
pub struct CH1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH1OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Receive channel n occupied interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH1OM_A::MASKED)
    }
    ///Receive channel n occupied interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH1OM_A::UNMASKED)
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
///CH2OM
pub type CH2OM_A = CH1OM_A;
///Field `CH2OM` reader - CH2OM
pub type CH2OM_R = CH1OM_R;
///Field `CH2OM` writer - CH2OM
pub struct CH2OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH2OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Receive channel n occupied interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH2OM_A::MASKED)
    }
    ///Receive channel n occupied interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH2OM_A::UNMASKED)
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
///CH3OM
pub type CH3OM_A = CH1OM_A;
///Field `CH3OM` reader - CH3OM
pub type CH3OM_R = CH1OM_R;
///Field `CH3OM` writer - CH3OM
pub struct CH3OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH3OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Receive channel n occupied interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH3OM_A::MASKED)
    }
    ///Receive channel n occupied interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH3OM_A::UNMASKED)
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
///CH4OM
pub type CH4OM_A = CH1OM_A;
///Field `CH4OM` reader - CH4OM
pub type CH4OM_R = CH1OM_R;
///Field `CH4OM` writer - CH4OM
pub struct CH4OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH4OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Receive channel n occupied interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH4OM_A::MASKED)
    }
    ///Receive channel n occupied interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH4OM_A::UNMASKED)
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
///CH5OM
pub type CH5OM_A = CH1OM_A;
///Field `CH5OM` reader - CH5OM
pub type CH5OM_R = CH1OM_R;
///Field `CH5OM` writer - CH5OM
pub struct CH5OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH5OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Receive channel n occupied interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH5OM_A::MASKED)
    }
    ///Receive channel n occupied interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH5OM_A::UNMASKED)
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
///CH6OM
pub type CH6OM_A = CH1OM_A;
///Field `CH6OM` reader - CH6OM
pub type CH6OM_R = CH1OM_R;
///Field `CH6OM` writer - CH6OM
pub struct CH6OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH6OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Receive channel n occupied interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH6OM_A::MASKED)
    }
    ///Receive channel n occupied interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH6OM_A::UNMASKED)
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
///CH1FM
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1FM_A {
    ///1: Transmit channel n free interrupt masked
    MASKED = 1,
    ///0: Transmit channel n free interrupt not masked
    UNMASKED = 0,
}
impl From<CH1FM_A> for bool {
    #[inline(always)]
    fn from(variant: CH1FM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1FM` reader - CH1FM
pub struct CH1FM_R(crate::FieldReader<bool, CH1FM_A>);
impl CH1FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1FM_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CH1FM_A {
        match self.bits {
            true => CH1FM_A::MASKED,
            false => CH1FM_A::UNMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == CH1FM_A::MASKED
    }
    ///Checks if the value of the field is `UNMASKED`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == CH1FM_A::UNMASKED
    }
}
impl core::ops::Deref for CH1FM_R {
    type Target = crate::FieldReader<bool, CH1FM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CH1FM` writer - CH1FM
pub struct CH1FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1FM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH1FM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Transmit channel n free interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH1FM_A::MASKED)
    }
    ///Transmit channel n free interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH1FM_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///CH2FM
pub type CH2FM_A = CH1FM_A;
///Field `CH2FM` reader - CH2FM
pub type CH2FM_R = CH1FM_R;
///Field `CH2FM` writer - CH2FM
pub struct CH2FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2FM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH2FM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Transmit channel n free interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH2FM_A::MASKED)
    }
    ///Transmit channel n free interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH2FM_A::UNMASKED)
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
///CH3FM
pub type CH3FM_A = CH1FM_A;
///Field `CH3FM` reader - CH3FM
pub type CH3FM_R = CH1FM_R;
///Field `CH3FM` writer - CH3FM
pub struct CH3FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3FM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH3FM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Transmit channel n free interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH3FM_A::MASKED)
    }
    ///Transmit channel n free interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH3FM_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///CH4FM
pub type CH4FM_A = CH1FM_A;
///Field `CH4FM` reader - CH4FM
pub type CH4FM_R = CH1FM_R;
///Field `CH4FM` writer - CH4FM
pub struct CH4FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4FM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH4FM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Transmit channel n free interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH4FM_A::MASKED)
    }
    ///Transmit channel n free interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH4FM_A::UNMASKED)
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
///CH5FM
pub type CH5FM_A = CH1FM_A;
///Field `CH5FM` reader - CH5FM
pub type CH5FM_R = CH1FM_R;
///Field `CH5FM` writer - CH5FM
pub struct CH5FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5FM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH5FM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Transmit channel n free interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH5FM_A::MASKED)
    }
    ///Transmit channel n free interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH5FM_A::UNMASKED)
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
///CH6FM
pub type CH6FM_A = CH1FM_A;
///Field `CH6FM` reader - CH6FM
pub type CH6FM_R = CH1FM_R;
///Field `CH6FM` writer - CH6FM
pub struct CH6FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6FM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH6FM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Transmit channel n free interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CH6FM_A::MASKED)
    }
    ///Transmit channel n free interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(CH6FM_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    ///Bit 0 - CH1OM
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - CH2OM
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - CH3OM
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - CH4OM
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - CH5OM
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - CH6OM
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 16 - CH1FM
    #[inline(always)]
    pub fn ch1fm(&self) -> CH1FM_R {
        CH1FM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - CH2FM
    #[inline(always)]
    pub fn ch2fm(&self) -> CH2FM_R {
        CH2FM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - CH3FM
    #[inline(always)]
    pub fn ch3fm(&self) -> CH3FM_R {
        CH3FM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - CH4FM
    #[inline(always)]
    pub fn ch4fm(&self) -> CH4FM_R {
        CH4FM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - CH5FM
    #[inline(always)]
    pub fn ch5fm(&self) -> CH5FM_R {
        CH5FM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - CH6FM
    #[inline(always)]
    pub fn ch6fm(&self) -> CH6FM_R {
        CH6FM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - CH1OM
    #[inline(always)]
    pub fn ch1om(&mut self) -> CH1OM_W {
        CH1OM_W { w: self }
    }
    ///Bit 1 - CH2OM
    #[inline(always)]
    pub fn ch2om(&mut self) -> CH2OM_W {
        CH2OM_W { w: self }
    }
    ///Bit 2 - CH3OM
    #[inline(always)]
    pub fn ch3om(&mut self) -> CH3OM_W {
        CH3OM_W { w: self }
    }
    ///Bit 3 - CH4OM
    #[inline(always)]
    pub fn ch4om(&mut self) -> CH4OM_W {
        CH4OM_W { w: self }
    }
    ///Bit 4 - CH5OM
    #[inline(always)]
    pub fn ch5om(&mut self) -> CH5OM_W {
        CH5OM_W { w: self }
    }
    ///Bit 5 - CH6OM
    #[inline(always)]
    pub fn ch6om(&mut self) -> CH6OM_W {
        CH6OM_W { w: self }
    }
    ///Bit 16 - CH1FM
    #[inline(always)]
    pub fn ch1fm(&mut self) -> CH1FM_W {
        CH1FM_W { w: self }
    }
    ///Bit 17 - CH2FM
    #[inline(always)]
    pub fn ch2fm(&mut self) -> CH2FM_W {
        CH2FM_W { w: self }
    }
    ///Bit 18 - CH3FM
    #[inline(always)]
    pub fn ch3fm(&mut self) -> CH3FM_W {
        CH3FM_W { w: self }
    }
    ///Bit 19 - CH4FM
    #[inline(always)]
    pub fn ch4fm(&mut self) -> CH4FM_W {
        CH4FM_W { w: self }
    }
    ///Bit 20 - CH5FM
    #[inline(always)]
    pub fn ch5fm(&mut self) -> CH5FM_W {
        CH5FM_W { w: self }
    }
    ///Bit 21 - CH6FM
    #[inline(always)]
    pub fn ch6fm(&mut self) -> CH6FM_W {
        CH6FM_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///IPCC Processor 2 mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2mr](index.html) module
pub struct C2MR_SPEC;
impl crate::RegisterSpec for C2MR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2mr::R](R) reader structure
impl crate::Readable for C2MR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2mr::W](W) writer structure
impl crate::Writable for C2MR_SPEC {
    type Writer = W;
}
///`reset()` method sets C2MR to value 0xffff_ffff
impl crate::Resettable for C2MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
