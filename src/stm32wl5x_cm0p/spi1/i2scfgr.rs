///Register `I2SCFGR` reader
pub struct R(crate::R<I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2SCFGR` writer
pub struct W(crate::W<I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCFGR_SPEC>;
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
impl From<crate::W<I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///CHLEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHLEN_A {
    ///0: 16-bit wide
    SIXTEENBIT = 0,
    ///1: 32-bit wide
    THIRTYTWOBIT = 1,
}
impl From<CHLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHLEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CHLEN` reader - CHLEN
pub struct CHLEN_R(crate::FieldReader<bool, CHLEN_A>);
impl CHLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHLEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CHLEN_A {
        match self.bits {
            false => CHLEN_A::SIXTEENBIT,
            true => CHLEN_A::THIRTYTWOBIT,
        }
    }
    ///Checks if the value of the field is `SIXTEENBIT`
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        **self == CHLEN_A::SIXTEENBIT
    }
    ///Checks if the value of the field is `THIRTYTWOBIT`
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        **self == CHLEN_A::THIRTYTWOBIT
    }
}
impl core::ops::Deref for CHLEN_R {
    type Target = crate::FieldReader<bool, CHLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CHLEN` writer - CHLEN
pub struct CHLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHLEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CHLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///16-bit wide
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(CHLEN_A::SIXTEENBIT)
    }
    ///32-bit wide
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(CHLEN_A::THIRTYTWOBIT)
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
///DATLEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATLEN_A {
    ///0: 16-bit data length
    SIXTEENBIT = 0,
    ///1: 24-bit data length
    TWENTYFOURBIT = 1,
    ///2: 32-bit data length
    THIRTYTWOBIT = 2,
}
impl From<DATLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN_A) -> Self {
        variant as _
    }
}
///Field `DATLEN` reader - DATLEN
pub struct DATLEN_R(crate::FieldReader<u8, DATLEN_A>);
impl DATLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATLEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DATLEN_A> {
        match self.bits {
            0 => Some(DATLEN_A::SIXTEENBIT),
            1 => Some(DATLEN_A::TWENTYFOURBIT),
            2 => Some(DATLEN_A::THIRTYTWOBIT),
            _ => None,
        }
    }
    ///Checks if the value of the field is `SIXTEENBIT`
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        **self == DATLEN_A::SIXTEENBIT
    }
    ///Checks if the value of the field is `TWENTYFOURBIT`
    #[inline(always)]
    pub fn is_twenty_four_bit(&self) -> bool {
        **self == DATLEN_A::TWENTYFOURBIT
    }
    ///Checks if the value of the field is `THIRTYTWOBIT`
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        **self == DATLEN_A::THIRTYTWOBIT
    }
}
impl core::ops::Deref for DATLEN_R {
    type Target = crate::FieldReader<u8, DATLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DATLEN` writer - DATLEN
pub struct DATLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DATLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///16-bit data length
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::SIXTEENBIT)
    }
    ///24-bit data length
    #[inline(always)]
    pub fn twenty_four_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::TWENTYFOURBIT)
    }
    ///32-bit data length
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::THIRTYTWOBIT)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
///CKPOL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKPOL_A {
    ///0: I2S clock inactive state is low level
    IDLELOW = 0,
    ///1: I2S clock inactive state is high level
    IDLEHIGH = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CKPOL` reader - CKPOL
pub struct CKPOL_R(crate::FieldReader<bool, CKPOL_A>);
impl CKPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKPOL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::IDLELOW,
            true => CKPOL_A::IDLEHIGH,
        }
    }
    ///Checks if the value of the field is `IDLELOW`
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        **self == CKPOL_A::IDLELOW
    }
    ///Checks if the value of the field is `IDLEHIGH`
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        **self == CKPOL_A::IDLEHIGH
    }
}
impl core::ops::Deref for CKPOL_R {
    type Target = crate::FieldReader<bool, CKPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CKPOL` writer - CKPOL
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CKPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///I2S clock inactive state is low level
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CKPOL_A::IDLELOW)
    }
    ///I2S clock inactive state is high level
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CKPOL_A::IDLEHIGH)
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
///I2SSTD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2SSTD_A {
    ///0: I2S Philips standard
    PHILIPS = 0,
    ///1: MSB justified standard
    MSB = 1,
    ///2: LSB justified standard
    LSB = 2,
    ///3: PCM standard
    PCM = 3,
}
impl From<I2SSTD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD_A) -> Self {
        variant as _
    }
}
///Field `I2SSTD` reader - I2SSTD
pub struct I2SSTD_R(crate::FieldReader<u8, I2SSTD_A>);
impl I2SSTD_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SSTD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SSTD_A {
        match self.bits {
            0 => I2SSTD_A::PHILIPS,
            1 => I2SSTD_A::MSB,
            2 => I2SSTD_A::LSB,
            3 => I2SSTD_A::PCM,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PHILIPS`
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        **self == I2SSTD_A::PHILIPS
    }
    ///Checks if the value of the field is `MSB`
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        **self == I2SSTD_A::MSB
    }
    ///Checks if the value of the field is `LSB`
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        **self == I2SSTD_A::LSB
    }
    ///Checks if the value of the field is `PCM`
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        **self == I2SSTD_A::PCM
    }
}
impl core::ops::Deref for I2SSTD_R {
    type Target = crate::FieldReader<u8, I2SSTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SSTD` writer - I2SSTD
pub struct I2SSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SSTD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2SSTD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///I2S Philips standard
    #[inline(always)]
    pub fn philips(self) -> &'a mut W {
        self.variant(I2SSTD_A::PHILIPS)
    }
    ///MSB justified standard
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(I2SSTD_A::MSB)
    }
    ///LSB justified standard
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(I2SSTD_A::LSB)
    }
    ///PCM standard
    #[inline(always)]
    pub fn pcm(self) -> &'a mut W {
        self.variant(I2SSTD_A::PCM)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
