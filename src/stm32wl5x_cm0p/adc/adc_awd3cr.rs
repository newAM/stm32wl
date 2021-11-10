///Register `ADC_AWD3CR` reader
pub struct R(crate::R<ADC_AWD3CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_AWD3CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_AWD3CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_AWD3CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC_AWD3CR` writer
pub struct W(crate::W<ADC_AWD3CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_AWD3CR_SPEC>;
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
impl From<crate::W<ADC_AWD3CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_AWD3CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AWD3CH` reader - AWD3CH
pub struct AWD3CH_R(crate::FieldReader<u32, u32>);
impl AWD3CH_R {
    pub(crate) fn new(bits: u32) -> Self {
        AWD3CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD3CH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AWD3CH` writer - AWD3CH
pub struct AWD3CH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD3CH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:17 - AWD3CH
    #[inline(always)]
    pub fn awd3ch(&self) -> AWD3CH_R {
        AWD3CH_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    ///Bits 0:17 - AWD3CH
    #[inline(always)]
    pub fn awd3ch(&mut self) -> AWD3CH_W {
        AWD3CH_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC Analog Watchdog 3 Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_awd3cr](index.html) module
pub struct ADC_AWD3CR_SPEC;
impl crate::RegisterSpec for ADC_AWD3CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_awd3cr::R](R) reader structure
impl crate::Readable for ADC_AWD3CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc_awd3cr::W](W) writer structure
impl crate::Writable for ADC_AWD3CR_SPEC {
    type Writer = W;
}
///`reset()` method sets ADC_AWD3CR to value 0
impl crate::Resettable for ADC_AWD3CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
