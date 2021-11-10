///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///End of operation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOP_A {
    ///0: No EOP operation occurred
    NOEVENT = 0,
    ///1: An EOP event occurred
    EVENT = 1,
}
impl From<EOP_A> for bool {
    #[inline(always)]
    fn from(variant: EOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOP` reader - End of operation
pub struct EOP_R(crate::FieldReader<bool, EOP_A>);
impl EOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOP_A {
        match self.bits {
            false => EOP_A::NOEVENT,
            true => EOP_A::EVENT,
        }
    }
    ///Checks if the value of the field is `NOEVENT`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EOP_A::NOEVENT
    }
    ///Checks if the value of the field is `EVENT`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == EOP_A::EVENT
    }
}
impl core::ops::Deref for EOP_R {
    type Target = crate::FieldReader<bool, EOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///End of operation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOP_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<EOP_AW> for bool {
    #[inline(always)]
    fn from(variant: EOP_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOP` writer - End of operation
pub struct EOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOP_AW::CLEAR)
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
///Operation error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPERR_A {
    ///0: No memory opreation error happened
    NOERROR = 0,
    ///1: Memory operation error happened
    ERROR = 1,
}
impl From<OPERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OPERR` reader - Operation error
pub struct OPERR_R(crate::FieldReader<bool, OPERR_A>);
impl OPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPERR_A {
        match self.bits {
            false => OPERR_A::NOERROR,
            true => OPERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == OPERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == OPERR_A::ERROR
    }
}
impl core::ops::Deref for OPERR_R {
    type Target = crate::FieldReader<bool, OPERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Operation error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPERR_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<OPERR_AW> for bool {
    #[inline(always)]
    fn from(variant: OPERR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPERR` writer - Operation error
