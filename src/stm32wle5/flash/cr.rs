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
///Programming
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PG_A {
    ///0: Flash programming disabled
    DISABLED = 0,
    ///1: Flash programming enabled
    ENABLED = 1,
}
impl From<PG_A> for bool {
    #[inline(always)]
    fn from(variant: PG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PG` reader - Programming
pub struct PG_R(crate::FieldReader<bool, PG_A>);
impl PG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PG_A {
        match self.bits {
            false => PG_A::DISABLED,
            true => PG_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PG_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PG_A::ENABLED
    }
}
impl core::ops::Deref for PG_R {
    type Target = crate::FieldReader<bool, PG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PG` writer - Programming
pub struct PG_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Flash programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PG_A::DISABLED)
    }
    ///Flash programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PG_A::ENABLED)
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
///Page erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER_A {
    ///0: Page erase disabled
    DISABLED = 0,
    ///1: Page erase enabled
    ENABLED = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PER` reader - Page erase
pub struct PER_R(crate::FieldReader<bool, PER_A>);
impl PER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PER_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::DISABLED,
            true => PER_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PER_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PER_A::ENABLED
    }
}
impl core::ops::Deref for PER_R {
    type Target = crate::FieldReader<bool, PER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PER` writer - Page erase
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Page erase disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PER_A::DISABLED)
    }
    ///Page erase enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PER_A::ENABLED)
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
///Mass erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MER_A {
    ///0: No mass erase
    NOERASE = 0,
    ///1: Trigger mass erase
    MASSERASE = 1,
}
impl From<MER_A> for bool {
    #[inline(always)]
    fn from(variant: MER_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MER` reader - Mass erase
pub struct MER_R(crate::FieldReader<bool, MER_A>);
impl MER_R {
    pub(crate) fn new(bits: bool) -> Self {
        MER_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MER_A {
        match self.bits {
            false => MER_A::NOERASE,
            true => MER_A::MASSERASE,
        }
    }
    ///Checks if the value of the field is `NOERASE`
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        **self == MER_A::NOERASE
    }
    ///Checks if the value of the field is `MASSERASE`
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        **self == MER_A::MASSERASE
    }
}
impl core::ops::Deref for MER_R {
    type Target = crate::FieldReader<bool, MER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MER` writer - Mass erase
pub struct MER_W<'a> {
    w: &'a mut W,
}
impl<'a> MER_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No mass erase
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut W {
        self.variant(MER_A::NOERASE)
    }
    ///Trigger mass erase
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut W {
        self.variant(MER_A::MASSERASE)
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
///Field `PNB` reader - Page number
pub struct PNB_R(crate::FieldReader<u8, u8>);
impl PNB_R {
    pub(crate) fn new(bits: u8) -> Self {
        PNB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PNB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PNB` writer - Page number
pub struct PNB_W<'a> {
    w: &'a mut W,
}
impl<'a> PNB_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 3)) | ((value as u32 & 0x7f) << 3);
        self.w
    }
}
///Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT_A {
    ///0: Options modification completed or idle
    DONE = 0,
}
impl From<STRT_A> for bool {
    #[inline(always)]
    fn from(variant: STRT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT` reader - Start
pub struct STRT_R(crate::FieldReader<bool, STRT_A>);
impl STRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        STRT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<STRT_A> {
        match self.bits {
            false => Some(STRT_A::DONE),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DONE`
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        **self == STRT_A::DONE
    }
}
impl core::ops::Deref for STRT_R {
    type Target = crate::FieldReader<bool, STRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT_AW {
    ///1: Trigger options programming operation
    START = 1,
}
impl From<STRT_AW> for bool {
    #[inline(always)]
    fn from(variant: STRT_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT` writer - Start
pub struct STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STRT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STRT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Trigger options programming operation
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STRT_AW::START)
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
///Options modification start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTSTRT_A {
    ///0: Options modification completed or idle
    DONE = 0,
}
impl From<OPTSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTSTRT` reader - Options modification start
pub struct OPTSTRT_R(crate::FieldReader<bool, OPTSTRT_A>);
impl OPTSTRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTSTRT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<OPTSTRT_A> {
        match self.bits {
            false => Some(OPTSTRT_A::DONE),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DONE`
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        **self == OPTSTRT_A::DONE
    }
}
impl core::ops::Deref for OPTSTRT_R {
    type Target = crate::FieldReader<bool, OPTSTRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Options modification start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTSTRT_AW {
    ///1: Trigger options programming operation
    START = 1,
}
impl From<OPTSTRT_AW> for bool {
    #[inline(always)]
    fn from(variant: OPTSTRT_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTSTRT` writer - Options modification start
pub struct OPTSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTSTRT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPTSTRT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Trigger options programming operation
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(OPTSTRT_AW::START)
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
///Fast programming
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSTPG_A {
    ///0: Fast programming disabled
    DISABLED = 0,
    ///1: Fast programming enabled
    ENABLED = 1,
}
impl From<FSTPG_A> for bool {
    #[inline(always)]
    fn from(variant: FSTPG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FSTPG` reader - Fast programming
pub struct FSTPG_R(crate::FieldReader<bool, FSTPG_A>);
impl FSTPG_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSTPG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FSTPG_A {
        match self.bits {
            false => FSTPG_A::DISABLED,
            true => FSTPG_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FSTPG_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FSTPG_A::ENABLED
    }
}
impl core::ops::Deref for FSTPG_R {
    type Target = crate::FieldReader<bool, FSTPG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FSTPG` writer - Fast programming
pub struct FSTPG_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTPG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FSTPG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Fast programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FSTPG_A::DISABLED)
    }
    ///Fast programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FSTPG_A::ENABLED)
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
///End of operation interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOPIE_A {
    ///0: End of program interrupt disable
    DISABLED = 0,
    ///1: End of program interrupt enable
    ENABLED = 1,
}
impl From<EOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOPIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOPIE` reader - End of operation interrupt enable
pub struct EOPIE_R(crate::FieldReader<bool, EOPIE_A>);
impl EOPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOPIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOPIE_A {
        match self.bits {
            false => EOPIE_A::DISABLED,
            true => EOPIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EOPIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EOPIE_A::ENABLED
    }
}
impl core::ops::Deref for EOPIE_R {
    type Target = crate::FieldReader<bool, EOPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EOPIE` writer - End of operation interrupt enable
pub struct EOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///End of program interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOPIE_A::DISABLED)
    }
    ///End of program interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOPIE_A::ENABLED)
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
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    ///0: OPERR Error interrupt disable
    DISABLED = 0,
    ///1: OPERR Error interrupt enable
    ENABLED = 1,
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
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ERRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ERRIE_A::ENABLED
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
    ///OPERR Error interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    ///OPERR Error interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
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
///PCROP read error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDERRIE_A {
    ///0: PCROP read error interrupt disable
    DISABLED = 0,
    ///1: PCROP read error interrupt enable
    ENABLED = 1,
}
impl From<RDERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDERRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RDERRIE` reader - PCROP read error interrupt enable
pub struct RDERRIE_R(crate::FieldReader<bool, RDERRIE_A>);
impl RDERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDERRIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RDERRIE_A {
        match self.bits {
            false => RDERRIE_A::DISABLED,
            true => RDERRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RDERRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RDERRIE_A::ENABLED
    }
}
impl core::ops::Deref for RDERRIE_R {
    type Target = crate::FieldReader<bool, RDERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RDERRIE` writer - PCROP read error interrupt enable
pub struct RDERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDERRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RDERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///PCROP read error interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDERRIE_A::DISABLED)
    }
    ///PCROP read error interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDERRIE_A::ENABLED)
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
///Force the option byte loading
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OBL_LAUNCH_A {
    ///0: Option byte loaded
    COMPLETE = 0,
    ///1: Option byte loading to be done
    NOTCOMPLETE = 1,
}
impl From<OBL_LAUNCH_A> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OBL_LAUNCH` reader - Force the option byte loading
pub struct OBL_LAUNCH_R(crate::FieldReader<bool, OBL_LAUNCH_A>);
impl OBL_LAUNCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBL_LAUNCH_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OBL_LAUNCH_A {
        match self.bits {
            false => OBL_LAUNCH_A::COMPLETE,
            true => OBL_LAUNCH_A::NOTCOMPLETE,
        }
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == OBL_LAUNCH_A::COMPLETE
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == OBL_LAUNCH_A::NOTCOMPLETE
    }
}
impl core::ops::Deref for OBL_LAUNCH_R {
    type Target = crate::FieldReader<bool, OBL_LAUNCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Force the option byte loading
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OBL_LAUNCH_AW {
    ///1: Reload option byte
    RELOAD = 1,
}
impl From<OBL_LAUNCH_AW> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCH_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OBL_LAUNCH` writer - Force the option byte loading
pub struct OBL_LAUNCH_W<'a> {
    w: &'a mut W,
}
impl<'a> OBL_LAUNCH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OBL_LAUNCH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Reload option byte
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(OBL_LAUNCH_AW::RELOAD)
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
///Options Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTLOCK_A {
    ///0: FLASH_CR options are unlocked
    UNLOCKED = 0,
}
impl From<OPTLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTLOCK` reader - Options Lock
pub struct OPTLOCK_R(crate::FieldReader<bool, OPTLOCK_A>);
impl OPTLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTLOCK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<OPTLOCK_A> {
        match self.bits {
            false => Some(OPTLOCK_A::UNLOCKED),
            _ => None,
        }
    }
    ///Checks if the value of the field is `UNLOCKED`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == OPTLOCK_A::UNLOCKED
    }
}
impl core::ops::Deref for OPTLOCK_R {
    type Target = crate::FieldReader<bool, OPTLOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Options Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTLOCK_AW {
    ///1: FLASH_CR options are locked
    LOCKED = 1,
}
impl From<OPTLOCK_AW> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCK_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTLOCK` writer - Options Lock
pub struct OPTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTLOCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPTLOCK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///FLASH_CR options are locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(OPTLOCK_AW::LOCKED)
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
///FLASH_CR Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    ///0: FLASH_CR is unlocked
    UNLOCKED = 0,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` reader - FLASH_CR Lock
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_A> {
        match self.bits {
            false => Some(LOCK_A::UNLOCKED),
            _ => None,
        }
    }
    ///Checks if the value of the field is `UNLOCKED`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == LOCK_A::UNLOCKED
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///FLASH_CR Lock
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_AW {
    ///1: FLASH_CR is locked
    LOCKED = 1,
}
impl From<LOCK_AW> for bool {
    #[inline(always)]
    fn from(variant: LOCK_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` writer - FLASH_CR Lock
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LOCK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///FLASH_CR is locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_AW::LOCKED)
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
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Mass erase
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bits 3:9 - Page number
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - FLASH_CR Lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W { w: self }
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
    ///Bit 2 - Mass erase
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W {
        MER_W { w: self }
    }
    ///Bits 3:9 - Page number
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W {
        PNB_W { w: self }
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W {
        STRT_W { w: self }
    }
    ///Bit 17 - Options modification start
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W {
        OPTSTRT_W { w: self }
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&mut self) -> FSTPG_W {
        FSTPG_W { w: self }
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W {
        EOPIE_W { w: self }
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    ///Bit 26 - PCROP read error interrupt enable
    #[inline(always)]
    pub fn rderrie(&mut self) -> RDERRIE_W {
        RDERRIE_W { w: self }
    }
    ///Bit 27 - Force the option byte loading
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W {
        OBL_LAUNCH_W { w: self }
    }
    ///Bit 30 - Options Lock
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W {
        OPTLOCK_W { w: self }
    }
    ///Bit 31 - FLASH_CR Lock
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash control register
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
///`reset()` method sets CR to value 0xc000_0000
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}