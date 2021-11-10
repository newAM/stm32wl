///Register `PDCRC` reader
pub struct R(crate::R<PDCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PDCRC` writer
pub struct W(crate::W<PDCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRC_SPEC>;
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
impl From<crate::W<PDCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRC_SPEC>) -> Self {
        W(writer)
    }
}
///Port PC\[y\]
///pull-down (y=13 to 15)
pub type PD15_A = PD13_A;
///Field `PD15` reader - Port PC\[y\]
///pull-down (y=13 to 15)
pub type PD15_R = PD13_R;
///Field `PD15` writer - Port PC\[y\]
///pull-down (y=13 to 15)
pub struct PD15_W<'a> {
    w: &'a mut W,
}
impl<'a> PD15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD15_A::DISABLED)
    }
    ///Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD15_A::ENABLED)
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
///PD14
pub type PD14_A = PD13_A;
///Field `PD14` reader - PD14
pub type PD14_R = PD13_R;
///Field `PD14` writer - PD14
pub struct PD14_W<'a> {
    w: &'a mut W,
}
impl<'a> PD14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD14_A::DISABLED)
    }
    ///Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD14_A::ENABLED)
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
///PD13
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD13_A {
    ///0: Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    DISABLED = 0,
    ///1: Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    ENABLED = 1,
}
impl From<PD13_A> for bool {
    #[inline(always)]
    fn from(variant: PD13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PD13` reader - PD13
pub struct PD13_R(crate::FieldReader<bool, PD13_A>);
impl PD13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD13_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PD13_A {
        match self.bits {
            false => PD13_A::DISABLED,
            true => PD13_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PD13_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PD13_A::ENABLED
    }
}
impl core::ops::Deref for PD13_R {
    type Target = crate::FieldReader<bool, PD13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PD13` writer - PD13
pub struct PD13_W<'a> {
    w: &'a mut W,
}
impl<'a> PD13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD13_A::DISABLED)
    }
    ///Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD13_A::ENABLED)
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
///PD2
pub type PD2_A = PD0_A;
///Field `PD2` reader - PD2
pub type PD2_R = PD0_R;
///Field `PD2` writer - PD2
pub struct PD2_W<'a> {
    w: &'a mut W,
}
impl<'a> PD2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD2_A::DISABLED)
    }
    ///Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD2_A::ENABLED)
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
///PD1
pub type PD1_A = PD0_A;
///Field `PD1` reader - PD1
pub type PD1_R = PD0_R;
///Field `PD1` writer - PD1
pub struct PD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PD1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD1_A::DISABLED)
    }
    ///Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD1_A::ENABLED)
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
///PD0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD0_A {
    ///0: Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    DISABLED = 0,
    ///1: Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    ENABLED = 1,
}
impl From<PD0_A> for bool {
    #[inline(always)]
    fn from(variant: PD0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PD0` reader - PD0
pub struct PD0_R(crate::FieldReader<bool, PD0_A>);
impl PD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PD0_A {
        match self.bits {
            false => PD0_A::DISABLED,
            true => PD0_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PD0_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PD0_A::ENABLED
    }
}
impl core::ops::Deref for PD0_R {
    type Target = crate::FieldReader<bool, PD0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PD0` writer - PD0
pub struct PD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD0_A::DISABLED)
    }
    ///Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD0_A::ENABLED)
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
///PD3
pub type PD3_A = PD0_A;
///Field `PD3` reader - PD3
pub type PD3_R = PD0_R;
///Field `PD3` writer - PD3
pub struct PD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PD3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD3_A::DISABLED)
    }
    ///Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD3_A::ENABLED)
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
///PD4
pub type PD4_A = PD0_A;
///Field `PD4` reader - PD4
pub type PD4_R = PD0_R;
///Field `PD4` writer - PD4
pub struct PD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PD4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD4_A::DISABLED)
    }
    ///Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD4_A::ENABLED)
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
///PD5
pub type PD5_A = PD0_A;
///Field `PD5` reader - PD5
pub type PD5_R = PD0_R;
///Field `PD5` writer - PD5
pub struct PD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PD5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD5_A::DISABLED)
    }
    ///Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD5_A::ENABLED)
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
///PD6
pub type PD6_A = PD0_A;
///Field `PD6` reader - PD6
pub type PD6_R = PD0_R;
///Field `PD6` writer - PD6
pub struct PD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PD6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PD6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD6_A::DISABLED)
    }
    ///Enable the pull-down on PC\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD6_A::ENABLED)
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
impl R {
    ///Bit 15 - Port PC\[y\]
    ///pull-down (y=13 to 15)
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - PD14
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - PD13
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 2 - PD2
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - PD1
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - PD0
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 3 - PD3
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - PD4
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - PD5
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - PD6
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    ///Bit 15 - Port PC\[y\]
    ///pull-down (y=13 to 15)
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W {
        PD15_W { w: self }
    }
    ///Bit 14 - PD14
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W {
        PD14_W { w: self }
    }
    ///Bit 13 - PD13
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W {
        PD13_W { w: self }
    }
    ///Bit 2 - PD2
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W {
        PD2_W { w: self }
    }
    ///Bit 1 - PD1
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W {
        PD1_W { w: self }
    }
    ///Bit 0 - PD0
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W {
        PD0_W { w: self }
    }
    ///Bit 3 - PD3
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W {
        PD3_W { w: self }
    }
    ///Bit 4 - PD4
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W {
        PD4_W { w: self }
    }
    ///Bit 5 - PD5
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W {
        PD5_W { w: self }
    }
    ///Bit 6 - PD6
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W {
        PD6_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power Port C pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrc](index.html) module
pub struct PDCRC_SPEC;
impl crate::RegisterSpec for PDCRC_SPEC {
    type Ux = u32;
}
///`read()` method returns [pdcrc::R](R) reader structure
impl crate::Readable for PDCRC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pdcrc::W](W) writer structure
impl crate::Writable for PDCRC_SPEC {
    type Writer = W;
}
///`reset()` method sets PDCRC to value 0
impl crate::Resettable for PDCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
