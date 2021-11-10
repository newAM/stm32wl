///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Rx buffer DMA enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAEN_A {
    ///0: Rx buffer DMA disabled
    DISABLED = 0,
    ///1: Rx buffer DMA enabled
    ENABLED = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMAEN` reader - Rx buffer DMA enable
pub struct RXDMAEN_R(crate::FieldReader<bool, RXDMAEN_A>);
impl RXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::DISABLED,
            true => RXDMAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXDMAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXDMAEN_A::ENABLED
    }
}
impl core::ops::Deref for RXDMAEN_R {
    type Target = crate::FieldReader<bool, RXDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXDMAEN` writer - Rx buffer DMA enable
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::DISABLED)
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::ENABLED)
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
///Tx buffer DMA enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAEN_A {
    ///0: Tx buffer DMA disabled
    DISABLED = 0,
    ///1: Tx buffer DMA enabled
    ENABLED = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDMAEN` reader - Tx buffer DMA enable
pub struct TXDMAEN_R(crate::FieldReader<bool, TXDMAEN_A>);
impl TXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::DISABLED,
            true => TXDMAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXDMAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXDMAEN_A::ENABLED
    }
}
impl core::ops::Deref for TXDMAEN_R {
    type Target = crate::FieldReader<bool, TXDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXDMAEN` writer - Tx buffer DMA enable
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::DISABLED)
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::ENABLED)
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
///SS output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOE_A {
    ///0: SS output is disabled in master mode
    DISABLED = 0,
    ///1: SS output is enabled in master mode
    ENABLED = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSOE` reader - SS output enable
pub struct SSOE_R(crate::FieldReader<bool, SSOE_A>);
impl SSOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSOE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::DISABLED,
            true => SSOE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SSOE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SSOE_A::ENABLED
    }
}
impl core::ops::Deref for SSOE_R {
    type Target = crate::FieldReader<bool, SSOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SSOE` writer - SS output enable
