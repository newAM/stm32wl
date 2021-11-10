///Register `SMPR` reader
pub struct R(crate::R<SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMPR` writer
pub struct W(crate::W<SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR_SPEC>;
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
impl From<crate::W<SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR_SPEC>) -> Self {
        W(writer)
    }
}
///SMP1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP1_A {
    ///0: 1.5 ADC clock cycles
    CYCLES1_5 = 0,
    ///1: 3.5 ADC clock cycles
    CYCLES3_5 = 1,
    ///2: 7.5 ADC clock cycles
    CYCLES7_5 = 2,
    ///3: 12.5 ADC clock cycles
    CYCLES12_5 = 3,
    ///4: 19.5 ADC clock cycles
    CYCLES19_5 = 4,
    ///5: 39.5 ADC clock cycles
    CYCLES39_5 = 5,
    ///6: 79.5 ADC clock cycles
    CYCLES79_5 = 6,
    ///7: 160.5 ADC clock cycles
    CYCLES160_5 = 7,
}
impl From<SMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP1_A) -> Self {
        variant as _
    }
}
///Field `SMP1` reader - SMP1
pub struct SMP1_R(crate::FieldReader<u8, SMP1_A>);
impl SMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMP1_A {
        match self.bits {
            0 => SMP1_A::CYCLES1_5,
            1 => SMP1_A::CYCLES3_5,
            2 => SMP1_A::CYCLES7_5,
            3 => SMP1_A::CYCLES12_5,
            4 => SMP1_A::CYCLES19_5,
            5 => SMP1_A::CYCLES39_5,
            6 => SMP1_A::CYCLES79_5,
            7 => SMP1_A::CYCLES160_5,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `CYCLES1_5`
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        **self == SMP1_A::CYCLES1_5
    }
    ///Checks if the value of the field is `CYCLES3_5`
    #[inline(always)]
    pub fn is_cycles3_5(&self) -> bool {
        **self == SMP1_A::CYCLES3_5
    }
    ///Checks if the value of the field is `CYCLES7_5`
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        **self == SMP1_A::CYCLES7_5
    }
    ///Checks if the value of the field is `CYCLES12_5`
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        **self == SMP1_A::CYCLES12_5
    }
    ///Checks if the value of the field is `CYCLES19_5`
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        **self == SMP1_A::CYCLES19_5
    }
    ///Checks if the value of the field is `CYCLES39_5`
    #[inline(always)]
    pub fn is_cycles39_5(&self) -> bool {
        **self == SMP1_A::CYCLES39_5
    }
    ///Checks if the value of the field is `CYCLES79_5`
    #[inline(always)]
    pub fn is_cycles79_5(&self) -> bool {
        **self == SMP1_A::CYCLES79_5
    }
    ///Checks if the value of the field is `CYCLES160_5`
    #[inline(always)]
    pub fn is_cycles160_5(&self) -> bool {
        **self == SMP1_A::CYCLES160_5
    }
}
impl core::ops::Deref for SMP1_R {
    type Target = crate::FieldReader<u8, SMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SMP1` writer - SMP1
pub struct SMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMP1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES1_5)
    }
    ///3.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles3_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES3_5)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES7_5)
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES12_5)
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES19_5)
    }
    ///39.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles39_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES39_5)
    }
    ///79.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles79_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES79_5)
    }
    ///160.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles160_5(self) -> &'a mut W {
        self.variant(SMP1_A::CYCLES160_5)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
///SMP2
pub type SMP2_A = SMP1_A;
///Field `SMP2` reader - SMP2
pub type SMP2_R = SMP1_R;
///Field `SMP2` writer - SMP2
pub struct SMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMP2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES1_5)
    }
    ///3.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles3_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES3_5)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES7_5)
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES12_5)
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES19_5)
    }
    ///39.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles39_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES39_5)
    }
    ///79.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles79_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES79_5)
    }
    ///160.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles160_5(self) -> &'a mut W {
        self.variant(SMP2_A::CYCLES160_5)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
///SMPSEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSEL0_A {
    ///0: Sampling time of CHANNELx use the setting of SMP1 register
    SMP1 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2 register
    SMP2 = 1,
}
impl From<SMPSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL0` reader - SMPSEL
pub struct SMPSEL0_R(crate::FieldReader<bool, SMPSEL0_A>);
impl SMPSEL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSEL0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMPSEL0_A {
        match self.bits {
            false => SMPSEL0_A::SMP1,
            true => SMPSEL0_A::SMP2,
        }
    }
    ///Checks if the value of the field is `SMP1`
    #[inline(always)]
    pub fn is_smp1(&self) -> bool {
        **self == SMPSEL0_A::SMP1
    }
    ///Checks if the value of the field is `SMP2`
    #[inline(always)]
    pub fn is_smp2(&self) -> bool {
        **self == SMPSEL0_A::SMP2
    }
}
impl core::ops::Deref for SMPSEL0_R {
    type Target = crate::FieldReader<bool, SMPSEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SMPSEL0` writer - SMPSEL
