///Register `MCR` reader
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MCR` writer
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
///DAC Channel 1 mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE1_A {
    ///0: Normal mode - DAC channelx is connected to external pin with Buffer enabled
    NORMALPINBUFFER = 0,
    ///1: Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    NORMALPINCHIPBUFFER = 1,
    ///2: Normal mode - DAC channelx is connected to external pin with Buffer disabled
    NORMALPINNOBUFFER = 2,
    ///3: Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    NORMALCHIPNOBUFFER = 3,
    ///4: S&H mode - DAC channelx is connected to external pin with Buffer enabled
    SHPINBUFFER = 4,
    ///5: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    SHPINCHIPBUFFER = 5,
    ///6: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
    SHPINNOBUFFER = 6,
    ///7: S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    SHCHIPNOBUFFER = 7,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
///Field `MODE1` reader - DAC Channel 1 mode
pub struct MODE1_R(crate::FieldReader<u8, MODE1_A>);
impl MODE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::NORMALPINBUFFER,
            1 => MODE1_A::NORMALPINCHIPBUFFER,
            2 => MODE1_A::NORMALPINNOBUFFER,
            3 => MODE1_A::NORMALCHIPNOBUFFER,
            4 => MODE1_A::SHPINBUFFER,
            5 => MODE1_A::SHPINCHIPBUFFER,
            6 => MODE1_A::SHPINNOBUFFER,
            7 => MODE1_A::SHCHIPNOBUFFER,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NORMALPINBUFFER`
    #[inline(always)]
    pub fn is_normal_pin_buffer(&self) -> bool {
        **self == MODE1_A::NORMALPINBUFFER
    }
    ///Checks if the value of the field is `NORMALPINCHIPBUFFER`
    #[inline(always)]
    pub fn is_normal_pin_chip_buffer(&self) -> bool {
        **self == MODE1_A::NORMALPINCHIPBUFFER
    }
    ///Checks if the value of the field is `NORMALPINNOBUFFER`
    #[inline(always)]
    pub fn is_normal_pin_no_buffer(&self) -> bool {
        **self == MODE1_A::NORMALPINNOBUFFER
    }
    ///Checks if the value of the field is `NORMALCHIPNOBUFFER`
    #[inline(always)]
    pub fn is_normal_chip_no_buffer(&self) -> bool {
        **self == MODE1_A::NORMALCHIPNOBUFFER
    }
    ///Checks if the value of the field is `SHPINBUFFER`
    #[inline(always)]
    pub fn is_shpin_buffer(&self) -> bool {
        **self == MODE1_A::SHPINBUFFER
    }
    ///Checks if the value of the field is `SHPINCHIPBUFFER`
    #[inline(always)]
    pub fn is_shpin_chip_buffer(&self) -> bool {
        **self == MODE1_A::SHPINCHIPBUFFER
    }
    ///Checks if the value of the field is `SHPINNOBUFFER`
    #[inline(always)]
    pub fn is_shpin_no_buffer(&self) -> bool {
        **self == MODE1_A::SHPINNOBUFFER
    }
    ///Checks if the value of the field is `SHCHIPNOBUFFER`
    #[inline(always)]
    pub fn is_shchip_no_buffer(&self) -> bool {
        **self == MODE1_A::SHCHIPNOBUFFER
    }
}
impl core::ops::Deref for MODE1_R {
    type Target = crate::FieldReader<u8, MODE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MODE1` writer - DAC Channel 1 mode
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Normal mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn normal_pin_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NORMALPINBUFFER)
    }
    ///Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn normal_pin_chip_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NORMALPINCHIPBUFFER)
    }
    ///Normal mode - DAC channelx is connected to external pin with Buffer disabled
    #[inline(always)]
    pub fn normal_pin_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NORMALPINNOBUFFER)
    }
    ///Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn normal_chip_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NORMALCHIPNOBUFFER)
    }
    ///S&H mode - DAC channelx is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn shpin_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::SHPINBUFFER)
    }
    ///S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn shpin_chip_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::SHPINCHIPBUFFER)
    }
    ///S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn shpin_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::SHPINNOBUFFER)
    }
    ///S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn shchip_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::SHCHIPNOBUFFER)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - DAC Channel 1 mode
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - DAC Channel 1 mode
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mcr](index.html) module
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mcr::R](R) reader structure
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mcr::W](W) writer structure
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}