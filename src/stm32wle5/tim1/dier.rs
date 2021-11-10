///Register `DIER` reader
pub struct R(crate::R<DIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIER` writer
pub struct W(crate::W<DIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIER_SPEC>;
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
impl From<crate::W<DIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIER_SPEC>) -> Self {
        W(writer)
    }
}
///Trigger DMA request enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDE_A {
    ///0: Trigger DMA request disabled
    DISABLED = 0,
    ///1: Trigger DMA request enabled
    ENABLED = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TDE` reader - Trigger DMA request enable
pub struct TDE_R(crate::FieldReader<bool, TDE_A>);
impl TDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::DISABLED,
            true => TDE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TDE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TDE_A::ENABLED
    }
}
impl core::ops::Deref for TDE_R {
    type Target = crate::FieldReader<bool, TDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TDE` writer - Trigger DMA request enable
pub struct TDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Trigger DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDE_A::DISABLED)
    }
    ///Trigger DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDE_A::ENABLED)
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
///COM DMA request enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMDE_A {
    ///0: COM DMA request disabled
    DISABLED = 0,
    ///1: COM DMA request enabled
    ENABLED = 1,
}
impl From<COMDE_A> for bool {
    #[inline(always)]
    fn from(variant: COMDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COMDE` reader - COM DMA request enable
pub struct COMDE_R(crate::FieldReader<bool, COMDE_A>);
impl COMDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMDE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COMDE_A {
        match self.bits {
            false => COMDE_A::DISABLED,
            true => COMDE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMDE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMDE_A::ENABLED
    }
}
impl core::ops::Deref for COMDE_R {
    type Target = crate::FieldReader<bool, COMDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `COMDE` writer - COM DMA request enable
pub struct COMDE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMDE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COMDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///COM DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMDE_A::DISABLED)
    }
    ///COM DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMDE_A::ENABLED)
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
///Capture/Compare 4 DMA request enable
pub type CC4DE_A = CC1DE_A;
///Field `CC4DE` reader - Capture/Compare 4 DMA request enable
pub type CC4DE_R = CC1DE_R;
///Field `CC4DE` writer - Capture/Compare 4 DMA request enable
pub struct CC4DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4DE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC4DE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4DE_A::DISABLED)
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4DE_A::ENABLED)
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
///Capture/Compare 3 DMA request enable
pub type CC3DE_A = CC1DE_A;
///Field `CC3DE` reader - Capture/Compare 3 DMA request enable
pub type CC3DE_R = CC1DE_R;
///Field `CC3DE` writer - Capture/Compare 3 DMA request enable
pub struct CC3DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3DE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC3DE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC3DE_A::DISABLED)
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC3DE_A::ENABLED)
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
///Capture/Compare 2 DMA request enable
pub type CC2DE_A = CC1DE_A;
///Field `CC2DE` reader - Capture/Compare 2 DMA request enable
pub type CC2DE_R = CC1DE_R;
///Field `CC2DE` writer - Capture/Compare 2 DMA request enable
pub struct CC2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2DE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2DE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC2DE_A::DISABLED)
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC2DE_A::ENABLED)
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
///Capture/Compare 1 DMA request enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1DE_A {
    ///0: CCx DMA request disabled
    DISABLED = 0,
    ///1: CCx DMA request enabled
    ENABLED = 1,
}
impl From<CC1DE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1DE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1DE` reader - Capture/Compare 1 DMA request enable
pub struct CC1DE_R(crate::FieldReader<bool, CC1DE_A>);
impl CC1DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1DE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1DE_A {
        match self.bits {
            false => CC1DE_A::DISABLED,
            true => CC1DE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CC1DE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CC1DE_A::ENABLED
    }
}
impl core::ops::Deref for CC1DE_R {
    type Target = crate::FieldReader<bool, CC1DE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1DE` writer - Capture/Compare 1 DMA request enable
