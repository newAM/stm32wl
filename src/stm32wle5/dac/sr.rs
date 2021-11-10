///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///DAC Channel 1 busy writing sample time flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWST1_A {
    ///0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written
    IDLE = 0,
    ///1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written
    BUSY = 1,
}
impl From<BWST1_A> for bool {
    #[inline(always)]
    fn from(variant: BWST1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BWST1` reader - DAC Channel 1 busy writing sample time flag
pub struct BWST1_R(crate::FieldReader<bool, BWST1_A>);
impl BWST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BWST1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BWST1_A {
        match self.bits {
            false => BWST1_A::IDLE,
            true => BWST1_A::BUSY,
        }
    }
    ///Checks if the value of the field is `IDLE`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == BWST1_A::IDLE
    }
    ///Checks if the value of the field is `BUSY`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == BWST1_A::BUSY
    }
}
impl core::ops::Deref for BWST1_R {
    type Target = crate::FieldReader<bool, BWST1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///DAC Channel 1 calibration offset status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_FLAG1_A {
    ///0: Calibration trimming value is lower than the offset correction value
    LOWER = 0,
    ///1: Calibration trimming value is equal or greater than the offset correction value
    EQUAL_HIGHER = 1,
}
impl From<CAL_FLAG1_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_FLAG1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CAL_FLAG1` reader - DAC Channel 1 calibration offset status
pub struct CAL_FLAG1_R(crate::FieldReader<bool, CAL_FLAG1_A>);
impl CAL_FLAG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAL_FLAG1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CAL_FLAG1_A {
        match self.bits {
            false => CAL_FLAG1_A::LOWER,
            true => CAL_FLAG1_A::EQUAL_HIGHER,
        }
    }
    ///Checks if the value of the field is `LOWER`
    #[inline(always)]
    pub fn is_lower(&self) -> bool {
        **self == CAL_FLAG1_A::LOWER
    }
    ///Checks if the value of the field is `EQUAL_HIGHER`
    #[inline(always)]
    pub fn is_equal_higher(&self) -> bool {
        **self == CAL_FLAG1_A::EQUAL_HIGHER
    }
}
impl core::ops::Deref for CAL_FLAG1_R {
    type Target = crate::FieldReader<bool, CAL_FLAG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///DAC channel1 DMA underrun flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDR1_A {
    ///0: No DMA underrun error condition occurred for DAC channel x
    NOERROR = 0,
    ///1: DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
    ERROR = 1,
}
impl From<DMAUDR1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAUDR1` reader - DAC channel1 DMA underrun flag
pub struct DMAUDR1_R(crate::FieldReader<bool, DMAUDR1_A>);
impl DMAUDR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDR1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAUDR1_A {
        match self.bits {
            false => DMAUDR1_A::NOERROR,
            true => DMAUDR1_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == DMAUDR1_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == DMAUDR1_A::ERROR
    }
}
impl core::ops::Deref for DMAUDR1_R {
    type Target = crate::FieldReader<bool, DMAUDR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAUDR1` writer - DAC channel1 DMA underrun flag
pub struct DMAUDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAUDR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No DMA underrun error condition occurred for DAC channel x
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(DMAUDR1_A::NOERROR)
    }
    ///DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(DMAUDR1_A::ERROR)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    ///Bit 15 - DAC Channel 1 busy writing sample time flag
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - DAC Channel 1 calibration offset status
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W {
        DMAUDR1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
