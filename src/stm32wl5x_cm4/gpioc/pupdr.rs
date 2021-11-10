///Register `PUPDR` reader
pub struct R(crate::R<PUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUPDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PUPDR` writer
pub struct W(crate::W<PUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUPDR_SPEC>;
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
impl From<crate::W<PUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUPDR_SPEC>) -> Self {
        W(writer)
    }
}
///Port x configuration bits (y = 0..15)
pub type PUPDR15_A = PUPDR0_A;
///Field `PUPDR15` reader - Port x configuration bits (y = 0..15)
pub type PUPDR15_R = PUPDR0_R;
///Field `PUPDR15` writer - Port x configuration bits (y = 0..15)
pub struct PUPDR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PUPDR15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR15_A::FLOATING)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15_A::PULLUP)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15_A::PULLDOWN)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type PUPDR14_A = PUPDR0_A;
///Field `PUPDR14` reader - Port x configuration bits (y = 0..15)
pub type PUPDR14_R = PUPDR0_R;
///Field `PUPDR14` writer - Port x configuration bits (y = 0..15)
pub struct PUPDR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PUPDR14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR14_A::FLOATING)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR14_A::PULLUP)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR14_A::PULLDOWN)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type PUPDR13_A = PUPDR0_A;
///Field `PUPDR13` reader - Port x configuration bits (y = 0..15)
pub type PUPDR13_R = PUPDR0_R;
///Field `PUPDR13` writer - Port x configuration bits (y = 0..15)
pub struct PUPDR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PUPDR13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR13_A::FLOATING)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR13_A::PULLUP)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR13_A::PULLDOWN)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type PUPDR6_A = PUPDR0_A;
///Field `PUPDR6` reader - Port x configuration bits (y = 0..15)
pub type PUPDR6_R = PUPDR0_R;
///Field `PUPDR6` writer - Port x configuration bits (y = 0..15)
pub struct PUPDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PUPDR6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR6_A::FLOATING)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR6_A::PULLUP)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR6_A::PULLDOWN)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type PUPDR5_A = PUPDR0_A;
///Field `PUPDR5` reader - Port x configuration bits (y = 0..15)
pub type PUPDR5_R = PUPDR0_R;
///Field `PUPDR5` writer - Port x configuration bits (y = 0..15)
pub struct PUPDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PUPDR5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR5_A::FLOATING)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR5_A::PULLUP)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR5_A::PULLDOWN)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type PUPDR4_A = PUPDR0_A;
///Field `PUPDR4` reader - Port x configuration bits (y = 0..15)
pub type PUPDR4_R = PUPDR0_R;
///Field `PUPDR4` writer - Port x configuration bits (y = 0..15)
pub struct PUPDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PUPDR4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR4_A::FLOATING)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR4_A::PULLUP)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR4_A::PULLDOWN)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type PUPDR3_A = PUPDR0_A;
///Field `PUPDR3` reader - Port x configuration bits (y = 0..15)
pub type PUPDR3_R = PUPDR0_R;
///Field `PUPDR3` writer - Port x configuration bits (y = 0..15)
pub struct PUPDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PUPDR3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR3_A::FLOATING)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR3_A::PULLUP)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR3_A::PULLDOWN)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type PUPDR2_A = PUPDR0_A;
///Field `PUPDR2` reader - Port x configuration bits (y = 0..15)
pub type PUPDR2_R = PUPDR0_R;
///Field `PUPDR2` writer - Port x configuration bits (y = 0..15)
pub struct PUPDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PUPDR2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR2_A::FLOATING)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR2_A::PULLUP)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR2_A::PULLDOWN)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type PUPDR1_A = PUPDR0_A;
///Field `PUPDR1` reader - Port x configuration bits (y = 0..15)
pub type PUPDR1_R = PUPDR0_R;
///Field `PUPDR1` writer - Port x configuration bits (y = 0..15)
pub struct PUPDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PUPDR1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR1_A::FLOATING)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR1_A::PULLUP)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR1_A::PULLDOWN)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PUPDR0_A {
    ///0: No pull-up, pull-down
    FLOATING = 0,
    ///1: Pull-up
    PULLUP = 1,
    ///2: Pull-down
    PULLDOWN = 2,
}
impl From<PUPDR0_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPDR0_A) -> Self {
        variant as _
    }
}
///Field `PUPDR0` reader - Port x configuration bits (y = 0..15)
pub struct PUPDR0_R(crate::FieldReader<u8, PUPDR0_A>);
impl PUPDR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUPDR0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PUPDR0_A> {
        match self.bits {
            0 => Some(PUPDR0_A::FLOATING),
            1 => Some(PUPDR0_A::PULLUP),
            2 => Some(PUPDR0_A::PULLDOWN),
            _ => None,
        }
    }
    ///Checks if the value of the field is `FLOATING`
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        **self == PUPDR0_A::FLOATING
    }
    ///Checks if the value of the field is `PULLUP`
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PUPDR0_A::PULLUP
    }
    ///Checks if the value of the field is `PULLDOWN`
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PUPDR0_A::PULLDOWN
    }
}
impl core::ops::Deref for PUPDR0_R {
    type Target = crate::FieldReader<u8, PUPDR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PUPDR0` writer - Port x configuration bits (y = 0..15)
pub struct PUPDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PUPDR0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR0_A::FLOATING)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR0_A::PULLUP)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR0_A::PULLDOWN)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr15(&self) -> PUPDR15_R {
        PUPDR15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr14(&self) -> PUPDR14_R {
        PUPDR14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr13(&self) -> PUPDR13_R {
        PUPDR13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR6_R {
        PUPDR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR5_R {
        PUPDR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR4_R {
        PUPDR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR2_R {
        PUPDR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr15(&mut self) -> PUPDR15_W {
        PUPDR15_W { w: self }
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr14(&mut self) -> PUPDR14_W {
        PUPDR14_W { w: self }
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr13(&mut self) -> PUPDR13_W {
        PUPDR13_W { w: self }
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr6(&mut self) -> PUPDR6_W {
        PUPDR6_W { w: self }
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr5(&mut self) -> PUPDR5_W {
        PUPDR5_W { w: self }
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr4(&mut self) -> PUPDR4_W {
        PUPDR4_W { w: self }
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr3(&mut self) -> PUPDR3_W {
        PUPDR3_W { w: self }
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr2(&mut self) -> PUPDR2_W {
        PUPDR2_W { w: self }
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr1(&mut self) -> PUPDR1_W {
        PUPDR1_W { w: self }
    }
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr0(&mut self) -> PUPDR0_W {
        PUPDR0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port pull-up/pull-down register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pupdr](index.html) module
pub struct PUPDR_SPEC;
impl crate::RegisterSpec for PUPDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pupdr::R](R) reader structure
impl crate::Readable for PUPDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pupdr::W](W) writer structure
impl crate::Writable for PUPDR_SPEC {
    type Writer = W;
}
///`reset()` method sets PUPDR to value 0
impl crate::Resettable for PUPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