pub struct SSOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SSOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SS output is disabled in master mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOE_A::DISABLED)
    }
    ///SS output is enabled in master mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOE_A::ENABLED)
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
///NSS pulse management
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSSP_A {
    ///0: No NSS pulse
    NOPULSE = 0,
    ///1: NSS pulse generated
    PULSEGENERATED = 1,
}
impl From<NSSP_A> for bool {
    #[inline(always)]
    fn from(variant: NSSP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NSSP` reader - NSS pulse management
pub struct NSSP_R(crate::FieldReader<bool, NSSP_A>);
impl NSSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSSP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NSSP_A {
        match self.bits {
            false => NSSP_A::NOPULSE,
            true => NSSP_A::PULSEGENERATED,
        }
    }
    ///Checks if the value of the field is `NOPULSE`
    #[inline(always)]
    pub fn is_no_pulse(&self) -> bool {
        **self == NSSP_A::NOPULSE
    }
    ///Checks if the value of the field is `PULSEGENERATED`
    #[inline(always)]
    pub fn is_pulse_generated(&self) -> bool {
        **self == NSSP_A::PULSEGENERATED
    }
}
impl core::ops::Deref for NSSP_R {
    type Target = crate::FieldReader<bool, NSSP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NSSP` writer - NSS pulse management
pub struct NSSP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSSP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NSSP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No NSS pulse
    #[inline(always)]
    pub fn no_pulse(self) -> &'a mut W {
        self.variant(NSSP_A::NOPULSE)
    }
    ///NSS pulse generated
    #[inline(always)]
    pub fn pulse_generated(self) -> &'a mut W {
        self.variant(NSSP_A::PULSEGENERATED)
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
///Frame format
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRF_A {
    ///0: SPI Motorola mode
    MOTOROLA = 0,
    ///1: SPI TI mode
    TI = 1,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRF` reader - Frame format
pub struct FRF_R(crate::FieldReader<bool, FRF_A>);
impl FRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            false => FRF_A::MOTOROLA,
            true => FRF_A::TI,
        }
    }
    ///Checks if the value of the field is `MOTOROLA`
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        **self == FRF_A::MOTOROLA
    }
    ///Checks if the value of the field is `TI`
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        **self == FRF_A::TI
    }
}
impl core::ops::Deref for FRF_R {
    type Target = crate::FieldReader<bool, FRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FRF` writer - Frame format
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SPI Motorola mode
    #[inline(always)]
    pub fn motorola(self) -> &'a mut W {
        self.variant(FRF_A::MOTOROLA)
    }
    ///SPI TI mode
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRF_A::TI)
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
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    ///0: Error interrupt masked
    MASKED = 0,
    ///1: Error interrupt not masked
    NOTMASKED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRIE` reader - Error interrupt enable
pub struct ERRIE_R(crate::FieldReader<bool, ERRIE_A>);
impl ERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::MASKED,
            true => ERRIE_A::NOTMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == ERRIE_A::MASKED
    }
    ///Checks if the value of the field is `NOTMASKED`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        **self == ERRIE_A::NOTMASKED
    }
}
impl core::ops::Deref for ERRIE_R {
    type Target = crate::FieldReader<bool, ERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ERRIE` writer - Error interrupt enable
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Error interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(ERRIE_A::MASKED)
    }
    ///Error interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(ERRIE_A::NOTMASKED)
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
///RX buffer not empty interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIE_A {
    ///0: RXE interrupt masked
    MASKED = 0,
    ///1: RXE interrupt not masked
    NOTMASKED = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNEIE` reader - RX buffer not empty interrupt enable
pub struct RXNEIE_R(crate::FieldReader<bool, RXNEIE_A>);
impl RXNEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNEIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::MASKED,
            true => RXNEIE_A::NOTMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == RXNEIE_A::MASKED
    }
    ///Checks if the value of the field is `NOTMASKED`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        **self == RXNEIE_A::NOTMASKED
    }
}
impl core::ops::Deref for RXNEIE_R {
    type Target = crate::FieldReader<bool, RXNEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXNEIE` writer - RX buffer not empty interrupt enable
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXNEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///RXE interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::MASKED)
    }
    ///RXE interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::NOTMASKED)
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
///Tx buffer empty interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIE_A {
    ///0: TXE interrupt masked
    MASKED = 0,
    ///1: TXE interrupt not masked
    NOTMASKED = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXEIE` reader - Tx buffer empty interrupt enable
pub struct TXEIE_R(crate::FieldReader<bool, TXEIE_A>);
impl TXEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::MASKED,
            true => TXEIE_A::NOTMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == TXEIE_A::MASKED
    }
    ///Checks if the value of the field is `NOTMASKED`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        **self == TXEIE_A::NOTMASKED
    }
}
impl core::ops::Deref for TXEIE_R {
    type Target = crate::FieldReader<bool, TXEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXEIE` writer - Tx buffer empty interrupt enable
