///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///True random number generator enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNGEN_A {
    ///0: Random number generator is disabled
    DISABLED = 0,
    ///1: Random number generator is enabled
    ENABLED = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RNGEN` reader - True random number generator enable
pub struct RNGEN_R(crate::FieldReader<bool, RNGEN_A>);
impl RNGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::DISABLED,
            true => RNGEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RNGEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RNGEN_A::ENABLED
    }
}
impl core::ops::Deref for RNGEN_R {
    type Target = crate::FieldReader<bool, RNGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RNGEN` writer - True random number generator enable
pub struct RNGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RNGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Random number generator is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGEN_A::DISABLED)
    }
    ///Random number generator is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGEN_A::ENABLED)
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
///Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_A {
    ///0: RNG interrupt is disabled
    DISABLED = 0,
    ///1: RNG interrupt is enabled
    ENABLED = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IE` reader - Interrupt Enable
pub struct IE_R(crate::FieldReader<bool, IE_A>);
impl IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::DISABLED,
            true => IE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IE_A::ENABLED
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IE` writer - Interrupt Enable
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///RNG interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IE_A::DISABLED)
    }
    ///RNG interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IE_A::ENABLED)
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
///Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CED_A {
    ///0: Clock error detection is enabled
    ENABLED = 0,
    ///1: Clock error detection is disabled
    DISABLED = 1,
}
impl From<CED_A> for bool {
    #[inline(always)]
    fn from(variant: CED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CED` reader - Interrupt Enable
pub struct CED_R(crate::FieldReader<bool, CED_A>);
impl CED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CED_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CED_A {
        match self.bits {
            false => CED_A::ENABLED,
            true => CED_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CED_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CED_A::DISABLED
    }
}
impl core::ops::Deref for CED_R {
    type Target = crate::FieldReader<bool, CED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CED` writer - Interrupt Enable
pub struct CED_W<'a> {
    w: &'a mut W,
}
impl<'a> CED_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock error detection is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CED_A::ENABLED)
    }
    ///Clock error detection is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CED_A::DISABLED)
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
///RNG_CONFIG3
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RNG_CONFIG3_A {
    ///13: Recommended value for config A (NIST certifiable)
    CONFIGA = 13,
    ///0: Recommended value for config B (not NIST certifiable)
    CONFIGB = 0,
}
impl From<RNG_CONFIG3_A> for u8 {
    #[inline(always)]
    fn from(variant: RNG_CONFIG3_A) -> Self {
        variant as _
    }
}
///Field `RNG_CONFIG3` reader - RNG_CONFIG3
pub struct RNG_CONFIG3_R(crate::FieldReader<u8, RNG_CONFIG3_A>);
impl RNG_CONFIG3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNG_CONFIG3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RNG_CONFIG3_A> {
        match self.bits {
            13 => Some(RNG_CONFIG3_A::CONFIGA),
            0 => Some(RNG_CONFIG3_A::CONFIGB),
            _ => None,
        }
    }
    ///Checks if the value of the field is `CONFIGA`
    #[inline(always)]
    pub fn is_config_a(&self) -> bool {
        **self == RNG_CONFIG3_A::CONFIGA
    }
    ///Checks if the value of the field is `CONFIGB`
    #[inline(always)]
    pub fn is_config_b(&self) -> bool {
        **self == RNG_CONFIG3_A::CONFIGB
    }
}
impl core::ops::Deref for RNG_CONFIG3_R {
    type Target = crate::FieldReader<u8, RNG_CONFIG3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RNG_CONFIG3` writer - RNG_CONFIG3
pub struct RNG_CONFIG3_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_CONFIG3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RNG_CONFIG3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Recommended value for config A (NIST certifiable)
    #[inline(always)]
    pub fn config_a(self) -> &'a mut W {
        self.variant(RNG_CONFIG3_A::CONFIGA)
    }
    ///Recommended value for config B (not NIST certifiable)
    #[inline(always)]
    pub fn config_b(self) -> &'a mut W {
        self.variant(RNG_CONFIG3_A::CONFIGB)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///NISTC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NISTC_A {
    ///0: Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used
    DEFAULT = 0,
    ///1: Custom values for NIST compliant RNG
    CUSTOM = 1,
}
impl From<NISTC_A> for bool {
    #[inline(always)]
    fn from(variant: NISTC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NISTC` reader - NISTC
pub struct NISTC_R(crate::FieldReader<bool, NISTC_A>);
impl NISTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        NISTC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NISTC_A {
        match self.bits {
            false => NISTC_A::DEFAULT,
            true => NISTC_A::CUSTOM,
        }
    }
    ///Checks if the value of the field is `DEFAULT`
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == NISTC_A::DEFAULT
    }
    ///Checks if the value of the field is `CUSTOM`
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        **self == NISTC_A::CUSTOM
    }
}
impl core::ops::Deref for NISTC_R {
    type Target = crate::FieldReader<bool, NISTC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NISTC` writer - NISTC
pub struct NISTC_W<'a> {
    w: &'a mut W,
}
impl<'a> NISTC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NISTC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(NISTC_A::DEFAULT)
    }
    ///Custom values for NIST compliant RNG
    #[inline(always)]
    pub fn custom(self) -> &'a mut W {
        self.variant(NISTC_A::CUSTOM)
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
///RNG_CONFIG2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RNG_CONFIG2_A {
    ///0: Recommended value for config A and B
    CONFIGA_B = 0,
}
impl From<RNG_CONFIG2_A> for u8 {
    #[inline(always)]
    fn from(variant: RNG_CONFIG2_A) -> Self {
        variant as _
    }
}
///Field `RNG_CONFIG2` reader - RNG_CONFIG2
pub struct RNG_CONFIG2_R(crate::FieldReader<u8, RNG_CONFIG2_A>);
impl RNG_CONFIG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNG_CONFIG2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RNG_CONFIG2_A> {
        match self.bits {
            0 => Some(RNG_CONFIG2_A::CONFIGA_B),
            _ => None,
        }
    }
    ///Checks if the value of the field is `CONFIGA_B`
    #[inline(always)]
    pub fn is_config_a_b(&self) -> bool {
        **self == RNG_CONFIG2_A::CONFIGA_B
    }
}
impl core::ops::Deref for RNG_CONFIG2_R {
    type Target = crate::FieldReader<u8, RNG_CONFIG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RNG_CONFIG2` writer - RNG_CONFIG2
pub struct RNG_CONFIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_CONFIG2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RNG_CONFIG2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Recommended value for config A and B
    #[inline(always)]
    pub fn config_a_b(self) -> &'a mut W {
        self.variant(RNG_CONFIG2_A::CONFIGA_B)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
///CLKDIV
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKDIV_A {
    ///0: Internal RNG clock after divider is similar to incoming RNG clock
    NODIV = 0,
    ///1: Divide RNG clock by 2^1
    DIV_2_1 = 1,
    ///2: Divide RNG clock by 2^2
    DIV_2_2 = 2,
    ///3: Divide RNG clock by 2^3
    DIV_2_3 = 3,
    ///4: Divide RNG clock by 2^4
    DIV_2_4 = 4,
    ///5: Divide RNG clock by 2^5
    DIV_2_5 = 5,
    ///6: Divide RNG clock by 2^6
    DIV_2_6 = 6,
    ///7: Divide RNG clock by 2^7
    DIV_2_7 = 7,
    ///8: Divide RNG clock by 2^8
    DIV_2_8 = 8,
    ///9: Divide RNG clock by 2^9
    DIV_2_9 = 9,
    ///10: Divide RNG clock by 2^10
    DIV_2_10 = 10,
    ///11: Divide RNG clock by 2^11
    DIV_2_11 = 11,
    ///12: Divide RNG clock by 2^12
    DIV_2_12 = 12,
    ///13: Divide RNG clock by 2^13
    DIV_2_13 = 13,
    ///14: Divide RNG clock by 2^14
    DIV_2_14 = 14,
    ///15: Divide RNG clock by 2^15
    DIV_2_15 = 15,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as _
    }
}
///Field `CLKDIV` reader - CLKDIV
pub struct CLKDIV_R(crate::FieldReader<u8, CLKDIV_A>);
impl CLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CLKDIV_A {
        match self.bits {
            0 => CLKDIV_A::NODIV,
            1 => CLKDIV_A::DIV_2_1,
            2 => CLKDIV_A::DIV_2_2,
            3 => CLKDIV_A::DIV_2_3,
            4 => CLKDIV_A::DIV_2_4,
            5 => CLKDIV_A::DIV_2_5,
            6 => CLKDIV_A::DIV_2_6,
            7 => CLKDIV_A::DIV_2_7,
            8 => CLKDIV_A::DIV_2_8,
            9 => CLKDIV_A::DIV_2_9,
            10 => CLKDIV_A::DIV_2_10,
            11 => CLKDIV_A::DIV_2_11,
            12 => CLKDIV_A::DIV_2_12,
            13 => CLKDIV_A::DIV_2_13,
            14 => CLKDIV_A::DIV_2_14,
            15 => CLKDIV_A::DIV_2_15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NODIV`
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        **self == CLKDIV_A::NODIV
    }
    ///Checks if the value of the field is `DIV_2_1`
    #[inline(always)]
    pub fn is_div_2_1(&self) -> bool {
        **self == CLKDIV_A::DIV_2_1
    }
    ///Checks if the value of the field is `DIV_2_2`
    #[inline(always)]
    pub fn is_div_2_2(&self) -> bool {
        **self == CLKDIV_A::DIV_2_2
    }
    ///Checks if the value of the field is `DIV_2_3`
    #[inline(always)]
    pub fn is_div_2_3(&self) -> bool {
        **self == CLKDIV_A::DIV_2_3
    }
    ///Checks if the value of the field is `DIV_2_4`
    #[inline(always)]
    pub fn is_div_2_4(&self) -> bool {
        **self == CLKDIV_A::DIV_2_4
    }
    ///Checks if the value of the field is `DIV_2_5`
    #[inline(always)]
    pub fn is_div_2_5(&self) -> bool {
        **self == CLKDIV_A::DIV_2_5
    }
    ///Checks if the value of the field is `DIV_2_6`
    #[inline(always)]
    pub fn is_div_2_6(&self) -> bool {
        **self == CLKDIV_A::DIV_2_6
    }
    ///Checks if the value of the field is `DIV_2_7`
    #[inline(always)]
    pub fn is_div_2_7(&self) -> bool {
        **self == CLKDIV_A::DIV_2_7
    }
    ///Checks if the value of the field is `DIV_2_8`
    #[inline(always)]
    pub fn is_div_2_8(&self) -> bool {
        **self == CLKDIV_A::DIV_2_8
    }
    ///Checks if the value of the field is `DIV_2_9`
    #[inline(always)]
    pub fn is_div_2_9(&self) -> bool {
        **self == CLKDIV_A::DIV_2_9
    }
    ///Checks if the value of the field is `DIV_2_10`
    #[inline(always)]
    pub fn is_div_2_10(&self) -> bool {
        **self == CLKDIV_A::DIV_2_10
    }
    ///Checks if the value of the field is `DIV_2_11`
    #[inline(always)]
    pub fn is_div_2_11(&self) -> bool {
        **self == CLKDIV_A::DIV_2_11
    }
    ///Checks if the value of the field is `DIV_2_12`
    #[inline(always)]
    pub fn is_div_2_12(&self) -> bool {
        **self == CLKDIV_A::DIV_2_12
    }
    ///Checks if the value of the field is `DIV_2_13`
    #[inline(always)]
    pub fn is_div_2_13(&self) -> bool {
        **self == CLKDIV_A::DIV_2_13
    }
    ///Checks if the value of the field is `DIV_2_14`
    #[inline(always)]
    pub fn is_div_2_14(&self) -> bool {
        **self == CLKDIV_A::DIV_2_14
    }
    ///Checks if the value of the field is `DIV_2_15`
    #[inline(always)]
    pub fn is_div_2_15(&self) -> bool {
        **self == CLKDIV_A::DIV_2_15
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u8, CLKDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CLKDIV` writer - CLKDIV
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CLKDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Internal RNG clock after divider is similar to incoming RNG clock
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(CLKDIV_A::NODIV)
    }
    ///Divide RNG clock by 2^1
    #[inline(always)]
    pub fn div_2_1(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_1)
    }
    ///Divide RNG clock by 2^2
    #[inline(always)]
    pub fn div_2_2(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_2)
    }
    ///Divide RNG clock by 2^3
    #[inline(always)]
    pub fn div_2_3(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_3)
    }
    ///Divide RNG clock by 2^4
    #[inline(always)]
    pub fn div_2_4(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_4)
    }
    ///Divide RNG clock by 2^5
    #[inline(always)]
    pub fn div_2_5(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_5)
    }
    ///Divide RNG clock by 2^6
    #[inline(always)]
    pub fn div_2_6(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_6)
    }
    ///Divide RNG clock by 2^7
    #[inline(always)]
    pub fn div_2_7(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_7)
    }
    ///Divide RNG clock by 2^8
    #[inline(always)]
    pub fn div_2_8(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_8)
    }
    ///Divide RNG clock by 2^9
    #[inline(always)]
    pub fn div_2_9(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_9)
    }
    ///Divide RNG clock by 2^10
    #[inline(always)]
    pub fn div_2_10(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_10)
    }
    ///Divide RNG clock by 2^11
    #[inline(always)]
    pub fn div_2_11(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_11)
    }
    ///Divide RNG clock by 2^12
    #[inline(always)]
    pub fn div_2_12(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_12)
    }
    ///Divide RNG clock by 2^13
    #[inline(always)]
    pub fn div_2_13(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_13)
    }
    ///Divide RNG clock by 2^14
    #[inline(always)]
    pub fn div_2_14(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_14)
    }
    ///Divide RNG clock by 2^15
    #[inline(always)]
    pub fn div_2_15(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2_15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
///RNG_CONFIG1
///
///Value on reset: 8
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RNG_CONFIG1_A {
    ///15: Recommended value for config A (NIST certifiable)
    CONFIGA = 15,
    ///24: Recommended value for config B (not NIST certifiable)
    CONFIGB = 24,
}
impl From<RNG_CONFIG1_A> for u8 {
    #[inline(always)]
    fn from(variant: RNG_CONFIG1_A) -> Self {
        variant as _
    }
}
///Field `RNG_CONFIG1` reader - RNG_CONFIG1
pub struct RNG_CONFIG1_R(crate::FieldReader<u8, RNG_CONFIG1_A>);
impl RNG_CONFIG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNG_CONFIG1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RNG_CONFIG1_A> {
        match self.bits {
            15 => Some(RNG_CONFIG1_A::CONFIGA),
            24 => Some(RNG_CONFIG1_A::CONFIGB),
            _ => None,
        }
    }
    ///Checks if the value of the field is `CONFIGA`
    #[inline(always)]
    pub fn is_config_a(&self) -> bool {
        **self == RNG_CONFIG1_A::CONFIGA
    }
    ///Checks if the value of the field is `CONFIGB`
    #[inline(always)]
    pub fn is_config_b(&self) -> bool {
        **self == RNG_CONFIG1_A::CONFIGB
    }
}
impl core::ops::Deref for RNG_CONFIG1_R {
    type Target = crate::FieldReader<u8, RNG_CONFIG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RNG_CONFIG1` writer - RNG_CONFIG1
pub struct RNG_CONFIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_CONFIG1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RNG_CONFIG1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Recommended value for config A (NIST certifiable)
    #[inline(always)]
    pub fn config_a(self) -> &'a mut W {
        self.variant(RNG_CONFIG1_A::CONFIGA)
    }
    ///Recommended value for config B (not NIST certifiable)
    #[inline(always)]
    pub fn config_b(self) -> &'a mut W {
        self.variant(RNG_CONFIG1_A::CONFIGB)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | ((value as u32 & 0x3f) << 20);
        self.w
    }
}
///Field `CONDRST` reader - Conditioning soft reset
pub struct CONDRST_R(crate::FieldReader<bool, bool>);
impl CONDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONDRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONDRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CONDRST` writer - Conditioning soft reset
pub struct CONDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CONDRST_W<'a> {
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
///CONFIGLOCK
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONFIGLOCK_A {
    ///0: Writes to the RNG_CR configuration bits \[29:4\]
    ///are allowed
    ENABLED = 0,
    ///1: Writes to the RNG_CR configuration bits \[29:4\]
    ///are ignored until the next RNG reset
    DISABLED = 1,
}
impl From<CONFIGLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: CONFIGLOCK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CONFIGLOCK` reader - CONFIGLOCK
pub struct CONFIGLOCK_R(crate::FieldReader<bool, CONFIGLOCK_A>);
impl CONFIGLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONFIGLOCK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CONFIGLOCK_A {
        match self.bits {
            false => CONFIGLOCK_A::ENABLED,
            true => CONFIGLOCK_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CONFIGLOCK_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CONFIGLOCK_A::DISABLED
    }
}
impl core::ops::Deref for CONFIGLOCK_R {
    type Target = crate::FieldReader<bool, CONFIGLOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CONFIGLOCK` writer - CONFIGLOCK
pub struct CONFIGLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIGLOCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CONFIGLOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Writes to the RNG_CR configuration bits \[29:4\]
    ///are allowed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CONFIGLOCK_A::ENABLED)
    }
    ///Writes to the RNG_CR configuration bits \[29:4\]
    ///are ignored until the next RNG reset
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CONFIGLOCK_A::DISABLED)
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
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Interrupt Enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 5 - Interrupt Enable
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bits 8:11 - RNG_CONFIG3
    #[inline(always)]
    pub fn rng_config3(&self) -> RNG_CONFIG3_R {
        RNG_CONFIG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - NISTC
    #[inline(always)]
    pub fn nistc(&self) -> NISTC_R {
        NISTC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bits 13:15 - RNG_CONFIG2
    #[inline(always)]
    pub fn rng_config2(&self) -> RNG_CONFIG2_R {
        RNG_CONFIG2_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    ///Bits 16:19 - CLKDIV
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:25 - RNG_CONFIG1
    #[inline(always)]
    pub fn rng_config1(&self) -> RNG_CONFIG1_R {
        RNG_CONFIG1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 30 - Conditioning soft reset
    #[inline(always)]
    pub fn condrst(&self) -> CONDRST_R {
        CONDRST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - CONFIGLOCK
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W {
        RNGEN_W { w: self }
    }
    ///Bit 3 - Interrupt Enable
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    ///Bit 5 - Interrupt Enable
    #[inline(always)]
    pub fn ced(&mut self) -> CED_W {
        CED_W { w: self }
    }
    ///Bits 8:11 - RNG_CONFIG3
    #[inline(always)]
    pub fn rng_config3(&mut self) -> RNG_CONFIG3_W {
        RNG_CONFIG3_W { w: self }
    }
    ///Bit 12 - NISTC
    #[inline(always)]
    pub fn nistc(&mut self) -> NISTC_W {
        NISTC_W { w: self }
    }
    ///Bits 13:15 - RNG_CONFIG2
    #[inline(always)]
    pub fn rng_config2(&mut self) -> RNG_CONFIG2_W {
        RNG_CONFIG2_W { w: self }
    }
    ///Bits 16:19 - CLKDIV
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    ///Bits 20:25 - RNG_CONFIG1
    #[inline(always)]
    pub fn rng_config1(&mut self) -> RNG_CONFIG1_W {
        RNG_CONFIG1_W { w: self }
    }
    ///Bit 30 - Conditioning soft reset
    #[inline(always)]
    pub fn condrst(&mut self) -> CONDRST_W {
        CONDRST_W { w: self }
    }
    ///Bit 31 - CONFIGLOCK
    #[inline(always)]
    pub fn configlock(&mut self) -> CONFIGLOCK_W {
        CONFIGLOCK_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
///`reset()` method sets CR to value 0x0080_0000
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