pub struct OPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OPERR_AW::CLEAR)
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
///Programming error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGERR_A {
    ///0: No size programming error happened
    NOERROR = 0,
    ///1: Programming error happened
    ERROR = 1,
}
impl From<PROGERR_A> for bool {
    #[inline(always)]
    fn from(variant: PROGERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PROGERR` reader - Programming error
pub struct PROGERR_R(crate::FieldReader<bool, PROGERR_A>);
impl PROGERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROGERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PROGERR_A {
        match self.bits {
            false => PROGERR_A::NOERROR,
            true => PROGERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == PROGERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == PROGERR_A::ERROR
    }
}
impl core::ops::Deref for PROGERR_R {
    type Target = crate::FieldReader<bool, PROGERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Programming error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGERR_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<PROGERR_AW> for bool {
    #[inline(always)]
    fn from(variant: PROGERR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PROGERR` writer - Programming error
pub struct PROGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PROGERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PROGERR_AW::CLEAR)
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
///Write protected error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPERR_A {
    ///0: No write protection error happened
    NOERROR = 0,
    ///1: Write protection error happened
    ERROR = 1,
}
impl From<WRPERR_A> for bool {
    #[inline(always)]
    fn from(variant: WRPERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WRPERR` reader - Write protected error
pub struct WRPERR_R(crate::FieldReader<bool, WRPERR_A>);
impl WRPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRPERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WRPERR_A {
        match self.bits {
            false => WRPERR_A::NOERROR,
            true => WRPERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == WRPERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == WRPERR_A::ERROR
    }
}
impl core::ops::Deref for WRPERR_R {
    type Target = crate::FieldReader<bool, WRPERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Write protected error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPERR_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<WRPERR_AW> for bool {
    #[inline(always)]
    fn from(variant: WRPERR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `WRPERR` writer - Write protected error
pub struct WRPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WRPERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WRPERR_AW::CLEAR)
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
///Programming alignment error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGAERR_A {
    ///0: No programming alignment error happened
    NOERROR = 0,
    ///1: Programming alignment error happened
    ERROR = 1,
}
impl From<PGAERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGAERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PGAERR` reader - Programming alignment error
pub struct PGAERR_R(crate::FieldReader<bool, PGAERR_A>);
impl PGAERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGAERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PGAERR_A {
        match self.bits {
            false => PGAERR_A::NOERROR,
            true => PGAERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == PGAERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == PGAERR_A::ERROR
    }
}
impl core::ops::Deref for PGAERR_R {
    type Target = crate::FieldReader<bool, PGAERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Programming alignment error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGAERR_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<PGAERR_AW> for bool {
    #[inline(always)]
    fn from(variant: PGAERR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PGAERR` writer - Programming alignment error
pub struct PGAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGAERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PGAERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGAERR_AW::CLEAR)
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
///Size error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZERR_A {
    ///0: No size error happened
    NOERROR = 0,
    ///1: Size error happened
    ERROR = 1,
}
impl From<SIZERR_A> for bool {
    #[inline(always)]
    fn from(variant: SIZERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SIZERR` reader - Size error
pub struct SIZERR_R(crate::FieldReader<bool, SIZERR_A>);
impl SIZERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIZERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SIZERR_A {
        match self.bits {
            false => SIZERR_A::NOERROR,
            true => SIZERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == SIZERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == SIZERR_A::ERROR
    }
}
impl core::ops::Deref for SIZERR_R {
    type Target = crate::FieldReader<bool, SIZERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Size error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZERR_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<SIZERR_AW> for bool {
    #[inline(always)]
    fn from(variant: SIZERR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SIZERR` writer - Size error
pub struct SIZERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SIZERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SIZERR_AW::CLEAR)
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
///Programming sequence error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGSERR_A {
    ///0: No fast programming sequence error happened
    NOERROR = 0,
    ///1: Fast programming sequence error happened
    ERROR = 1,
}
impl From<PGSERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGSERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PGSERR` reader - Programming sequence error
pub struct PGSERR_R(crate::FieldReader<bool, PGSERR_A>);
impl PGSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGSERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PGSERR_A {
        match self.bits {
            false => PGSERR_A::NOERROR,
            true => PGSERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == PGSERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == PGSERR_A::ERROR
    }
}
impl core::ops::Deref for PGSERR_R {
    type Target = crate::FieldReader<bool, PGSERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Programming sequence error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGSERR_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<PGSERR_AW> for bool {
    #[inline(always)]
    fn from(variant: PGSERR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PGSERR` writer - Programming sequence error
pub struct PGSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGSERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PGSERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGSERR_AW::CLEAR)
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
///Fast programming data miss error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISSERR_A {
    ///0: No fast programming data miss error happened
    NOERROR = 0,
    ///1: Fast programming data miss error happened
    ERROR = 1,
}
impl From<MISSERR_A> for bool {
    #[inline(always)]
    fn from(variant: MISSERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MISSERR` reader - Fast programming data miss error
pub struct MISSERR_R(crate::FieldReader<bool, MISSERR_A>);
impl MISSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISSERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MISSERR_A {
        match self.bits {
            false => MISSERR_A::NOERROR,
            true => MISSERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == MISSERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == MISSERR_A::ERROR
    }
}
impl core::ops::Deref for MISSERR_R {
    type Target = crate::FieldReader<bool, MISSERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Fast programming data miss error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISSERR_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<MISSERR_AW> for bool {
    #[inline(always)]
    fn from(variant: MISSERR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `MISSERR` writer - Fast programming data miss error
pub struct MISSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MISSERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MISSERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MISSERR_AW::CLEAR)
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
///Fast programming error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FASTERR_A {
    ///0: No fast programming error happened
    NOERROR = 0,
    ///1: Fast programming error happened
    ERROR = 1,
}
impl From<FASTERR_A> for bool {
    #[inline(always)]
    fn from(variant: FASTERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FASTERR` reader - Fast programming error
pub struct FASTERR_R(crate::FieldReader<bool, FASTERR_A>);
impl FASTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FASTERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FASTERR_A {
        match self.bits {
            false => FASTERR_A::NOERROR,
            true => FASTERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == FASTERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == FASTERR_A::ERROR
    }
}
impl core::ops::Deref for FASTERR_R {
    type Target = crate::FieldReader<bool, FASTERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Fast programming error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FASTERR_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<FASTERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FASTERR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FASTERR` writer - Fast programming error
pub struct FASTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FASTERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FASTERR_AW::CLEAR)
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
///User Option OPTIVAL indication
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTNV_A {
    ///0: The OBL user option OPTVAL indicates "valid"
    VALID = 0,
    ///1: The OBL user option OPTVAL indicates "invalid"
    INVALID = 1,
}
impl From<OPTNV_A> for bool {
    #[inline(always)]
    fn from(variant: OPTNV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTNV` reader - User Option OPTIVAL indication
pub struct OPTNV_R(crate::FieldReader<bool, OPTNV_A>);
impl OPTNV_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTNV_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPTNV_A {
        match self.bits {
            false => OPTNV_A::VALID,
            true => OPTNV_A::INVALID,
        }
    }
    ///Checks if the value of the field is `VALID`
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == OPTNV_A::VALID
    }
    ///Checks if the value of the field is `INVALID`
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        **self == OPTNV_A::INVALID
    }
}
impl core::ops::Deref for OPTNV_R {
    type Target = crate::FieldReader<bool, OPTNV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///PCROP read error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDERR_A {
    ///0: No read-only error happened
    NOERROR = 0,
    ///1: Read-only error happened
    ERROR = 1,
}
impl From<RDERR_A> for bool {
    #[inline(always)]
    fn from(variant: RDERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RDERR` reader - PCROP read error
pub struct RDERR_R(crate::FieldReader<bool, RDERR_A>);
impl RDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RDERR_A {
        match self.bits {
            false => RDERR_A::NOERROR,
            true => RDERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == RDERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == RDERR_A::ERROR
    }
}
impl core::ops::Deref for RDERR_R {
    type Target = crate::FieldReader<bool, RDERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///PCROP read error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDERR_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<RDERR_AW> for bool {
    #[inline(always)]
    fn from(variant: RDERR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RDERR` writer - PCROP read error
pub struct RDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RDERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RDERR_AW::CLEAR)
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
///Option validity error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTVERR_A {
    ///0: No error in option and engineering bits
    NOERROR = 0,
    ///1: Error in option and engineering bits
    ERROR = 1,
}
impl From<OPTVERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTVERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTVERR` reader - Option validity error
pub struct OPTVERR_R(crate::FieldReader<bool, OPTVERR_A>);
impl OPTVERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTVERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPTVERR_A {
        match self.bits {
            false => OPTVERR_A::NOERROR,
            true => OPTVERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == OPTVERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == OPTVERR_A::ERROR
    }
}
impl core::ops::Deref for OPTVERR_R {
    type Target = crate::FieldReader<bool, OPTVERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Option validity error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTVERR_AW {
    ///1: Clear the flag
    CLEAR = 1,
}
impl From<OPTVERR_AW> for bool {
    #[inline(always)]
    fn from(variant: OPTVERR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTVERR` writer - Option validity error
pub struct OPTVERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTVERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPTVERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OPTVERR_AW::CLEAR)
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
///Busy
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSY_A {
    ///0: No write/erase operation is in progress
    INACTIVE = 0,
    ///1: No write/erase operation is in progress
    ACTIVE = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSY` reader - Busy
pub struct BSY_R(crate::FieldReader<bool, BSY_A>);
impl BSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::INACTIVE,
            true => BSY_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `INACTIVE`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == BSY_A::INACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == BSY_A::ACTIVE
    }
}
impl core::ops::Deref for BSY_R {
    type Target = crate::FieldReader<bool, BSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Programming or erase configuration busy
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGBSY_A {
    ///0: PG, PNB, PER, MER bits available for writing
    FREE = 0,
    ///1: PG, PNB, PER, MER bits not available for writing (operation ongoing)
    BUSY = 1,
}
impl From<CFGBSY_A> for bool {
    #[inline(always)]
    fn from(variant: CFGBSY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CFGBSY` reader - Programming or erase configuration busy
pub struct CFGBSY_R(crate::FieldReader<bool, CFGBSY_A>);
impl CFGBSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGBSY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CFGBSY_A {
        match self.bits {
            false => CFGBSY_A::FREE,
            true => CFGBSY_A::BUSY,
        }
    }
    ///Checks if the value of the field is `FREE`
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        **self == CFGBSY_A::FREE
    }
    ///Checks if the value of the field is `BUSY`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == CFGBSY_A::BUSY
    }
}
impl core::ops::Deref for CFGBSY_R {
    type Target = crate::FieldReader<bool, CFGBSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Programming / erase operation suspended
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESD_A {
    ///0: Flash program and erase operations granted
    GRANTED = 0,
    ///1: Any new Flash program and erase operation is suspended until this bit is cleared. This bit is set when the PES bit in FLASH_ACR is set
    SUSPENDED = 1,
}
impl From<PESD_A> for bool {
    #[inline(always)]
    fn from(variant: PESD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PESD` reader - Programming / erase operation suspended
pub struct PESD_R(crate::FieldReader<bool, PESD_A>);
impl PESD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PESD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PESD_A {
        match self.bits {
            false => PESD_A::GRANTED,
            true => PESD_A::SUSPENDED,
        }
    }
    ///Checks if the value of the field is `GRANTED`
    #[inline(always)]
    pub fn is_granted(&self) -> bool {
        **self == PESD_A::GRANTED
    }
    ///Checks if the value of the field is `SUSPENDED`
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        **self == PESD_A::SUSPENDED
    }
}
impl core::ops::Deref for PESD_R {
    type Target = crate::FieldReader<bool, PESD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Write protected error
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn misserr(&self) -> MISSERR_R {
        MISSERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 13 - User Option OPTIVAL indication
    #[inline(always)]
    pub fn optnv(&self) -> OPTNV_R {
        OPTNV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Option validity error
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 18 - Programming or erase configuration busy
    #[inline(always)]
    pub fn cfgbsy(&self) -> CFGBSY_R {
        CFGBSY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Programming / erase operation suspended
    #[inline(always)]
    pub fn pesd(&self) -> PESD_R {
        PESD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W {
        EOP_W { w: self }
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W {
        OPERR_W { w: self }
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W {
        PROGERR_W { w: self }
    }
    ///Bit 4 - Write protected error
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W {
        WRPERR_W { w: self }
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W {
        PGAERR_W { w: self }
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W {
        SIZERR_W { w: self }
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W {
        PGSERR_W { w: self }
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn misserr(&mut self) -> MISSERR_W {
        MISSERR_W { w: self }
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&mut self) -> FASTERR_W {
        FASTERR_W { w: self }
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&mut self) -> RDERR_W {
        RDERR_W { w: self }
    }
    ///Bit 15 - Option validity error
    #[inline(always)]
    pub fn optverr(&mut self) -> OPTVERR_W {
        OPTVERR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
