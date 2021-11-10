///Register `CCER` reader
pub struct R(crate::R<CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCER` writer
pub struct W(crate::W<CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCER_SPEC>;
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
impl From<crate::W<CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCER_SPEC>) -> Self {
        W(writer)
    }
}
///Capture/Compare 4 output Polarity
pub type CC4NP_A = CC1NP_A;
///Field `CC4NP` reader - Capture/Compare 4 output Polarity
pub type CC4NP_R = CC1NP_R;
///Field `CC4NP` writer - Capture/Compare 4 output Polarity
pub struct CC4NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4NP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC4NP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OCxN active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(CC4NP_A::ACTIVEHIGH)
    }
    ///OCxN active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(CC4NP_A::ACTIVELOW)
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
///Capture/Compare 3 output Polarity
pub type CC4P_A = CC1P_A;
///Field `CC4P` reader - Capture/Compare 3 output Polarity
pub type CC4P_R = CC1P_R;
///Field `CC4P` writer - Capture/Compare 3 output Polarity
pub struct CC4P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4P_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC4P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Noninverted/rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CC4P_A::RISINGEDGE)
    }
    ///Inverted/falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CC4P_A::FALLINGEDGE)
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
///Capture/Compare 4 output enable
pub type CC4E_A = CC1E_A;
///Field `CC4E` reader - Capture/Compare 4 output enable
pub type CC4E_R = CC1E_R;
///Field `CC4E` writer - Capture/Compare 4 output enable
pub struct CC4E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC4E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Capture disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4E_A::DISABLED)
    }
    ///Capture enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4E_A::ENABLED)
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
///Capture/Compare 3 output Polarity
pub type CC3NP_A = CC1NP_A;
///Field `CC3NP` reader - Capture/Compare 3 output Polarity
pub type CC3NP_R = CC1NP_R;
///Field `CC3NP` writer - Capture/Compare 3 output Polarity
pub struct CC3NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3NP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC3NP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OCxN active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(CC3NP_A::ACTIVEHIGH)
    }
    ///OCxN active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(CC3NP_A::ACTIVELOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Capture/Compare 3 output Polarity
pub type CC3P_A = CC1P_A;
///Field `CC3P` reader - Capture/Compare 3 output Polarity
pub type CC3P_R = CC1P_R;
///Field `CC3P` writer - Capture/Compare 3 output Polarity
pub struct CC3P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3P_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC3P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Noninverted/rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CC3P_A::RISINGEDGE)
    }
    ///Inverted/falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CC3P_A::FALLINGEDGE)
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
///Capture/Compare 3 output enable
pub type CC3E_A = CC1E_A;
///Field `CC3E` reader - Capture/Compare 3 output enable
pub type CC3E_R = CC1E_R;
///Field `CC3E` writer - Capture/Compare 3 output enable
pub struct CC3E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC3E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Capture disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC3E_A::DISABLED)
    }
    ///Capture enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC3E_A::ENABLED)
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
///Capture/Compare 2 output Polarity
pub type CC2NP_A = CC1NP_A;
///Field `CC2NP` reader - Capture/Compare 2 output Polarity
pub type CC2NP_R = CC1NP_R;
///Field `CC2NP` writer - Capture/Compare 2 output Polarity
pub struct CC2NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2NP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2NP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OCxN active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(CC2NP_A::ACTIVEHIGH)
    }
    ///OCxN active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(CC2NP_A::ACTIVELOW)
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
///Capture/Compare 2 output Polarity
pub type CC2P_A = CC1P_A;
///Field `CC2P` reader - Capture/Compare 2 output Polarity
pub type CC2P_R = CC1P_R;
///Field `CC2P` writer - Capture/Compare 2 output Polarity
pub struct CC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2P_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Noninverted/rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CC2P_A::RISINGEDGE)
    }
    ///Inverted/falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CC2P_A::FALLINGEDGE)
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
///Capture/Compare 2 output enable
pub type CC2E_A = CC1E_A;
///Field `CC2E` reader - Capture/Compare 2 output enable
pub type CC2E_R = CC1E_R;
///Field `CC2E` writer - Capture/Compare 2 output enable
pub struct CC2E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Capture disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC2E_A::DISABLED)
    }
    ///Capture enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC2E_A::ENABLED)
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
///Capture/Compare 1 output Polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1NP_A {
    ///0: OCxN active high
    ACTIVEHIGH = 0,
    ///1: OCxN active low
    ACTIVELOW = 1,
}
impl From<CC1NP_A> for bool {
    #[inline(always)]
    fn from(variant: CC1NP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1NP` reader - Capture/Compare 1 output Polarity
pub struct CC1NP_R(crate::FieldReader<bool, CC1NP_A>);
impl CC1NP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1NP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1NP_A {
        match self.bits {
            false => CC1NP_A::ACTIVEHIGH,
            true => CC1NP_A::ACTIVELOW,
        }
    }
    ///Checks if the value of the field is `ACTIVEHIGH`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        **self == CC1NP_A::ACTIVEHIGH
    }
    ///Checks if the value of the field is `ACTIVELOW`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        **self == CC1NP_A::ACTIVELOW
    }
}
impl core::ops::Deref for CC1NP_R {
    type Target = crate::FieldReader<bool, CC1NP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1NP` writer - Capture/Compare 1 output Polarity
pub struct CC1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1NP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1NP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OCxN active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(CC1NP_A::ACTIVEHIGH)
    }
    ///OCxN active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(CC1NP_A::ACTIVELOW)
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
///Capture/Compare 1 output Polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1P_A {
    ///0: Noninverted/rising edge
    RISINGEDGE = 0,
    ///1: Inverted/falling edge
    FALLINGEDGE = 1,
}
impl From<CC1P_A> for bool {
    #[inline(always)]
    fn from(variant: CC1P_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1P` reader - Capture/Compare 1 output Polarity
pub struct CC1P_R(crate::FieldReader<bool, CC1P_A>);
impl CC1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1P_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1P_A {
        match self.bits {
            false => CC1P_A::RISINGEDGE,
            true => CC1P_A::FALLINGEDGE,
        }
    }
    ///Checks if the value of the field is `RISINGEDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == CC1P_A::RISINGEDGE
    }
    ///Checks if the value of the field is `FALLINGEDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == CC1P_A::FALLINGEDGE
    }
}
impl core::ops::Deref for CC1P_R {
    type Target = crate::FieldReader<bool, CC1P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1P` writer - Capture/Compare 1 output Polarity
pub struct CC1P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1P_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Noninverted/rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CC1P_A::RISINGEDGE)
    }
    ///Inverted/falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CC1P_A::FALLINGEDGE)
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
///Capture/Compare 1 output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1E_A {
    ///0: Capture disabled
    DISABLED = 0,
    ///1: Capture enabled
    ENABLED = 1,
}
impl From<CC1E_A> for bool {
    #[inline(always)]
    fn from(variant: CC1E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1E` reader - Capture/Compare 1 output enable
pub struct CC1E_R(crate::FieldReader<bool, CC1E_A>);
impl CC1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1E_A {
        match self.bits {
            false => CC1E_A::DISABLED,
            true => CC1E_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CC1E_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CC1E_A::ENABLED
    }
}
impl core::ops::Deref for CC1E_R {
    type Target = crate::FieldReader<bool, CC1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1E` writer - Capture/Compare 1 output enable
pub struct CC1E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Capture disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1E_A::DISABLED)
    }
    ///Capture enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1E_A::ENABLED)
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
    ///Bit 15 - Capture/Compare 4 output Polarity
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 13 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Capture/Compare 4 output enable
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 9 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Capture/Compare 3 output enable
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 15 - Capture/Compare 4 output Polarity
    #[inline(always)]
    pub fn cc4np(&mut self) -> CC4NP_W {
        CC4NP_W { w: self }
    }
    ///Bit 13 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W {
        CC4P_W { w: self }
    }
    ///Bit 12 - Capture/Compare 4 output enable
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W {
        CC4E_W { w: self }
    }
    ///Bit 11 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W {
        CC3NP_W { w: self }
    }
    ///Bit 9 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W {
        CC3P_W { w: self }
    }
    ///Bit 8 - Capture/Compare 3 output enable
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W {
        CC3E_W { w: self }
    }
    ///Bit 7 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W {
        CC2NP_W { w: self }
    }
    ///Bit 5 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W {
        CC2P_W { w: self }
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W {
        CC2E_W { w: self }
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W {
        CC1NP_W { w: self }
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W {
        CC1P_W { w: self }
    }
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W {
        CC1E_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccer](index.html) module
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccer::R](R) reader structure
impl crate::Readable for CCER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccer::W](W) writer structure
impl crate::Writable for CCER_SPEC {
    type Writer = W;
}
///`reset()` method sets CCER to value 0
impl crate::Resettable for CCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
