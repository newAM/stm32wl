///Register `EXTICR1` reader
pub struct R(crate::R<EXTICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTICR1` writer
pub struct W(crate::W<EXTICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR1_SPEC>;
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
impl From<crate::W<EXTICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR1_SPEC>) -> Self {
        W(writer)
    }
}
///EXTI 3 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI3_A {
    ///0: Select PA3 as the source input for the EXTI3 external interrupt
    PA3 = 0,
    ///1: Select PB3 as the source input for the EXTI3 external interrupt
    PB3 = 1,
    ///2: Select PC3 as the source input for the EXTI3 external interrupt
    PC3 = 2,
    ///7: Select PH3 as the source input for the EXTI3 external interrupt
    PH3 = 7,
}
impl From<EXTI3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI3_A) -> Self {
        variant as _
    }
}
///Field `EXTI3` reader - EXTI 3 configuration bits
pub struct EXTI3_R(crate::FieldReader<u8, EXTI3_A>);
impl EXTI3_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI3_A> {
        match self.bits {
            0 => Some(EXTI3_A::PA3),
            1 => Some(EXTI3_A::PB3),
            2 => Some(EXTI3_A::PC3),
            7 => Some(EXTI3_A::PH3),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA3`
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        **self == EXTI3_A::PA3
    }
    ///Checks if the value of the field is `PB3`
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        **self == EXTI3_A::PB3
    }
    ///Checks if the value of the field is `PC3`
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        **self == EXTI3_A::PC3
    }
    ///Checks if the value of the field is `PH3`
    #[inline(always)]
    pub fn is_ph3(&self) -> bool {
        **self == EXTI3_A::PH3
    }
}
impl core::ops::Deref for EXTI3_R {
    type Target = crate::FieldReader<u8, EXTI3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI3` writer - EXTI 3 configuration bits
pub struct EXTI3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pa3(self) -> &'a mut W {
        self.variant(EXTI3_A::PA3)
    }
    ///Select PB3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(EXTI3_A::PB3)
    }
    ///Select PC3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pc3(self) -> &'a mut W {
        self.variant(EXTI3_A::PC3)
    }
    ///Select PH3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn ph3(self) -> &'a mut W {
        self.variant(EXTI3_A::PH3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
///EXTI 2 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI2_A {
    ///0: Select PA2 as the source input for the EXTI2 external interrupt
    PA2 = 0,
    ///1: Select PB2 as the source input for the EXTI2 external interrupt
    PB2 = 1,
    ///2: Select PC2 as the source input for the EXTI2 external interrupt
    PC2 = 2,
}
impl From<EXTI2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI2_A) -> Self {
        variant as _
    }
}
///Field `EXTI2` reader - EXTI 2 configuration bits
pub struct EXTI2_R(crate::FieldReader<u8, EXTI2_A>);
impl EXTI2_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI2_A> {
        match self.bits {
            0 => Some(EXTI2_A::PA2),
            1 => Some(EXTI2_A::PB2),
            2 => Some(EXTI2_A::PC2),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA2`
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        **self == EXTI2_A::PA2
    }
    ///Checks if the value of the field is `PB2`
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        **self == EXTI2_A::PB2
    }
    ///Checks if the value of the field is `PC2`
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        **self == EXTI2_A::PC2
    }
}
impl core::ops::Deref for EXTI2_R {
    type Target = crate::FieldReader<u8, EXTI2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI2` writer - EXTI 2 configuration bits
pub struct EXTI2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(EXTI2_A::PA2)
    }
    ///Select PB2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(EXTI2_A::PB2)
    }
    ///Select PC2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pc2(self) -> &'a mut W {
        self.variant(EXTI2_A::PC2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
///EXTI 1 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI1_A {
    ///0: Select PA1 as the source input for the EXTI1 external interrupt
    PA1 = 0,
    ///1: Select PB1 as the source input for the EXTI1 external interrupt
    PB1 = 1,
    ///2: Select PC1 as the source input for the EXTI1 external interrupt
    PC1 = 2,
}
impl From<EXTI1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI1_A) -> Self {
        variant as _
    }
}
///Field `EXTI1` reader - EXTI 1 configuration bits
pub struct EXTI1_R(crate::FieldReader<u8, EXTI1_A>);
impl EXTI1_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI1_A> {
        match self.bits {
            0 => Some(EXTI1_A::PA1),
            1 => Some(EXTI1_A::PB1),
            2 => Some(EXTI1_A::PC1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA1`
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        **self == EXTI1_A::PA1
    }
    ///Checks if the value of the field is `PB1`
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        **self == EXTI1_A::PB1
    }
    ///Checks if the value of the field is `PC1`
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        **self == EXTI1_A::PC1
    }
}
impl core::ops::Deref for EXTI1_R {
    type Target = crate::FieldReader<u8, EXTI1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI1` writer - EXTI 1 configuration bits
pub struct EXTI1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pa1(self) -> &'a mut W {
        self.variant(EXTI1_A::PA1)
    }
    ///Select PB1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pb1(self) -> &'a mut W {
        self.variant(EXTI1_A::PB1)
    }
    ///Select PC1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pc1(self) -> &'a mut W {
        self.variant(EXTI1_A::PC1)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
///EXTI 0 configuration bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI0_A {
    ///0: Select PA0 as the source input for the EXTI0 external interrupt
    PA0 = 0,
    ///1: Select PB0 as the source input for the EXTI0 external interrupt
    PB0 = 1,
    ///2: Select PC0 as the source input for the EXTI0 external interrupt
    PC0 = 2,
}
impl From<EXTI0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_A) -> Self {
        variant as _
    }
}
///Field `EXTI0` reader - EXTI 0 configuration bits
pub struct EXTI0_R(crate::FieldReader<u8, EXTI0_A>);
impl EXTI0_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI0_A> {
        match self.bits {
            0 => Some(EXTI0_A::PA0),
            1 => Some(EXTI0_A::PB0),
            2 => Some(EXTI0_A::PC0),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA0`
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        **self == EXTI0_A::PA0
    }
    ///Checks if the value of the field is `PB0`
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        **self == EXTI0_A::PB0
    }
    ///Checks if the value of the field is `PC0`
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        **self == EXTI0_A::PC0
    }
}
impl core::ops::Deref for EXTI0_R {
    type Target = crate::FieldReader<u8, EXTI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI0` writer - EXTI 0 configuration bits
pub struct EXTI0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(EXTI0_A::PA0)
    }
    ///Select PB0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(EXTI0_A::PB0)
    }
    ///Select PC0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pc0(self) -> &'a mut W {
        self.variant(EXTI0_A::PC0)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bits 12:14 - EXTI 3 configuration bits
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bits 8:10 - EXTI 2 configuration bits
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bits 4:6 - EXTI 1 configuration bits
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bits 0:2 - EXTI 0 configuration bits
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 12:14 - EXTI 3 configuration bits
    #[inline(always)]
    pub fn exti3(&mut self) -> EXTI3_W {
        EXTI3_W { w: self }
    }
    ///Bits 8:10 - EXTI 2 configuration bits
    #[inline(always)]
    pub fn exti2(&mut self) -> EXTI2_W {
        EXTI2_W { w: self }
    }
    ///Bits 4:6 - EXTI 1 configuration bits
    #[inline(always)]
    pub fn exti1(&mut self) -> EXTI1_W {
        EXTI1_W { w: self }
    }
    ///Bits 0:2 - EXTI 0 configuration bits
    #[inline(always)]
    pub fn exti0(&mut self) -> EXTI0_W {
        EXTI0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///external interrupt configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr1](index.html) module
pub struct EXTICR1_SPEC;
impl crate::RegisterSpec for EXTICR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [exticr1::R](R) reader structure
impl crate::Readable for EXTICR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exticr1::W](W) writer structure
impl crate::Writable for EXTICR1_SPEC {
    type Writer = W;
}
///`reset()` method sets EXTICR1 to value 0
impl crate::Resettable for EXTICR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
