///Register `C13CR` reader
pub struct R(crate::R<C13CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C13CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C13CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C13CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C13CR` writer
pub struct W(crate::W<C13CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C13CR_SPEC>;
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
impl From<crate::W<C13CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C13CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYNC_ID` reader - SYNC_ID
pub struct SYNC_ID_R(crate::FieldReader<u8, u8>);
impl SYNC_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNC_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNC_ID` writer - SYNC_ID
pub struct SYNC_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_ID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
///Field `NBREQ` reader - NBREQ
pub struct NBREQ_R(crate::FieldReader<u8, u8>);
impl NBREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NBREQ` writer - NBREQ
pub struct NBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NBREQ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
///Field `SPOL` reader - SPOL
pub struct SPOL_R(crate::FieldReader<u8, u8>);
impl SPOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SPOL` writer - SPOL
pub struct SPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
///Field `SE` reader - SE
pub struct SE_R(crate::FieldReader<bool, bool>);
impl SE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SE` writer - SE
pub struct SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_W<'a> {
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
///Field `EGE` reader - EGE
pub struct EGE_R(crate::FieldReader<bool, bool>);
impl EGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EGE` writer - EGE
pub struct EGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EGE_W<'a> {
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
///Field `SOIE` reader - SOIE
pub struct SOIE_R(crate::FieldReader<bool, bool>);
impl SOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SOIE` writer - SOIE
pub struct SOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOIE_W<'a> {
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
///Field `DMAREQ_ID` reader - DMAREQ_ID
pub struct DMAREQ_ID_R(crate::FieldReader<u8, u8>);
impl DMAREQ_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMAREQ_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAREQ_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAREQ_ID` writer - DMAREQ_ID
pub struct DMAREQ_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQ_ID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bits 24:28 - SYNC_ID
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bits 19:23 - NBREQ
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 17:18 - SPOL
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    ///Bit 16 - SE
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 9 - EGE
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - SOIE
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 0:7 - DMAREQ_ID
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 24:28 - SYNC_ID
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W {
        SYNC_ID_W { w: self }
    }
    ///Bits 19:23 - NBREQ
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W {
        NBREQ_W { w: self }
    }
    ///Bits 17:18 - SPOL
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W {
        SPOL_W { w: self }
    }
    ///Bit 16 - SE
    #[inline(always)]
    pub fn se(&mut self) -> SE_W {
        SE_W { w: self }
    }
    ///Bit 9 - EGE
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W {
        EGE_W { w: self }
    }
    ///Bit 8 - SOIE
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W {
        SOIE_W { w: self }
    }
    ///Bits 0:7 - DMAREQ_ID
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W {
        DMAREQ_ID_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///C13CR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c13cr](index.html) module
pub struct C13CR_SPEC;
impl crate::RegisterSpec for C13CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c13cr::R](R) reader structure
impl crate::Readable for C13CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c13cr::W](W) writer structure
impl crate::Writable for C13CR_SPEC {
    type Writer = W;
}
///`reset()` method sets C13CR to value 0
impl crate::Resettable for C13CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}