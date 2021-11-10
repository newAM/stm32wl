///Register `CFGR1` reader
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR1` writer
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///DMAEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    ///0: DMA disabled
    DISABLED = 0,
    ///1: DMA enabled
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMAEN
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAEN_A::ENABLED
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAEN` writer - DMAEN
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    ///DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
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
///DMACFG
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMACFG_A {
    ///0: DMA one shot mode selected
    ONESHOT = 0,
    ///1: DMA circular mode selected
    CIRCULAR = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMACFG` reader - DMACFG
pub struct DMACFG_R(crate::FieldReader<bool, DMACFG_A>);
impl DMACFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMACFG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::ONESHOT,
            true => DMACFG_A::CIRCULAR,
        }
    }
    ///Checks if the value of the field is `ONESHOT`
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        **self == DMACFG_A::ONESHOT
    }
    ///Checks if the value of the field is `CIRCULAR`
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        **self == DMACFG_A::CIRCULAR
    }
}
impl core::ops::Deref for DMACFG_R {
    type Target = crate::FieldReader<bool, DMACFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMACFG` writer - DMACFG
pub struct DMACFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACFG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMACFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA one shot mode selected
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(DMACFG_A::ONESHOT)
    }
    ///DMA circular mode selected
    #[inline(always)]
    pub fn circular(self) -> &'a mut W {
        self.variant(DMACFG_A::CIRCULAR)
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
///SCANDIR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCANDIR_A {
    ///0: Upward scan (from CHSEL0 to CHSEL17)
    UPWARD = 0,
    ///1: Backward scan (from CHSEL17 to CHSEL0)
    BACKWARD = 1,
}
impl From<SCANDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SCANDIR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCANDIR` reader - SCANDIR
pub struct SCANDIR_R(crate::FieldReader<bool, SCANDIR_A>);
impl SCANDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCANDIR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SCANDIR_A {
        match self.bits {
            false => SCANDIR_A::UPWARD,
            true => SCANDIR_A::BACKWARD,
        }
    }
    ///Checks if the value of the field is `UPWARD`
    #[inline(always)]
    pub fn is_upward(&self) -> bool {
        **self == SCANDIR_A::UPWARD
    }
    ///Checks if the value of the field is `BACKWARD`
    #[inline(always)]
    pub fn is_backward(&self) -> bool {
        **self == SCANDIR_A::BACKWARD
    }
}
impl core::ops::Deref for SCANDIR_R {
    type Target = crate::FieldReader<bool, SCANDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SCANDIR` writer - SCANDIR