pub struct TXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///TXE interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXEIE_A::MASKED)
    }
    ///TXE interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXEIE_A::NOTMASKED)
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
///Data size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DS_A {
    ///3: 4-bit
    FOURBIT = 3,
    ///4: 5-bit
    FIVEBIT = 4,
    ///5: 6-bit
    SIXBIT = 5,
    ///6: 7-bit
    SEVENBIT = 6,
    ///7: 8-bit
    EIGHTBIT = 7,
    ///8: 9-bit
    NINEBIT = 8,
    ///9: 10-bit
    TENBIT = 9,
    ///10: 11-bit
    ELEVENBIT = 10,
    ///11: 12-bit
    TWELVEBIT = 11,
    ///12: 13-bit
    THIRTEENBIT = 12,
    ///13: 14-bit
    FOURTEENBIT = 13,
    ///14: 15-bit
    FIFTEENBIT = 14,
    ///15: 16-bit
    SIXTEENBIT = 15,
}
impl From<DS_A> for u8 {
    #[inline(always)]
    fn from(variant: DS_A) -> Self {
        variant as _
    }
}
///Field `DS` reader - Data size
pub struct DS_R(crate::FieldReader<u8, DS_A>);
impl DS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DS_A> {
        match self.bits {
            3 => Some(DS_A::FOURBIT),
            4 => Some(DS_A::FIVEBIT),
            5 => Some(DS_A::SIXBIT),
            6 => Some(DS_A::SEVENBIT),
            7 => Some(DS_A::EIGHTBIT),
            8 => Some(DS_A::NINEBIT),
            9 => Some(DS_A::TENBIT),
            10 => Some(DS_A::ELEVENBIT),
            11 => Some(DS_A::TWELVEBIT),
            12 => Some(DS_A::THIRTEENBIT),
            13 => Some(DS_A::FOURTEENBIT),
            14 => Some(DS_A::FIFTEENBIT),
            15 => Some(DS_A::SIXTEENBIT),
            _ => None,
        }
    }
    ///Checks if the value of the field is `FOURBIT`
    #[inline(always)]
    pub fn is_four_bit(&self) -> bool {
        **self == DS_A::FOURBIT
    }
    ///Checks if the value of the field is `FIVEBIT`
    #[inline(always)]
    pub fn is_five_bit(&self) -> bool {
        **self == DS_A::FIVEBIT
    }
    ///Checks if the value of the field is `SIXBIT`
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        **self == DS_A::SIXBIT
    }
    ///Checks if the value of the field is `SEVENBIT`
    #[inline(always)]
    pub fn is_seven_bit(&self) -> bool {
        **self == DS_A::SEVENBIT
    }
    ///Checks if the value of the field is `EIGHTBIT`
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        **self == DS_A::EIGHTBIT
    }
    ///Checks if the value of the field is `NINEBIT`
    #[inline(always)]
    pub fn is_nine_bit(&self) -> bool {
        **self == DS_A::NINEBIT
    }
    ///Checks if the value of the field is `TENBIT`
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        **self == DS_A::TENBIT
    }
    ///Checks if the value of the field is `ELEVENBIT`
    #[inline(always)]
    pub fn is_eleven_bit(&self) -> bool {
        **self == DS_A::ELEVENBIT
    }
    ///Checks if the value of the field is `TWELVEBIT`
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        **self == DS_A::TWELVEBIT
    }
    ///Checks if the value of the field is `THIRTEENBIT`
    #[inline(always)]
    pub fn is_thirteen_bit(&self) -> bool {
        **self == DS_A::THIRTEENBIT
    }
    ///Checks if the value of the field is `FOURTEENBIT`
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        **self == DS_A::FOURTEENBIT
    }
    ///Checks if the value of the field is `FIFTEENBIT`
    #[inline(always)]
    pub fn is_fifteen_bit(&self) -> bool {
        **self == DS_A::FIFTEENBIT
    }
    ///Checks if the value of the field is `SIXTEENBIT`
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        **self == DS_A::SIXTEENBIT
    }
}
impl core::ops::Deref for DS_R {
    type Target = crate::FieldReader<u8, DS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DS` writer - Data size
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///4-bit
    #[inline(always)]
    pub fn four_bit(self) -> &'a mut W {
        self.variant(DS_A::FOURBIT)
    }
    ///5-bit
    #[inline(always)]
    pub fn five_bit(self) -> &'a mut W {
        self.variant(DS_A::FIVEBIT)
    }
    ///6-bit
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(DS_A::SIXBIT)
    }
    ///7-bit
    #[inline(always)]
    pub fn seven_bit(self) -> &'a mut W {
        self.variant(DS_A::SEVENBIT)
    }
    ///8-bit
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(DS_A::EIGHTBIT)
    }
    ///9-bit
    #[inline(always)]
    pub fn nine_bit(self) -> &'a mut W {
        self.variant(DS_A::NINEBIT)
    }
    ///10-bit
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(DS_A::TENBIT)
    }
    ///11-bit
    #[inline(always)]
    pub fn eleven_bit(self) -> &'a mut W {
        self.variant(DS_A::ELEVENBIT)
    }
    ///12-bit
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(DS_A::TWELVEBIT)
    }
    ///13-bit
    #[inline(always)]
    pub fn thirteen_bit(self) -> &'a mut W {
        self.variant(DS_A::THIRTEENBIT)
    }
    ///14-bit
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut W {
        self.variant(DS_A::FOURTEENBIT)
    }
    ///15-bit
    #[inline(always)]
    pub fn fifteen_bit(self) -> &'a mut W {
        self.variant(DS_A::FIFTEENBIT)
    }
    ///16-bit
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DS_A::SIXTEENBIT)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///FIFO reception threshold
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRXTH_A {
    ///0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)
    HALF = 0,
    ///1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)
    QUARTER = 1,
}
impl From<FRXTH_A> for bool {
    #[inline(always)]
    fn from(variant: FRXTH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRXTH` reader - FIFO reception threshold
pub struct FRXTH_R(crate::FieldReader<bool, FRXTH_A>);
impl FRXTH_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRXTH_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FRXTH_A {
        match self.bits {
            false => FRXTH_A::HALF,
            true => FRXTH_A::QUARTER,
        }
    }
    ///Checks if the value of the field is `HALF`
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == FRXTH_A::HALF
    }
    ///Checks if the value of the field is `QUARTER`
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        **self == FRXTH_A::QUARTER
    }
}
impl core::ops::Deref for FRXTH_R {
    type Target = crate::FieldReader<bool, FRXTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FRXTH` writer - FIFO reception threshold
pub struct FRXTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FRXTH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FRXTH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FRXTH_A::HALF)
    }
    ///RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(FRXTH_A::QUARTER)
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
///Last DMA transfer for reception
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMA_RX_A {
    ///0: Number of data to transfer for receive is even
    EVEN = 0,
    ///1: Number of data to transfer for receive is odd
    ODD = 1,
}
impl From<LDMA_RX_A> for bool {
    #[inline(always)]
    fn from(variant: LDMA_RX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LDMA_RX` reader - Last DMA transfer for reception
pub struct LDMA_RX_R(crate::FieldReader<bool, LDMA_RX_A>);
impl LDMA_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDMA_RX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LDMA_RX_A {
        match self.bits {
            false => LDMA_RX_A::EVEN,
            true => LDMA_RX_A::ODD,
        }
    }
    ///Checks if the value of the field is `EVEN`
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == LDMA_RX_A::EVEN
    }
    ///Checks if the value of the field is `ODD`
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        **self == LDMA_RX_A::ODD
    }
}
impl core::ops::Deref for LDMA_RX_R {
    type Target = crate::FieldReader<bool, LDMA_RX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LDMA_RX` writer - Last DMA transfer for reception
pub struct LDMA_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LDMA_RX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LDMA_RX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Number of data to transfer for receive is even
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_RX_A::EVEN)
    }
    ///Number of data to transfer for receive is odd
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_RX_A::ODD)
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
///Last DMA transfer for transmission
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMA_TX_A {
    ///0: Number of data to transfer for transmit is even
    EVEN = 0,
    ///1: Number of data to transfer for transmit is odd
    ODD = 1,
}
impl From<LDMA_TX_A> for bool {
    #[inline(always)]
    fn from(variant: LDMA_TX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LDMA_TX` reader - Last DMA transfer for transmission
pub struct LDMA_TX_R(crate::FieldReader<bool, LDMA_TX_A>);
impl LDMA_TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDMA_TX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LDMA_TX_A {
        match self.bits {
            false => LDMA_TX_A::EVEN,
            true => LDMA_TX_A::ODD,
        }
    }
    ///Checks if the value of the field is `EVEN`
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == LDMA_TX_A::EVEN
    }
    ///Checks if the value of the field is `ODD`
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        **self == LDMA_TX_A::ODD
    }
}
impl core::ops::Deref for LDMA_TX_R {
    type Target = crate::FieldReader<bool, LDMA_TX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LDMA_TX` writer - Last DMA transfer for transmission
pub struct LDMA_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LDMA_TX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LDMA_TX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Number of data to transfer for transmit is even
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_TX_A::EVEN)
    }
    ///Number of data to transfer for transmit is odd
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_TX_A::ODD)
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
impl R {
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - NSS pulse management
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:11 - Data size
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - FIFO reception threshold
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Last DMA transfer for reception
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Last DMA transfer for transmission
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W {
        SSOE_W { w: self }
    }
    ///Bit 3 - NSS pulse management
    #[inline(always)]
    pub fn nssp(&mut self) -> NSSP_W {
        NSSP_W { w: self }
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    ///Bit 5 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W {
        TXEIE_W { w: self }
    }
    ///Bits 8:11 - Data size
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    ///Bit 12 - FIFO reception threshold
    #[inline(always)]
    pub fn frxth(&mut self) -> FRXTH_W {
        FRXTH_W { w: self }
    }
    ///Bit 13 - Last DMA transfer for reception
    #[inline(always)]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W {
        LDMA_RX_W { w: self }
    }
    ///Bit 14 - Last DMA transfer for transmission
    #[inline(always)]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W {
        LDMA_TX_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
