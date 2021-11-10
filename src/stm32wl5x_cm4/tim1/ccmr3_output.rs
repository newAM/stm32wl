///Register `CCMR3_Output` reader
pub struct R(crate::R<CCMR3_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR3_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR3_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR3_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR3_Output` writer
pub struct W(crate::W<CCMR3_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR3_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR3_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR3_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
///OC6M
pub type OC6M_3_A = OC5M_3_A;
///Field `OC6M_3` reader - OC6M
pub type OC6M_3_R = OC5M_3_R;
///Field `OC6M_3` writer - OC6M
pub struct OC6M_3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6M_3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC6M_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal output compare mode (modes 0-7)
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OC6M_3_A::NORMAL)
    }
    ///Extended output compare mode (modes 7-15)
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(OC6M_3_A::EXTENDED)
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
///OC5M
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC5M_3_A {
    ///0: Normal output compare mode (modes 0-7)
    NORMAL = 0,
    ///1: Extended output compare mode (modes 7-15)
    EXTENDED = 1,
}
impl From<OC5M_3_A> for bool {
    #[inline(always)]
    fn from(variant: OC5M_3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OC5M_3` reader - OC5M
pub struct OC5M_3_R(crate::FieldReader<bool, OC5M_3_A>);
impl OC5M_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5M_3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC5M_3_A {
        match self.bits {
            false => OC5M_3_A::NORMAL,
            true => OC5M_3_A::EXTENDED,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == OC5M_3_A::NORMAL
    }
    ///Checks if the value of the field is `EXTENDED`
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        **self == OC5M_3_A::EXTENDED
    }
}
impl core::ops::Deref for OC5M_3_R {
    type Target = crate::FieldReader<bool, OC5M_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OC5M_3` writer - OC5M
pub struct OC5M_3_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5M_3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC5M_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal output compare mode (modes 0-7)
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OC5M_3_A::NORMAL)
    }
    ///Extended output compare mode (modes 7-15)
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(OC5M_3_A::EXTENDED)
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
///OC6CE
pub type OC6CE_A = OC5CE_A;
///Field `OC6CE` reader - OC6CE
pub type OC6CE_R = OC5CE_R;
///Field `OC6CE` writer - OC6CE
pub struct OC6CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6CE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC6CE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OCxRef is not affected by the ocref_clr_int signal
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC6CE_A::DISABLED)
    }
    ///OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC6CE_A::ENABLED)
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
///OC6M
pub type OC6M_A = OC5M_A;
///Field `OC6M` reader - OC6M
pub type OC6M_R = OC5M_R;
///Field `OC6M` writer - OC6M
pub struct OC6M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6M_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC6M_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC6M_A::FROZEN)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC6M_A::ACTIVEONMATCH)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC6M_A::INACTIVEONMATCH)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC6M_A::TOGGLE)
    }
    ///OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC6M_A::FORCEINACTIVE)
    }
    ///OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC6M_A::FORCEACTIVE)
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC6M_A::PWMMODE1)
    }
    ///Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC6M_A::PWMMODE2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
///OC6PE
pub type OC6PE_A = OC5PE_A;
///Field `OC6PE` reader - OC6PE
pub type OC6PE_R = OC5PE_R;
///Field `OC6PE` writer - OC6PE
pub struct OC6PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6PE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC6PE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC6PE_A::DISABLED)
    }
    ///Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC6PE_A::ENABLED)
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
///OC6FE
pub type OC6FE_A = OC5FE_A;
///Field `OC6FE` reader - OC6FE
pub type OC6FE_R = OC5FE_R;
///Field `OC6FE` writer - OC6FE
pub struct OC6FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC6FE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC6FE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Fast output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC6FE_A::DISABLED)
    }
    ///Fast output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC6FE_A::ENABLED)
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
///OC5CE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC5CE_A {
    ///0: OCxRef is not affected by the ocref_clr_int signal
    DISABLED = 0,
    ///1: OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal
    ENABLED = 1,
}
impl From<OC5CE_A> for bool {
    #[inline(always)]
    fn from(variant: OC5CE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OC5CE` reader - OC5CE
pub struct OC5CE_R(crate::FieldReader<bool, OC5CE_A>);
impl OC5CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5CE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC5CE_A {
        match self.bits {
            false => OC5CE_A::DISABLED,
            true => OC5CE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OC5CE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OC5CE_A::ENABLED
    }
}
impl core::ops::Deref for OC5CE_R {
    type Target = crate::FieldReader<bool, OC5CE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OC5CE` writer - OC5CE
pub struct OC5CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5CE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC5CE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OCxRef is not affected by the ocref_clr_int signal
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC5CE_A::DISABLED)
    }
    ///OCxRef is cleared as soon as a High level is detected on ocref_clr_int signal
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC5CE_A::ENABLED)
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
///OC5M
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OC5M_A {
    ///0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    FROZEN = 0,
    ///1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    ACTIVEONMATCH = 1,
    ///2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    INACTIVEONMATCH = 2,
    ///3: OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    TOGGLE = 3,
    ///4: OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    FORCEINACTIVE = 4,
    ///5: OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    FORCEACTIVE = 5,
    ///6: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    PWMMODE1 = 6,
    ///7: Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    PWMMODE2 = 7,
}
impl From<OC5M_A> for u8 {
    #[inline(always)]
    fn from(variant: OC5M_A) -> Self {
        variant as _
    }
}
///Field `OC5M` reader - OC5M
pub struct OC5M_R(crate::FieldReader<u8, OC5M_A>);
impl OC5M_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC5M_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC5M_A {
        match self.bits {
            0 => OC5M_A::FROZEN,
            1 => OC5M_A::ACTIVEONMATCH,
            2 => OC5M_A::INACTIVEONMATCH,
            3 => OC5M_A::TOGGLE,
            4 => OC5M_A::FORCEINACTIVE,
            5 => OC5M_A::FORCEACTIVE,
            6 => OC5M_A::PWMMODE1,
            7 => OC5M_A::PWMMODE2,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `FROZEN`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        **self == OC5M_A::FROZEN
    }
    ///Checks if the value of the field is `ACTIVEONMATCH`
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        **self == OC5M_A::ACTIVEONMATCH
    }
    ///Checks if the value of the field is `INACTIVEONMATCH`
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        **self == OC5M_A::INACTIVEONMATCH
    }
    ///Checks if the value of the field is `TOGGLE`
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == OC5M_A::TOGGLE
    }
    ///Checks if the value of the field is `FORCEINACTIVE`
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        **self == OC5M_A::FORCEINACTIVE
    }
    ///Checks if the value of the field is `FORCEACTIVE`
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        **self == OC5M_A::FORCEACTIVE
    }
    ///Checks if the value of the field is `PWMMODE1`
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        **self == OC5M_A::PWMMODE1
    }
    ///Checks if the value of the field is `PWMMODE2`
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        **self == OC5M_A::PWMMODE2
    }
}
impl core::ops::Deref for OC5M_R {
    type Target = crate::FieldReader<u8, OC5M_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OC5M` writer - OC5M
pub struct OC5M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5M_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC5M_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs / OpmMode1: Retriggerable OPM mode 1 - In up-counting mode, the channel is active until a trigger event is detected (on TRGI signal). In down-counting mode, the channel is inactive
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC5M_A::FROZEN)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register / OpmMode2: Inversely to OpmMode1
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC5M_A::ACTIVEONMATCH)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register / Reserved
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC5M_A::INACTIVEONMATCH)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy / Reserved
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC5M_A::TOGGLE)
    }
    ///OCyREF is forced low / CombinedPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC is the logical OR between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC5M_A::FORCEINACTIVE)
    }
    ///OCyREF is forced high / CombinedPwmMode2: OCyREF has the same behavior as in PWM mode 2. OCyREFC is the logical AND between OC1REF and OC2REF
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC5M_A::FORCEACTIVE)
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active / AsymmetricPwmMode1: OCyREF has the same behavior as in PWM mode 1. OCyREFC outputs OC1REF when the counter is counting up, OC2REF when it is counting down
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC5M_A::PWMMODE1)
    }
    ///Inversely to PwmMode1 / AsymmetricPwmMode2: Inversely to AsymmetricPwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC5M_A::PWMMODE2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