pub struct SCANDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANDIR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SCANDIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Upward scan (from CHSEL0 to CHSEL17)
    #[inline(always)]
    pub fn upward(self) -> &'a mut W {
        self.variant(SCANDIR_A::UPWARD)
    }
    ///Backward scan (from CHSEL17 to CHSEL0)
    #[inline(always)]
    pub fn backward(self) -> &'a mut W {
        self.variant(SCANDIR_A::BACKWARD)
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
///RES
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_A {
    ///0: 12 bits
    BITS12 = 0,
    ///1: 10 bits
    BITS10 = 1,
    ///2: 8 bits
    BITS8 = 2,
    ///3: 6 bits
    BITS6 = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
///Field `RES` reader - RES
pub struct RES_R(crate::FieldReader<u8, RES_A>);
impl RES_R {
    pub(crate) fn new(bits: u8) -> Self {
        RES_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::BITS12,
            1 => RES_A::BITS10,
            2 => RES_A::BITS8,
            3 => RES_A::BITS6,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `BITS12`
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        **self == RES_A::BITS12
    }
    ///Checks if the value of the field is `BITS10`
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        **self == RES_A::BITS10
    }
    ///Checks if the value of the field is `BITS8`
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        **self == RES_A::BITS8
    }
    ///Checks if the value of the field is `BITS6`
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        **self == RES_A::BITS6
    }
}
impl core::ops::Deref for RES_R {
    type Target = crate::FieldReader<u8, RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RES` writer - RES
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///12 bits
    #[inline(always)]
    pub fn bits12(self) -> &'a mut W {
        self.variant(RES_A::BITS12)
    }
    ///10 bits
    #[inline(always)]
    pub fn bits10(self) -> &'a mut W {
        self.variant(RES_A::BITS10)
    }
    ///8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(RES_A::BITS8)
    }
    ///6 bits
    #[inline(always)]
    pub fn bits6(self) -> &'a mut W {
        self.variant(RES_A::BITS6)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
///ALIGN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN_A {
    ///0: Right alignment
    RIGHT = 0,
    ///1: Left alignment
    LEFT = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALIGN` reader - ALIGN
pub struct ALIGN_R(crate::FieldReader<bool, ALIGN_A>);
impl ALIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::RIGHT,
            true => ALIGN_A::LEFT,
        }
    }
    ///Checks if the value of the field is `RIGHT`
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        **self == ALIGN_A::RIGHT
    }
    ///Checks if the value of the field is `LEFT`
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        **self == ALIGN_A::LEFT
    }
}
impl core::ops::Deref for ALIGN_R {
    type Target = crate::FieldReader<bool, ALIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALIGN` writer - ALIGN
pub struct ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALIGN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Right alignment
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::RIGHT)
    }
    ///Left alignment
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::LEFT)
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
///EXTSEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTSEL_A {
    ///0: Timer 1 TRGO event
    TIM1_TRGO = 0,
    ///1: Timer 1 CC4 event
    TIM1_CC4 = 1,
    ///2: Timer 2 TRGO event
    TIM2_TRGO = 2,
    ///3: Timer 2 CH4 event
    TIM2_CH4 = 3,
    ///5: Timer 2 CH3 event
    TIM2_CH3 = 5,
    ///7: EXTI line 11 event
    EXTI_LINE11 = 7,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
///Field `EXTSEL` reader - EXTSEL
pub struct EXTSEL_R(crate::FieldReader<u8, EXTSEL_A>);
impl EXTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTSEL_A> {
        match self.bits {
            0 => Some(EXTSEL_A::TIM1_TRGO),
            1 => Some(EXTSEL_A::TIM1_CC4),
            2 => Some(EXTSEL_A::TIM2_TRGO),
            3 => Some(EXTSEL_A::TIM2_CH4),
            5 => Some(EXTSEL_A::TIM2_CH3),
            7 => Some(EXTSEL_A::EXTI_LINE11),
            _ => None,
        }
    }
    ///Checks if the value of the field is `TIM1_TRGO`
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM1_TRGO
    }
    ///Checks if the value of the field is `TIM1_CC4`
    #[inline(always)]
    pub fn is_tim1_cc4(&self) -> bool {
        **self == EXTSEL_A::TIM1_CC4
    }
    ///Checks if the value of the field is `TIM2_TRGO`
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM2_TRGO
    }
    ///Checks if the value of the field is `TIM2_CH4`
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        **self == EXTSEL_A::TIM2_CH4
    }
    ///Checks if the value of the field is `TIM2_CH3`
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        **self == EXTSEL_A::TIM2_CH3
    }
    ///Checks if the value of the field is `EXTI_LINE11`
    #[inline(always)]
    pub fn is_exti_line11(&self) -> bool {
        **self == EXTSEL_A::EXTI_LINE11
    }
}
impl core::ops::Deref for EXTSEL_R {
    type Target = crate::FieldReader<u8, EXTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTSEL` writer - EXTSEL
pub struct EXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_TRGO)
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1_cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_CC4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2_TRGO)
    }
    ///Timer 2 CH4 event
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2_CH4)
    }
    ///Timer 2 CH3 event
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2_CH3)
    }
    ///EXTI line 11 event
    #[inline(always)]
    pub fn exti_line11(self) -> &'a mut W {
        self.variant(EXTSEL_A::EXTI_LINE11)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
///EXTEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTEN_A {
    ///0: Hardware trigger detection disabled
    DISABLED = 0,
    ///1: Hardware trigger detection on the rising edge
    RISINGEDGE = 1,
    ///2: Hardware trigger detection on the falling edge
    FALLINGEDGE = 2,
    ///3: Hardware trigger detection on both the rising and falling edges
    BOTHEDGES = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
///Field `EXTEN` reader - EXTEN
pub struct EXTEN_R(crate::FieldReader<u8, EXTEN_A>);
impl EXTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::DISABLED,
            1 => EXTEN_A::RISINGEDGE,
            2 => EXTEN_A::FALLINGEDGE,
            3 => EXTEN_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EXTEN_A::DISABLED
    }
    ///Checks if the value of the field is `RISINGEDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == EXTEN_A::RISINGEDGE
    }
    ///Checks if the value of the field is `FALLINGEDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == EXTEN_A::FALLINGEDGE
    }
    ///Checks if the value of the field is `BOTHEDGES`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        **self == EXTEN_A::BOTHEDGES
    }
}
impl core::ops::Deref for EXTEN_R {
    type Target = crate::FieldReader<u8, EXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTEN` writer - EXTEN
pub struct EXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Hardware trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTEN_A::DISABLED)
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::RISINGEDGE)
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::FALLINGEDGE)
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTEN_A::BOTHEDGES)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///OVRMOD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRMOD_A {
    ///0: ADC_DR register is preserved with the old data when an overrun is detected
    PRESERVE = 0,
    ///1: ADC_DR register is overwritten with the last conversion result when an overrun is detected
    OVERWRITE = 1,
}
impl From<OVRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRMOD` reader - OVRMOD
pub struct OVRMOD_R(crate::FieldReader<bool, OVRMOD_A>);
impl OVRMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRMOD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRMOD_A {
        match self.bits {
            false => OVRMOD_A::PRESERVE,
            true => OVRMOD_A::OVERWRITE,
        }
    }
    ///Checks if the value of the field is `PRESERVE`
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        **self == OVRMOD_A::PRESERVE
    }
    ///Checks if the value of the field is `OVERWRITE`
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        **self == OVRMOD_A::OVERWRITE
    }
}
impl core::ops::Deref for OVRMOD_R {
    type Target = crate::FieldReader<bool, OVRMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OVRMOD` writer - OVRMOD
pub struct OVRMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRMOD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVRMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///ADC_DR register is preserved with the old data when an overrun is detected
    #[inline(always)]
    pub fn preserve(self) -> &'a mut W {
        self.variant(OVRMOD_A::PRESERVE)
    }
    ///ADC_DR register is overwritten with the last conversion result when an overrun is detected
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut W {
        self.variant(OVRMOD_A::OVERWRITE)
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
///CONT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    ///0: Single conversion mode
    SINGLE = 0,
    ///1: Continuous conversion mode
    CONTINUOUS = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CONT` reader - CONT
pub struct CONT_R(crate::FieldReader<bool, CONT_A>);
impl CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::SINGLE,
            true => CONT_A::CONTINUOUS,
        }
    }
    ///Checks if the value of the field is `SINGLE`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == CONT_A::SINGLE
    }
    ///Checks if the value of the field is `CONTINUOUS`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == CONT_A::CONTINUOUS
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, CONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CONT` writer - CONT
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Single conversion mode
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::SINGLE)
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::CONTINUOUS)
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
///WAIT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_A {
    ///0: Wait conversion mode off
    DISABLED = 0,
    ///1: Wait conversion mode on
    ENABLED = 1,
}
impl From<WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WAIT` reader - WAIT
pub struct WAIT_R(crate::FieldReader<bool, WAIT_A>);
impl WAIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAIT_A {
        match self.bits {
            false => WAIT_A::DISABLED,
            true => WAIT_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WAIT_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WAIT_A::ENABLED
    }
}
impl core::ops::Deref for WAIT_R {
    type Target = crate::FieldReader<bool, WAIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAIT` writer - WAIT
pub struct WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WAIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Wait conversion mode off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAIT_A::DISABLED)
    }
    ///Wait conversion mode on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAIT_A::ENABLED)
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
///AUTOFF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOFF_A {
    ///0: Auto-off mode disabled
    DISABLED = 0,
    ///1: Auto-off mode enabled
    ENABLED = 1,
}
impl From<AUTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOFF` reader - AUTOFF
pub struct AUTOFF_R(crate::FieldReader<bool, AUTOFF_A>);
impl AUTOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTOFF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AUTOFF_A {
        match self.bits {
            false => AUTOFF_A::DISABLED,
            true => AUTOFF_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AUTOFF_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AUTOFF_A::ENABLED
    }
}
impl core::ops::Deref for AUTOFF_R {
    type Target = crate::FieldReader<bool, AUTOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AUTOFF` writer - AUTOFF
pub struct AUTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOFF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AUTOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Auto-off mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOFF_A::DISABLED)
    }
    ///Auto-off mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOFF_A::ENABLED)
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
///DISCEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCEN_A {
    ///0: Discontinuous mode disabled
    DISABLED = 0,
    ///1: Discontinuous mode enabled
    ENABLED = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DISCEN` reader - DISCEN
pub struct DISCEN_R(crate::FieldReader<bool, DISCEN_A>);
impl DISCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISCEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::DISABLED,
            true => DISCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DISCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DISCEN_A::ENABLED
    }
}
impl core::ops::Deref for DISCEN_R {
    type Target = crate::FieldReader<bool, DISCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DISCEN` writer - DISCEN
pub struct DISCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DISCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Discontinuous mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::DISABLED)
    }
    ///Discontinuous mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::ENABLED)
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
///CHSELRMOD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELRMOD_A {
    ///0: Each bit of the ADC_CHSELR register enables an input
    BITPERINPUT = 0,
    ///1: ADC_CHSELR register is able to sequence up to 8 channels
    SEQUENCE = 1,
}
impl From<CHSELRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELRMOD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSELRMOD` reader - CHSELRMOD
pub struct CHSELRMOD_R(crate::FieldReader<bool, CHSELRMOD_A>);
impl CHSELRMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSELRMOD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CHSELRMOD_A {
        match self.bits {
            false => CHSELRMOD_A::BITPERINPUT,
            true => CHSELRMOD_A::SEQUENCE,
        }
    }
    ///Checks if the value of the field is `BITPERINPUT`
    #[inline(always)]
    pub fn is_bit_per_input(&self) -> bool {
        **self == CHSELRMOD_A::BITPERINPUT
    }
    ///Checks if the value of the field is `SEQUENCE`
    #[inline(always)]
    pub fn is_sequence(&self) -> bool {
        **self == CHSELRMOD_A::SEQUENCE
    }
}
impl core::ops::Deref for CHSELRMOD_R {
    type Target = crate::FieldReader<bool, CHSELRMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CHSELRMOD` writer - CHSELRMOD
pub struct CHSELRMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSELRMOD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CHSELRMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Each bit of the ADC_CHSELR register enables an input
    #[inline(always)]
    pub fn bit_per_input(self) -> &'a mut W {
        self.variant(CHSELRMOD_A::BITPERINPUT)
    }
    ///ADC_CHSELR register is able to sequence up to 8 channels
    #[inline(always)]
    pub fn sequence(self) -> &'a mut W {
        self.variant(CHSELRMOD_A::SEQUENCE)
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
///AWD1SGL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1SGL_A {
    ///0: Analog watchdog 1 enabled on all channels
    ALLCHANNELS = 0,
    ///1: Analog watchdog 1 enabled on a single channel
    SINGLECHANNEL = 1,
}
impl From<AWD1SGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1SGL` reader - AWD1SGL
pub struct AWD1SGL_R(crate::FieldReader<bool, AWD1SGL_A>);
impl AWD1SGL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1SGL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD1SGL_A {
        match self.bits {
            false => AWD1SGL_A::ALLCHANNELS,
            true => AWD1SGL_A::SINGLECHANNEL,
        }
    }
    ///Checks if the value of the field is `ALLCHANNELS`
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        **self == AWD1SGL_A::ALLCHANNELS
    }
    ///Checks if the value of the field is `SINGLECHANNEL`
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        **self == AWD1SGL_A::SINGLECHANNEL
    }
}
impl core::ops::Deref for AWD1SGL_R {
    type Target = crate::FieldReader<bool, AWD1SGL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AWD1SGL` writer - AWD1SGL
pub struct AWD1SGL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1SGL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD1SGL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut W {
        self.variant(AWD1SGL_A::ALLCHANNELS)
    }
    ///Analog watchdog 1 enabled on a single channel
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(AWD1SGL_A::SINGLECHANNEL)
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
///AWD1EN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1EN_A {
    ///0: Analog watchdog 1 disabled
    DISABLED = 0,
    ///1: Analog watchdog 1 enabled
    ENABLED = 1,
}
impl From<AWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1EN` reader - AWD1EN
pub struct AWD1EN_R(crate::FieldReader<bool, AWD1EN_A>);
impl AWD1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1EN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD1EN_A {
        match self.bits {
            false => AWD1EN_A::DISABLED,
            true => AWD1EN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AWD1EN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AWD1EN_A::ENABLED
    }
}
impl core::ops::Deref for AWD1EN_R {
    type Target = crate::FieldReader<bool, AWD1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AWD1EN` writer - AWD1EN
pub struct AWD1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Analog watchdog 1 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::DISABLED)
    }
    ///Analog watchdog 1 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::ENABLED)
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
///Field `AWD1CH` reader - AWD1CH
pub struct AWD1CH_R(crate::FieldReader<u8, u8>);
impl AWD1CH_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWD1CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD1CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AWD1CH` writer - AWD1CH
pub struct AWD1CH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1CH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
impl R {
    ///Bit 0 - DMAEN
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - DMACFG
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - SCANDIR
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bits 3:4 - RES
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    ///Bit 5 - ALIGN
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bits 6:8 - EXTSEL
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    ///Bits 10:11 - EXTEN
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bit 12 - OVRMOD
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - CONT
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - WAIT
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - AUTOFF
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 21 - CHSELRMOD
    #[inline(always)]
    pub fn chselrmod(&self) -> CHSELRMOD_R {
        CHSELRMOD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - AWD1SGL
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - AWD1EN
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bits 26:30 - AWD1CH
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - DMAEN
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    ///Bit 1 - DMACFG
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W {
        DMACFG_W { w: self }
    }
    ///Bit 2 - SCANDIR
    #[inline(always)]
    pub fn scandir(&mut self) -> SCANDIR_W {
        SCANDIR_W { w: self }
    }
    ///Bits 3:4 - RES
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    ///Bit 5 - ALIGN
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W {
        ALIGN_W { w: self }
    }
    ///Bits 6:8 - EXTSEL
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W {
        EXTSEL_W { w: self }
    }
    ///Bits 10:11 - EXTEN
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W {
        EXTEN_W { w: self }
    }
    ///Bit 12 - OVRMOD
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W {
        OVRMOD_W { w: self }
    }
    ///Bit 13 - CONT
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    ///Bit 14 - WAIT
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W {
        WAIT_W { w: self }
    }
    ///Bit 15 - AUTOFF
    #[inline(always)]
    pub fn autoff(&mut self) -> AUTOFF_W {
        AUTOFF_W { w: self }
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W {
        DISCEN_W { w: self }
    }
    ///Bit 21 - CHSELRMOD
    #[inline(always)]
    pub fn chselrmod(&mut self) -> CHSELRMOD_W {
        CHSELRMOD_W { w: self }
    }
    ///Bit 22 - AWD1SGL
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W {
        AWD1SGL_W { w: self }
    }
    ///Bit 23 - AWD1EN
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W {
        AWD1EN_W { w: self }
    }
    ///Bits 26:30 - AWD1CH
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W {
        AWD1CH_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr1](index.html) module
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr1::R](R) reader structure
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr1::W](W) writer structure
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
