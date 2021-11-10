///Register `OPTR` reader
pub struct R(crate::R<OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPTR` writer
pub struct W(crate::W<OPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTR_SPEC>;
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
impl From<crate::W<OPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTR_SPEC>) -> Self {
        W(writer)
    }
}
///Read protection level
///
///Value on reset: 170
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDP_A {
    ///170: Level 0, readout protection not active
    LEVEL0 = 170,
    ///204: Level 2, chip readout protection active
    LEVEL2 = 204,
    ///136: Level 1, memories readout protection active (writes 0x88)
    LEVEL1 = 136,
}
impl From<RDP_A> for u8 {
    #[inline(always)]
    fn from(variant: RDP_A) -> Self {
        variant as _
    }
}
///Field `RDP` reader - Read protection level
pub struct RDP_R(crate::FieldReader<u8, RDP_A>);
impl RDP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RDP_A> {
        match self.bits {
            170 => Some(RDP_A::LEVEL0),
            204 => Some(RDP_A::LEVEL2),
            136 => Some(RDP_A::LEVEL1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `LEVEL0`
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        **self == RDP_A::LEVEL0
    }
    ///Checks if the value of the field is `LEVEL2`
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        **self == RDP_A::LEVEL2
    }
    ///Checks if the value of the field is `LEVEL1`
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        **self == RDP_A::LEVEL1
    }
}
impl core::ops::Deref for RDP_R {
    type Target = crate::FieldReader<u8, RDP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RDP` writer - Read protection level
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RDP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Level 0, readout protection not active
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(RDP_A::LEVEL0)
    }
    ///Level 2, chip readout protection active
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(RDP_A::LEVEL2)
    }
    ///Level 1, memories readout protection active (writes 0x88)
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(RDP_A::LEVEL1)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///System security enabled flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESE_A {
    ///0: Security disabled
    DISABLED = 0,
    ///1: Security enabled
    ENABLED = 1,
}
impl From<ESE_A> for bool {
    #[inline(always)]
    fn from(variant: ESE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ESE` reader - System security enabled flag
pub struct ESE_R(crate::FieldReader<bool, ESE_A>);
impl ESE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ESE_A {
        match self.bits {
            false => ESE_A::DISABLED,
            true => ESE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ESE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ESE_A::ENABLED
    }
}
impl core::ops::Deref for ESE_R {
    type Target = crate::FieldReader<bool, ESE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ESE` writer - System security enabled flag
pub struct ESE_W<'a> {
    w: &'a mut W,
}
impl<'a> ESE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ESE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Security disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ESE_A::DISABLED)
    }
    ///Security enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ESE_A::ENABLED)
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
///BOR reset Level
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOR_LEV_A {
    ///0: BOR level 0. Reset level threshold is around 1.7 V
    LEVEL0 = 0,
    ///1: BOR level 1. Reset level threshold is around 2.0 V
    LEVEL1 = 1,
    ///2: BOR level 2. Reset level threshold is around 2.2 V
    LEVEL2 = 2,
    ///3: BOR level 3. Reset level threshold is around 2.5 V
    LEVEL3 = 3,
    ///4: BOR level 4. Reset level threshold is around 2.8 V
    LEVEL4 = 4,
}
impl From<BOR_LEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BOR_LEV_A) -> Self {
        variant as _
    }
}
///Field `BOR_LEV` reader - BOR reset Level
pub struct BOR_LEV_R(crate::FieldReader<u8, BOR_LEV_A>);
impl BOR_LEV_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOR_LEV_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<BOR_LEV_A> {
        match self.bits {
            0 => Some(BOR_LEV_A::LEVEL0),
            1 => Some(BOR_LEV_A::LEVEL1),
            2 => Some(BOR_LEV_A::LEVEL2),
            3 => Some(BOR_LEV_A::LEVEL3),
            4 => Some(BOR_LEV_A::LEVEL4),
            _ => None,
        }
    }
    ///Checks if the value of the field is `LEVEL0`
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        **self == BOR_LEV_A::LEVEL0
    }
    ///Checks if the value of the field is `LEVEL1`
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        **self == BOR_LEV_A::LEVEL1
    }
    ///Checks if the value of the field is `LEVEL2`
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        **self == BOR_LEV_A::LEVEL2
    }
    ///Checks if the value of the field is `LEVEL3`
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        **self == BOR_LEV_A::LEVEL3
    }
    ///Checks if the value of the field is `LEVEL4`
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        **self == BOR_LEV_A::LEVEL4
    }
}
impl core::ops::Deref for BOR_LEV_R {
    type Target = crate::FieldReader<u8, BOR_LEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BOR_LEV` writer - BOR reset Level
pub struct BOR_LEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_LEV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BOR_LEV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///BOR level 0. Reset level threshold is around 1.7 V
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BOR_LEV_A::LEVEL0)
    }
    ///BOR level 1. Reset level threshold is around 2.0 V
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BOR_LEV_A::LEVEL1)
    }
    ///BOR level 2. Reset level threshold is around 2.2 V
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BOR_LEV_A::LEVEL2)
    }
    ///BOR level 3. Reset level threshold is around 2.5 V
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BOR_LEV_A::LEVEL3)
    }
    ///BOR level 4. Reset level threshold is around 2.8 V
    #[inline(always)]
    pub fn level4(self) -> &'a mut W {
        self.variant(BOR_LEV_A::LEVEL4)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
///nRST_STOP
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRST_STOP_A {
    ///0: Reset generated when entering the Standby mode
    ENABLED = 0,
    ///1: No reset generated when entering the Standby mode
    DISABLED = 1,
}
impl From<NRST_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `nRST_STOP` reader - nRST_STOP
pub struct NRST_STOP_R(crate::FieldReader<bool, NRST_STOP_A>);
impl NRST_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NRST_STOP_A {
        match self.bits {
            false => NRST_STOP_A::ENABLED,
            true => NRST_STOP_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == NRST_STOP_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NRST_STOP_A::DISABLED
    }
}
impl core::ops::Deref for NRST_STOP_R {
    type Target = crate::FieldReader<bool, NRST_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `nRST_STOP` writer - nRST_STOP
pub struct NRST_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NRST_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Reset generated when entering the Standby mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NRST_STOP_A::ENABLED)
    }
    ///No reset generated when entering the Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NRST_STOP_A::DISABLED)
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
///nRST_STDBY
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRST_STDBY_A {
    ///0: Reset generated when entering the Standby mode
    ENABLED = 0,
    ///1: No reset generated when entering the Standby mode
    DISABLED = 1,
}
impl From<NRST_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `nRST_STDBY` reader - nRST_STDBY
pub struct NRST_STDBY_R(crate::FieldReader<bool, NRST_STDBY_A>);
impl NRST_STDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STDBY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NRST_STDBY_A {
        match self.bits {
            false => NRST_STDBY_A::ENABLED,
            true => NRST_STDBY_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == NRST_STDBY_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NRST_STDBY_A::DISABLED
    }
}
impl core::ops::Deref for NRST_STDBY_R {
    type Target = crate::FieldReader<bool, NRST_STDBY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `nRST_STDBY` writer - nRST_STDBY
pub struct NRST_STDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STDBY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NRST_STDBY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Reset generated when entering the Standby mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NRST_STDBY_A::ENABLED)
    }
    ///No reset generated when entering the Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NRST_STDBY_A::DISABLED)
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
///nRSTSHDW
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRST_SHDW_A {
    ///0: Reset generated when entering the Shutdown mode
    ENABLED = 0,
    ///1: No reset generated when entering the Shutdown mode
    DISABLED = 1,
}
impl From<NRST_SHDW_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_SHDW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `nRST_SHDW` reader - nRSTSHDW
pub struct NRST_SHDW_R(crate::FieldReader<bool, NRST_SHDW_A>);
impl NRST_SHDW_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_SHDW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NRST_SHDW_A {
        match self.bits {
            false => NRST_SHDW_A::ENABLED,
            true => NRST_SHDW_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == NRST_SHDW_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NRST_SHDW_A::DISABLED
    }
}
impl core::ops::Deref for NRST_SHDW_R {
    type Target = crate::FieldReader<bool, NRST_SHDW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `nRST_SHDW` writer - nRSTSHDW
pub struct NRST_SHDW_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_SHDW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NRST_SHDW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Reset generated when entering the Shutdown mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NRST_SHDW_A::ENABLED)
    }
    ///No reset generated when entering the Shutdown mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NRST_SHDW_A::DISABLED)
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
///Independent watchdog selection
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDG_SW_A {
    ///0: Hardware independent watchdog
    HARDWARE = 0,
    ///1: Software independent watchdog
    SOFTWARE = 1,
}
impl From<IWDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDG_SW` reader - Independent watchdog selection
pub struct IWDG_SW_R(crate::FieldReader<bool, IWDG_SW_A>);
impl IWDG_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG_SW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IWDG_SW_A {
        match self.bits {
            false => IWDG_SW_A::HARDWARE,
            true => IWDG_SW_A::SOFTWARE,
        }
    }
    ///Checks if the value of the field is `HARDWARE`
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        **self == IWDG_SW_A::HARDWARE
    }
    ///Checks if the value of the field is `SOFTWARE`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        **self == IWDG_SW_A::SOFTWARE
    }
}
impl core::ops::Deref for IWDG_SW_R {
    type Target = crate::FieldReader<bool, IWDG_SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IWDG_SW` writer - Independent watchdog selection
pub struct IWDG_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_SW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IWDG_SW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Hardware independent watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(IWDG_SW_A::HARDWARE)
    }
    ///Software independent watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(IWDG_SW_A::SOFTWARE)
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
///Independent watchdog counter freeze in Stop mode
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDG_STOP_A {
    ///0: Independent watchdog counter frozen in Stop mode
    FROZEN = 0,
    ///1: Independent watchdog counter running in Stop mode
    RUNNING = 1,
}
impl From<IWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub struct IWDG_STOP_R(crate::FieldReader<bool, IWDG_STOP_A>);
impl IWDG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IWDG_STOP_A {
        match self.bits {
            false => IWDG_STOP_A::FROZEN,
            true => IWDG_STOP_A::RUNNING,
        }
    }
    ///Checks if the value of the field is `FROZEN`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        **self == IWDG_STOP_A::FROZEN
    }
    ///Checks if the value of the field is `RUNNING`
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        **self == IWDG_STOP_A::RUNNING
    }
}
impl core::ops::Deref for IWDG_STOP_R {
    type Target = crate::FieldReader<bool, IWDG_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub struct IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IWDG_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Independent watchdog counter frozen in Stop mode
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(IWDG_STOP_A::FROZEN)
    }
    ///Independent watchdog counter running in Stop mode
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(IWDG_STOP_A::RUNNING)
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
///Independent watchdog counter freeze in Standby mode
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDG_STDBY_A {
    ///0: Independent watchdog counter frozen in Standby mode
    FROZEN = 0,
    ///1: Independent watchdog counter running in Standby mode
    RUNNING = 1,
}
impl From<IWDG_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode
pub struct IWDG_STDBY_R(crate::FieldReader<bool, IWDG_STDBY_A>);
impl IWDG_STDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG_STDBY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IWDG_STDBY_A {
        match self.bits {
            false => IWDG_STDBY_A::FROZEN,
            true => IWDG_STDBY_A::RUNNING,
        }
    }
    ///Checks if the value of the field is `FROZEN`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        **self == IWDG_STDBY_A::FROZEN
    }
    ///Checks if the value of the field is `RUNNING`
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        **self == IWDG_STDBY_A::RUNNING
    }
}
impl core::ops::Deref for IWDG_STDBY_R {
    type Target = crate::FieldReader<bool, IWDG_STDBY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode
pub struct IWDG_STDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_STDBY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IWDG_STDBY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Independent watchdog counter frozen in Standby mode
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(IWDG_STDBY_A::FROZEN)
    }
    ///Independent watchdog counter running in Standby mode
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(IWDG_STDBY_A::RUNNING)
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
///Window watchdog selection
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDG_SW_A {
    ///0: Hardware window watchdog
    HARDWARE = 0,
    ///1: Software window watchdog
    SOFTWARE = 1,
}
impl From<WWDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: WWDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDG_SW` reader - Window watchdog selection
pub struct WWDG_SW_R(crate::FieldReader<bool, WWDG_SW_A>);
impl WWDG_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDG_SW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WWDG_SW_A {
        match self.bits {
            false => WWDG_SW_A::HARDWARE,
            true => WWDG_SW_A::SOFTWARE,
        }
    }
    ///Checks if the value of the field is `HARDWARE`
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        **self == WWDG_SW_A::HARDWARE
    }
    ///Checks if the value of the field is `SOFTWARE`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        **self == WWDG_SW_A::SOFTWARE
    }
}
impl core::ops::Deref for WWDG_SW_R {
    type Target = crate::FieldReader<bool, WWDG_SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WWDG_SW` writer - Window watchdog selection
pub struct WWDG_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDG_SW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WWDG_SW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Hardware window watchdog
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(WWDG_SW_A::HARDWARE)
    }
    ///Software window watchdog
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(WWDG_SW_A::SOFTWARE)
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
///Boot configuration
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBOOT1_A {
    ///0: When nSWBOOT0 is cleared, select boot mode together with nBOOT0
    CLEAR = 0,
    ///1: When nSWBOOT0 is cleared, select boot mode together with nBOOT0
    SET = 1,
}
impl From<NBOOT1_A> for bool {
    #[inline(always)]
    fn from(variant: NBOOT1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `nBOOT1` reader - Boot configuration
pub struct NBOOT1_R(crate::FieldReader<bool, NBOOT1_A>);
impl NBOOT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        NBOOT1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NBOOT1_A {
        match self.bits {
            false => NBOOT1_A::CLEAR,
            true => NBOOT1_A::SET,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == NBOOT1_A::CLEAR
    }
    ///Checks if the value of the field is `SET`
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == NBOOT1_A::SET
    }
}
impl core::ops::Deref for NBOOT1_R {
    type Target = crate::FieldReader<bool, NBOOT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `nBOOT1` writer - Boot configuration
pub struct NBOOT1_W<'a> {
    w: &'a mut W,
}
impl<'a> NBOOT1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NBOOT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///When nSWBOOT0 is cleared, select boot mode together with nBOOT0
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NBOOT1_A::CLEAR)
    }
    ///When nSWBOOT0 is cleared, select boot mode together with nBOOT0
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(NBOOT1_A::SET)
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
///SRAM2 parity check enable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM2_PE_A {
    ///0: SRAM2 Parity check enabled
    ENABLED = 0,
    ///1: SRAM2 Parity check disabled
    DISABLED = 1,
}
impl From<SRAM2_PE_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2_PE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2_PE` reader - SRAM2 parity check enable
pub struct SRAM2_PE_R(crate::FieldReader<bool, SRAM2_PE_A>);
impl SRAM2_PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2_PE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SRAM2_PE_A {
        match self.bits {
            false => SRAM2_PE_A::ENABLED,
            true => SRAM2_PE_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SRAM2_PE_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SRAM2_PE_A::DISABLED
    }
}
impl core::ops::Deref for SRAM2_PE_R {
    type Target = crate::FieldReader<bool, SRAM2_PE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SRAM2_PE` writer - SRAM2 parity check enable
pub struct SRAM2_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2_PE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SRAM2_PE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SRAM2 Parity check enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM2_PE_A::ENABLED)
    }
    ///SRAM2 Parity check disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM2_PE_A::DISABLED)
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
///SRAM2 Erase when system reset
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_RST_A {
    ///0: SRAM1 and SRAM2 erased when a system reset occurs
    RESET = 0,
    ///1: SRAM1 and SRAM2 not erased when a system reset occurs
    NOTRESET = 1,
}
impl From<SRAM_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_RST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM_RST` reader - SRAM2 Erase when system reset
pub struct SRAM_RST_R(crate::FieldReader<bool, SRAM_RST_A>);
impl SRAM_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_RST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SRAM_RST_A {
        match self.bits {
            false => SRAM_RST_A::RESET,
            true => SRAM_RST_A::NOTRESET,
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == SRAM_RST_A::RESET
    }
    ///Checks if the value of the field is `NOTRESET`
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        **self == SRAM_RST_A::NOTRESET
    }
}
impl core::ops::Deref for SRAM_RST_R {
    type Target = crate::FieldReader<bool, SRAM_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SRAM_RST` writer - SRAM2 Erase when system reset
pub struct SRAM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_RST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SRAM_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SRAM1 and SRAM2 erased when a system reset occurs
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SRAM_RST_A::RESET)
    }
    ///SRAM1 and SRAM2 not erased when a system reset occurs
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(SRAM_RST_A::NOTRESET)
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
///Software BOOT0 selection
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSWBOOT0_A {
    ///0: BOOT0 taken from nBOOT0 in this register
    BIT = 0,
    ///1: BOOT0 taken from GPIO PH3/BOOT0
    PIN = 1,
}
impl From<NSWBOOT0_A> for bool {
    #[inline(always)]
    fn from(variant: NSWBOOT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `nSWBOOT0` reader - Software BOOT0 selection
pub struct NSWBOOT0_R(crate::FieldReader<bool, NSWBOOT0_A>);
impl NSWBOOT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSWBOOT0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NSWBOOT0_A {
        match self.bits {
            false => NSWBOOT0_A::BIT,
            true => NSWBOOT0_A::PIN,
        }
    }
    ///Checks if the value of the field is `BIT`
    #[inline(always)]
    pub fn is_bit_(&self) -> bool {
        **self == NSWBOOT0_A::BIT
    }
    ///Checks if the value of the field is `PIN`
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        **self == NSWBOOT0_A::PIN
    }
}
impl core::ops::Deref for NSWBOOT0_R {
    type Target = crate::FieldReader<bool, NSWBOOT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `nSWBOOT0` writer - Software BOOT0 selection
pub struct NSWBOOT0_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWBOOT0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NSWBOOT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///BOOT0 taken from nBOOT0 in this register
    #[inline(always)]
    pub fn bit_(self) -> &'a mut W {
        self.variant(NSWBOOT0_A::BIT)
    }
    ///BOOT0 taken from GPIO PH3/BOOT0
    #[inline(always)]
    pub fn pin(self) -> &'a mut W {
        self.variant(NSWBOOT0_A::PIN)
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
///nBOOT0 option bit
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBOOT0_A {
    ///0: When nSWBOOT0 is cleared, select boot mode together with nBOOT1
    CLEAR = 0,
    ///1: When nSWBOOT0 is cleared, select boot mode together with nBOOT1
    SET = 1,
}
impl From<NBOOT0_A> for bool {
    #[inline(always)]
    fn from(variant: NBOOT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `nBOOT0` reader - nBOOT0 option bit
pub struct NBOOT0_R(crate::FieldReader<bool, NBOOT0_A>);
impl NBOOT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        NBOOT0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NBOOT0_A {
        match self.bits {
            false => NBOOT0_A::CLEAR,
            true => NBOOT0_A::SET,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == NBOOT0_A::CLEAR
    }
    ///Checks if the value of the field is `SET`
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == NBOOT0_A::SET
    }
}
impl core::ops::Deref for NBOOT0_R {
    type Target = crate::FieldReader<bool, NBOOT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `nBOOT0` writer - nBOOT0 option bit
pub struct NBOOT0_W<'a> {
    w: &'a mut W,
}
impl<'a> NBOOT0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NBOOT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///When nSWBOOT0 is cleared, select boot mode together with nBOOT1
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NBOOT0_A::CLEAR)
    }
    ///When nSWBOOT0 is cleared, select boot mode together with nBOOT1
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(NBOOT0_A::SET)
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
///CPU1 CM4 Unique Boot entry enable option bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_LOCK_A {
    ///0: Boot lock is disabled
    DISABLED = 0,
    ///1: Boot lock is enabled
    ENABLED = 1,
}
impl From<BOOT_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BOOT_LOCK` reader - CPU1 CM4 Unique Boot entry enable option bit
pub struct BOOT_LOCK_R(crate::FieldReader<bool, BOOT_LOCK_A>);
impl BOOT_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_LOCK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BOOT_LOCK_A {
        match self.bits {
            false => BOOT_LOCK_A::DISABLED,
            true => BOOT_LOCK_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BOOT_LOCK_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BOOT_LOCK_A::ENABLED
    }
}
impl core::ops::Deref for BOOT_LOCK_R {
    type Target = crate::FieldReader<bool, BOOT_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BOOT_LOCK` writer - CPU1 CM4 Unique Boot entry enable option bit
pub struct BOOT_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_LOCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BOOT_LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Boot lock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOOT_LOCK_A::DISABLED)
    }
    ///Boot lock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOOT_LOCK_A::ENABLED)
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
///Field `C2BOOT_LOCK` reader - CPU2 CM0+ Unique Boot entry enable option bit
pub struct C2BOOT_LOCK_R(crate::FieldReader<bool, bool>);
impl C2BOOT_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2BOOT_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2BOOT_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `C2BOOT_LOCK` writer - CPU2 CM0+ Unique Boot entry enable option bit
pub struct C2BOOT_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> C2BOOT_LOCK_W<'a> {
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
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - System security enabled flag
    #[inline(always)]
    pub fn ese(&self) -> ESE_R {
        ESE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 9:11 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - nRSTSHDW
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> NRST_SHDW_R {
        NRST_SHDW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    pub fn sram2_pe(&self) -> SRAM2_PE_R {
        SRAM2_PE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    pub fn sram_rst(&self) -> SRAM_RST_R {
        SRAM_RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - Software BOOT0 selection
    #[inline(always)]
    pub fn n_swboot0(&self) -> NSWBOOT0_R {
        NSWBOOT0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&self) -> NBOOT0_R {
        NBOOT0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 30 - CPU1 CM4 Unique Boot entry enable option bit
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - CPU2 CM0+ Unique Boot entry enable option bit
    #[inline(always)]
    pub fn c2boot_lock(&self) -> C2BOOT_LOCK_R {
        C2BOOT_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    ///Bit 8 - System security enabled flag
    #[inline(always)]
    pub fn ese(&mut self) -> ESE_W {
        ESE_W { w: self }
    }
    ///Bits 9:11 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W {
        BOR_LEV_W { w: self }
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> NRST_STOP_W {
        NRST_STOP_W { w: self }
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> NRST_STDBY_W {
        NRST_STDBY_W { w: self }
    }
    ///Bit 14 - nRSTSHDW
    #[inline(always)]
    pub fn n_rst_shdw(&mut self) -> NRST_SHDW_W {
        NRST_SHDW_W { w: self }
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W {
        IWDG_SW_W { w: self }
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W {
        IWDG_STOP_W { w: self }
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W {
        IWDG_STDBY_W { w: self }
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W {
        WWDG_SW_W { w: self }
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&mut self) -> NBOOT1_W {
        NBOOT1_W { w: self }
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W {
        SRAM2_PE_W { w: self }
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    pub fn sram_rst(&mut self) -> SRAM_RST_W {
        SRAM_RST_W { w: self }
    }
    ///Bit 26 - Software BOOT0 selection
    #[inline(always)]
    pub fn n_swboot0(&mut self) -> NSWBOOT0_W {
        NSWBOOT0_W { w: self }
    }
    ///Bit 27 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&mut self) -> NBOOT0_W {
        NBOOT0_W { w: self }
    }
    ///Bit 30 - CPU1 CM4 Unique Boot entry enable option bit
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W {
        BOOT_LOCK_W { w: self }
    }
    ///Bit 31 - CPU2 CM0+ Unique Boot entry enable option bit
    #[inline(always)]
    pub fn c2boot_lock(&mut self) -> C2BOOT_LOCK_W {
        C2BOOT_LOCK_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash option register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optr](index.html) module
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [optr::R](R) reader structure
impl crate::Readable for OPTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [optr::W](W) writer structure
impl crate::Writable for OPTR_SPEC {
    type Writer = W;
}
///`reset()` method sets OPTR to value 0x3fff_f0aa
impl crate::Resettable for OPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff_f0aa
    }
}
