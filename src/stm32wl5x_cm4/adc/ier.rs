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
///ADRDYIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRDYIE_A {
    ///0: ADRDY interrupt disabled
    DISABLED = 0,
    ///1: ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
    ENABLED = 1,
}
impl From<ADRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDYIE` reader - ADRDYIE
pub struct ADRDYIE_R(crate::FieldReader<bool, ADRDYIE_A>);
impl ADRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADRDYIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADRDYIE_A {
        match self.bits {
            false => ADRDYIE_A::DISABLED,
            true => ADRDYIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADRDYIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADRDYIE_A::ENABLED
    }
}
impl core::ops::Deref for ADRDYIE_R {
    type Target = crate::FieldReader<bool, ADRDYIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ADRDYIE` writer - ADRDYIE
pub struct ADRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRDYIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADRDYIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///ADRDY interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADRDYIE_A::DISABLED)
    }
    ///ADRDY interrupt enabled. An interrupt is generated when the ADRDY bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADRDYIE_A::ENABLED)
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
///EOSMPIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSMPIE_A {
    ///0: EOSMP interrupt disabled
    DISABLED = 0,
    ///1: EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
    ENABLED = 1,
}
impl From<EOSMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMPIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMPIE` reader - EOSMPIE
pub struct EOSMPIE_R(crate::FieldReader<bool, EOSMPIE_A>);
impl EOSMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOSMPIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOSMPIE_A {
        match self.bits {
            false => EOSMPIE_A::DISABLED,
            true => EOSMPIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EOSMPIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EOSMPIE_A::ENABLED
    }
}
impl core::ops::Deref for EOSMPIE_R {
    type Target = crate::FieldReader<bool, EOSMPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EOSMPIE` writer - EOSMPIE
pub struct EOSMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOSMPIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOSMPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///EOSMP interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOSMPIE_A::DISABLED)
    }
    ///EOSMP interrupt enabled. An interrupt is generated when the EOSMP bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOSMPIE_A::ENABLED)
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
///EOCIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCIE_A {
    ///0: EOC interrupt disabled
    DISABLED = 0,
    ///1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
    ENABLED = 1,
}
impl From<EOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCIE` reader - EOCIE
pub struct EOCIE_R(crate::FieldReader<bool, EOCIE_A>);
impl EOCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::DISABLED,
            true => EOCIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EOCIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EOCIE_A::ENABLED
    }
}
impl core::ops::Deref for EOCIE_R {
    type Target = crate::FieldReader<bool, EOCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EOCIE` writer - EOCIE
pub struct EOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///EOC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCIE_A::DISABLED)
    }
    ///EOC interrupt enabled. An interrupt is generated when the EOC bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCIE_A::ENABLED)
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
///EOSIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSIE_A {
    ///0: EOS interrupt disabled
    DISABLED = 0,
    ///1: EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    ENABLED = 1,
}
impl From<EOSIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOSIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSIE` reader - EOSIE
pub struct EOSIE_R(crate::FieldReader<bool, EOSIE_A>);
impl EOSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOSIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOSIE_A {
        match self.bits {
            false => EOSIE_A::DISABLED,
            true => EOSIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EOSIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EOSIE_A::ENABLED
    }
}
impl core::ops::Deref for EOSIE_R {
    type Target = crate::FieldReader<bool, EOSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EOSIE` writer - EOSIE
pub struct EOSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOSIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///EOS interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOSIE_A::DISABLED)
    }
    ///EOS interrupt enabled. An interrupt is generated when the EOS bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOSIE_A::ENABLED)
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
///OVRIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRIE_A {
    ///0: Overrun interrupt disabled
    DISABLED = 0,
    ///1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
    ENABLED = 1,
}
impl From<OVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRIE` reader - OVRIE
pub struct OVRIE_R(crate::FieldReader<bool, OVRIE_A>);
impl OVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRIE_A {
        match self.bits {
            false => OVRIE_A::DISABLED,
            true => OVRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OVRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OVRIE_A::ENABLED
    }
}
impl core::ops::Deref for OVRIE_R {
    type Target = crate::FieldReader<bool, OVRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OVRIE` writer - OVRIE
pub struct OVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRIE_A::DISABLED)
    }
    ///Overrun interrupt enabled. An interrupt is generated when the OVR bit is set.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRIE_A::ENABLED)
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
///AWD1IE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1IE_A {
    ///0: Analog watchdog interrupt disabled
    DISABLED = 0,
    ///1: Analog watchdog interrupt enabled
    ENABLED = 1,
}
impl From<AWD1IE_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1IE` reader - AWD1IE
pub struct AWD1IE_R(crate::FieldReader<bool, AWD1IE_A>);
impl AWD1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD1IE_A {
        match self.bits {
            false => AWD1IE_A::DISABLED,
            true => AWD1IE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AWD1IE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AWD1IE_A::ENABLED
    }
}
impl core::ops::Deref for AWD1IE_R {
    type Target = crate::FieldReader<bool, AWD1IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AWD1IE` writer - AWD1IE
