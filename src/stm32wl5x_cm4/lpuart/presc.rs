///Register `PRESC` reader
pub struct R(crate::R<PRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PRESC` writer
pub struct W(crate::W<PRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESC_SPEC>;
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
impl From<crate::W<PRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESC_SPEC>) -> Self {
        W(writer)
    }
}
///Clock prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER_A {
    ///0: /1
    DIV1 = 0,
    ///1: /2
    DIV2 = 1,
    ///2: /4
    DIV4 = 2,
    ///3: /6
    DIV6 = 3,
    ///4: /8
    DIV8 = 4,
    ///5: /10
    DIV10 = 5,
    ///6: /12
    DIV12 = 6,
    ///7: /16
    DIV16 = 7,
    ///8: /32
    DIV32 = 8,
    ///9: /64
    DIV64 = 9,
    ///10: /128
    DIV128 = 10,
    ///11: /256
    DIV256 = 11,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
///Field `PRESCALER` reader - Clock prescaler
pub struct PRESCALER_R(crate::FieldReader<u8, PRESCALER_A>);
impl PRESCALER_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRESCALER_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCALER_A> {
        match self.bits {
            0 => Some(PRESCALER_A::DIV1),
            1 => Some(PRESCALER_A::DIV2),
            2 => Some(PRESCALER_A::DIV4),
            3 => Some(PRESCALER_A::DIV6),
            4 => Some(PRESCALER_A::DIV8),
            5 => Some(PRESCALER_A::DIV10),
            6 => Some(PRESCALER_A::DIV12),
            7 => Some(PRESCALER_A::DIV16),
            8 => Some(PRESCALER_A::DIV32),
            9 => Some(PRESCALER_A::DIV64),
            10 => Some(PRESCALER_A::DIV128),
            11 => Some(PRESCALER_A::DIV256),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PRESCALER_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PRESCALER_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PRESCALER_A::DIV4
    }
    ///Checks if the value of the field is `DIV6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PRESCALER_A::DIV6
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PRESCALER_A::DIV8
    }
    ///Checks if the value of the field is `DIV10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        **self == PRESCALER_A::DIV10
    }
    ///Checks if the value of the field is `DIV12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        **self == PRESCALER_A::DIV12
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PRESCALER_A::DIV16
    }
    ///Checks if the value of the field is `DIV32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == PRESCALER_A::DIV32
    }
    ///Checks if the value of the field is `DIV64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == PRESCALER_A::DIV64
    }
    ///Checks if the value of the field is `DIV128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == PRESCALER_A::DIV128
    }
    ///Checks if the value of the field is `DIV256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == PRESCALER_A::DIV256
    }
}
impl core::ops::Deref for PRESCALER_R {
    type Target = crate::FieldReader<u8, PRESCALER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PRESCALER` writer - Clock prescaler
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV1)
    }
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV2)
    }
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV4)
    }
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV6)
    }
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV8)
    }
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV10)
    }
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV12)
    }
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV16)
    }
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV32)
    }
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV64)
    }
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV128)
    }
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV256)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Prescaler register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [presc](index.html) module
pub struct PRESC_SPEC;
impl crate::RegisterSpec for PRESC_SPEC {
    type Ux = u32;
}
///`read()` method returns [presc::R](R) reader structure
impl crate::Readable for PRESC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [presc::W](W) writer structure
impl crate::Writable for PRESC_SPEC {
    type Writer = W;
}
///`reset()` method sets PRESC to value 0
impl crate::Resettable for PRESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
