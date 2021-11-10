///Register `SMCR` reader
pub struct R(crate::R<SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMCR` writer
pub struct W(crate::W<SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCR_SPEC>;
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
impl From<crate::W<SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TS3_4` reader - Trigger selection
pub struct TS3_4_R(crate::FieldReader<u8, u8>);
impl TS3_4_R {
    pub(crate) fn new(bits: u8) -> Self {
        TS3_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS3_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TS3_4` writer - Trigger selection
pub struct TS3_4_W<'a> {
    w: &'a mut W,
}
impl<'a> TS3_4_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
///Slave mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMS_3_A {
    ///0: Slave mode disabled (see SMS\[0:2\])
    DISABLED = 0,
    ///1: SMS\[0:2\]
    ///must be 0b000 (DisabledOrCombined). Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter
    COMBINEDRESETTRIGGER = 1,
}
impl From<SMS_3_A> for bool {
    #[inline(always)]
    fn from(variant: SMS_3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SMS_3` reader - Slave mode selection
pub struct SMS_3_R(crate::FieldReader<bool, SMS_3_A>);
impl SMS_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMS_3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMS_3_A {
        match self.bits {
            false => SMS_3_A::DISABLED,
            true => SMS_3_A::COMBINEDRESETTRIGGER,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SMS_3_A::DISABLED
    }
    ///Checks if the value of the field is `COMBINEDRESETTRIGGER`
    #[inline(always)]
    pub fn is_combined_reset_trigger(&self) -> bool {
        **self == SMS_3_A::COMBINEDRESETTRIGGER
    }
}
impl core::ops::Deref for SMS_3_R {
    type Target = crate::FieldReader<bool, SMS_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SMS_3` writer - Slave mode selection
pub struct SMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS_3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMS_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Slave mode disabled (see SMS\[0:2\])
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMS_3_A::DISABLED)
    }
    ///SMS\[0:2\]
    ///must be 0b000 (DisabledOrCombined). Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter
    #[inline(always)]
    pub fn combined_reset_trigger(self) -> &'a mut W {
        self.variant(SMS_3_A::COMBINEDRESETTRIGGER)
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
///External trigger polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETP_A {
    ///0: ETR is noninverted, active at high level or rising edge
    NOTINVERTED = 0,
    ///1: ETR is inverted, active at low level or falling edge
    INVERTED = 1,
}
impl From<ETP_A> for bool {
    #[inline(always)]
    fn from(variant: ETP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ETP` reader - External trigger polarity
pub struct ETP_R(crate::FieldReader<bool, ETP_A>);
impl ETP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ETP_A {
        match self.bits {
            false => ETP_A::NOTINVERTED,
            true => ETP_A::INVERTED,
        }
    }
    ///Checks if the value of the field is `NOTINVERTED`
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == ETP_A::NOTINVERTED
    }
    ///Checks if the value of the field is `INVERTED`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == ETP_A::INVERTED
    }
}
impl core::ops::Deref for ETP_R {
    type Target = crate::FieldReader<bool, ETP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ETP` writer - External trigger polarity
pub struct ETP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ETP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///ETR is noninverted, active at high level or rising edge
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(ETP_A::NOTINVERTED)
    }
    ///ETR is inverted, active at low level or falling edge
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(ETP_A::INVERTED)
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
///External clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECE_A {
    ///0: External clock mode 2 disabled
    DISABLED = 0,
    ///1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal
    ENABLED = 1,
}
impl From<ECE_A> for bool {
    #[inline(always)]
    fn from(variant: ECE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ECE` reader - External clock enable
pub struct ECE_R(crate::FieldReader<bool, ECE_A>);
impl ECE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECE_A {
        match self.bits {
            false => ECE_A::DISABLED,
            true => ECE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ECE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ECE_A::ENABLED
    }
}
impl core::ops::Deref for ECE_R {
    type Target = crate::FieldReader<bool, ECE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ECE` writer - External clock enable
pub struct ECE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ECE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///External clock mode 2 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECE_A::DISABLED)
    }
    ///External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECE_A::ENABLED)
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
///External trigger prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETPS_A {
    ///0: Prescaler OFF
    DIV1 = 0,
    ///1: ETRP frequency divided by 2
    DIV2 = 1,
    ///2: ETRP frequency divided by 4
    DIV4 = 2,
    ///3: ETRP frequency divided by 8
    DIV8 = 3,
}
impl From<ETPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ETPS_A) -> Self {
        variant as _
    }
}
///Field `ETPS` reader - External trigger prescaler
pub struct ETPS_R(crate::FieldReader<u8, ETPS_A>);
impl ETPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETPS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ETPS_A {
        match self.bits {
            0 => ETPS_A::DIV1,
            1 => ETPS_A::DIV2,
            2 => ETPS_A::DIV4,
            3 => ETPS_A::DIV8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == ETPS_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == ETPS_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == ETPS_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == ETPS_A::DIV8
    }
}
impl core::ops::Deref for ETPS_R {
    type Target = crate::FieldReader<u8, ETPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ETPS` writer - External trigger prescaler
pub struct ETPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETPS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ETPS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Prescaler OFF
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(ETPS_A::DIV1)
    }
    ///ETRP frequency divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ETPS_A::DIV2)
    }
    ///ETRP frequency divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ETPS_A::DIV4)
    }
    ///ETRP frequency divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ETPS_A::DIV8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
