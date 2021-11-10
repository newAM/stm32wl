///Register `BDTR` reader
pub struct R(crate::R<BDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BDTR` writer
pub struct W(crate::W<BDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTR_SPEC>;
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
impl From<crate::W<BDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTR_SPEC>) -> Self {
        W(writer)
    }
}
///Break Bidirectional
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKBID_A {
    ///0: Break input BRK in input mode
    INPUT = 0,
    ///1: Break input BRK in bidirectional mode
    BIDIRECTIONAL = 1,
}
impl From<BKBID_A> for bool {
    #[inline(always)]
    fn from(variant: BKBID_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKBID` reader - Break Bidirectional
pub struct BKBID_R(crate::FieldReader<bool, BKBID_A>);
impl BKBID_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKBID_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKBID_A {
        match self.bits {
            false => BKBID_A::INPUT,
            true => BKBID_A::BIDIRECTIONAL,
        }
    }
    ///Checks if the value of the field is `INPUT`
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == BKBID_A::INPUT
    }
    ///Checks if the value of the field is `BIDIRECTIONAL`
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        **self == BKBID_A::BIDIRECTIONAL
    }
}
impl core::ops::Deref for BKBID_R {
    type Target = crate::FieldReader<bool, BKBID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKBID` writer - Break Bidirectional
pub struct BKBID_W<'a> {
    w: &'a mut W,
}
impl<'a> BKBID_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKBID_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Break input BRK in input mode
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(BKBID_A::INPUT)
    }
    ///Break input BRK in bidirectional mode
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut W {
        self.variant(BKBID_A::BIDIRECTIONAL)
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
///Break Disarm
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKDSRM_A {
    ///0: Break input BRK is armed
    ARMED = 0,
    ///1: Break input BRK is disarmed
    DISARMED = 1,
}
impl From<BKDSRM_A> for bool {
    #[inline(always)]
    fn from(variant: BKDSRM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKDSRM` reader - Break Disarm
pub struct BKDSRM_R(crate::FieldReader<bool, BKDSRM_A>);
impl BKDSRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKDSRM_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKDSRM_A {
        match self.bits {
            false => BKDSRM_A::ARMED,
            true => BKDSRM_A::DISARMED,
        }
    }
    ///Checks if the value of the field is `ARMED`
    #[inline(always)]
    pub fn is_armed(&self) -> bool {
        **self == BKDSRM_A::ARMED
    }
    ///Checks if the value of the field is `DISARMED`
    #[inline(always)]
    pub fn is_disarmed(&self) -> bool {
        **self == BKDSRM_A::DISARMED
    }
}
impl core::ops::Deref for BKDSRM_R {
    type Target = crate::FieldReader<bool, BKDSRM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKDSRM` writer - Break Disarm
pub struct BKDSRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BKDSRM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKDSRM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Break input BRK is armed
    #[inline(always)]
    pub fn armed(self) -> &'a mut W {
        self.variant(BKDSRM_A::ARMED)
    }
    ///Break input BRK is disarmed
    #[inline(always)]
    pub fn disarmed(self) -> &'a mut W {
        self.variant(BKDSRM_A::DISARMED)
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
///Main output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOE_A {
    ///0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit
    DISABLED = 0,
    ///1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
    ENABLED = 1,
}
impl From<MOE_A> for bool {
    #[inline(always)]
    fn from(variant: MOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MOE` reader - Main output enable
pub struct MOE_R(crate::FieldReader<bool, MOE_A>);
impl MOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MOE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MOE_A {
        match self.bits {
            false => MOE_A::DISABLED,
            true => MOE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MOE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MOE_A::ENABLED
    }
}
impl core::ops::Deref for MOE_R {
    type Target = crate::FieldReader<bool, MOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MOE` writer - Main output enable
pub struct MOE_W<'a> {
    w: &'a mut W,
}
impl<'a> MOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MOE_A::DISABLED)
    }
    ///OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MOE_A::ENABLED)
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
///Automatic output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AOE_A {
    ///0: MOE can be set only by software
    DISABLED = 0,
    ///1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    ENABLED = 1,
}
impl From<AOE_A> for bool {
    #[inline(always)]
    fn from(variant: AOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AOE` reader - Automatic output enable
pub struct AOE_R(crate::FieldReader<bool, AOE_A>);
impl AOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AOE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AOE_A {
        match self.bits {
            false => AOE_A::DISABLED,
            true => AOE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AOE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AOE_A::ENABLED
    }
}
impl core::ops::Deref for AOE_R {
    type Target = crate::FieldReader<bool, AOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AOE` writer - Automatic output enable
pub struct AOE_W<'a> {
    w: &'a mut W,
}
impl<'a> AOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///MOE can be set only by software
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AOE_A::DISABLED)
    }
    ///MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AOE_A::ENABLED)
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
///Break polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKP_A {
    ///0: Break input BRK is active low
    ACTIVELOW = 0,
    ///1: Break input BRK is active high
    ACTIVEHIGH = 1,
}
impl From<BKP_A> for bool {
    #[inline(always)]
    fn from(variant: BKP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKP` reader - Break polarity
pub struct BKP_R(crate::FieldReader<bool, BKP_A>);
impl BKP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKP_A {
        match self.bits {
            false => BKP_A::ACTIVELOW,
            true => BKP_A::ACTIVEHIGH,
        }
    }
    ///Checks if the value of the field is `ACTIVELOW`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        **self == BKP_A::ACTIVELOW
    }
    ///Checks if the value of the field is `ACTIVEHIGH`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        **self == BKP_A::ACTIVEHIGH
    }
}
impl core::ops::Deref for BKP_R {
    type Target = crate::FieldReader<bool, BKP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKP` writer - Break polarity
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Break input BRK is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(BKP_A::ACTIVELOW)
    }
    ///Break input BRK is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(BKP_A::ACTIVEHIGH)
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
///Break enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKE_A {
    ///0: Break inputs (BRK and CCS clock failure event) disabled
    DISABLED = 0,
    ///1: Break inputs (BRK and CCS clock failure event) enabled
    ENABLED = 1,
}
impl From<BKE_A> for bool {
    #[inline(always)]
    fn from(variant: BKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKE` reader - Break enable
pub struct BKE_R(crate::FieldReader<bool, BKE_A>);
impl BKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKE_A {
        match self.bits {
            false => BKE_A::DISABLED,
            true => BKE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BKE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BKE_A::ENABLED
    }
}
impl core::ops::Deref for BKE_R {
    type Target = crate::FieldReader<bool, BKE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKE` writer - Break enable
pub struct BKE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Break inputs (BRK and CCS clock failure event) disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BKE_A::DISABLED)
    }
    ///Break inputs (BRK and CCS clock failure event) enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BKE_A::ENABLED)
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
///Off-state selection for Run mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSSR_A {
    ///0: OC/OCN outputs are disabled when inactive
    DISABLED = 0,
    ///1: OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1
    ENABLED = 1,
}
impl From<OSSR_A> for bool {
    #[inline(always)]
    fn from(variant: OSSR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OSSR` reader - Off-state selection for Run mode
pub struct OSSR_R(crate::FieldReader<bool, OSSR_A>);
impl OSSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSSR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSSR_A {
        match self.bits {
            false => OSSR_A::DISABLED,
            true => OSSR_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OSSR_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OSSR_A::ENABLED
    }
}
impl core::ops::Deref for OSSR_R {
    type Target = crate::FieldReader<bool, OSSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OSSR` writer - Off-state selection for Run mode
pub struct OSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSSR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OC/OCN outputs are disabled when inactive
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSR_A::DISABLED)
    }
    ///OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSSR_A::ENABLED)
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
///Off-state selection for Idle mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSSI_A {
    ///0: OC/OCN outputs are disabled when inactive
    DISABLED = 0,
    ///1: OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime
    ENABLED = 1,
}
impl From<OSSI_A> for bool {
    #[inline(always)]
    fn from(variant: OSSI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OSSI` reader - Off-state selection for Idle mode
pub struct OSSI_R(crate::FieldReader<bool, OSSI_A>);
impl OSSI_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSSI_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSSI_A {
        match self.bits {
            false => OSSI_A::DISABLED,
            true => OSSI_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OSSI_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OSSI_A::ENABLED
    }
}
impl core::ops::Deref for OSSI_R {
    type Target = crate::FieldReader<bool, OSSI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OSSI` writer - Off-state selection for Idle mode
pub struct OSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSI_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSSI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OC/OCN outputs are disabled when inactive
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSI_A::DISABLED)
    }
    ///OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSSI_A::ENABLED)
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
///Lock configuration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_A {
    ///0: No write protection
    OFF = 0,
    ///1: Level 1 write protection
    LEVEL1 = 1,
    ///2: Level 2 write protection
    LEVEL2 = 2,
    ///3: Level 3 write protection
    LEVEL3 = 3,
}
impl From<LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as _
    }
}
///Field `LOCK` reader - Lock configuration
pub struct LOCK_R(crate::FieldReader<u8, LOCK_A>);
impl LOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            0 => LOCK_A::OFF,
            1 => LOCK_A::LEVEL1,
            2 => LOCK_A::LEVEL2,
            3 => LOCK_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `OFF`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == LOCK_A::OFF
    }
    ///Checks if the value of the field is `LEVEL1`
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        **self == LOCK_A::LEVEL1
    }
    ///Checks if the value of the field is `LEVEL2`
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        **self == LOCK_A::LEVEL2
    }
    ///Checks if the value of the field is `LEVEL3`
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        **self == LOCK_A::LEVEL3
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<u8, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LOCK` writer - Lock configuration
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No write protection
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LOCK_A::OFF)
    }
    ///Level 1 write protection
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(LOCK_A::LEVEL1)
    }
    ///Level 2 write protection
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(LOCK_A::LEVEL2)
    }
    ///Level 3 write protection
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(LOCK_A::LEVEL3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///Field `DTG` reader - Dead-time generator setup
pub struct DTG_R(crate::FieldReader<u8, u8>);
impl DTG_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DTG` writer - Dead-time generator setup
pub struct DTG_W<'a> {
    w: &'a mut W,
}
impl<'a> DTG_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bit 28 - Break Bidirectional
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 26 - Break Disarm
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bit 28 - Break Bidirectional
    #[inline(always)]
    pub fn bkbid(&mut self) -> BKBID_W {
        BKBID_W { w: self }
    }
    ///Bit 26 - Break Disarm
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BKDSRM_W {
        BKDSRM_W { w: self }
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W {
        MOE_W { w: self }
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W {
        AOE_W { w: self }
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W {
        BKE_W { w: self }
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W {
        OSSR_W { w: self }
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W {
        OSSI_W { w: self }
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W {
        DTG_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM16/TIM17 break and dead-time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdtr](index.html) module
pub struct BDTR_SPEC;
impl crate::RegisterSpec for BDTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bdtr::R](R) reader structure
impl crate::Readable for BDTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bdtr::W](W) writer structure
impl crate::Writable for BDTR_SPEC {
    type Writer = W;
}
///`reset()` method sets BDTR to value 0
impl crate::Resettable for BDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
