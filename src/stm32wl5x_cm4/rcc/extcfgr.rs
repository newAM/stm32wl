///Register `EXTCFGR` reader
pub struct R(crate::R<EXTCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTCFGR` writer
pub struct W(crate::W<EXTCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTCFGR_SPEC>;
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
impl From<crate::W<EXTCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `C2HPREF` reader - CLK2 prescaler flag (CPU2)
pub struct C2HPREF_R(crate::FieldReader<bool, bool>);
impl C2HPREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2HPREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2HPREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHDHPREF_A {
    ///0: HCLK3 prescaler value not yet applied
    NOTAPPLIED = 0,
    ///1: HCLK3 prescaler value applied
    APPLIED = 1,
}
impl From<SHDHPREF_A> for bool {
    #[inline(always)]
    fn from(variant: SHDHPREF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SHDHPREF` reader - HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)
pub struct SHDHPREF_R(crate::FieldReader<bool, SHDHPREF_A>);
impl SHDHPREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHDHPREF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SHDHPREF_A {
        match self.bits {
            false => SHDHPREF_A::NOTAPPLIED,
            true => SHDHPREF_A::APPLIED,
        }
    }
    ///Checks if the value of the field is `NOTAPPLIED`
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        **self == SHDHPREF_A::NOTAPPLIED
    }
    ///Checks if the value of the field is `APPLIED`
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        **self == SHDHPREF_A::APPLIED
    }
}
impl core::ops::Deref for SHDHPREF_R {
    type Target = crate::FieldReader<bool, SHDHPREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `C2HPRE` reader - dual core device only\]
///HCLK2 prescaler (CPU2)
pub struct C2HPRE_R(crate::FieldReader<u8, u8>);
impl C2HPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        C2HPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2HPRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `C2HPRE` writer - dual core device only\]
///HCLK2 prescaler (CPU2)
pub struct C2HPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> C2HPRE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHDHPRE_A {
    ///0: SYSCLK not divided
    DIV1 = 0,
    ///1: SYSCLK divided by 3
    DIV3 = 1,
    ///2: SYSCLK divided by 5
    DIV5 = 2,
    ///5: SYSCLK divided by 6
    DIV6 = 5,
    ///6: SYSCLK divided by 10
    DIV10 = 6,
    ///7: SYSCLK divided by 32
    DIV32 = 7,
    ///8: SYSCLK divided by 2
    DIV2 = 8,
    ///9: SYSCLK divided by 4
    DIV4 = 9,
    ///10: SYSCLK divided by 8
    DIV8 = 10,
    ///11: SYSCLK divided by 16
    DIV16 = 11,
    ///12: SYSCLK divided by 64
    DIV64 = 12,
    ///13: SYSCLK divided by 128
    DIV128 = 13,
    ///14: SYSCLK divided by 128
    DIV256 = 14,
    ///15: SYSCLK divided by 512
    DIV512 = 15,
}
impl From<SHDHPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: SHDHPRE_A) -> Self {
        variant as _
    }
}
///Field `SHDHPRE` reader - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
pub struct SHDHPRE_R(crate::FieldReader<u8, SHDHPRE_A>);
impl SHDHPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SHDHPRE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SHDHPRE_A> {
        match self.bits {
            0 => Some(SHDHPRE_A::DIV1),
            1 => Some(SHDHPRE_A::DIV3),
            2 => Some(SHDHPRE_A::DIV5),
            5 => Some(SHDHPRE_A::DIV6),
            6 => Some(SHDHPRE_A::DIV10),
            7 => Some(SHDHPRE_A::DIV32),
            8 => Some(SHDHPRE_A::DIV2),
            9 => Some(SHDHPRE_A::DIV4),
            10 => Some(SHDHPRE_A::DIV8),
            11 => Some(SHDHPRE_A::DIV16),
            12 => Some(SHDHPRE_A::DIV64),
            13 => Some(SHDHPRE_A::DIV128),
            14 => Some(SHDHPRE_A::DIV256),
            15 => Some(SHDHPRE_A::DIV512),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == SHDHPRE_A::DIV1
    }
    ///Checks if the value of the field is `DIV3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        **self == SHDHPRE_A::DIV3
    }
    ///Checks if the value of the field is `DIV5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        **self == SHDHPRE_A::DIV5
    }
    ///Checks if the value of the field is `DIV6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == SHDHPRE_A::DIV6
    }
    ///Checks if the value of the field is `DIV10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        **self == SHDHPRE_A::DIV10
    }
    ///Checks if the value of the field is `DIV32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == SHDHPRE_A::DIV32
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == SHDHPRE_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == SHDHPRE_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == SHDHPRE_A::DIV8
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == SHDHPRE_A::DIV16
    }
    ///Checks if the value of the field is `DIV64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == SHDHPRE_A::DIV64
    }
    ///Checks if the value of the field is `DIV128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == SHDHPRE_A::DIV128
    }
    ///Checks if the value of the field is `DIV256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == SHDHPRE_A::DIV256
    }
    ///Checks if the value of the field is `DIV512`
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        **self == SHDHPRE_A::DIV512
    }
}
impl core::ops::Deref for SHDHPRE_R {
    type Target = crate::FieldReader<u8, SHDHPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SHDHPRE` writer - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
pub struct SHDHPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHDHPRE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SHDHPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///SYSCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV1)
    }
    ///SYSCLK divided by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV3)
    }
    ///SYSCLK divided by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV5)
    }
    ///SYSCLK divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV6)
    }
    ///SYSCLK divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV10)
    }
    ///SYSCLK divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV32)
    }
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV2)
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV4)
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV8)
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV16)
    }
    ///SYSCLK divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV64)
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV128)
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV256)
    }
    ///SYSCLK divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(SHDHPRE_A::DIV512)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bit 17 - CLK2 prescaler flag (CPU2)
    #[inline(always)]
    pub fn c2hpref(&self) -> C2HPREF_R {
        C2HPREF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)
    #[inline(always)]
    pub fn shdhpref(&self) -> SHDHPREF_R {
        SHDHPREF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 4:7 - dual core device only\]
    ///HCLK2 prescaler (CPU2)
    #[inline(always)]
    pub fn c2hpre(&self) -> C2HPRE_R {
        C2HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 0:3 - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
    #[inline(always)]
    pub fn shdhpre(&self) -> SHDHPRE_R {
        SHDHPRE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 4:7 - dual core device only\]
    ///HCLK2 prescaler (CPU2)
    #[inline(always)]
    pub fn c2hpre(&mut self) -> C2HPRE_W {
        C2HPRE_W { w: self }
    }
    ///Bits 0:3 - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
    #[inline(always)]
    pub fn shdhpre(&mut self) -> SHDHPRE_W {
        SHDHPRE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Extended clock recovery register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [extcfgr](index.html) module
pub struct EXTCFGR_SPEC;
impl crate::RegisterSpec for EXTCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [extcfgr::R](R) reader structure
impl crate::Readable for EXTCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [extcfgr::W](W) writer structure
impl crate::Writable for EXTCFGR_SPEC {
    type Writer = W;
}
///`reset()` method sets EXTCFGR to value 0x0003_0000
impl crate::Resettable for EXTCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0000
    }
}