///External trigger filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETF_A {
    ///0: No filter, sampling is done at fDTS
    NOFILTER = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    FCK_INT_N2 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    FCK_INT_N4 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    FCK_INT_N8 = 3,
    ///4: fSAMPLING=fDTS/2, N=6
    FDTS_DIV2_N6 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    FDTS_DIV2_N8 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    FDTS_DIV4_N6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    FDTS_DIV4_N8 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    FDTS_DIV8_N6 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    FDTS_DIV8_N8 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    FDTS_DIV16_N5 = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    FDTS_DIV16_N6 = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    FDTS_DIV16_N8 = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    FDTS_DIV32_N5 = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    FDTS_DIV32_N6 = 14,
    ///15: fSAMPLING=fDTS/32, N=8
    FDTS_DIV32_N8 = 15,
}
impl From<ETF_A> for u8 {
    #[inline(always)]
    fn from(variant: ETF_A) -> Self {
        variant as _
    }
}
///Field `ETF` reader - External trigger filter
pub struct ETF_R(crate::FieldReader<u8, ETF_A>);
impl ETF_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ETF_A {
        match self.bits {
            0 => ETF_A::NOFILTER,
            1 => ETF_A::FCK_INT_N2,
            2 => ETF_A::FCK_INT_N4,
            3 => ETF_A::FCK_INT_N8,
            4 => ETF_A::FDTS_DIV2_N6,
            5 => ETF_A::FDTS_DIV2_N8,
            6 => ETF_A::FDTS_DIV4_N6,
            7 => ETF_A::FDTS_DIV4_N8,
            8 => ETF_A::FDTS_DIV8_N6,
            9 => ETF_A::FDTS_DIV8_N8,
            10 => ETF_A::FDTS_DIV16_N5,
            11 => ETF_A::FDTS_DIV16_N6,
            12 => ETF_A::FDTS_DIV16_N8,
            13 => ETF_A::FDTS_DIV32_N5,
            14 => ETF_A::FDTS_DIV32_N6,
            15 => ETF_A::FDTS_DIV32_N8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOFILTER`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        **self == ETF_A::NOFILTER
    }
    ///Checks if the value of the field is `FCK_INT_N2`
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        **self == ETF_A::FCK_INT_N2
    }
    ///Checks if the value of the field is `FCK_INT_N4`
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        **self == ETF_A::FCK_INT_N4
    }
    ///Checks if the value of the field is `FCK_INT_N8`
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        **self == ETF_A::FCK_INT_N8
    }
    ///Checks if the value of the field is `FDTS_DIV2_N6`
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        **self == ETF_A::FDTS_DIV2_N6
    }
    ///Checks if the value of the field is `FDTS_DIV2_N8`
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        **self == ETF_A::FDTS_DIV2_N8
    }
    ///Checks if the value of the field is `FDTS_DIV4_N6`
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        **self == ETF_A::FDTS_DIV4_N6
    }
    ///Checks if the value of the field is `FDTS_DIV4_N8`
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        **self == ETF_A::FDTS_DIV4_N8
    }
    ///Checks if the value of the field is `FDTS_DIV8_N6`
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        **self == ETF_A::FDTS_DIV8_N6
    }
    ///Checks if the value of the field is `FDTS_DIV8_N8`
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        **self == ETF_A::FDTS_DIV8_N8
    }
    ///Checks if the value of the field is `FDTS_DIV16_N5`
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        **self == ETF_A::FDTS_DIV16_N5
    }
    ///Checks if the value of the field is `FDTS_DIV16_N6`
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        **self == ETF_A::FDTS_DIV16_N6
    }
    ///Checks if the value of the field is `FDTS_DIV16_N8`
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        **self == ETF_A::FDTS_DIV16_N8
    }
    ///Checks if the value of the field is `FDTS_DIV32_N5`
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        **self == ETF_A::FDTS_DIV32_N5
    }
    ///Checks if the value of the field is `FDTS_DIV32_N6`
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        **self == ETF_A::FDTS_DIV32_N6
    }
    ///Checks if the value of the field is `FDTS_DIV32_N8`
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        **self == ETF_A::FDTS_DIV32_N8
    }
}
impl core::ops::Deref for ETF_R {
    type Target = crate::FieldReader<u8, ETF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ETF` writer - External trigger filter
pub struct ETF_W<'a> {
    w: &'a mut W,
}
impl<'a> ETF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ETF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(ETF_A::NOFILTER)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(ETF_A::FCK_INT_N2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(ETF_A::FCK_INT_N4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(ETF_A::FCK_INT_N8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV2_N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV2_N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV4_N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV4_N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV8_N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV8_N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV16_N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV16_N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV16_N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV32_N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV32_N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV32_N8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Master/slave mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSM_A {
    ///0: No action
    NOSYNC = 0,
    ///1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event
    SYNC = 1,
}
impl From<MSM_A> for bool {
    #[inline(always)]
    fn from(variant: MSM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSM` reader - Master/slave mode
pub struct MSM_R(crate::FieldReader<bool, MSM_A>);
impl MSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSM_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSM_A {
        match self.bits {
            false => MSM_A::NOSYNC,
            true => MSM_A::SYNC,
        }
    }
    ///Checks if the value of the field is `NOSYNC`
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        **self == MSM_A::NOSYNC
    }
    ///Checks if the value of the field is `SYNC`
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        **self == MSM_A::SYNC
    }
}
impl core::ops::Deref for MSM_R {
    type Target = crate::FieldReader<bool, MSM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MSM` writer - Master/slave mode
pub struct MSM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action
    #[inline(always)]
    pub fn no_sync(self) -> &'a mut W {
        self.variant(MSM_A::NOSYNC)
    }
    ///The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(MSM_A::SYNC)
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
///Trigger selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TS_A {
    ///0: Internal Trigger 0 (ITR0)
    ITR0 = 0,
    ///1: Internal Trigger 1 (ITR1)
    ITR1 = 1,
    ///2: Internal Trigger 2 (ITR2)
    ITR2 = 2,
    ///4: TI1 Edge Detector (TI1F_ED)
    TI1F_ED = 4,
    ///5: Filtered Timer Input 1 (TI1FP1)
    TI1FP1 = 5,
    ///6: Filtered Timer Input 2 (TI2FP2)
    TI2FP2 = 6,
    ///7: External Trigger input (ETRF)
    ETRF = 7,
}
impl From<TS_A> for u8 {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as _
    }
}
///Field `TS` reader - Trigger selection
pub struct TS_R(crate::FieldReader<u8, TS_A>);
impl TS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TS_A> {
        match self.bits {
            0 => Some(TS_A::ITR0),
            1 => Some(TS_A::ITR1),
            2 => Some(TS_A::ITR2),
            4 => Some(TS_A::TI1F_ED),
            5 => Some(TS_A::TI1FP1),
            6 => Some(TS_A::TI2FP2),
            7 => Some(TS_A::ETRF),
            _ => None,
        }
    }
    ///Checks if the value of the field is `ITR0`
    #[inline(always)]
    pub fn is_itr0(&self) -> bool {
        **self == TS_A::ITR0
    }
    ///Checks if the value of the field is `ITR1`
    #[inline(always)]
    pub fn is_itr1(&self) -> bool {
        **self == TS_A::ITR1
    }
    ///Checks if the value of the field is `ITR2`
    #[inline(always)]
    pub fn is_itr2(&self) -> bool {
        **self == TS_A::ITR2
    }
    ///Checks if the value of the field is `TI1F_ED`
    #[inline(always)]
    pub fn is_ti1f_ed(&self) -> bool {
        **self == TS_A::TI1F_ED
    }
    ///Checks if the value of the field is `TI1FP1`
    #[inline(always)]
    pub fn is_ti1fp1(&self) -> bool {
        **self == TS_A::TI1FP1
    }
    ///Checks if the value of the field is `TI2FP2`
    #[inline(always)]
    pub fn is_ti2fp2(&self) -> bool {
        **self == TS_A::TI2FP2
    }
    ///Checks if the value of the field is `ETRF`
    #[inline(always)]
    pub fn is_etrf(&self) -> bool {
        **self == TS_A::ETRF
    }
}
impl core::ops::Deref for TS_R {
    type Target = crate::FieldReader<u8, TS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TS` writer - Trigger selection
pub struct TS_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Internal Trigger 0 (ITR0)
    #[inline(always)]
    pub fn itr0(self) -> &'a mut W {
        self.variant(TS_A::ITR0)
    }
    ///Internal Trigger 1 (ITR1)
    #[inline(always)]
    pub fn itr1(self) -> &'a mut W {
        self.variant(TS_A::ITR1)
    }
    ///Internal Trigger 2 (ITR2)
    #[inline(always)]
    pub fn itr2(self) -> &'a mut W {
        self.variant(TS_A::ITR2)
    }
    ///TI1 Edge Detector (TI1F_ED)
    #[inline(always)]
    pub fn ti1f_ed(self) -> &'a mut W {
        self.variant(TS_A::TI1F_ED)
    }
    ///Filtered Timer Input 1 (TI1FP1)
    #[inline(always)]
    pub fn ti1fp1(self) -> &'a mut W {
        self.variant(TS_A::TI1FP1)
    }
    ///Filtered Timer Input 2 (TI2FP2)
    #[inline(always)]
    pub fn ti2fp2(self) -> &'a mut W {
        self.variant(TS_A::TI2FP2)
    }
    ///External Trigger input (ETRF)
    #[inline(always)]
    pub fn etrf(self) -> &'a mut W {
        self.variant(TS_A::ETRF)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
///Field `OCCS` reader - OCREF clear selection
pub struct OCCS_R(crate::FieldReader<bool, bool>);
impl OCCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OCCS` writer - OCREF clear selection
pub struct OCCS_W<'a> {
    w: &'a mut W,
}
impl<'a> OCCS_W<'a> {
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
///Slave mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMS_A {
    ///0: Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock. If SMS\[3\]=1 then Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter
    DISABLEDORCOMBINED = 0,
    ///1: Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level
    ENCODERMODE1 = 1,
    ///2: Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level
    ENCODERMODE2 = 2,
    ///3: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input
    ENCODERMODE3 = 3,
    ///4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers
    RESETMODE = 4,
    ///5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled
    GATEDMODE = 5,
    ///6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled
    TRIGGERMODE = 6,
    ///7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter
    EXTCLOCKMODE = 7,
}
impl From<SMS_A> for u8 {
    #[inline(always)]
    fn from(variant: SMS_A) -> Self {
        variant as _
    }
}
///Field `SMS` reader - Slave mode selection
pub struct SMS_R(crate::FieldReader<u8, SMS_A>);
impl SMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMS_A {
        match self.bits {
            0 => SMS_A::DISABLEDORCOMBINED,
            1 => SMS_A::ENCODERMODE1,
            2 => SMS_A::ENCODERMODE2,
            3 => SMS_A::ENCODERMODE3,
            4 => SMS_A::RESETMODE,
            5 => SMS_A::GATEDMODE,
            6 => SMS_A::TRIGGERMODE,
            7 => SMS_A::EXTCLOCKMODE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DISABLEDORCOMBINED`
    #[inline(always)]
    pub fn is_disabled_or_combined(&self) -> bool {
        **self == SMS_A::DISABLEDORCOMBINED
    }
    ///Checks if the value of the field is `ENCODERMODE1`
    #[inline(always)]
    pub fn is_encoder_mode1(&self) -> bool {
        **self == SMS_A::ENCODERMODE1
    }
    ///Checks if the value of the field is `ENCODERMODE2`
    #[inline(always)]
    pub fn is_encoder_mode2(&self) -> bool {
        **self == SMS_A::ENCODERMODE2
    }
    ///Checks if the value of the field is `ENCODERMODE3`
    #[inline(always)]
    pub fn is_encoder_mode3(&self) -> bool {
        **self == SMS_A::ENCODERMODE3
    }
    ///Checks if the value of the field is `RESETMODE`
    #[inline(always)]
    pub fn is_reset_mode(&self) -> bool {
        **self == SMS_A::RESETMODE
    }
    ///Checks if the value of the field is `GATEDMODE`
    #[inline(always)]
    pub fn is_gated_mode(&self) -> bool {
        **self == SMS_A::GATEDMODE
    }
    ///Checks if the value of the field is `TRIGGERMODE`
    #[inline(always)]
    pub fn is_trigger_mode(&self) -> bool {
        **self == SMS_A::TRIGGERMODE
    }
    ///Checks if the value of the field is `EXTCLOCKMODE`
    #[inline(always)]
    pub fn is_ext_clock_mode(&self) -> bool {
        **self == SMS_A::EXTCLOCKMODE
    }
}
impl core::ops::Deref for SMS_R {
    type Target = crate::FieldReader<u8, SMS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SMS` writer - Slave mode selection
pub struct SMS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock. If SMS\[3\]=1 then Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter
    #[inline(always)]
    pub fn disabled_or_combined(self) -> &'a mut W {
        self.variant(SMS_A::DISABLEDORCOMBINED)
    }
    ///Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level
    #[inline(always)]
    pub fn encoder_mode1(self) -> &'a mut W {
        self.variant(SMS_A::ENCODERMODE1)
    }
    ///Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level
    #[inline(always)]
    pub fn encoder_mode2(self) -> &'a mut W {
        self.variant(SMS_A::ENCODERMODE2)
    }
    ///Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input
    #[inline(always)]
    pub fn encoder_mode3(self) -> &'a mut W {
        self.variant(SMS_A::ENCODERMODE3)
    }
    ///Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers
    #[inline(always)]
    pub fn reset_mode(self) -> &'a mut W {
        self.variant(SMS_A::RESETMODE)
    }
    ///Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled
    #[inline(always)]
    pub fn gated_mode(self) -> &'a mut W {
        self.variant(SMS_A::GATEDMODE)
    }
    ///Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled
    #[inline(always)]
    pub fn trigger_mode(self) -> &'a mut W {
        self.variant(SMS_A::TRIGGERMODE)
    }
    ///External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter
    #[inline(always)]
    pub fn ext_clock_mode(self) -> &'a mut W {
        self.variant(SMS_A::EXTCLOCKMODE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bits 20:21 - Trigger selection
    #[inline(always)]
    pub fn ts3_4(&self) -> TS3_4_R {
        TS3_4_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bit 16 - Slave mode selection
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 3 - OCREF clear selection
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 20:21 - Trigger selection
    #[inline(always)]
    pub fn ts3_4(&mut self) -> TS3_4_W {
        TS3_4_W { w: self }
    }
    ///Bit 16 - Slave mode selection
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W {
        SMS_3_W { w: self }
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W {
        ETP_W { w: self }
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W {
        ECE_W { w: self }
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W {
        ETPS_W { w: self }
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W {
        ETF_W { w: self }
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W { w: self }
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W { w: self }
    }
    ///Bit 3 - OCREF clear selection
    #[inline(always)]
    pub fn occs(&mut self) -> OCCS_W {
        OCCS_W { w: self }
    }
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W {
        SMS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///slave mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smcr](index.html) module
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [smcr::R](R) reader structure
impl crate::Readable for SMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smcr::W](W) writer structure
impl crate::Writable for SMCR_SPEC {
    type Writer = W;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}