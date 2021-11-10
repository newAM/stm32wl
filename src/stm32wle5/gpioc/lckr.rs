///Register `LCKR` reader
pub struct R(crate::R<LCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCKR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LCKR` writer
pub struct W(crate::W<LCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCKR_SPEC>;
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
impl From<crate::W<LCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCKR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LCKK` reader - Port x lock bit y (y= 0..15)
pub struct LCKK_R(crate::FieldReader<bool, bool>);
impl LCKK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCKK` writer - Port x lock bit y (y= 0..15)
pub struct LCKK_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKK_W<'a> {
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
///Field `LCK15` reader - Port x lock bit y (y= 0..15)
pub struct LCK15_R(crate::FieldReader<bool, bool>);
impl LCK15_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK15` writer - Port x lock bit y (y= 0..15)
pub struct LCK15_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK15_W<'a> {
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
///Field `LCK14` reader - Port x lock bit y (y= 0..15)
pub struct LCK14_R(crate::FieldReader<bool, bool>);
impl LCK14_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK14` writer - Port x lock bit y (y= 0..15)
pub struct LCK14_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK14_W<'a> {
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
///Field `LCK13` reader - Port x lock bit y (y= 0..15)
pub struct LCK13_R(crate::FieldReader<bool, bool>);
impl LCK13_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK13` writer - Port x lock bit y (y= 0..15)
pub struct LCK13_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK13_W<'a> {
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
///Field `LCK6` reader - Port x lock bit y (y= 0..15)
pub struct LCK6_R(crate::FieldReader<bool, bool>);
impl LCK6_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK6` writer - Port x lock bit y (y= 0..15)
pub struct LCK6_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK6_W<'a> {
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
///Field `LCK5` reader - Port x lock bit y (y= 0..15)
pub struct LCK5_R(crate::FieldReader<bool, bool>);
impl LCK5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK5` writer - Port x lock bit y (y= 0..15)
pub struct LCK5_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK5_W<'a> {
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
///Field `LCK4` reader - Port x lock bit y (y= 0..15)
pub struct LCK4_R(crate::FieldReader<bool, bool>);
impl LCK4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK4` writer - Port x lock bit y (y= 0..15)
pub struct LCK4_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK4_W<'a> {
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
///Field `LCK3` reader - Port x lock bit y (y= 0..15)
pub struct LCK3_R(crate::FieldReader<bool, bool>);
impl LCK3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK3` writer - Port x lock bit y (y= 0..15)
pub struct LCK3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK3_W<'a> {
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
///Field `LCK2` reader - Port x lock bit y (y= 0..15)
pub struct LCK2_R(crate::FieldReader<bool, bool>);
impl LCK2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK2` writer - Port x lock bit y (y= 0..15)
pub struct LCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK2_W<'a> {
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
///Field `LCK1` reader - Port x lock bit y (y= 0..15)
pub struct LCK1_R(crate::FieldReader<bool, bool>);
impl LCK1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK1` writer - Port x lock bit y (y= 0..15)
pub struct LCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK1_W<'a> {
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
///Field `LCK0` reader - Port x lock bit y (y= 0..15)
pub struct LCK0_R(crate::FieldReader<bool, bool>);
impl LCK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK0` writer - Port x lock bit y (y= 0..15)
pub struct LCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK0_W<'a> {
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
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 6 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W {
        LCKK_W { w: self }
    }
    ///Bit 15 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck15(&mut self) -> LCK15_W {
        LCK15_W { w: self }
    }
    ///Bit 14 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck14(&mut self) -> LCK14_W {
        LCK14_W { w: self }
    }
    ///Bit 13 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck13(&mut self) -> LCK13_W {
        LCK13_W { w: self }
    }
    ///Bit 6 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck6(&mut self) -> LCK6_W {
        LCK6_W { w: self }
    }
    ///Bit 5 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck5(&mut self) -> LCK5_W {
        LCK5_W { w: self }
    }
    ///Bit 4 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck4(&mut self) -> LCK4_W {
        LCK4_W { w: self }
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK3_W {
        LCK3_W { w: self }
    }
    ///Bit 2 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck2(&mut self) -> LCK2_W {
        LCK2_W { w: self }
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK1_W {
        LCK1_W { w: self }
    }
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK0_W {
        LCK0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port configuration lock register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lckr](index.html) module
pub struct LCKR_SPEC;
impl crate::RegisterSpec for LCKR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lckr::R](R) reader structure
impl crate::Readable for LCKR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lckr::W](W) writer structure
impl crate::Writable for LCKR_SPEC {
    type Writer = W;
}
///`reset()` method sets LCKR to value 0
impl crate::Resettable for LCKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
