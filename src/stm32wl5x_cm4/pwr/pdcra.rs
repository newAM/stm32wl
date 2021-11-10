///Register `PDCRA` reader
pub struct R(crate::R<PDCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PDCRA` writer
pub struct W(crate::W<PDCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRA_SPEC>;
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
impl From<crate::W<PDCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRA_SPEC>) -> Self {
        W(writer)
    }
}
///PD15
pub type PD15_A = PD10_A;
///Field `PD15` reader - PD15
pub type PD15_R = PD10_R;
///Field `PD15` writer - PD15
pub struct PD15_W<'a> {
    w: &'a mut W,
}
impl<'a> PD15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD15_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD15_A::ENABLED)
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
///ull-down
pub type PD14_A = PD10_A;
///Field `PD14` reader - ull-down
pub type PD14_R = PD10_R;
///Field `PD14` writer - ull-down
pub struct PD14_W<'a> {
    w: &'a mut W,
}
impl<'a> PD14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD14_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD14_A::ENABLED)
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
///PD13
pub type PD13_A = PD10_A;
///Field `PD13` reader - PD13
pub type PD13_R = PD10_R;
///Field `PD13` writer - PD13
pub struct PD13_W<'a> {
    w: &'a mut W,
}
impl<'a> PD13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD13_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD13_A::ENABLED)
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
///Port PA\[y\]
///pull-down (y=0 to 12)
pub type PD12_A = PD10_A;
///Field `PD12` reader - Port PA\[y\]
///pull-down (y=0 to 12)
pub type PD12_R = PD10_R;
///Field `PD12` writer - Port PA\[y\]
///pull-down (y=0 to 12)
pub struct PD12_W<'a> {
    w: &'a mut W,
}
impl<'a> PD12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD12_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD12_A::ENABLED)
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
///PD11
pub type PD11_A = PD10_A;
///Field `PD11` reader - PD11
pub type PD11_R = PD10_R;
///Field `PD11` writer - PD11
pub struct PD11_W<'a> {
    w: &'a mut W,
}
impl<'a> PD11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD11_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD11_A::ENABLED)
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
///PD10
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD10_A {
    ///0: Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    DISABLED = 0,
    ///1: Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    ENABLED = 1,
}
impl From<PD10_A> for bool {
    #[inline(always)]
    fn from(variant: PD10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PD10` reader - PD10
pub struct PD10_R(crate::FieldReader<bool, PD10_A>);
impl PD10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD10_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PD10_A {
        match self.bits {
            false => PD10_A::DISABLED,
            true => PD10_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PD10_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PD10_A::ENABLED
    }
}
impl core::ops::Deref for PD10_R {
    type Target = crate::FieldReader<bool, PD10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PD10` writer - PD10
pub struct PD10_W<'a> {
    w: &'a mut W,
}
impl<'a> PD10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD10_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD10_A::ENABLED)
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
///PD9
pub type PD9_A = PD0_A;
///Field `PD9` reader - PD9
pub type PD9_R = PD0_R;
///Field `PD9` writer - PD9
pub struct PD9_W<'a> {
    w: &'a mut W,
}
impl<'a> PD9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD9_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD9_A::ENABLED)
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
///PD8
pub type PD8_A = PD0_A;
///Field `PD8` reader - PD8
pub type PD8_R = PD0_R;
///Field `PD8` writer - PD8
pub struct PD8_W<'a> {
    w: &'a mut W,
}
impl<'a> PD8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD8_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD8_A::ENABLED)
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
///PD7
pub type PD7_A = PD0_A;
///Field `PD7` reader - PD7
pub type PD7_R = PD0_R;
///Field `PD7` writer - PD7
pub struct PD7_W<'a> {
    w: &'a mut W,
}
impl<'a> PD7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD7_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD7_A::ENABLED)
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
///PD6
pub type PD6_A = PD0_A;
///Field `PD6` reader - PD6
pub type PD6_R = PD0_R;
///Field `PD6` writer - PD6
pub struct PD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PD6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD6_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD6_A::ENABLED)
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
///PD5
pub type PD5_A = PD0_A;
///Field `PD5` reader - PD5
pub type PD5_R = PD0_R;
///Field `PD5` writer - PD5
pub struct PD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PD5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD5_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD5_A::ENABLED)
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
///PD4
pub type PD4_A = PD0_A;
///Field `PD4` reader - PD4
pub type PD4_R = PD0_R;
///Field `PD4` writer - PD4
pub struct PD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PD4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD4_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD4_A::ENABLED)
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
///PD3
pub type PD3_A = PD0_A;
///Field `PD3` reader - PD3
pub type PD3_R = PD0_R;
///Field `PD3` writer - PD3
pub struct PD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PD3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD3_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD3_A::ENABLED)
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
///PD2
pub type PD2_A = PD0_A;
///Field `PD2` reader - PD2
pub type PD2_R = PD0_R;
///Field `PD2` writer - PD2
pub struct PD2_W<'a> {
    w: &'a mut W,
}
impl<'a> PD2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD2_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD2_A::ENABLED)
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
///PD1
pub type PD1_A = PD0_A;
///Field `PD1` reader - PD1
pub type PD1_R = PD0_R;
///Field `PD1` writer - PD1
pub struct PD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PD1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD1_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD1_A::ENABLED)
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
///PD0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD0_A {
    ///0: Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    DISABLED = 0,
    ///1: Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    ENABLED = 1,
}
impl From<PD0_A> for bool {
    #[inline(always)]
    fn from(variant: PD0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PD0` reader - PD0
pub struct PD0_R(crate::FieldReader<bool, PD0_A>);
impl PD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PD0_A {
        match self.bits {
            false => PD0_A::DISABLED,
            true => PD0_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PD0_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PD0_A::ENABLED
    }
}
impl core::ops::Deref for PD0_R {
    type Target = crate::FieldReader<bool, PD0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PD0` writer - PD0
pub struct PD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD0_A::DISABLED)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD0_A::ENABLED)
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
    ///Bit 15 - PD15
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - ull-down
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - PD13
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Port PA\[y\]
    ///pull-down (y=0 to 12)
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - PD11
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - PD10
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - PD9
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - PD8
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - PD7
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - PD6
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - PD5
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - PD4
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - PD3
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - PD2
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - PD1
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - PD0
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 15 - PD15
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W {
        PD15_W { w: self }
    }
    ///Bit 14 - ull-down
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W {
        PD14_W { w: self }
    }
    ///Bit 13 - PD13
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W {
        PD13_W { w: self }
    }
    ///Bit 12 - Port PA\[y\]
    ///pull-down (y=0 to 12)
    #[inline(always)]
    pub fn pd12(&mut self) -> PD12_W {
        PD12_W { w: self }
    }
    ///Bit 11 - PD11
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W {
        PD11_W { w: self }
    }
    ///Bit 10 - PD10
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W {
        PD10_W { w: self }
    }
    ///Bit 9 - PD9
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W {
        PD9_W { w: self }
    }
    ///Bit 8 - PD8
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W {
        PD8_W { w: self }
    }
    ///Bit 7 - PD7
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W {
        PD7_W { w: self }
    }
    ///Bit 6 - PD6
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W {
        PD6_W { w: self }
    }
    ///Bit 5 - PD5
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W {
        PD5_W { w: self }
    }
    ///Bit 4 - PD4
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W {
        PD4_W { w: self }
    }
    ///Bit 3 - PD3
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W {
        PD3_W { w: self }
    }
    ///Bit 2 - PD2
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W {
        PD2_W { w: self }
    }
    ///Bit 1 - PD1
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W {
        PD1_W { w: self }
    }
    ///Bit 0 - PD0
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W {
        PD0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power Port A pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcra](index.html) module
pub struct PDCRA_SPEC;
impl crate::RegisterSpec for PDCRA_SPEC {
    type Ux = u32;
}
///`read()` method returns [pdcra::R](R) reader structure
impl crate::Readable for PDCRA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pdcra::W](W) writer structure
impl crate::Writable for PDCRA_SPEC {
    type Writer = W;
}
///`reset()` method sets PDCRA to value 0
impl crate::Resettable for PDCRA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
