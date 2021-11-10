///Register `RTSR1` reader
pub struct R(crate::R<RTSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RTSR1` writer
pub struct W(crate::W<RTSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR1_SPEC>;
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
impl From<crate::W<RTSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR1_SPEC>) -> Self {
        W(writer)
    }
}
///Rising trigger event configuration bit of Configurable Event input
pub type RT21_A = RT0_A;
///Field `RT21` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT21_R = RT0_R;
///Field `RT21` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT21_W<'a> {
    w: &'a mut W,
}
impl<'a> RT21_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT21_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT21_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT22_A = RT0_A;
///Field `RT22` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT22_R = RT0_R;
///Field `RT22` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT22_W<'a> {
    w: &'a mut W,
}
impl<'a> RT22_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT22_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT22_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///Rising trigger event configuration bit of Configurable Event input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT0_A {
    ///0: Rising edge trigger is disabled
    DISABLED = 0,
    ///1: Rising edge trigger is enabled
    ENABLED = 1,
}
impl From<RT0_A> for bool {
    #[inline(always)]
    fn from(variant: RT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RT0` reader - Rising trigger event configuration bit of Configurable Event input
pub struct RT0_R(crate::FieldReader<bool, RT0_A>);
impl RT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RT0_A {
        match self.bits {
            false => RT0_A::DISABLED,
            true => RT0_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RT0_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RT0_A::ENABLED
    }
}
impl core::ops::Deref for RT0_R {
    type Target = crate::FieldReader<bool, RT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RT0` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RT0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT0_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT0_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT1_A = RT0_A;
///Field `RT1` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT1_R = RT0_R;
///Field `RT1` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT1_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT1_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT2_A = RT0_A;
///Field `RT2` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT2_R = RT0_R;
///Field `RT2` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RT2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT2_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT2_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT3_A = RT0_A;
///Field `RT3` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT3_R = RT0_R;
///Field `RT3` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT3_W<'a> {
    w: &'a mut W,
}
impl<'a> RT3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT3_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT3_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT4_A = RT0_A;
///Field `RT4` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT4_R = RT0_R;
///Field `RT4` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT4_W<'a> {
    w: &'a mut W,
}
impl<'a> RT4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT4_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT4_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT5_A = RT0_A;
///Field `RT5` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT5_R = RT0_R;
///Field `RT5` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT5_W<'a> {
    w: &'a mut W,
}
impl<'a> RT5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT5_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT5_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT6_A = RT0_A;
///Field `RT6` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT6_R = RT0_R;
///Field `RT6` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT6_W<'a> {
    w: &'a mut W,
}
impl<'a> RT6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT6_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT6_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT7_A = RT0_A;
///Field `RT7` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT7_R = RT0_R;
///Field `RT7` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT7_W<'a> {
    w: &'a mut W,
}
impl<'a> RT7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT7_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT7_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT8_A = RT0_A;
///Field `RT8` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT8_R = RT0_R;
///Field `RT8` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT8_W<'a> {
    w: &'a mut W,
}
impl<'a> RT8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT8_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT8_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT9_A = RT0_A;
///Field `RT9` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT9_R = RT0_R;
///Field `RT9` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT9_W<'a> {
    w: &'a mut W,
}
impl<'a> RT9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT9_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT9_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT10_A = RT0_A;
///Field `RT10` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT10_R = RT0_R;
///Field `RT10` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT10_W<'a> {
    w: &'a mut W,
}
impl<'a> RT10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT10_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT10_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Rising trigger event configuration bit of Configurable Event input
pub type RT11_A = RT0_A;
///Field `RT11` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT11_R = RT0_R;
///Field `RT11` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT11_W<'a> {
    w: &'a mut W,
}
impl<'a> RT11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT11_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT11_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT12_A = RT0_A;
///Field `RT12` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT12_R = RT0_R;
///Field `RT12` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT12_W<'a> {
    w: &'a mut W,
}
impl<'a> RT12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT12_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT12_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT13_A = RT0_A;
///Field `RT13` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT13_R = RT0_R;
///Field `RT13` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT13_W<'a> {
    w: &'a mut W,
}
impl<'a> RT13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT13_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT13_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT14_A = RT0_A;
///Field `RT14` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT14_R = RT0_R;
///Field `RT14` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT14_W<'a> {
    w: &'a mut W,
}
impl<'a> RT14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT14_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT14_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT15_A = RT0_A;
///Field `RT15` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT15_R = RT0_R;
///Field `RT15` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT15_W<'a> {
    w: &'a mut W,
}
impl<'a> RT15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT15_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT15_A::ENABLED)
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
///Rising trigger event configuration bit of Configurable Event input
pub type RT16_A = RT0_A;
///Field `RT16` reader - Rising trigger event configuration bit of Configurable Event input
pub type RT16_R = RT0_R;
///Field `RT16` writer - Rising trigger event configuration bit of Configurable Event input
pub struct RT16_W<'a> {
    w: &'a mut W,
}
impl<'a> RT16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RT16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT16_A::DISABLED)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT16_A::ENABLED)
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
impl R {
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt21(&self) -> RT21_R {
        RT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt22(&self) -> RT22_R {
        RT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt21(&mut self) -> RT21_W {
        RT21_W { w: self }
    }
    ///Bit 22 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt22(&mut self) -> RT22_W {
        RT22_W { w: self }
    }
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt0(&mut self) -> RT0_W {
        RT0_W { w: self }
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt1(&mut self) -> RT1_W {
        RT1_W { w: self }
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt2(&mut self) -> RT2_W {
        RT2_W { w: self }
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt3(&mut self) -> RT3_W {
        RT3_W { w: self }
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt4(&mut self) -> RT4_W {
        RT4_W { w: self }
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt5(&mut self) -> RT5_W {
        RT5_W { w: self }
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt6(&mut self) -> RT6_W {
        RT6_W { w: self }
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt7(&mut self) -> RT7_W {
        RT7_W { w: self }
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt8(&mut self) -> RT8_W {
        RT8_W { w: self }
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt9(&mut self) -> RT9_W {
        RT9_W { w: self }
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt10(&mut self) -> RT10_W {
        RT10_W { w: self }
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt11(&mut self) -> RT11_W {
        RT11_W { w: self }
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt12(&mut self) -> RT12_W {
        RT12_W { w: self }
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt13(&mut self) -> RT13_W {
        RT13_W { w: self }
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt14(&mut self) -> RT14_W {
        RT14_W { w: self }
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt15(&mut self) -> RT15_W {
        RT15_W { w: self }
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn rt16(&mut self) -> RT16_W {
        RT16_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///rising trigger selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtsr1](index.html) module
pub struct RTSR1_SPEC;
impl crate::RegisterSpec for RTSR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rtsr1::R](R) reader structure
impl crate::Readable for RTSR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rtsr1::W](W) writer structure
impl crate::Writable for RTSR1_SPEC {
    type Writer = W;
}
///`reset()` method sets RTSR1 to value 0
impl crate::Resettable for RTSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