///PCMSYNC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCMSYNC_A {
    ///0: Short frame synchronisation
    SHORT = 0,
    ///1: Long frame synchronisation
    LONG = 1,
}
impl From<PCMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PCMSYNC` reader - PCMSYNC
pub struct PCMSYNC_R(crate::FieldReader<bool, PCMSYNC_A>);
impl PCMSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCMSYNC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCMSYNC_A {
        match self.bits {
            false => PCMSYNC_A::SHORT,
            true => PCMSYNC_A::LONG,
        }
    }
    ///Checks if the value of the field is `SHORT`
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        **self == PCMSYNC_A::SHORT
    }
    ///Checks if the value of the field is `LONG`
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        **self == PCMSYNC_A::LONG
    }
}
impl core::ops::Deref for PCMSYNC_R {
    type Target = crate::FieldReader<bool, PCMSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PCMSYNC` writer - PCMSYNC
pub struct PCMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMSYNC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PCMSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Short frame synchronisation
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(PCMSYNC_A::SHORT)
    }
    ///Long frame synchronisation
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(PCMSYNC_A::LONG)
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
///I2SCFG
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2SCFG_A {
    ///0: Slave - transmit
    SLAVETX = 0,
    ///1: Slave - receive
    SLAVERX = 1,
    ///2: Master - transmit
    MASTERTX = 2,
    ///3: Master - receive
    MASTERRX = 3,
}
impl From<I2SCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SCFG_A) -> Self {
        variant as _
    }
}
///Field `I2SCFG` reader - I2SCFG
pub struct I2SCFG_R(crate::FieldReader<u8, I2SCFG_A>);
impl I2SCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SCFG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SCFG_A {
        match self.bits {
            0 => I2SCFG_A::SLAVETX,
            1 => I2SCFG_A::SLAVERX,
            2 => I2SCFG_A::MASTERTX,
            3 => I2SCFG_A::MASTERRX,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `SLAVETX`
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        **self == I2SCFG_A::SLAVETX
    }
    ///Checks if the value of the field is `SLAVERX`
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        **self == I2SCFG_A::SLAVERX
    }
    ///Checks if the value of the field is `MASTERTX`
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        **self == I2SCFG_A::MASTERTX
    }
    ///Checks if the value of the field is `MASTERRX`
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        **self == I2SCFG_A::MASTERRX
    }
}
impl core::ops::Deref for I2SCFG_R {
    type Target = crate::FieldReader<u8, I2SCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SCFG` writer - I2SCFG
pub struct I2SCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SCFG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2SCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Slave - transmit
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut W {
        self.variant(I2SCFG_A::SLAVETX)
    }
    ///Slave - receive
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut W {
        self.variant(I2SCFG_A::SLAVERX)
    }
    ///Master - transmit
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut W {
        self.variant(I2SCFG_A::MASTERTX)
    }
    ///Master - receive
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut W {
        self.variant(I2SCFG_A::MASTERRX)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///I2SE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SE_A {
    ///0: I2S peripheral is disabled
    DISABLED = 0,
    ///1: I2S peripheral is enabled
    ENABLED = 1,
}
impl From<I2SE_A> for bool {
    #[inline(always)]
    fn from(variant: I2SE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `I2SE` reader - I2SE
pub struct I2SE_R(crate::FieldReader<bool, I2SE_A>);
impl I2SE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SE_A {
        match self.bits {
            false => I2SE_A::DISABLED,
            true => I2SE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == I2SE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == I2SE_A::ENABLED
    }
}
impl core::ops::Deref for I2SE_R {
    type Target = crate::FieldReader<bool, I2SE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SE` writer - I2SE
pub struct I2SE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2SE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///I2S peripheral is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2SE_A::DISABLED)
    }
    ///I2S peripheral is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2SE_A::ENABLED)
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
///I2SMOD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SMOD_A {
    ///0: SPI mode is selected
    SPIMODE = 0,
    ///1: I2S mode is selected
    I2SMODE = 1,
}
impl From<I2SMOD_A> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `I2SMOD` reader - I2SMOD
pub struct I2SMOD_R(crate::FieldReader<bool, I2SMOD_A>);
impl I2SMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SMOD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SMOD_A {
        match self.bits {
            false => I2SMOD_A::SPIMODE,
            true => I2SMOD_A::I2SMODE,
        }
    }
    ///Checks if the value of the field is `SPIMODE`
    #[inline(always)]
    pub fn is_spimode(&self) -> bool {
        **self == I2SMOD_A::SPIMODE
    }
    ///Checks if the value of the field is `I2SMODE`
    #[inline(always)]
    pub fn is_i2smode(&self) -> bool {
        **self == I2SMOD_A::I2SMODE
    }
}
impl core::ops::Deref for I2SMOD_R {
    type Target = crate::FieldReader<bool, I2SMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SMOD` writer - I2SMOD
pub struct I2SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SMOD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2SMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SPI mode is selected
    #[inline(always)]
    pub fn spimode(self) -> &'a mut W {
        self.variant(I2SMOD_A::SPIMODE)
    }
    ///I2S mode is selected
    #[inline(always)]
    pub fn i2smode(self) -> &'a mut W {
        self.variant(I2SMOD_A::I2SMODE)
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
///Field `ASTRTEN` reader - ASTRTEN
pub struct ASTRTEN_R(crate::FieldReader<bool, bool>);
impl ASTRTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASTRTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASTRTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ASTRTEN` writer - ASTRTEN
pub struct ASTRTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASTRTEN_W<'a> {
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
impl R {
    ///Bit 0 - CHLEN
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - ASTRTEN
    #[inline(always)]
    pub fn astrten(&self) -> ASTRTEN_R {
        ASTRTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - CHLEN
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W {
        CHLEN_W { w: self }
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W {
        DATLEN_W { w: self }
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W {
        I2SSTD_W { w: self }
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W {
        PCMSYNC_W { w: self }
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W {
        I2SCFG_W { w: self }
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    pub fn i2se(&mut self) -> I2SE_W {
        I2SE_W { w: self }
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W {
        I2SMOD_W { w: self }
    }
    ///Bit 12 - ASTRTEN
    #[inline(always)]
    pub fn astrten(&mut self) -> ASTRTEN_W {
        ASTRTEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2scfgr](index.html) module
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2scfgr::R](R) reader structure
impl crate::Readable for I2SCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2scfgr::W](W) writer structure
impl crate::Writable for I2SCFGR_SPEC {
    type Writer = W;
}
///`reset()` method sets I2SCFGR to value 0
impl crate::Resettable for I2SCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
