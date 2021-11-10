///Register `C2AHB1ENR` reader
pub struct R(crate::R<C2AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2AHB1ENR` writer
pub struct W(crate::W<C2AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB1ENR_SPEC>;
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
impl From<crate::W<C2AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
///CPU2 CRC clock enable
pub type CRCEN_A = DMA1EN_A;
///Field `CRCEN` reader - CPU2 CRC clock enable
pub type CRCEN_R = DMA1EN_R;
///Field `CRCEN` writer - CPU2 CRC clock enable
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::ENABLED)
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
///CPU2 DMAMUX1 clock enable
pub type DMAMUX1EN_A = DMA1EN_A;
///Field `DMAMUX1EN` reader - CPU2 DMAMUX1 clock enable
pub type DMAMUX1EN_R = DMA1EN_R;
///Field `DMAMUX1EN` writer - CPU2 DMAMUX1 clock enable
pub struct DMAMUX1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAMUX1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAMUX1EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAMUX1EN_A::ENABLED)
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
///CPU2 DMA2 clock enable
pub type DMA2EN_A = DMA1EN_A;
///Field `DMA2EN` reader - CPU2 DMA2 clock enable
pub type DMA2EN_R = DMA1EN_R;
///Field `DMA2EN` writer - CPU2 DMA2 clock enable
pub struct DMA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMA2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA2EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA2EN_A::ENABLED)
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
///CPU2 DMA1 clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1EN_A {
    ///0: Clock disabled
    DISABLED = 0,
    ///1: Clock enabled
    ENABLED = 1,
}
impl From<DMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMA1EN` reader - CPU2 DMA1 clock enable
pub struct DMA1EN_R(crate::FieldReader<bool, DMA1EN_A>);
impl DMA1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1EN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA1EN_A {
        match self.bits {
            false => DMA1EN_A::DISABLED,
            true => DMA1EN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMA1EN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMA1EN_A::ENABLED
    }
}
impl core::ops::Deref for DMA1EN_R {
    type Target = crate::FieldReader<bool, DMA1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMA1EN` writer - CPU2 DMA1 clock enable
pub struct DMA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMA1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
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
    ///Bit 12 - CPU2 CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 2 - CPU2 DMAMUX1 clock enable
    #[inline(always)]
    pub fn dmamux1en(&self) -> DMAMUX1EN_R {
        DMAMUX1EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - CPU2 DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - CPU2 DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 12 - CPU2 CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    ///Bit 2 - CPU2 DMAMUX1 clock enable
    #[inline(always)]
    pub fn dmamux1en(&mut self) -> DMAMUX1EN_W {
        DMAMUX1EN_W { w: self }
    }
    ///Bit 1 - CPU2 DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W {
        DMA2EN_W { w: self }
    }
    ///Bit 0 - CPU2 DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W {
        DMA1EN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 AHB1 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2ahb1enr](index.html) module
pub struct C2AHB1ENR_SPEC;
impl crate::RegisterSpec for C2AHB1ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2ahb1enr::R](R) reader structure
impl crate::Readable for C2AHB1ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2ahb1enr::W](W) writer structure
impl crate::Writable for C2AHB1ENR_SPEC {
    type Writer = W;
}
///`reset()` method sets C2AHB1ENR to value 0
impl crate::Resettable for C2AHB1ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
