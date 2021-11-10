///Register `ADC_DR` reader
pub struct R(crate::R<ADC_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DATA` reader - DATA
pub struct DATA_R(crate::FieldReader<u16, u16>);
impl DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:15 - DATA
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
///ADC data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc_dr](index.html) module
pub struct ADC_DR_SPEC;
impl crate::RegisterSpec for ADC_DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc_dr::R](R) reader structure
impl crate::Readable for ADC_DR_SPEC {
    type Reader = R;
}
///`reset()` method sets ADC_DR to value 0
impl crate::Resettable for ADC_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}