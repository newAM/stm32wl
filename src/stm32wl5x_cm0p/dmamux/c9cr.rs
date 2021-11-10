///Register `C9CR` reader
pub struct R(crate::R<C9CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C9CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C9CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C9CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C9CR` writer
pub struct W(crate::W<C9CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C9CR_SPEC>;
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
impl From<crate::W<C9CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C9CR_SPEC>) -> Self {
        W(writer)
    }
}
///Synchronization identification
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC_ID_A {
    ///0: Signal `EXTIx` selected as synchronization input
    EXTI0 = 0,
    ///1: Signal `EXTIx` selected as synchronization input
    EXTI1 = 1,
    ///2: Signal `EXTIx` selected as synchronization input
    EXTI2 = 2,
    ///3: Signal `EXTIx` selected as synchronization input
    EXTI3 = 3,
    ///4: Signal `EXTIx` selected as synchronization input
    EXTI4 = 4,
    ///5: Signal `EXTIx` selected as synchronization input
    EXTI5 = 5,
    ///6: Signal `EXTIx` selected as synchronization input
    EXTI6 = 6,
    ///7: Signal `EXTIx` selected as synchronization input
    EXTI7 = 7,
    ///8: Signal `EXTIx` selected as synchronization input
    EXTI8 = 8,
    ///9: Signal `EXTIx` selected as synchronization input
    EXTI9 = 9,
    ///10: Signal `EXTIx` selected as synchronization input
    EXTI10 = 10,
    ///11: Signal `EXTIx` selected as synchronization input
    EXTI11 = 11,
    ///12: Signal `EXTIx` selected as synchronization input
    EXTI12 = 12,
    ///13: Signal `EXTIx` selected as synchronization input
    EXTI13 = 13,
    ///14: Signal `EXTIx` selected as synchronization input
    EXTI14 = 14,
    ///15: Signal `EXTIx` selected as synchronization input
    EXTI15 = 15,
    ///16: Signal `dmamux1_evt0` selected as synchronization input
    DMAMUX1_EVT0 = 16,
    ///17: Signal `dmamux1_evt1` selected as synchronization input
    DMAMUX1_EVT1 = 17,
    ///18: Signal `lptim1_out` selected as synchronization input
    LPTIM1_OUT = 18,
    ///19: Signal `lptim2_out` selected as synchronization input
    LPTIM2_OUT = 19,
    ///20: Signal `lptim3_out` selected as synchronization input
    LPTIM3_OUT = 20,
}
impl From<SYNC_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_ID_A) -> Self {
        variant as _
    }
}
///Field `SYNC_ID` reader - Synchronization identification
pub struct SYNC_ID_R(crate::FieldReader<u8, SYNC_ID_A>);
impl SYNC_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNC_ID_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNC_ID_A> {
        match self.bits {
            0 => Some(SYNC_ID_A::EXTI0),
            1 => Some(SYNC_ID_A::EXTI1),
            2 => Some(SYNC_ID_A::EXTI2),
            3 => Some(SYNC_ID_A::EXTI3),
            4 => Some(SYNC_ID_A::EXTI4),
            5 => Some(SYNC_ID_A::EXTI5),
            6 => Some(SYNC_ID_A::EXTI6),
            7 => Some(SYNC_ID_A::EXTI7),
            8 => Some(SYNC_ID_A::EXTI8),
            9 => Some(SYNC_ID_A::EXTI9),
            10 => Some(SYNC_ID_A::EXTI10),
            11 => Some(SYNC_ID_A::EXTI11),
            12 => Some(SYNC_ID_A::EXTI12),
            13 => Some(SYNC_ID_A::EXTI13),
            14 => Some(SYNC_ID_A::EXTI14),
            15 => Some(SYNC_ID_A::EXTI15),
            16 => Some(SYNC_ID_A::DMAMUX1_EVT0),
            17 => Some(SYNC_ID_A::DMAMUX1_EVT1),
            18 => Some(SYNC_ID_A::LPTIM1_OUT),
            19 => Some(SYNC_ID_A::LPTIM2_OUT),
            20 => Some(SYNC_ID_A::LPTIM3_OUT),
            _ => None,
        }
    }
    ///Checks if the value of the field is `EXTI0`
    #[inline(always)]
    pub fn is_exti0(&self) -> bool {
        **self == SYNC_ID_A::EXTI0
    }
    ///Checks if the value of the field is `EXTI1`
    #[inline(always)]
    pub fn is_exti1(&self) -> bool {
        **self == SYNC_ID_A::EXTI1
    }
    ///Checks if the value of the field is `EXTI2`
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        **self == SYNC_ID_A::EXTI2
    }
    ///Checks if the value of the field is `EXTI3`
    #[inline(always)]
    pub fn is_exti3(&self) -> bool {
        **self == SYNC_ID_A::EXTI3
    }
    ///Checks if the value of the field is `EXTI4`
    #[inline(always)]
    pub fn is_exti4(&self) -> bool {
        **self == SYNC_ID_A::EXTI4
    }
    ///Checks if the value of the field is `EXTI5`
    #[inline(always)]
    pub fn is_exti5(&self) -> bool {
        **self == SYNC_ID_A::EXTI5
    }
    ///Checks if the value of the field is `EXTI6`
    #[inline(always)]
    pub fn is_exti6(&self) -> bool {
        **self == SYNC_ID_A::EXTI6
    }
    ///Checks if the value of the field is `EXTI7`
    #[inline(always)]
    pub fn is_exti7(&self) -> bool {
        **self == SYNC_ID_A::EXTI7
    }
    ///Checks if the value of the field is `EXTI8`
    #[inline(always)]
    pub fn is_exti8(&self) -> bool {
        **self == SYNC_ID_A::EXTI8
    }
    ///Checks if the value of the field is `EXTI9`
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        **self == SYNC_ID_A::EXTI9
    }
    ///Checks if the value of the field is `EXTI10`
    #[inline(always)]
    pub fn is_exti10(&self) -> bool {
        **self == SYNC_ID_A::EXTI10
    }
    ///Checks if the value of the field is `EXTI11`
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        **self == SYNC_ID_A::EXTI11
    }
    ///Checks if the value of the field is `EXTI12`
    #[inline(always)]
    pub fn is_exti12(&self) -> bool {
        **self == SYNC_ID_A::EXTI12
    }
    ///Checks if the value of the field is `EXTI13`
    #[inline(always)]
    pub fn is_exti13(&self) -> bool {
        **self == SYNC_ID_A::EXTI13
    }
    ///Checks if the value of the field is `EXTI14`
    #[inline(always)]
    pub fn is_exti14(&self) -> bool {
        **self == SYNC_ID_A::EXTI14
    }
    ///Checks if the value of the field is `EXTI15`
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        **self == SYNC_ID_A::EXTI15
    }
    ///Checks if the value of the field is `DMAMUX1_EVT0`
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        **self == SYNC_ID_A::DMAMUX1_EVT0
    }
    ///Checks if the value of the field is `DMAMUX1_EVT1`
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        **self == SYNC_ID_A::DMAMUX1_EVT1
    }
    ///Checks if the value of the field is `LPTIM1_OUT`
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        **self == SYNC_ID_A::LPTIM1_OUT
    }
    ///Checks if the value of the field is `LPTIM2_OUT`
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        **self == SYNC_ID_A::LPTIM2_OUT
    }
    ///Checks if the value of the field is `LPTIM3_OUT`
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        **self == SYNC_ID_A::LPTIM3_OUT
    }
}
impl core::ops::Deref for SYNC_ID_R {
    type Target = crate::FieldReader<u8, SYNC_ID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNC_ID` writer - Synchronization identification
pub struct SYNC_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_ID_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SYNC_ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti0(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI0)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti1(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI1)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti2(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI2)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti3(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI3)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti4(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI4)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti5(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI5)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti6(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI6)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti7(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI7)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti8(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI8)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI9)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti10(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI10)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI11)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti12(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI12)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti13(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI13)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti14(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI14)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTI15)
    }
    ///Signal `dmamux1_evt0` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX1_EVT0)
    }
    ///Signal `dmamux1_evt1` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX1_EVT1)
    }
    ///Signal `lptim1_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::LPTIM1_OUT)
    }
    ///Signal `lptim2_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::LPTIM2_OUT)
    }
    ///Signal `lptim3_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::LPTIM3_OUT)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
