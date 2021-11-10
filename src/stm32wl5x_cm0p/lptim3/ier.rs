///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Repetition register update OK interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPOKIE_A {
    ///0: Repetition register update OK interrupt disabled
    DISABLED = 0,
    ///1: Repetition register update OK interrupt enabled
    ENABLED = 1,
}
impl From<REPOKIE_A> for bool {
    #[inline(always)]
    fn from(variant: REPOKIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `REPOKIE` reader - Repetition register update OK interrupt Enable
pub struct REPOKIE_R(crate::FieldReader<bool, REPOKIE_A>);
impl REPOKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REPOKIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REPOKIE_A {
        match self.bits {
            false => REPOKIE_A::DISABLED,
            true => REPOKIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REPOKIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REPOKIE_A::ENABLED
    }
}
impl core::ops::Deref for REPOKIE_R {
    type Target = crate::FieldReader<bool, REPOKIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REPOKIE` writer - Repetition register update OK interrupt Enable
pub struct REPOKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REPOKIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: REPOKIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Repetition register update OK interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPOKIE_A::DISABLED)
    }
    ///Repetition register update OK interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPOKIE_A::ENABLED)
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
///Update event interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UEIE_A {
    ///0: Update event interrupt disabled
    DISABLED = 0,
    ///1: Update event interrupt enabled
    ENABLED = 1,
}
impl From<UEIE_A> for bool {
    #[inline(always)]
    fn from(variant: UEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UEIE` reader - Update event interrupt enable
pub struct UEIE_R(crate::FieldReader<bool, UEIE_A>);
impl UEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UEIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UEIE_A {
        match self.bits {
            false => UEIE_A::DISABLED,
            true => UEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == UEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == UEIE_A::ENABLED
    }
}
impl core::ops::Deref for UEIE_R {
    type Target = crate::FieldReader<bool, UEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UEIE` writer - Update event interrupt enable
pub struct UEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Update event interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UEIE_A::DISABLED)
    }
    ///Update event interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UEIE_A::ENABLED)
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
///Direction change to down Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWNIE_A {
    ///0: DOWN interrupt disabled
    DISABLED = 0,
    ///1: DOWN interrupt enabled
    ENABLED = 1,
}
impl From<DOWNIE_A> for bool {
    #[inline(always)]
    fn from(variant: DOWNIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOWNIE` reader - Direction change to down Interrupt Enable
pub struct DOWNIE_R(crate::FieldReader<bool, DOWNIE_A>);
impl DOWNIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOWNIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DOWNIE_A {
        match self.bits {
            false => DOWNIE_A::DISABLED,
            true => DOWNIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DOWNIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DOWNIE_A::ENABLED
    }
}
impl core::ops::Deref for DOWNIE_R {
    type Target = crate::FieldReader<bool, DOWNIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DOWNIE` writer - Direction change to down Interrupt Enable
pub struct DOWNIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWNIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DOWNIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DOWN interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DOWNIE_A::DISABLED)
    }
    ///DOWN interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DOWNIE_A::ENABLED)
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
///Direction change to UP Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPIE_A {
    ///0: UP interrupt disabled
    DISABLED = 0,
    ///1: UP interrupt enabled
    ENABLED = 1,
}
impl From<UPIE_A> for bool {
    #[inline(always)]
    fn from(variant: UPIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UPIE` reader - Direction change to UP Interrupt Enable
pub struct UPIE_R(crate::FieldReader<bool, UPIE_A>);
impl UPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UPIE_A {
        match self.bits {
            false => UPIE_A::DISABLED,
            true => UPIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == UPIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == UPIE_A::ENABLED
    }
}
impl core::ops::Deref for UPIE_R {
    type Target = crate::FieldReader<bool, UPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UPIE` writer - Direction change to UP Interrupt Enable
pub struct UPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///UP interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPIE_A::DISABLED)
    }
    ///UP interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPIE_A::ENABLED)
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
///Autoreload register update OK Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARROKIE_A {
    ///0: ARROK interrupt disabled
    DISABLED = 0,
    ///1: ARROK interrupt enabled
    ENABLED = 1,
}
impl From<ARROKIE_A> for bool {
    #[inline(always)]
    fn from(variant: ARROKIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable
pub struct ARROKIE_R(crate::FieldReader<bool, ARROKIE_A>);
impl ARROKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARROKIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARROKIE_A {
        match self.bits {
            false => ARROKIE_A::DISABLED,
            true => ARROKIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ARROKIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ARROKIE_A::ENABLED
    }
}
impl core::ops::Deref for ARROKIE_R {
    type Target = crate::FieldReader<bool, ARROKIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable
pub struct ARROKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARROKIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ARROKIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///ARROK interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARROKIE_A::DISABLED)
    }
    ///ARROK interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARROKIE_A::ENABLED)
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
///Compare register update OK Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPOKIE_A {
    ///0: CMPOK interrupt disabled
    DISABLED = 0,
    ///1: CMPOK interrupt enabled
    ENABLED = 1,
}
impl From<CMPOKIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOKIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPOKIE` reader - Compare register update OK Interrupt Enable
pub struct CMPOKIE_R(crate::FieldReader<bool, CMPOKIE_A>);
impl CMPOKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPOKIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMPOKIE_A {
        match self.bits {
            false => CMPOKIE_A::DISABLED,
            true => CMPOKIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMPOKIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMPOKIE_A::ENABLED
    }
}
impl core::ops::Deref for CMPOKIE_R {
    type Target = crate::FieldReader<bool, CMPOKIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CMPOKIE` writer - Compare register update OK Interrupt Enable
pub struct CMPOKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPOKIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CMPOKIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CMPOK interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMPOKIE_A::DISABLED)
    }
    ///CMPOK interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMPOKIE_A::ENABLED)
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
///External trigger valid edge Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTTRIGIE_A {
    ///0: EXTTRIG interrupt disabled
    DISABLED = 0,
    ///1: EXTTRIG interrupt enabled
    ENABLED = 1,
}
impl From<EXTTRIGIE_A> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable
pub struct EXTTRIGIE_R(crate::FieldReader<bool, EXTTRIGIE_A>);
impl EXTTRIGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTTRIGIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTTRIGIE_A {
        match self.bits {
            false => EXTTRIGIE_A::DISABLED,
            true => EXTTRIGIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EXTTRIGIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EXTTRIGIE_A::ENABLED
    }
}
impl core::ops::Deref for EXTTRIGIE_R {
    type Target = crate::FieldReader<bool, EXTTRIGIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable
pub struct EXTTRIGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTTRIGIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTTRIGIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///EXTTRIG interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTTRIGIE_A::DISABLED)
    }
    ///EXTTRIG interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTTRIGIE_A::ENABLED)
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
///Autoreload match Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARRMIE_A {
    ///0: ARRM interrupt disabled
    DISABLED = 0,
    ///1: ARRM interrupt enabled
    ENABLED = 1,
}
impl From<ARRMIE_A> for bool {
    #[inline(always)]
    fn from(variant: ARRMIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ARRMIE` reader - Autoreload match Interrupt Enable
pub struct ARRMIE_R(crate::FieldReader<bool, ARRMIE_A>);
impl ARRMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARRMIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARRMIE_A {
        match self.bits {
            false => ARRMIE_A::DISABLED,
            true => ARRMIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ARRMIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ARRMIE_A::ENABLED
    }
}
impl core::ops::Deref for ARRMIE_R {
    type Target = crate::FieldReader<bool, ARRMIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ARRMIE` writer - Autoreload match Interrupt Enable
pub struct ARRMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARRMIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ARRMIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///ARRM interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARRMIE_A::DISABLED)
    }
    ///ARRM interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARRMIE_A::ENABLED)
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
///Compare match Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMIE_A {
    ///0: CMPM interrupt disabled
    DISABLED = 0,
    ///1: CMPM interrupt enabled
    ENABLED = 1,
}
impl From<CMPMIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPMIE` reader - Compare match Interrupt Enable
pub struct CMPMIE_R(crate::FieldReader<bool, CMPMIE_A>);
impl CMPMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPMIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMPMIE_A {
        match self.bits {
            false => CMPMIE_A::DISABLED,
            true => CMPMIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMPMIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMPMIE_A::ENABLED
    }
}
impl core::ops::Deref for CMPMIE_R {
    type Target = crate::FieldReader<bool, CMPMIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CMPMIE` writer - Compare match Interrupt Enable
pub struct CMPMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CMPMIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CMPM interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMPMIE_A::DISABLED)
    }
    ///CMPM interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMPMIE_A::ENABLED)
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
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    pub fn repokie(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Compare register update OK Interrupt Enable
    #[inline(always)]
    pub fn cmpokie(&self) -> CMPOKIE_R {
        CMPOKIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Compare match Interrupt Enable
    #[inline(always)]
    pub fn cmpmie(&self) -> CMPMIE_R {
        CMPMIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    pub fn repokie(&mut self) -> REPOKIE_W {
        REPOKIE_W { w: self }
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&mut self) -> UEIE_W {
        UEIE_W { w: self }
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W {
        DOWNIE_W { w: self }
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W {
        UPIE_W { w: self }
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W {
        ARROKIE_W { w: self }
    }
    ///Bit 3 - Compare register update OK Interrupt Enable
    #[inline(always)]
    pub fn cmpokie(&mut self) -> CMPOKIE_W {
        CMPOKIE_W { w: self }
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W {
        EXTTRIGIE_W { w: self }
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W {
        ARRMIE_W { w: self }
    }
    ///Bit 0 - Compare match Interrupt Enable
    #[inline(always)]
    pub fn cmpmie(&mut self) -> CMPMIE_W {
        CMPMIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
