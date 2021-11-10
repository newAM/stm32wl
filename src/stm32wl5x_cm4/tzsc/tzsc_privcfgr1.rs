///Register `TZSC_PRIVCFGR1` reader
pub struct R(crate::R<TZSC_PRIVCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_PRIVCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_PRIVCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_PRIVCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_PRIVCFGR1` writer
pub struct W(crate::W<TZSC_PRIVCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_PRIVCFGR1_SPEC>;
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
impl From<crate::W<TZSC_PRIVCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_PRIVCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AESPRIV` reader - AESPRIV
pub struct AESPRIV_R(crate::FieldReader<bool, bool>);
impl AESPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AESPRIV` writer - AESPRIV
pub struct AESPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> AESPRIV_W<'a> {
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
///Field `RNGPRIV` reader - RNGPRIV
pub struct RNGPRIV_R(crate::FieldReader<bool, bool>);
impl RNGPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RNGPRIV` writer - RNGPRIV
pub struct RNGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGPRIV_W<'a> {
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
///Field `SUBGHZSPIPRIV` reader - SUBGHZSPIPRIV
pub struct SUBGHZSPIPRIV_R(crate::FieldReader<bool, bool>);
impl SUBGHZSPIPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUBGHZSPIPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBGHZSPIPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SUBGHZSPIPRIV` writer - SUBGHZSPIPRIV
pub struct SUBGHZSPIPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPIPRIV_W<'a> {
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
///Field `PKAPRIV` reader - PKAPRIV
pub struct PKAPRIV_R(crate::FieldReader<bool, bool>);
impl PKAPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKAPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKAPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PKAPRIV` writer - PKAPRIV
pub struct PKAPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAPRIV_W<'a> {
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
    ///Bit 2 - AESPRIV
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - RNGPRIV
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - SUBGHZSPIPRIV
    #[inline(always)]
    pub fn subghzspipriv(&self) -> SUBGHZSPIPRIV_R {
        SUBGHZSPIPRIV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 13 - PKAPRIV
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - AESPRIV
    #[inline(always)]
    pub fn aespriv(&mut self) -> AESPRIV_W {
        AESPRIV_W { w: self }
    }
    ///Bit 3 - RNGPRIV
    #[inline(always)]
    pub fn rngpriv(&mut self) -> RNGPRIV_W {
        RNGPRIV_W { w: self }
    }
    ///Bit 4 - SUBGHZSPIPRIV
    #[inline(always)]
    pub fn subghzspipriv(&mut self) -> SUBGHZSPIPRIV_W {
        SUBGHZSPIPRIV_W { w: self }
    }
    ///Bit 13 - PKAPRIV
    #[inline(always)]
    pub fn pkapriv(&mut self) -> PKAPRIV_W {
        PKAPRIV_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC privilege configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_privcfgr1](index.html) module
pub struct TZSC_PRIVCFGR1_SPEC;
impl crate::RegisterSpec for TZSC_PRIVCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_privcfgr1::R](R) reader structure
impl crate::Readable for TZSC_PRIVCFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_privcfgr1::W](W) writer structure
impl crate::Writable for TZSC_PRIVCFGR1_SPEC {
    type Writer = W;
}
///`reset()` method sets TZSC_PRIVCFGR1 to value 0
impl crate::Resettable for TZSC_PRIVCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
