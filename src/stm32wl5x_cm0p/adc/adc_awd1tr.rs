///Register `ADC_AWD1TR` reader
pub struct R(crate::R<ADC_AWD1TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_AWD1TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_AWD1TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_AWD1TR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_AWD1TR` writer
pub struct W(crate::W<ADC_AWD1TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_AWD1TR_SPEC>;
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
impl From<crate::W<ADC_AWD1TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_AWD1TR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LT1` reader - LT1
pub struct LT1_R(crate::FieldReader<u16, u16>);
impl LT1_R {
    pub(crate) fn new(bits: u16) -> Self {
        LT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LT1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LT1` writer - LT1
pub struct LT1_W<'a> {
    w: &'a mut W,
}
impl<'a> LT1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
///Field `HT1` reader - HT1
pub struct HT1_R(crate::FieldReader<u16, u16>);
impl HT1_R {
    pub(crate) fn new(bits: u16) -> Self {
        HT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HT1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HT1` writer - HT1
pub struct HT1_W<'a> {
    w: &'a mut W,
}
impl<'a> HT1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:11 - LT1
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - HT1
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - LT1
    #[inline(always)]
    pub fn lt1(&mut self) -> LT1_W {
        LT1_W { w: self }
    }
    ///Bits 16:27 - HT1
    #[inline(always)]
    pub fn ht1(&mut self) -> HT1_W {
        HT1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC watchdog threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_awd1tr](index.html) module
pub struct ADC_AWD1TR_SPEC;
impl crate::RegisterSpec for ADC_AWD1TR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_awd1tr::R](R) reader structure
impl crate::Readable for ADC_AWD1TR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_awd1tr::W](W) writer structure
impl crate::Writable for ADC_AWD1TR_SPEC {
    type Writer = W;
}
///`reset()` method sets ADC_AWD1TR to value 0x0fff_0000
impl crate::Resettable for ADC_AWD1TR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}