///OC5PE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC5PE_A {
    ///0: Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    DISABLED = 0,
    ///1: Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    ENABLED = 1,
}
impl From<OC5PE_A> for bool {
    #[inline(always)]
    fn from(variant: OC5PE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OC5PE` reader - OC5PE
pub struct OC5PE_R(crate::FieldReader<bool, OC5PE_A>);
impl OC5PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5PE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC5PE_A {
        match self.bits {
            false => OC5PE_A::DISABLED,
            true => OC5PE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OC5PE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OC5PE_A::ENABLED
    }
}
impl core::ops::Deref for OC5PE_R {
    type Target = crate::FieldReader<bool, OC5PE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OC5PE` writer - OC5PE
pub struct OC5PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5PE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC5PE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC5PE_A::DISABLED)
    }
    ///Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC5PE_A::ENABLED)
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
///OC5FE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC5FE_A {
    ///0: Fast output disabled
    DISABLED = 0,
    ///1: Fast output enabled
    ENABLED = 1,
}
impl From<OC5FE_A> for bool {
    #[inline(always)]
    fn from(variant: OC5FE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OC5FE` reader - OC5FE
pub struct OC5FE_R(crate::FieldReader<bool, OC5FE_A>);
impl OC5FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5FE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC5FE_A {
        match self.bits {
            false => OC5FE_A::DISABLED,
            true => OC5FE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OC5FE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OC5FE_A::ENABLED
    }
}
impl core::ops::Deref for OC5FE_R {
    type Target = crate::FieldReader<bool, OC5FE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OC5FE` writer - OC5FE
pub struct OC5FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5FE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC5FE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Fast output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC5FE_A::DISABLED)
    }
    ///Fast output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC5FE_A::ENABLED)
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
impl R {
    ///Bit 24 - OC6M
    #[inline(always)]
    pub fn oc6m_3(&self) -> OC6M_3_R {
        OC6M_3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 16 - OC5M
    #[inline(always)]
    pub fn oc5m_3(&self) -> OC5M_3_R {
        OC5M_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - OC6CE
    #[inline(always)]
    pub fn oc6ce(&self) -> OC6CE_R {
        OC6CE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 12:14 - OC6M
    #[inline(always)]
    pub fn oc6m(&self) -> OC6M_R {
        OC6M_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bit 11 - OC6PE
    #[inline(always)]
    pub fn oc6pe(&self) -> OC6PE_R {
        OC6PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - OC6FE
    #[inline(always)]
    pub fn oc6fe(&self) -> OC6FE_R {
        OC6FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 7 - OC5CE
    #[inline(always)]
    pub fn oc5ce(&self) -> OC5CE_R {
        OC5CE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 4:6 - OC5M
    #[inline(always)]
    pub fn oc5m(&self) -> OC5M_R {
        OC5M_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 3 - OC5PE
    #[inline(always)]
    pub fn oc5pe(&self) -> OC5PE_R {
        OC5PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - OC5FE
    #[inline(always)]
    pub fn oc5fe(&self) -> OC5FE_R {
        OC5FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 24 - OC6M
    #[inline(always)]
    pub fn oc6m_3(&mut self) -> OC6M_3_W {
        OC6M_3_W { w: self }
    }
    ///Bit 16 - OC5M
    #[inline(always)]
    pub fn oc5m_3(&mut self) -> OC5M_3_W {
        OC5M_3_W { w: self }
    }
    ///Bit 15 - OC6CE
    #[inline(always)]
    pub fn oc6ce(&mut self) -> OC6CE_W {
        OC6CE_W { w: self }
    }
    ///Bits 12:14 - OC6M
    #[inline(always)]
    pub fn oc6m(&mut self) -> OC6M_W {
        OC6M_W { w: self }
    }
    ///Bit 11 - OC6PE
    #[inline(always)]
    pub fn oc6pe(&mut self) -> OC6PE_W {
        OC6PE_W { w: self }
    }
    ///Bit 10 - OC6FE
    #[inline(always)]
    pub fn oc6fe(&mut self) -> OC6FE_W {
        OC6FE_W { w: self }
    }
    ///Bit 7 - OC5CE
    #[inline(always)]
    pub fn oc5ce(&mut self) -> OC5CE_W {
        OC5CE_W { w: self }
    }
    ///Bits 4:6 - OC5M
    #[inline(always)]
    pub fn oc5m(&mut self) -> OC5M_W {
        OC5M_W { w: self }
    }
    ///Bit 3 - OC5PE
    #[inline(always)]
    pub fn oc5pe(&mut self) -> OC5PE_W {
        OC5PE_W { w: self }
    }
    ///Bit 2 - OC5FE
    #[inline(always)]
    pub fn oc5fe(&mut self) -> OC5FE_W {
        OC5FE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr3_output](index.html) module
pub struct CCMR3_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR3_OUTPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr3_output::R](R) reader structure
impl crate::Readable for CCMR3_OUTPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr3_output::W](W) writer structure
impl crate::Writable for CCMR3_OUTPUT_SPEC {
    type Writer = W;
}
///`reset()` method sets CCMR3_Output to value 0
impl crate::Resettable for CCMR3_OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}