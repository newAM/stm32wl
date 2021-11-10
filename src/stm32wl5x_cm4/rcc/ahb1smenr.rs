///Register `AHB1SMENR` reader
pub struct R(crate::R<AHB1SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1SMENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB1SMENR` writer
pub struct W(crate::W<AHB1SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1SMENR_SPEC>;
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
impl From<crate::W<AHB1SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1SMENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CRCSMEN` reader - CRC clock enable during CPU1 CSleep mode.
pub struct CRCSMEN_R(crate::FieldReader<bool, bool>);
impl CRCSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CRCSMEN` writer - CRC clock enable during CPU1 CSleep mode.
pub struct CRCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSMEN_W<'a> {
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
///Field `DMAMUX1SMEN` reader - DMAMUX1 clock enable during CPU1 CSleep mode.
pub struct DMAMUX1SMEN_R(crate::FieldReader<bool, bool>);
impl DMAMUX1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUX1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAMUX1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAMUX1SMEN` writer - DMAMUX1 clock enable during CPU1 CSleep mode.
pub struct DMAMUX1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1SMEN_W<'a> {
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
///Field `DMA2SMEN` reader - DMA2 clock enable during CPU1 CSleep mode
pub struct DMA2SMEN_R(crate::FieldReader<bool, bool>);
impl DMA2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMA2SMEN` writer - DMA2 clock enable during CPU1 CSleep mode
pub struct DMA2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2SMEN_W<'a> {
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
///Field `DMA1SMEN` reader - DMA1 clock enable during CPU1 CSleep mode.
pub struct DMA1SMEN_R(crate::FieldReader<bool, bool>);
impl DMA1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMA1SMEN` writer - DMA1 clock enable during CPU1 CSleep mode.
pub struct DMA1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1SMEN_W<'a> {
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
    ///Bit 12 - CRC clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 2 - DMAMUX1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - DMA2 clock enable during CPU1 CSleep mode
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - DMA1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 12 - CRC clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W {
        CRCSMEN_W { w: self }
    }
    ///Bit 2 - DMAMUX1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W {
        DMAMUX1SMEN_W { w: self }
    }
    ///Bit 1 - DMA2 clock enable during CPU1 CSleep mode
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W {
        DMA2SMEN_W { w: self }
    }
    ///Bit 0 - DMA1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W {
        DMA1SMEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB1 peripheral clocks enable in Sleep modes register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1smenr](index.html) module
pub struct AHB1SMENR_SPEC;
impl crate::RegisterSpec for AHB1SMENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb1smenr::R](R) reader structure
impl crate::Readable for AHB1SMENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb1smenr::W](W) writer structure
impl crate::Writable for AHB1SMENR_SPEC {
    type Writer = W;
}
///`reset()` method sets AHB1SMENR to value 0x1007
impl crate::Resettable for AHB1SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1007
    }
}