pub struct AWD1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD1IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD1IE_A::DISABLED)
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD1IE_A::ENABLED)
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
///AWD2IE
pub type AWD2IE_A = AWD1IE_A;
///Field `AWD2IE` reader - AWD2IE
pub type AWD2IE_R = AWD1IE_R;
///Field `AWD2IE` writer - AWD2IE
pub struct AWD2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD2IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD2IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD2IE_A::DISABLED)
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD2IE_A::ENABLED)
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
///AWD3IE
pub type AWD3IE_A = AWD1IE_A;
///Field `AWD3IE` reader - AWD3IE
pub type AWD3IE_R = AWD1IE_R;
///Field `AWD3IE` writer - AWD3IE
pub struct AWD3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD3IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD3IE_A::DISABLED)
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD3IE_A::ENABLED)
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
///EOCALIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCALIE_A {
    ///0: End of calibration interrupt disabled
    DISABLED = 0,
    ///1: End of calibration interrupt enabled
    ENABLED = 1,
}
impl From<EOCALIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCALIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCALIE` reader - EOCALIE
pub struct EOCALIE_R(crate::FieldReader<bool, EOCALIE_A>);
impl EOCALIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCALIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOCALIE_A {
        match self.bits {
            false => EOCALIE_A::DISABLED,
            true => EOCALIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EOCALIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EOCALIE_A::ENABLED
    }
}
impl core::ops::Deref for EOCALIE_R {
    type Target = crate::FieldReader<bool, EOCALIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EOCALIE` writer - EOCALIE
pub struct EOCALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCALIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOCALIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///End of calibration interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCALIE_A::DISABLED)
    }
    ///End of calibration interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCALIE_A::ENABLED)
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
///CCRDYIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCRDYIE_A {
    ///0: Channel configuration ready interrupt disabled
    DISABLED = 0,
    ///1: Channel configuration ready interrupt enabled
    ENABLED = 1,
}
impl From<CCRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCRDYIE` reader - CCRDYIE
pub struct CCRDYIE_R(crate::FieldReader<bool, CCRDYIE_A>);
impl CCRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCRDYIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCRDYIE_A {
        match self.bits {
            false => CCRDYIE_A::DISABLED,
            true => CCRDYIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CCRDYIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CCRDYIE_A::ENABLED
    }
}
impl core::ops::Deref for CCRDYIE_R {
    type Target = crate::FieldReader<bool, CCRDYIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCRDYIE` writer - CCRDYIE
pub struct CCRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRDYIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CCRDYIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Channel configuration ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCRDYIE_A::DISABLED)
    }
    ///Channel configuration ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCRDYIE_A::ENABLED)
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
impl R {
    ///Bit 0 - ADRDYIE
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - EOSMPIE
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - EOCIE
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - EOSIE
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - OVRIE
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 7 - AWD1IE
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - AWD2IE
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - AWD3IE
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 11 - EOCALIE
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 13 - CCRDYIE
    #[inline(always)]
    pub fn ccrdyie(&self) -> CCRDYIE_R {
        CCRDYIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - ADRDYIE
    #[inline(always)]
    pub fn adrdyie(&mut self) -> ADRDYIE_W {
        ADRDYIE_W { w: self }
    }
    ///Bit 1 - EOSMPIE
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EOSMPIE_W {
        EOSMPIE_W { w: self }
    }
    ///Bit 2 - EOCIE
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W {
        EOCIE_W { w: self }
    }
    ///Bit 3 - EOSIE
    #[inline(always)]
    pub fn eosie(&mut self) -> EOSIE_W {
        EOSIE_W { w: self }
    }
    ///Bit 4 - OVRIE
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W {
        OVRIE_W { w: self }
    }
    ///Bit 7 - AWD1IE
    #[inline(always)]
    pub fn awd1ie(&mut self) -> AWD1IE_W {
        AWD1IE_W { w: self }
    }
    ///Bit 8 - AWD2IE
    #[inline(always)]
    pub fn awd2ie(&mut self) -> AWD2IE_W {
        AWD2IE_W { w: self }
    }
    ///Bit 9 - AWD3IE
    #[inline(always)]
    pub fn awd3ie(&mut self) -> AWD3IE_W {
        AWD3IE_W { w: self }
    }
    ///Bit 11 - EOCALIE
    #[inline(always)]
    pub fn eocalie(&mut self) -> EOCALIE_W {
        EOCALIE_W { w: self }
    }
    ///Bit 13 - CCRDYIE
    #[inline(always)]
    pub fn ccrdyie(&mut self) -> CCRDYIE_W {
        CCRDYIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC interrupt enable register
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