///Field `NBREQ` reader - Number of DMA requests minus 1 to forward
pub struct NBREQ_R(crate::FieldReader<u8, u8>);
impl NBREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NBREQ` writer - Number of DMA requests minus 1 to forward
pub struct NBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NBREQ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
///Synchronization polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPOL_A {
    ///0: No event, i.e. no synchronization nor detection
    NOEDGE = 0,
    ///1: Rising edge
    RISINGEDGE = 1,
    ///2: Falling edge
    FALLINGEDGE = 2,
    ///3: Rising and falling edges
    BOTHEDGES = 3,
}
impl From<SPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as _
    }
}
///Field `SPOL` reader - Synchronization polarity
pub struct SPOL_R(crate::FieldReader<u8, SPOL_A>);
impl SPOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPOL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            0 => SPOL_A::NOEDGE,
            1 => SPOL_A::RISINGEDGE,
            2 => SPOL_A::FALLINGEDGE,
            3 => SPOL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOEDGE`
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        **self == SPOL_A::NOEDGE
    }
    ///Checks if the value of the field is `RISINGEDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == SPOL_A::RISINGEDGE
    }
    ///Checks if the value of the field is `FALLINGEDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == SPOL_A::FALLINGEDGE
    }
    ///Checks if the value of the field is `BOTHEDGES`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        **self == SPOL_A::BOTHEDGES
    }
}
impl core::ops::Deref for SPOL_R {
    type Target = crate::FieldReader<u8, SPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SPOL` writer - Synchronization polarity
pub struct SPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPOL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No event, i.e. no synchronization nor detection
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(SPOL_A::NOEDGE)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(SPOL_A::RISINGEDGE)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(SPOL_A::FALLINGEDGE)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(SPOL_A::BOTHEDGES)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
///Synchronization enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SE_A {
    ///0: Synchronization disabled
    DISABLED = 0,
    ///1: Synchronization enabled
    ENABLED = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SE` reader - Synchronization enable
pub struct SE_R(crate::FieldReader<bool, SE_A>);
impl SE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::DISABLED,
            true => SE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SE_A::ENABLED
    }
}
impl core::ops::Deref for SE_R {
    type Target = crate::FieldReader<bool, SE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SE` writer - Synchronization enable
pub struct SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Synchronization disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SE_A::DISABLED)
    }
    ///Synchronization enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SE_A::ENABLED)
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
///Event generation enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGE_A {
    ///0: Event generation disabled
    DISABLED = 0,
    ///1: Event generation enabled
    ENABLED = 1,
}
impl From<EGE_A> for bool {
    #[inline(always)]
    fn from(variant: EGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EGE` reader - Event generation enable
pub struct EGE_R(crate::FieldReader<bool, EGE_A>);
impl EGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EGE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EGE_A {
        match self.bits {
            false => EGE_A::DISABLED,
            true => EGE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EGE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EGE_A::ENABLED
    }
}
impl core::ops::Deref for EGE_R {
    type Target = crate::FieldReader<bool, EGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EGE` writer - Event generation enable
pub struct EGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EGE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Event generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EGE_A::DISABLED)
    }
    ///Event generation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EGE_A::ENABLED)
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
///Synchronization overrun interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOIE_A {
    ///0: Synchronization overrun interrupt disabled
    DISABLED = 0,
    ///1: Synchronization overrun interrupt enabled
    ENABLED = 1,
}
impl From<SOIE_A> for bool {
    #[inline(always)]
    fn from(variant: SOIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SOIE` reader - Synchronization overrun interrupt enable
pub struct SOIE_R(crate::FieldReader<bool, SOIE_A>);
impl SOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SOIE_A {
        match self.bits {
            false => SOIE_A::DISABLED,
            true => SOIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SOIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SOIE_A::ENABLED
    }
}
impl core::ops::Deref for SOIE_R {
    type Target = crate::FieldReader<bool, SOIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SOIE` writer - Synchronization overrun interrupt enable
pub struct SOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SOIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Synchronization overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOIE_A::DISABLED)
    }
    ///Synchronization overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOIE_A::ENABLED)
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
///DMA request identification
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMAREQ_ID_A {
    ///0: No signal selected as request input
    NONE = 0,
    ///1: Signal `dmamux1_req_gen0` selected as request input
    DMAMUX1_REQ_GEN0 = 1,
    ///2: Signal `dmamux1_req_gen1` selected as request input
    DMAMUX1_REQ_GEN1 = 2,
    ///3: Signal `dmamux1_req_gen2` selected as request input
    DMAMUX1_REQ_GEN2 = 3,
    ///4: Signal `dmamux1_req_gen3` selected as request input
    DMAMUX1_REQ_GEN3 = 4,
    ///5: Signal `adc1_dma` selected as request input
    ADC = 5,
    ///6: Signal `dac_out1_dma` selected as request input
    DAT_OUT1 = 6,
    ///7: Signal `spi1_rx_dma` selected as request input
    SPI1_RX_DMA = 7,
    ///8: Signal `spi1_tx_dma` selected as request input
    SPI1_TX_DMA = 8,
    ///9: Signal `spi2_rx_dma` selected as request input
    SPI2_RX_DMA = 9,
    ///10: Signal `spi2_tx_dma` selected as request input
    SPI2_TX_DMA = 10,
    ///11: Signal `i2c1_rx_dma` selected as request input
    I2C1_RX_DMA = 11,
    ///12: Signal `i2c1_tx_dma` selected as request input
    I2C1_TX_DMA = 12,
    ///13: Signal `i2c2_rx_dma` selected as request input
    I2C2_RX_DMA = 13,
    ///14: Signal `i2c2_tx_dma` selected as request input
    I2C2_TX_DMA = 14,
    ///15: Signal `i2c3_rx_dma` selected as request input
    I2C3_RX_DMA = 15,
    ///16: Signal `i2c3_tx_dma` selected as request input
    I2C3_TX_DMA = 16,
    ///17: Signal `usart1_rx_dma` selected as request input
    USART1_RX_DMA = 17,
    ///18: Signal `usart1_tx_dma` selected as request input
    USART1_TX_DMA = 18,
    ///19: Signal `usart2_rx_dma` selected as request input
    USART2_RX_DMA = 19,
    ///20: Signal `usart2_tx_dma` selected as request input
    USART2_TX_DMA = 20,
    ///21: Signal `lpuart1_rx_dma` selected as request input
    LPUART1_RX_DMA = 21,
    ///22: Signal `lpuart1_tx_dma` selected as request input
    LPUART1_TX_DMA = 22,
    ///23: Signal `tim1_ch1` selected as request input
    TIM1_CH1 = 23,
    ///24: Signal `tim1_ch2` selected as request input
    TIM1_CH2 = 24,
    ///25: Signal `tim1_ch3` selected as request input
    TIM1_CH3 = 25,
    ///26: Signal `tim1_ch4` selected as request input
    TIM1_CH4 = 26,
    ///27: Signal `tim1_up` selected as request input
    TIM1_UP = 27,
    ///28: Signal `tim1_trig` selected as request input
    TIM1_TRIG = 28,
    ///29: Signal `tim1_com` selected as request input
    TIM1_COM = 29,
    ///30: Signal `tim2_ch1` selected as request input
    TIM2_CH1 = 30,
    ///31: Signal `tim2_ch2` selected as request input
    TIM2_CH2 = 31,
    ///32: Signal `tim2_ch3` selected as request input
    TIM2_CH3 = 32,
    ///33: Signal `tim2_ch4` selected as request input
    TIM2_CH4 = 33,
    ///34: Signal `tim2_up` selected as request input
    TIM2_UP = 34,
    ///35: Signal `tim16_ch1` selected as request input
    TIM16_CH1 = 35,
    ///36: Signal `tim16_up` selected as request input
    TIM16_UP = 36,
    ///37: Signal `tim17_ch1` selected as request input
    TIM17_CH1 = 37,
    ///38: Signal `tim17_up` selected as request input
    TIM17_UP = 38,
    ///39: Signal `aes_in` selected as request input
    AES_IN = 39,
    ///40: Signal `aes_out` selected as request input
    AES_OUT = 40,
    ///41: Signal `subghzspi_rx` selected as request input
    SUBGHZSPI_RX = 41,
    ///42: Signal `subghzspi_tx` selected as request input
    SUBGHZSPI_TX = 42,
}
impl From<DMAREQ_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAREQ_ID_A) -> Self {
        variant as _
    }
}
///Field `DMAREQ_ID` reader - DMA request identification
pub struct DMAREQ_ID_R(crate::FieldReader<u8, DMAREQ_ID_A>);
impl DMAREQ_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMAREQ_ID_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAREQ_ID_A> {
        match self.bits {
            0 => Some(DMAREQ_ID_A::NONE),
            1 => Some(DMAREQ_ID_A::DMAMUX1_REQ_GEN0),
            2 => Some(DMAREQ_ID_A::DMAMUX1_REQ_GEN1),
            3 => Some(DMAREQ_ID_A::DMAMUX1_REQ_GEN2),
            4 => Some(DMAREQ_ID_A::DMAMUX1_REQ_GEN3),
            5 => Some(DMAREQ_ID_A::ADC),
            6 => Some(DMAREQ_ID_A::DAT_OUT1),
            7 => Some(DMAREQ_ID_A::SPI1_RX_DMA),
            8 => Some(DMAREQ_ID_A::SPI1_TX_DMA),
            9 => Some(DMAREQ_ID_A::SPI2_RX_DMA),
            10 => Some(DMAREQ_ID_A::SPI2_TX_DMA),
            11 => Some(DMAREQ_ID_A::I2C1_RX_DMA),
            12 => Some(DMAREQ_ID_A::I2C1_TX_DMA),
            13 => Some(DMAREQ_ID_A::I2C2_RX_DMA),
            14 => Some(DMAREQ_ID_A::I2C2_TX_DMA),
            15 => Some(DMAREQ_ID_A::I2C3_RX_DMA),
            16 => Some(DMAREQ_ID_A::I2C3_TX_DMA),
            17 => Some(DMAREQ_ID_A::USART1_RX_DMA),
            18 => Some(DMAREQ_ID_A::USART1_TX_DMA),
            19 => Some(DMAREQ_ID_A::USART2_RX_DMA),
            20 => Some(DMAREQ_ID_A::USART2_TX_DMA),
            21 => Some(DMAREQ_ID_A::LPUART1_RX_DMA),
            22 => Some(DMAREQ_ID_A::LPUART1_TX_DMA),
            23 => Some(DMAREQ_ID_A::TIM1_CH1),
            24 => Some(DMAREQ_ID_A::TIM1_CH2),
            25 => Some(DMAREQ_ID_A::TIM1_CH3),
            26 => Some(DMAREQ_ID_A::TIM1_CH4),
            27 => Some(DMAREQ_ID_A::TIM1_UP),
            28 => Some(DMAREQ_ID_A::TIM1_TRIG),
            29 => Some(DMAREQ_ID_A::TIM1_COM),
            30 => Some(DMAREQ_ID_A::TIM2_CH1),
            31 => Some(DMAREQ_ID_A::TIM2_CH2),
            32 => Some(DMAREQ_ID_A::TIM2_CH3),
            33 => Some(DMAREQ_ID_A::TIM2_CH4),
            34 => Some(DMAREQ_ID_A::TIM2_UP),
            35 => Some(DMAREQ_ID_A::TIM16_CH1),
            36 => Some(DMAREQ_ID_A::TIM16_UP),
            37 => Some(DMAREQ_ID_A::TIM17_CH1),
            38 => Some(DMAREQ_ID_A::TIM17_UP),
            39 => Some(DMAREQ_ID_A::AES_IN),
            40 => Some(DMAREQ_ID_A::AES_OUT),
            41 => Some(DMAREQ_ID_A::SUBGHZSPI_RX),
            42 => Some(DMAREQ_ID_A::SUBGHZSPI_TX),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NONE`
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == DMAREQ_ID_A::NONE
    }
    ///Checks if the value of the field is `DMAMUX1_REQ_GEN0`
    #[inline(always)]
    pub fn is_dmamux1_req_gen0(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX1_REQ_GEN0
    }
    ///Checks if the value of the field is `DMAMUX1_REQ_GEN1`
    #[inline(always)]
    pub fn is_dmamux1_req_gen1(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX1_REQ_GEN1
    }
    ///Checks if the value of the field is `DMAMUX1_REQ_GEN2`
    #[inline(always)]
    pub fn is_dmamux1_req_gen2(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX1_REQ_GEN2
    }
    ///Checks if the value of the field is `DMAMUX1_REQ_GEN3`
    #[inline(always)]
    pub fn is_dmamux1_req_gen3(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX1_REQ_GEN3
    }
    ///Checks if the value of the field is `ADC`
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        **self == DMAREQ_ID_A::ADC
    }
    ///Checks if the value of the field is `DAT_OUT1`
    #[inline(always)]
    pub fn is_dat_out1(&self) -> bool {
        **self == DMAREQ_ID_A::DAT_OUT1
    }
    ///Checks if the value of the field is `SPI1_RX_DMA`
    #[inline(always)]
    pub fn is_spi1_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::SPI1_RX_DMA
    }
    ///Checks if the value of the field is `SPI1_TX_DMA`
    #[inline(always)]
    pub fn is_spi1_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::SPI1_TX_DMA
    }
    ///Checks if the value of the field is `SPI2_RX_DMA`
    #[inline(always)]
    pub fn is_spi2_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::SPI2_RX_DMA
    }
    ///Checks if the value of the field is `SPI2_TX_DMA`
    #[inline(always)]
    pub fn is_spi2_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::SPI2_TX_DMA
    }
    ///Checks if the value of the field is `I2C1_RX_DMA`
    #[inline(always)]
    pub fn is_i2c1_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::I2C1_RX_DMA
    }
    ///Checks if the value of the field is `I2C1_TX_DMA`
    #[inline(always)]
    pub fn is_i2c1_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::I2C1_TX_DMA
    }
    ///Checks if the value of the field is `I2C2_RX_DMA`
    #[inline(always)]
    pub fn is_i2c2_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::I2C2_RX_DMA
    }
    ///Checks if the value of the field is `I2C2_TX_DMA`
    #[inline(always)]
    pub fn is_i2c2_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::I2C2_TX_DMA
    }
    ///Checks if the value of the field is `I2C3_RX_DMA`
    #[inline(always)]
    pub fn is_i2c3_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::I2C3_RX_DMA
    }
    ///Checks if the value of the field is `I2C3_TX_DMA`
    #[inline(always)]
    pub fn is_i2c3_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::I2C3_TX_DMA
    }
    ///Checks if the value of the field is `USART1_RX_DMA`
    #[inline(always)]
    pub fn is_usart1_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::USART1_RX_DMA
    }
    ///Checks if the value of the field is `USART1_TX_DMA`
    #[inline(always)]
    pub fn is_usart1_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::USART1_TX_DMA
    }
    ///Checks if the value of the field is `USART2_RX_DMA`
    #[inline(always)]
    pub fn is_usart2_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::USART2_RX_DMA
    }
    ///Checks if the value of the field is `USART2_TX_DMA`
    #[inline(always)]
    pub fn is_usart2_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::USART2_TX_DMA
    }
    ///Checks if the value of the field is `LPUART1_RX_DMA`
    #[inline(always)]
    pub fn is_lpuart1_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::LPUART1_RX_DMA
    }
    ///Checks if the value of the field is `LPUART1_TX_DMA`
    #[inline(always)]
    pub fn is_lpuart1_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::LPUART1_TX_DMA
    }
    ///Checks if the value of the field is `TIM1_CH1`
    #[inline(always)]
    pub fn is_tim1_ch1(&self) -> bool {
        **self == DMAREQ_ID_A::TIM1_CH1
    }
    ///Checks if the value of the field is `TIM1_CH2`
    #[inline(always)]
    pub fn is_tim1_ch2(&self) -> bool {
        **self == DMAREQ_ID_A::TIM1_CH2
    }
    ///Checks if the value of the field is `TIM1_CH3`
    #[inline(always)]
    pub fn is_tim1_ch3(&self) -> bool {
        **self == DMAREQ_ID_A::TIM1_CH3
    }
    ///Checks if the value of the field is `TIM1_CH4`
    #[inline(always)]
    pub fn is_tim1_ch4(&self) -> bool {
        **self == DMAREQ_ID_A::TIM1_CH4
    }
    ///Checks if the value of the field is `TIM1_UP`
    #[inline(always)]
    pub fn is_tim1_up(&self) -> bool {
        **self == DMAREQ_ID_A::TIM1_UP
    }
    ///Checks if the value of the field is `TIM1_TRIG`
    #[inline(always)]
    pub fn is_tim1_trig(&self) -> bool {
        **self == DMAREQ_ID_A::TIM1_TRIG
    }
    ///Checks if the value of the field is `TIM1_COM`
    #[inline(always)]
    pub fn is_tim1_com(&self) -> bool {
        **self == DMAREQ_ID_A::TIM1_COM
    }
    ///Checks if the value of the field is `TIM2_CH1`
    #[inline(always)]
    pub fn is_tim2_ch1(&self) -> bool {
        **self == DMAREQ_ID_A::TIM2_CH1
    }
    ///Checks if the value of the field is `TIM2_CH2`
    #[inline(always)]
    pub fn is_tim2_ch2(&self) -> bool {
        **self == DMAREQ_ID_A::TIM2_CH2
    }
    ///Checks if the value of the field is `TIM2_CH3`
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        **self == DMAREQ_ID_A::TIM2_CH3
    }
    ///Checks if the value of the field is `TIM2_CH4`
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        **self == DMAREQ_ID_A::TIM2_CH4
    }
    ///Checks if the value of the field is `TIM2_UP`
    #[inline(always)]
    pub fn is_tim2_up(&self) -> bool {
        **self == DMAREQ_ID_A::TIM2_UP
    }
    ///Checks if the value of the field is `TIM16_CH1`
    #[inline(always)]
    pub fn is_tim16_ch1(&self) -> bool {
        **self == DMAREQ_ID_A::TIM16_CH1
    }
    ///Checks if the value of the field is `TIM16_UP`
    #[inline(always)]
    pub fn is_tim16_up(&self) -> bool {
        **self == DMAREQ_ID_A::TIM16_UP
    }
    ///Checks if the value of the field is `TIM17_CH1`
    #[inline(always)]
    pub fn is_tim17_ch1(&self) -> bool {
        **self == DMAREQ_ID_A::TIM17_CH1
    }
    ///Checks if the value of the field is `TIM17_UP`
    #[inline(always)]
    pub fn is_tim17_up(&self) -> bool {
        **self == DMAREQ_ID_A::TIM17_UP
    }
    ///Checks if the value of the field is `AES_IN`
    #[inline(always)]
    pub fn is_aes_in(&self) -> bool {
        **self == DMAREQ_ID_A::AES_IN
    }
    ///Checks if the value of the field is `AES_OUT`
    #[inline(always)]
    pub fn is_aes_out(&self) -> bool {
        **self == DMAREQ_ID_A::AES_OUT
    }
    ///Checks if the value of the field is `SUBGHZSPI_RX`
    #[inline(always)]
    pub fn is_subghzspi_rx(&self) -> bool {
        **self == DMAREQ_ID_A::SUBGHZSPI_RX
    }
    ///Checks if the value of the field is `SUBGHZSPI_TX`
    #[inline(always)]
    pub fn is_subghzspi_tx(&self) -> bool {
        **self == DMAREQ_ID_A::SUBGHZSPI_TX
    }
}
impl core::ops::Deref for DMAREQ_ID_R {
    type Target = crate::FieldReader<u8, DMAREQ_ID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAREQ_ID` writer - DMA request identification
pub struct DMAREQ_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQ_ID_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAREQ_ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No signal selected as request input
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::NONE)
    }
    ///Signal `dmamux1_req_gen0` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen0(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN0)
    }
    ///Signal `dmamux1_req_gen1` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN1)
    }
    ///Signal `dmamux1_req_gen2` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN2)
    }
    ///Signal `dmamux1_req_gen3` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN3)
    }
    ///Signal `adc1_dma` selected as request input
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::ADC)
    }
    ///Signal `dac_out1_dma` selected as request input
    #[inline(always)]
    pub fn dat_out1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DAT_OUT1)
    }
    ///Signal `spi1_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI1_RX_DMA)
    }
    ///Signal `spi1_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI1_TX_DMA)
    }
    ///Signal `spi2_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI2_RX_DMA)
    }
    ///Signal `spi2_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI2_TX_DMA)
    }
    ///Signal `i2c1_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C1_RX_DMA)
    }
    ///Signal `i2c1_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C1_TX_DMA)
    }
    ///Signal `i2c2_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C2_RX_DMA)
    }
    ///Signal `i2c2_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C2_TX_DMA)
    }
    ///Signal `i2c3_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c3_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C3_RX_DMA)
    }
    ///Signal `i2c3_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c3_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C3_TX_DMA)
    }
    ///Signal `usart1_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART1_RX_DMA)
    }
    ///Signal `usart1_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART1_TX_DMA)
    }
    ///Signal `usart2_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART2_RX_DMA)
    }
    ///Signal `usart2_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART2_TX_DMA)
    }
    ///Signal `lpuart1_rx_dma` selected as request input
    #[inline(always)]
    pub fn lpuart1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::LPUART1_RX_DMA)
    }
    ///Signal `lpuart1_tx_dma` selected as request input
    #[inline(always)]
    pub fn lpuart1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::LPUART1_TX_DMA)
    }
    ///Signal `tim1_ch1` selected as request input
    #[inline(always)]
    pub fn tim1_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_CH1)
    }
    ///Signal `tim1_ch2` selected as request input
    #[inline(always)]
    pub fn tim1_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_CH2)
    }
    ///Signal `tim1_ch3` selected as request input
    #[inline(always)]
    pub fn tim1_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_CH3)
    }
    ///Signal `tim1_ch4` selected as request input
    #[inline(always)]
    pub fn tim1_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_CH4)
    }
    ///Signal `tim1_up` selected as request input
    #[inline(always)]
    pub fn tim1_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_UP)
    }
    ///Signal `tim1_trig` selected as request input
    #[inline(always)]
    pub fn tim1_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_TRIG)
    }
    ///Signal `tim1_com` selected as request input
    #[inline(always)]
    pub fn tim1_com(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_COM)
    }
    ///Signal `tim2_ch1` selected as request input
    #[inline(always)]
    pub fn tim2_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM2_CH1)
    }
    ///Signal `tim2_ch2` selected as request input
    #[inline(always)]
    pub fn tim2_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM2_CH2)
    }
    ///Signal `tim2_ch3` selected as request input
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM2_CH3)
    }
    ///Signal `tim2_ch4` selected as request input
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM2_CH4)
    }
    ///Signal `tim2_up` selected as request input
    #[inline(always)]
    pub fn tim2_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM2_UP)
    }
    ///Signal `tim16_ch1` selected as request input
    #[inline(always)]
    pub fn tim16_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM16_CH1)
    }
    ///Signal `tim16_up` selected as request input
    #[inline(always)]
    pub fn tim16_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM16_UP)
    }
    ///Signal `tim17_ch1` selected as request input
    #[inline(always)]
    pub fn tim17_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM17_CH1)
    }
    ///Signal `tim17_up` selected as request input
    #[inline(always)]
    pub fn tim17_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM17_UP)
    }
    ///Signal `aes_in` selected as request input
    #[inline(always)]
    pub fn aes_in(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::AES_IN)
    }
    ///Signal `aes_out` selected as request input
    #[inline(always)]
    pub fn aes_out(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::AES_OUT)
    }
    ///Signal `subghzspi_rx` selected as request input
    #[inline(always)]
    pub fn subghzspi_rx(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SUBGHZSPI_RX)
    }
    ///Signal `subghzspi_tx` selected as request input
    #[inline(always)]
    pub fn subghzspi_tx(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SUBGHZSPI_TX)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bits 24:28 - Synchronization identification
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 17:18 - Synchronization polarity
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 0:7 - DMA request identification
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 24:28 - Synchronization identification
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W {
        SYNC_ID_W { w: self }
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W {
        NBREQ_W { w: self }
    }
    ///Bits 17:18 - Synchronization polarity
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W {
        SPOL_W { w: self }
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&mut self) -> SE_W {
        SE_W { w: self }
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W {
        EGE_W { w: self }
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W {
        SOIE_W { w: self }
    }
    ///Bits 0:7 - DMA request identification
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W {
        DMAREQ_ID_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///request line multiplexer channel x configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c9cr](index.html) module
pub struct C9CR_SPEC;
impl crate::RegisterSpec for C9CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c9cr::R](R) reader structure
impl crate::Readable for C9CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c9cr::W](W) writer structure
impl crate::Writable for C9CR_SPEC {
    type Writer = W;
}
///`reset()` method sets C9CR to value 0
impl crate::Resettable for C9CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