pub struct CC1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1DE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1DE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1DE_A::DISABLED)
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1DE_A::ENABLED)
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
///Update DMA request enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDE_A {
    ///0: Update DMA request disabled
    DISABLED = 0,
    ///1: Update DMA request enabled
    ENABLED = 1,
}
impl From<UDE_A> for bool {
    #[inline(always)]
    fn from(variant: UDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UDE` reader - Update DMA request enable
pub struct UDE_R(crate::FieldReader<bool, UDE_A>);
impl UDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UDE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UDE_A {
        match self.bits {
            false => UDE_A::DISABLED,
            true => UDE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == UDE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == UDE_A::ENABLED
    }
}
impl core::ops::Deref for UDE_R {
    type Target = crate::FieldReader<bool, UDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UDE` writer - Update DMA request enable
pub struct UDE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Update DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDE_A::DISABLED)
    }
    ///Update DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDE_A::ENABLED)
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
///Break interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIE_A {
    ///0: Break interrupt disabled
    DISABLED = 0,
    ///1: Break interrupt enabled
    ENABLED = 1,
}
impl From<BIE_A> for bool {
    #[inline(always)]
    fn from(variant: BIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BIE` reader - Break interrupt enable
pub struct BIE_R(crate::FieldReader<bool, BIE_A>);
impl BIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BIE_A {
        match self.bits {
            false => BIE_A::DISABLED,
            true => BIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BIE_A::ENABLED
    }
}
impl core::ops::Deref for BIE_R {
    type Target = crate::FieldReader<bool, BIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BIE` writer - Break interrupt enable
pub struct BIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Break interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BIE_A::DISABLED)
    }
    ///Break interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BIE_A::ENABLED)
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
///Trigger interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    ///0: Trigger interrupt disabled
    DISABLED = 0,
    ///1: Trigger interrupt enabled
    ENABLED = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIE` reader - Trigger interrupt enable
pub struct TIE_R(crate::FieldReader<bool, TIE_A>);
impl TIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::DISABLED,
            true => TIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TIE_A::ENABLED
    }
}
impl core::ops::Deref for TIE_R {
    type Target = crate::FieldReader<bool, TIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIE` writer - Trigger interrupt enable
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Trigger interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIE_A::DISABLED)
    }
    ///Trigger interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIE_A::ENABLED)
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
///COM interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMIE_A {
    ///0: COM interrupt disabled
    DISABLED = 0,
    ///1: COM interrupt enabled
    ENABLED = 1,
}
impl From<COMIE_A> for bool {
    #[inline(always)]
    fn from(variant: COMIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COMIE` reader - COM interrupt enable
pub struct COMIE_R(crate::FieldReader<bool, COMIE_A>);
impl COMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COMIE_A {
        match self.bits {
            false => COMIE_A::DISABLED,
            true => COMIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMIE_A::ENABLED
    }
}
impl core::ops::Deref for COMIE_R {
    type Target = crate::FieldReader<bool, COMIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `COMIE` writer - COM interrupt enable
pub struct COMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COMIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///COM interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMIE_A::DISABLED)
    }
    ///COM interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMIE_A::ENABLED)
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
///Capture/Compare 4 interrupt enable
pub type CC4IE_A = CC1IE_A;
///Field `CC4IE` reader - Capture/Compare 4 interrupt enable
pub type CC4IE_R = CC1IE_R;
///Field `CC4IE` writer - Capture/Compare 4 interrupt enable
pub struct CC4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC4IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4IE_A::DISABLED)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4IE_A::ENABLED)
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
///Capture/Compare 3 interrupt enable
pub type CC3IE_A = CC1IE_A;
///Field `CC3IE` reader - Capture/Compare 3 interrupt enable
pub type CC3IE_R = CC1IE_R;
///Field `CC3IE` writer - Capture/Compare 3 interrupt enable
pub struct CC3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC3IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC3IE_A::DISABLED)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC3IE_A::ENABLED)
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
///Capture/Compare 2 interrupt enable
pub type CC2IE_A = CC1IE_A;
///Field `CC2IE` reader - Capture/Compare 2 interrupt enable
pub type CC2IE_R = CC1IE_R;
///Field `CC2IE` writer - Capture/Compare 2 interrupt enable
pub struct CC2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC2IE_A::DISABLED)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC2IE_A::ENABLED)
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
///Capture/Compare 1 interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IE_A {
    ///0: CCx interrupt disabled
    DISABLED = 0,
    ///1: CCx interrupt enabled
    ENABLED = 1,
}
impl From<CC1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1IE` reader - Capture/Compare 1 interrupt enable
pub struct CC1IE_R(crate::FieldReader<bool, CC1IE_A>);
impl CC1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1IE_A {
        match self.bits {
            false => CC1IE_A::DISABLED,
            true => CC1IE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CC1IE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CC1IE_A::ENABLED
    }
}
impl core::ops::Deref for CC1IE_R {
    type Target = crate::FieldReader<bool, CC1IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1IE` writer - Capture/Compare 1 interrupt enable
pub struct CC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1IE_A::DISABLED)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1IE_A::ENABLED)
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
///Update interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIE_A {
    ///0: Update interrupt disabled
    DISABLED = 0,
    ///1: Update interrupt enabled
    ENABLED = 1,
}
impl From<UIE_A> for bool {
    #[inline(always)]
    fn from(variant: UIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UIE` reader - Update interrupt enable
pub struct UIE_R(crate::FieldReader<bool, UIE_A>);
impl UIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UIE_A {
        match self.bits {
            false => UIE_A::DISABLED,
            true => UIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == UIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == UIE_A::ENABLED
    }
}
impl core::ops::Deref for UIE_R {
    type Target = crate::FieldReader<bool, UIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UIE` writer - Update interrupt enable
pub struct UIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Update interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UIE_A::DISABLED)
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UIE_A::ENABLED)
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
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W {
        TDE_W { w: self }
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&mut self) -> COMDE_W {
        COMDE_W { w: self }
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&mut self) -> CC4DE_W {
        CC4DE_W { w: self }
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&mut self) -> CC3DE_W {
        CC3DE_W { w: self }
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W {
        CC2DE_W { w: self }
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W {
        CC1DE_W { w: self }
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W {
        UDE_W { w: self }
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W {
        BIE_W { w: self }
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W {
        COMIE_W { w: self }
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W {
        CC4IE_W { w: self }
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W {
        CC3IE_W { w: self }
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W {
        CC2IE_W { w: self }
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W {
        CC1IE_W { w: self }
    }
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W {
        UIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA/interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dier](index.html) module
pub struct DIER_SPEC;
impl crate::RegisterSpec for DIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [dier::R](R) reader structure
impl crate::Readable for DIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dier::W](W) writer structure
impl crate::Writable for DIER_SPEC {
    type Writer = W;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