pub struct SMPSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL0_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL0_A::SMP2)
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
///SMPSEL
pub type SMPSEL1_A = SMPSEL0_A;
///Field `SMPSEL1` reader - SMPSEL
pub type SMPSEL1_R = SMPSEL0_R;
///Field `SMPSEL1` writer - SMPSEL
pub struct SMPSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL1_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL1_A::SMP2)
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
///SMPSEL
pub type SMPSEL2_A = SMPSEL0_A;
///Field `SMPSEL2` reader - SMPSEL
pub type SMPSEL2_R = SMPSEL0_R;
///Field `SMPSEL2` writer - SMPSEL
pub struct SMPSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL2_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL2_A::SMP2)
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
///SMPSEL
pub type SMPSEL3_A = SMPSEL0_A;
///Field `SMPSEL3` reader - SMPSEL
pub type SMPSEL3_R = SMPSEL0_R;
///Field `SMPSEL3` writer - SMPSEL
pub struct SMPSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL3_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL3_A::SMP2)
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
///SMPSEL
pub type SMPSEL4_A = SMPSEL0_A;
///Field `SMPSEL4` reader - SMPSEL
pub type SMPSEL4_R = SMPSEL0_R;
///Field `SMPSEL4` writer - SMPSEL
pub struct SMPSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL4_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL4_A::SMP2)
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
///SMPSEL
pub type SMPSEL5_A = SMPSEL0_A;
///Field `SMPSEL5` reader - SMPSEL
pub type SMPSEL5_R = SMPSEL0_R;
///Field `SMPSEL5` writer - SMPSEL
pub struct SMPSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL5_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL5_A::SMP2)
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
///SMPSEL
pub type SMPSEL6_A = SMPSEL0_A;
///Field `SMPSEL6` reader - SMPSEL
pub type SMPSEL6_R = SMPSEL0_R;
///Field `SMPSEL6` writer - SMPSEL
pub struct SMPSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL6_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL6_A::SMP2)
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
///SMPSEL
pub type SMPSEL7_A = SMPSEL0_A;
///Field `SMPSEL7` reader - SMPSEL
pub type SMPSEL7_R = SMPSEL0_R;
///Field `SMPSEL7` writer - SMPSEL
pub struct SMPSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL7_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL7_A::SMP2)
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
///SMPSEL
pub type SMPSEL8_A = SMPSEL0_A;
///Field `SMPSEL8` reader - SMPSEL
pub type SMPSEL8_R = SMPSEL0_R;
///Field `SMPSEL8` writer - SMPSEL
pub struct SMPSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL8_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL8_A::SMP2)
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
///SMPSEL
pub type SMPSEL9_A = SMPSEL0_A;
///Field `SMPSEL9` reader - SMPSEL
pub type SMPSEL9_R = SMPSEL0_R;
///Field `SMPSEL9` writer - SMPSEL
pub struct SMPSEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL9_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL9_A::SMP2)
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
///SMPSEL
pub type SMPSEL10_A = SMPSEL0_A;
///Field `SMPSEL10` reader - SMPSEL
pub type SMPSEL10_R = SMPSEL0_R;
///Field `SMPSEL10` writer - SMPSEL
pub struct SMPSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL10_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL10_A::SMP2)
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
///SMPSEL
pub type SMPSEL11_A = SMPSEL0_A;
///Field `SMPSEL11` reader - SMPSEL
pub type SMPSEL11_R = SMPSEL0_R;
///Field `SMPSEL11` writer - SMPSEL
pub struct SMPSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL11_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL11_A::SMP2)
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
///SMPSEL
pub type SMPSEL12_A = SMPSEL0_A;
///Field `SMPSEL12` reader - SMPSEL
pub type SMPSEL12_R = SMPSEL0_R;
///Field `SMPSEL12` writer - SMPSEL
pub struct SMPSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL12_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL12_A::SMP2)
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
///SMPSEL
pub type SMPSEL13_A = SMPSEL0_A;
///Field `SMPSEL13` reader - SMPSEL
pub type SMPSEL13_R = SMPSEL0_R;
///Field `SMPSEL13` writer - SMPSEL
pub struct SMPSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL13_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL13_A::SMP2)
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
///SMPSEL
pub type SMPSEL14_A = SMPSEL0_A;
///Field `SMPSEL14` reader - SMPSEL
pub type SMPSEL14_R = SMPSEL0_R;
///Field `SMPSEL14` writer - SMPSEL
pub struct SMPSEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL14_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL14_A::SMP2)
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
///SMPSEL
pub type SMPSEL15_A = SMPSEL0_A;
///Field `SMPSEL15` reader - SMPSEL
pub type SMPSEL15_R = SMPSEL0_R;
///Field `SMPSEL15` writer - SMPSEL
pub struct SMPSEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL15_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL15_A::SMP2)
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
///SMPSEL
pub type SMPSEL16_A = SMPSEL0_A;
///Field `SMPSEL16` reader - SMPSEL
pub type SMPSEL16_R = SMPSEL0_R;
///Field `SMPSEL16` writer - SMPSEL
pub struct SMPSEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL16_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL16_A::SMP2)
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
///SMPSEL
pub type SMPSEL17_A = SMPSEL0_A;
///Field `SMPSEL17` reader - SMPSEL
pub type SMPSEL17_R = SMPSEL0_R;
///Field `SMPSEL17` writer - SMPSEL
pub struct SMPSEL17_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL17_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMPSEL17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut W {
        self.variant(SMPSEL17_A::SMP1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut W {
        self.variant(SMPSEL17_A::SMP2)
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
impl R {
    ///Bits 0:2 - SMP1
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 4:6 - SMP2
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 8 - SMPSEL
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - SMPSEL
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - SMPSEL
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - SMPSEL
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - SMPSEL
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - SMPSEL
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - SMPSEL
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - SMPSEL
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - SMPSEL
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - SMPSEL
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - SMPSEL
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - SMPSEL
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - SMPSEL
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - SMPSEL
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - SMPSEL
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - SMPSEL
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - SMPSEL
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - SMPSEL
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:2 - SMP1
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W {
        SMP1_W { w: self }
    }
    ///Bits 4:6 - SMP2
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W {
        SMP2_W { w: self }
    }
    ///Bit 8 - SMPSEL
    #[inline(always)]
    pub fn smpsel0(&mut self) -> SMPSEL0_W {
        SMPSEL0_W { w: self }
    }
    ///Bit 9 - SMPSEL
    #[inline(always)]
    pub fn smpsel1(&mut self) -> SMPSEL1_W {
        SMPSEL1_W { w: self }
    }
    ///Bit 10 - SMPSEL
    #[inline(always)]
    pub fn smpsel2(&mut self) -> SMPSEL2_W {
        SMPSEL2_W { w: self }
    }
    ///Bit 11 - SMPSEL
    #[inline(always)]
    pub fn smpsel3(&mut self) -> SMPSEL3_W {
        SMPSEL3_W { w: self }
    }
    ///Bit 12 - SMPSEL
    #[inline(always)]
    pub fn smpsel4(&mut self) -> SMPSEL4_W {
        SMPSEL4_W { w: self }
    }
    ///Bit 13 - SMPSEL
    #[inline(always)]
    pub fn smpsel5(&mut self) -> SMPSEL5_W {
        SMPSEL5_W { w: self }
    }
    ///Bit 14 - SMPSEL
    #[inline(always)]
    pub fn smpsel6(&mut self) -> SMPSEL6_W {
        SMPSEL6_W { w: self }
    }
    ///Bit 15 - SMPSEL
    #[inline(always)]
    pub fn smpsel7(&mut self) -> SMPSEL7_W {
        SMPSEL7_W { w: self }
    }
    ///Bit 16 - SMPSEL
    #[inline(always)]
    pub fn smpsel8(&mut self) -> SMPSEL8_W {
        SMPSEL8_W { w: self }
    }
    ///Bit 17 - SMPSEL
    #[inline(always)]
    pub fn smpsel9(&mut self) -> SMPSEL9_W {
        SMPSEL9_W { w: self }
    }
    ///Bit 18 - SMPSEL
    #[inline(always)]
    pub fn smpsel10(&mut self) -> SMPSEL10_W {
        SMPSEL10_W { w: self }
    }
    ///Bit 19 - SMPSEL
    #[inline(always)]
    pub fn smpsel11(&mut self) -> SMPSEL11_W {
        SMPSEL11_W { w: self }
    }
    ///Bit 20 - SMPSEL
    #[inline(always)]
    pub fn smpsel12(&mut self) -> SMPSEL12_W {
        SMPSEL12_W { w: self }
    }
    ///Bit 21 - SMPSEL
    #[inline(always)]
    pub fn smpsel13(&mut self) -> SMPSEL13_W {
        SMPSEL13_W { w: self }
    }
    ///Bit 22 - SMPSEL
    #[inline(always)]
    pub fn smpsel14(&mut self) -> SMPSEL14_W {
        SMPSEL14_W { w: self }
    }
    ///Bit 23 - SMPSEL
    #[inline(always)]
    pub fn smpsel15(&mut self) -> SMPSEL15_W {
        SMPSEL15_W { w: self }
    }
    ///Bit 24 - SMPSEL
    #[inline(always)]
    pub fn smpsel16(&mut self) -> SMPSEL16_W {
        SMPSEL16_W { w: self }
    }
    ///Bit 25 - SMPSEL
    #[inline(always)]
    pub fn smpsel17(&mut self) -> SMPSEL17_W {
        SMPSEL17_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC sampling time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpr](index.html) module
pub struct SMPR_SPEC;
impl crate::RegisterSpec for SMPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [smpr::R](R) reader structure
impl crate::Readable for SMPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smpr::W](W) writer structure
impl crate::Writable for SMPR_SPEC {
    type Writer = W;
}
///`reset()` method sets SMPR to value 0
impl crate::Resettable for SMPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
