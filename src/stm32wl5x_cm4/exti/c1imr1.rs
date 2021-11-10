///Register `C1IMR1` reader
pub struct R(crate::R<C1IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1IMR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C1IMR1` writer
pub struct W(crate::W<C1IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1IMR1_SPEC>;
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
impl From<crate::W<C1IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1IMR1_SPEC>) -> Self {
        W(writer)
    }
}
///wakeup with interrupt Mask on event input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IM0_A {
    ///0: Interrupt request line is masked
    MASKED = 0,
    ///1: Interrupt request line is unmasked
    UNMASKED = 1,
}
impl From<IM0_A> for bool {
    #[inline(always)]
    fn from(variant: IM0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IM0` reader - wakeup with interrupt Mask on event input
pub struct IM0_R(crate::FieldReader<bool, IM0_A>);
impl IM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IM0_A {
        match self.bits {
            false => IM0_A::MASKED,
            true => IM0_A::UNMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == IM0_A::MASKED
    }
    ///Checks if the value of the field is `UNMASKED`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == IM0_A::UNMASKED
    }
}
impl core::ops::Deref for IM0_R {
    type Target = crate::FieldReader<bool, IM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IM0` writer - wakeup with interrupt Mask on event input
pub struct IM0_W<'a> {
    w: &'a mut W,
}
impl<'a> IM0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM0_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM0_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM1_A = IM0_A;
///Field `IM1` reader - wakeup with interrupt Mask on event input
pub type IM1_R = IM0_R;
///Field `IM1` writer - wakeup with interrupt Mask on event input
pub struct IM1_W<'a> {
    w: &'a mut W,
}
impl<'a> IM1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM1_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM1_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM2_A = IM0_A;
///Field `IM2` reader - wakeup with interrupt Mask on event input
pub type IM2_R = IM0_R;
///Field `IM2` writer - wakeup with interrupt Mask on event input
pub struct IM2_W<'a> {
    w: &'a mut W,
}
impl<'a> IM2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM2_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM2_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM3_A = IM0_A;
///Field `IM3` reader - wakeup with interrupt Mask on event input
pub type IM3_R = IM0_R;
///Field `IM3` writer - wakeup with interrupt Mask on event input
pub struct IM3_W<'a> {
    w: &'a mut W,
}
impl<'a> IM3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM3_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM3_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM4_A = IM0_A;
///Field `IM4` reader - wakeup with interrupt Mask on event input
pub type IM4_R = IM0_R;
///Field `IM4` writer - wakeup with interrupt Mask on event input
pub struct IM4_W<'a> {
    w: &'a mut W,
}
impl<'a> IM4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM4_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM4_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM5_A = IM0_A;
///Field `IM5` reader - wakeup with interrupt Mask on event input
pub type IM5_R = IM0_R;
///Field `IM5` writer - wakeup with interrupt Mask on event input
pub struct IM5_W<'a> {
    w: &'a mut W,
}
impl<'a> IM5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM5_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM5_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM6_A = IM0_A;
///Field `IM6` reader - wakeup with interrupt Mask on event input
pub type IM6_R = IM0_R;
///Field `IM6` writer - wakeup with interrupt Mask on event input
pub struct IM6_W<'a> {
    w: &'a mut W,
}
impl<'a> IM6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM6_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM6_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM7_A = IM0_A;
///Field `IM7` reader - wakeup with interrupt Mask on event input
pub type IM7_R = IM0_R;
///Field `IM7` writer - wakeup with interrupt Mask on event input
pub struct IM7_W<'a> {
    w: &'a mut W,
}
impl<'a> IM7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM7_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM7_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM8_A = IM0_A;
///Field `IM8` reader - wakeup with interrupt Mask on event input
pub type IM8_R = IM0_R;
///Field `IM8` writer - wakeup with interrupt Mask on event input
pub struct IM8_W<'a> {
    w: &'a mut W,
}
impl<'a> IM8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM8_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM8_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM9_A = IM0_A;
///Field `IM9` reader - wakeup with interrupt Mask on event input
pub type IM9_R = IM0_R;
///Field `IM9` writer - wakeup with interrupt Mask on event input
pub struct IM9_W<'a> {
    w: &'a mut W,
}
impl<'a> IM9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM9_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM9_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM10_A = IM0_A;
///Field `IM10` reader - wakeup with interrupt Mask on event input
pub type IM10_R = IM0_R;
///Field `IM10` writer - wakeup with interrupt Mask on event input
pub struct IM10_W<'a> {
    w: &'a mut W,
}
impl<'a> IM10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM10_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM10_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM11_A = IM0_A;
///Field `IM11` reader - wakeup with interrupt Mask on event input
pub type IM11_R = IM0_R;
///Field `IM11` writer - wakeup with interrupt Mask on event input
pub struct IM11_W<'a> {
    w: &'a mut W,
}
impl<'a> IM11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM11_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM11_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM12_A = IM0_A;
///Field `IM12` reader - wakeup with interrupt Mask on event input
pub type IM12_R = IM0_R;
///Field `IM12` writer - wakeup with interrupt Mask on event input
pub struct IM12_W<'a> {
    w: &'a mut W,
}
impl<'a> IM12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM12_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM12_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM13_A = IM0_A;
///Field `IM13` reader - wakeup with interrupt Mask on event input
pub type IM13_R = IM0_R;
///Field `IM13` writer - wakeup with interrupt Mask on event input
pub struct IM13_W<'a> {
    w: &'a mut W,
}
impl<'a> IM13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM13_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM13_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM14_A = IM0_A;
///Field `IM14` reader - wakeup with interrupt Mask on event input
pub type IM14_R = IM0_R;
///Field `IM14` writer - wakeup with interrupt Mask on event input
pub struct IM14_W<'a> {
    w: &'a mut W,
}
impl<'a> IM14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM14_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM14_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM15_A = IM0_A;
///Field `IM15` reader - wakeup with interrupt Mask on event input
pub type IM15_R = IM0_R;
///Field `IM15` writer - wakeup with interrupt Mask on event input
pub struct IM15_W<'a> {
    w: &'a mut W,
}
impl<'a> IM15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM15_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM15_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM16_A = IM0_A;
///Field `IM16` reader - wakeup with interrupt Mask on event input
pub type IM16_R = IM0_R;
///Field `IM16` writer - wakeup with interrupt Mask on event input
pub struct IM16_W<'a> {
    w: &'a mut W,
}
impl<'a> IM16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM16_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM16_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM17_A = IM0_A;
///Field `IM17` reader - wakeup with interrupt Mask on event input
pub type IM17_R = IM0_R;
///Field `IM17` writer - wakeup with interrupt Mask on event input
pub struct IM17_W<'a> {
    w: &'a mut W,
}
impl<'a> IM17_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM17_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM17_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM18_A = IM0_A;
///Field `IM18` reader - wakeup with interrupt Mask on event input
pub type IM18_R = IM0_R;
///Field `IM18` writer - wakeup with interrupt Mask on event input
pub struct IM18_W<'a> {
    w: &'a mut W,
}
impl<'a> IM18_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM18_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM18_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM19_A = IM0_A;
///Field `IM19` reader - wakeup with interrupt Mask on event input
pub type IM19_R = IM0_R;
///Field `IM19` writer - wakeup with interrupt Mask on event input
pub struct IM19_W<'a> {
    w: &'a mut W,
}
impl<'a> IM19_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM19_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM19_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM20_A = IM0_A;
///Field `IM20` reader - wakeup with interrupt Mask on event input
pub type IM20_R = IM0_R;
///Field `IM20` writer - wakeup with interrupt Mask on event input
pub struct IM20_W<'a> {
    w: &'a mut W,
}
impl<'a> IM20_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM20_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM20_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM21_A = IM0_A;
///Field `IM21` reader - wakeup with interrupt Mask on event input
pub type IM21_R = IM0_R;
///Field `IM21` writer - wakeup with interrupt Mask on event input
pub struct IM21_W<'a> {
    w: &'a mut W,
}
impl<'a> IM21_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM21_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM21_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM22_A = IM0_A;
///Field `IM22` reader - wakeup with interrupt Mask on event input
pub type IM22_R = IM0_R;
///Field `IM22` writer - wakeup with interrupt Mask on event input
pub struct IM22_W<'a> {
    w: &'a mut W,
}
impl<'a> IM22_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM22_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM22_A::UNMASKED)
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
///wakeup with interrupt Mask on event input
pub type IM23_A = IM0_A;
///Field `IM23` reader - wakeup with interrupt Mask on event input
pub type IM23_R = IM0_R;
///Field `IM23` writer - wakeup with interrupt Mask on event input
pub struct IM23_W<'a> {
    w: &'a mut W,
}
impl<'a> IM23_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM23_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM23_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///wakeup with interrupt Mask on event input
pub type IM24_A = IM0_A;
///Field `IM24` reader - wakeup with interrupt Mask on event input
pub type IM24_R = IM0_R;
///Field `IM24` writer - wakeup with interrupt Mask on event input
pub struct IM24_W<'a> {
    w: &'a mut W,
}
impl<'a> IM24_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM24_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM24_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///wakeup with interrupt Mask on event input
pub type IM25_A = IM0_A;
///Field `IM25` reader - wakeup with interrupt Mask on event input
pub type IM25_R = IM0_R;
///Field `IM25` writer - wakeup with interrupt Mask on event input
pub struct IM25_W<'a> {
    w: &'a mut W,
}
impl<'a> IM25_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM25_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM25_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM25_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///wakeup with interrupt Mask on event input
pub type IM26_A = IM0_A;
///Field `IM26` reader - wakeup with interrupt Mask on event input
pub type IM26_R = IM0_R;
///Field `IM26` writer - wakeup with interrupt Mask on event input
pub struct IM26_W<'a> {
    w: &'a mut W,
}
impl<'a> IM26_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM26_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM26_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM26_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///wakeup with interrupt Mask on event input
pub type IM27_A = IM0_A;
///Field `IM27` reader - wakeup with interrupt Mask on event input
pub type IM27_R = IM0_R;
///Field `IM27` writer - wakeup with interrupt Mask on event input
pub struct IM27_W<'a> {
    w: &'a mut W,
}
impl<'a> IM27_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM27_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM27_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM27_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
///wakeup with interrupt Mask on event input
pub type IM28_A = IM0_A;
///Field `IM28` reader - wakeup with interrupt Mask on event input
pub type IM28_R = IM0_R;
///Field `IM28` writer - wakeup with interrupt Mask on event input
pub struct IM28_W<'a> {
    w: &'a mut W,
}
impl<'a> IM28_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM28_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM28_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM28_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///wakeup with interrupt Mask on event input
pub type IM29_A = IM0_A;
///Field `IM29` reader - wakeup with interrupt Mask on event input
pub type IM29_R = IM0_R;
///Field `IM29` writer - wakeup with interrupt Mask on event input
pub struct IM29_W<'a> {
    w: &'a mut W,
}
impl<'a> IM29_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM29_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM29_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM29_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
///wakeup with interrupt Mask on event input
pub type IM30_A = IM0_A;
///Field `IM30` reader - wakeup with interrupt Mask on event input
pub type IM30_R = IM0_R;
///Field `IM30` writer - wakeup with interrupt Mask on event input
pub struct IM30_W<'a> {
    w: &'a mut W,
}
impl<'a> IM30_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM30_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM30_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM30_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
///wakeup with interrupt Mask on event input
pub type IM31_A = IM0_A;
///Field `IM31` reader - wakeup with interrupt Mask on event input
pub type IM31_R = IM0_R;
///Field `IM31` writer - wakeup with interrupt Mask on event input
pub struct IM31_W<'a> {
    w: &'a mut W,
}
impl<'a> IM31_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IM31_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM31_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM31_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bit 0 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im18(&self) -> IM18_R {
        IM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im24(&self) -> IM24_R {
        IM24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im25(&self) -> IM25_R {
        IM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im26(&self) -> IM26_R {
        IM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im27(&self) -> IM27_R {
        IM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im28(&self) -> IM28_R {
        IM28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im29(&self) -> IM29_R {
        IM29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im30(&self) -> IM30_R {
        IM30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im31(&self) -> IM31_R {
        IM31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im0(&mut self) -> IM0_W {
        IM0_W { w: self }
    }
    ///Bit 1 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im1(&mut self) -> IM1_W {
        IM1_W { w: self }
    }
    ///Bit 2 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im2(&mut self) -> IM2_W {
        IM2_W { w: self }
    }
    ///Bit 3 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im3(&mut self) -> IM3_W {
        IM3_W { w: self }
    }
    ///Bit 4 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im4(&mut self) -> IM4_W {
        IM4_W { w: self }
    }
    ///Bit 5 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im5(&mut self) -> IM5_W {
        IM5_W { w: self }
    }
    ///Bit 6 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im6(&mut self) -> IM6_W {
        IM6_W { w: self }
    }
    ///Bit 7 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im7(&mut self) -> IM7_W {
        IM7_W { w: self }
    }
    ///Bit 8 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im8(&mut self) -> IM8_W {
        IM8_W { w: self }
    }
    ///Bit 9 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im9(&mut self) -> IM9_W {
        IM9_W { w: self }
    }
    ///Bit 10 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im10(&mut self) -> IM10_W {
        IM10_W { w: self }
    }
    ///Bit 11 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im11(&mut self) -> IM11_W {
        IM11_W { w: self }
    }
    ///Bit 12 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im12(&mut self) -> IM12_W {
        IM12_W { w: self }
    }
    ///Bit 13 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im13(&mut self) -> IM13_W {
        IM13_W { w: self }
    }
    ///Bit 14 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im14(&mut self) -> IM14_W {
        IM14_W { w: self }
    }
    ///Bit 15 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im15(&mut self) -> IM15_W {
        IM15_W { w: self }
    }
    ///Bit 16 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im16(&mut self) -> IM16_W {
        IM16_W { w: self }
    }
    ///Bit 17 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im17(&mut self) -> IM17_W {
        IM17_W { w: self }
    }
    ///Bit 18 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im18(&mut self) -> IM18_W {
        IM18_W { w: self }
    }
    ///Bit 19 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im19(&mut self) -> IM19_W {
        IM19_W { w: self }
    }
    ///Bit 20 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im20(&mut self) -> IM20_W {
        IM20_W { w: self }
    }
    ///Bit 21 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im21(&mut self) -> IM21_W {
        IM21_W { w: self }
    }
    ///Bit 22 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im22(&mut self) -> IM22_W {
        IM22_W { w: self }
    }
    ///Bit 23 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im23(&mut self) -> IM23_W {
        IM23_W { w: self }
    }
    ///Bit 24 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im24(&mut self) -> IM24_W {
        IM24_W { w: self }
    }
    ///Bit 25 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im25(&mut self) -> IM25_W {
        IM25_W { w: self }
    }
    ///Bit 26 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im26(&mut self) -> IM26_W {
        IM26_W { w: self }
    }
    ///Bit 27 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im27(&mut self) -> IM27_W {
        IM27_W { w: self }
    }
    ///Bit 28 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im28(&mut self) -> IM28_W {
        IM28_W { w: self }
    }
    ///Bit 29 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im29(&mut self) -> IM29_W {
        IM29_W { w: self }
    }
    ///Bit 30 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im30(&mut self) -> IM30_W {
        IM30_W { w: self }
    }
    ///Bit 31 - wakeup with interrupt Mask on event input
    #[inline(always)]
    pub fn im31(&mut self) -> IM31_W {
        IM31_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1imr1](index.html) module
pub struct C1IMR1_SPEC;
impl crate::RegisterSpec for C1IMR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1imr1::R](R) reader structure
impl crate::Readable for C1IMR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c1imr1::W](W) writer structure
impl crate::Writable for C1IMR1_SPEC {
    type Writer = W;
}
///`reset()` method sets C1IMR1 to value 0
impl crate::Resettable for C1IMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}