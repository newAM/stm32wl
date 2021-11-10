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
impl R {
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W {
        LCKK_W { w: self }
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK3_W {
        LCK3_W { w: self }
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
