///Register `C2CR3` reader
pub struct R(crate::R<C2CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2CR3` writer
pub struct W(crate::W<C2CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR3_SPEC>;
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
impl From<crate::W<C2CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Enable internal wakeup line for CPU2
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIWUL_A {
    ///0: Internal wakeup line interrupt to CPU2 disabled
    DISABLED = 0,
    ///1: Internal wakeup line interrupt to CPU2 enabled
    ENABLED = 1,
}
impl From<EIWUL_A> for bool {
    #[inline(always)]
    fn from(variant: EIWUL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EIWUL` reader - Enable internal wakeup line for CPU2
pub struct EIWUL_R(crate::FieldReader<bool, EIWUL_A>);
impl EIWUL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIWUL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EIWUL_A {
        match self.bits {
            false => EIWUL_A::DISABLED,
            true => EIWUL_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EIWUL_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EIWUL_A::ENABLED
    }
}
impl core::ops::Deref for EIWUL_R {
    type Target = crate::FieldReader<bool, EIWUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EIWUL` writer - Enable internal wakeup line for CPU2
pub struct EIWUL_W<'a> {
    w: &'a mut W,
}
impl<'a> EIWUL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EIWUL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal wakeup line interrupt to CPU2 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIWUL_A::DISABLED)
    }
    ///Internal wakeup line interrupt to CPU2 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIWUL_A::ENABLED)
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
///akeup for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWRFIRQ_A {
    ///0: Radio IRQ\[2:0\]
    ///is disabled and does not trigger a wakeup from Standby event to CPU2.
    DISABLED = 0,
    ///1: Radio IRQ\[2:0\]
    ///is enabled and triggers a wakeup from Standby event to CPU2.
    ENABLED = 1,
}
impl From<EWRFIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: EWRFIRQ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWRFIRQ` reader - akeup for CPU2
pub struct EWRFIRQ_R(crate::FieldReader<bool, EWRFIRQ_A>);
impl EWRFIRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWRFIRQ_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWRFIRQ_A {
        match self.bits {
            false => EWRFIRQ_A::DISABLED,
            true => EWRFIRQ_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWRFIRQ_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWRFIRQ_A::ENABLED
    }
}
impl core::ops::Deref for EWRFIRQ_R {
    type Target = crate::FieldReader<bool, EWRFIRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWRFIRQ` writer - akeup for CPU2
pub struct EWRFIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EWRFIRQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWRFIRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Radio IRQ\[2:0\]
    ///is disabled and does not trigger a wakeup from Standby event to CPU2.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWRFIRQ_A::DISABLED)
    }
    ///Radio IRQ\[2:0\]
    ///is enabled and triggers a wakeup from Standby event to CPU2.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWRFIRQ_A::ENABLED)
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
///EWRFBUSY
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWRFBUSY_A {
    ///0: Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU2 when a rising or a falling edge occurs
    DISABLED = 0,
    ///1: Radio Busy is enabled and triggers a wakeup from Standby event to CPU2 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4
    ENABLED = 1,
}
impl From<EWRFBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: EWRFBUSY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWRFBUSY` reader - EWRFBUSY
pub struct EWRFBUSY_R(crate::FieldReader<bool, EWRFBUSY_A>);
impl EWRFBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWRFBUSY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWRFBUSY_A {
        match self.bits {
            false => EWRFBUSY_A::DISABLED,
            true => EWRFBUSY_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWRFBUSY_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWRFBUSY_A::ENABLED
    }
}
impl core::ops::Deref for EWRFBUSY_R {
    type Target = crate::FieldReader<bool, EWRFBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWRFBUSY` writer - EWRFBUSY
pub struct EWRFBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> EWRFBUSY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWRFBUSY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU2 when a rising or a falling edge occurs
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWRFBUSY_A::DISABLED)
    }
    ///Radio Busy is enabled and triggers a wakeup from Standby event to CPU2 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWRFBUSY_A::ENABLED)
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
///Apply pull-up and pull-down configuration for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APC_A {
    ///0: I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied
    DISABLED = 0,
    ///1: PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os
    ENABLED = 1,
}
impl From<APC_A> for bool {
    #[inline(always)]
    fn from(variant: APC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `APC` reader - Apply pull-up and pull-down configuration for CPU2
pub struct APC_R(crate::FieldReader<bool, APC_A>);
impl APC_R {
    pub(crate) fn new(bits: bool) -> Self {
        APC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> APC_A {
        match self.bits {
            false => APC_A::DISABLED,
            true => APC_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == APC_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == APC_A::ENABLED
    }
}
impl core::ops::Deref for APC_R {
    type Target = crate::FieldReader<bool, APC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `APC` writer - Apply pull-up and pull-down configuration for CPU2
pub struct APC_W<'a> {
    w: &'a mut W,
}
impl<'a> APC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: APC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(APC_A::DISABLED)
    }
    ///PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(APC_A::ENABLED)
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
///Enable wakeup PVD for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWPVD_A {
    ///0: PVD not enabled by the sub-GHz radio active state
    DISABLED = 0,
    ///1: PVD enabled while the sub-GHz radio is active
    ENABLED = 1,
}
impl From<EWPVD_A> for bool {
    #[inline(always)]
    fn from(variant: EWPVD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWPVD` reader - Enable wakeup PVD for CPU2
pub struct EWPVD_R(crate::FieldReader<bool, EWPVD_A>);
impl EWPVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWPVD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWPVD_A {
        match self.bits {
            false => EWPVD_A::DISABLED,
            true => EWPVD_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWPVD_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWPVD_A::ENABLED
    }
}
impl core::ops::Deref for EWPVD_R {
    type Target = crate::FieldReader<bool, EWPVD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWPVD` writer - Enable wakeup PVD for CPU2
pub struct EWPVD_W<'a> {
    w: &'a mut W,
}
impl<'a> EWPVD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWPVD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///PVD not enabled by the sub-GHz radio active state
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWPVD_A::DISABLED)
    }
    ///PVD enabled while the sub-GHz radio is active
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWPVD_A::ENABLED)
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
///Enable Wakeup pin WKUP3 for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP3_A {
    ///0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    DISABLED = 0,
    ///1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    ENABLED = 1,
}
impl From<EWUP3_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP3` reader - Enable Wakeup pin WKUP3 for CPU2
pub struct EWUP3_R(crate::FieldReader<bool, EWUP3_A>);
impl EWUP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWUP3_A {
        match self.bits {
            false => EWUP3_A::DISABLED,
            true => EWUP3_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWUP3_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWUP3_A::ENABLED
    }
}
impl core::ops::Deref for EWUP3_R {
    type Target = crate::FieldReader<bool, EWUP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWUP3` writer - Enable Wakeup pin WKUP3 for CPU2
pub struct EWUP3_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWUP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP3_A::DISABLED)
    }
    ///WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP3_A::ENABLED)
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
///Enable Wakeup pin WKUP2 for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP2_A {
    ///0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    DISABLED = 0,
    ///1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    ENABLED = 1,
}
impl From<EWUP2_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP2` reader - Enable Wakeup pin WKUP2 for CPU2
pub struct EWUP2_R(crate::FieldReader<bool, EWUP2_A>);
impl EWUP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWUP2_A {
        match self.bits {
            false => EWUP2_A::DISABLED,
            true => EWUP2_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWUP2_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWUP2_A::ENABLED
    }
}
impl core::ops::Deref for EWUP2_R {
    type Target = crate::FieldReader<bool, EWUP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWUP2` writer - Enable Wakeup pin WKUP2 for CPU2
pub struct EWUP2_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWUP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP2_A::DISABLED)
    }
    ///WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP2_A::ENABLED)
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
///Enable Wakeup pin WKUP1 for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP1_A {
    ///0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    DISABLED = 0,
    ///1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    ENABLED = 1,
}
impl From<EWUP1_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP1` reader - Enable Wakeup pin WKUP1 for CPU2
pub struct EWUP1_R(crate::FieldReader<bool, EWUP1_A>);
impl EWUP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWUP1_A {
        match self.bits {
            false => EWUP1_A::DISABLED,
            true => EWUP1_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWUP1_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWUP1_A::ENABLED
    }
}
impl core::ops::Deref for EWUP1_R {
    type Target = crate::FieldReader<bool, EWUP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWUP1` writer - Enable Wakeup pin WKUP1 for CPU2
pub struct EWUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWUP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP1_A::DISABLED)
    }
    ///WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP1_A::ENABLED)
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
    ///Bit 15 - Enable internal wakeup line for CPU2
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 13 - akeup for CPU2
    #[inline(always)]
    pub fn ewrfirq(&self) -> EWRFIRQ_R {
        EWRFIRQ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 11 - EWRFBUSY
    #[inline(always)]
    pub fn ewrfbusy(&self) -> EWRFBUSY_R {
        EWRFBUSY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration for CPU2
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 8 - Enable wakeup PVD for CPU2
    #[inline(always)]
    pub fn ewpvd(&self) -> EWPVD_R {
        EWPVD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 for CPU2
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 for CPU2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Enable Wakeup pin WKUP1 for CPU2
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 15 - Enable internal wakeup line for CPU2
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W {
        EIWUL_W { w: self }
    }
    ///Bit 13 - akeup for CPU2
    #[inline(always)]
    pub fn ewrfirq(&mut self) -> EWRFIRQ_W {
        EWRFIRQ_W { w: self }
    }
    ///Bit 11 - EWRFBUSY
    #[inline(always)]
    pub fn ewrfbusy(&mut self) -> EWRFBUSY_W {
        EWRFBUSY_W { w: self }
    }
    ///Bit 10 - Apply pull-up and pull-down configuration for CPU2
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W {
        APC_W { w: self }
    }
    ///Bit 8 - Enable wakeup PVD for CPU2
    #[inline(always)]
    pub fn ewpvd(&mut self) -> EWPVD_W {
        EWPVD_W { w: self }
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 for CPU2
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W {
        EWUP3_W { w: self }
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 for CPU2
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W {
        EWUP2_W { w: self }
    }
    ///Bit 0 - Enable Wakeup pin WKUP1 for CPU2
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W {
        EWUP1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power CPU2 control register 3 \[dual core device only\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2cr3](index.html) module
pub struct C2CR3_SPEC;
impl crate::RegisterSpec for C2CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2cr3::R](R) reader structure
impl crate::Readable for C2CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2cr3::W](W) writer structure
impl crate::Writable for C2CR3_SPEC {
    type Writer = W;
}
///`reset()` method sets C2CR3 to value 0x8000
impl crate::Resettable for C2CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
