///Register `MODER` reader
pub struct R(crate::R<MODER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MODER` writer
pub struct W(crate::W<MODER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODER_SPEC>;
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
impl From<crate::W<MODER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODER_SPEC>) -> Self {
        W(writer)
    }
}
///Port x configuration bits (y = 0..15)
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODER3_A {
    ///0: Input mode (reset state)
    INPUT = 0,
    ///1: General purpose output mode
    OUTPUT = 1,
    ///2: Alternate function mode
    ALTERNATE = 2,
    ///3: Analog mode
    ANALOG = 3,
}
impl From<MODER3_A> for u8 {
    #[inline(always)]
    fn from(variant: MODER3_A) -> Self {
        variant as _
    }
}
///Field `MODER3` reader - Port x configuration bits (y = 0..15)
pub struct MODER3_R(crate::FieldReader<u8, MODER3_A>);
impl MODER3_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODER3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODER3_A {
        match self.bits {
            0 => MODER3_A::INPUT,
            1 => MODER3_A::OUTPUT,
            2 => MODER3_A::ALTERNATE,
            3 => MODER3_A::ANALOG,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `INPUT`
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == MODER3_A::INPUT
    }
    ///Checks if the value of the field is `OUTPUT`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == MODER3_A::OUTPUT
    }
    ///Checks if the value of the field is `ALTERNATE`
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        **self == MODER3_A::ALTERNATE
    }
    ///Checks if the value of the field is `ANALOG`
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        **self == MODER3_A::ANALOG
    }
}
impl core::ops::Deref for MODER3_R {
    type Target = crate::FieldReader<u8, MODER3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MODER3` writer - Port x configuration bits (y = 0..15)
pub struct MODER3_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER3_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER3_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER3_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER3_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn moder3(&mut self) -> MODER3_W {
        MODER3_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [moder](index.html) module
pub struct MODER_SPEC;
impl crate::RegisterSpec for MODER_SPEC {
    type Ux = u32;
}
///`read()` method returns [moder::R](R) reader structure
impl crate::Readable for MODER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [moder::W](W) writer structure
impl crate::Writable for MODER_SPEC {
    type Writer = W;
}
///`reset()` method sets MODER to value 0xc0
impl crate::Resettable for MODER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
