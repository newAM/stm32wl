///Register `CCMR1_Input` reader
pub struct R(crate::R<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR1_Input` writer
pub struct W(crate::W<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_INPUT_SPEC>;
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
impl From<crate::W<CCMR1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
///IC1F
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC1F_A {
    ///0: No filter, sampling is done at fDTS
    NOFILTER = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    FCK_INT_N2 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    FCK_INT_N4 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    FCK_INT_N8 = 3,
    ///4: fSAMPLING=fDTS/2, N=6
    FDTS_DIV2_N6 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    FDTS_DIV2_N8 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    FDTS_DIV4_N6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    FDTS_DIV4_N8 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    FDTS_DIV8_N6 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    FDTS_DIV8_N8 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    FDTS_DIV16_N5 = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    FDTS_DIV16_N6 = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    FDTS_DIV16_N8 = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    FDTS_DIV32_N5 = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    FDTS_DIV32_N6 = 14,
    ///15: fSAMPLING=fDTS/32, N=8
    FDTS_DIV32_N8 = 15,
}
impl From<IC1F_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1F_A) -> Self {
        variant as _
    }
}
///Field `IC1F` reader - IC1F
pub struct IC1F_R(crate::FieldReader<u8, IC1F_A>);
impl IC1F_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC1F_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IC1F_A {
        match self.bits {
            0 => IC1F_A::NOFILTER,
            1 => IC1F_A::FCK_INT_N2,
            2 => IC1F_A::FCK_INT_N4,
            3 => IC1F_A::FCK_INT_N8,
            4 => IC1F_A::FDTS_DIV2_N6,
            5 => IC1F_A::FDTS_DIV2_N8,
            6 => IC1F_A::FDTS_DIV4_N6,
            7 => IC1F_A::FDTS_DIV4_N8,
            8 => IC1F_A::FDTS_DIV8_N6,
            9 => IC1F_A::FDTS_DIV8_N8,
            10 => IC1F_A::FDTS_DIV16_N5,
            11 => IC1F_A::FDTS_DIV16_N6,
            12 => IC1F_A::FDTS_DIV16_N8,
            13 => IC1F_A::FDTS_DIV32_N5,
            14 => IC1F_A::FDTS_DIV32_N6,
            15 => IC1F_A::FDTS_DIV32_N8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOFILTER`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        **self == IC1F_A::NOFILTER
    }
    ///Checks if the value of the field is `FCK_INT_N2`
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        **self == IC1F_A::FCK_INT_N2
    }
    ///Checks if the value of the field is `FCK_INT_N4`
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        **self == IC1F_A::FCK_INT_N4
    }
    ///Checks if the value of the field is `FCK_INT_N8`
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        **self == IC1F_A::FCK_INT_N8
    }
    ///Checks if the value of the field is `FDTS_DIV2_N6`
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        **self == IC1F_A::FDTS_DIV2_N6
    }
    ///Checks if the value of the field is `FDTS_DIV2_N8`
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        **self == IC1F_A::FDTS_DIV2_N8
    }
    ///Checks if the value of the field is `FDTS_DIV4_N6`
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        **self == IC1F_A::FDTS_DIV4_N6
    }
    ///Checks if the value of the field is `FDTS_DIV4_N8`
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        **self == IC1F_A::FDTS_DIV4_N8
    }
    ///Checks if the value of the field is `FDTS_DIV8_N6`
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        **self == IC1F_A::FDTS_DIV8_N6
    }
    ///Checks if the value of the field is `FDTS_DIV8_N8`
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        **self == IC1F_A::FDTS_DIV8_N8
    }
    ///Checks if the value of the field is `FDTS_DIV16_N5`
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        **self == IC1F_A::FDTS_DIV16_N5
    }
    ///Checks if the value of the field is `FDTS_DIV16_N6`
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        **self == IC1F_A::FDTS_DIV16_N6
    }
    ///Checks if the value of the field is `FDTS_DIV16_N8`
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        **self == IC1F_A::FDTS_DIV16_N8
    }
    ///Checks if the value of the field is `FDTS_DIV32_N5`
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        **self == IC1F_A::FDTS_DIV32_N5
    }
    ///Checks if the value of the field is `FDTS_DIV32_N6`
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        **self == IC1F_A::FDTS_DIV32_N6
    }
    ///Checks if the value of the field is `FDTS_DIV32_N8`
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        **self == IC1F_A::FDTS_DIV32_N8
    }
}
impl core::ops::Deref for IC1F_R {
    type Target = crate::FieldReader<u8, IC1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IC1F` writer - IC1F
pub struct IC1F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1F_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IC1F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC1F_A::NOFILTER)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(IC1F_A::FCK_INT_N2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(IC1F_A::FCK_INT_N4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FCK_INT_N8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV2_N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV2_N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV4_N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV4_N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV8_N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV8_N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV16_N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV16_N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV16_N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV32_N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV32_N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV32_N8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///IC1PSC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC1PSC_A {
    ///0: CCx channel is configured as output
    OUTPUT = 0,
    ///1: Capture is done once every 2 events
    CAPTURE_2 = 1,
    ///2: Capture is done once every 4 events
    CAPTURE_4 = 2,
    ///3: Capture is done once every 8 events
    CAPTURE_8 = 3,
}
impl From<IC1PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1PSC_A) -> Self {
        variant as _
    }
}
///Field `IC1PSC` reader - IC1PSC
pub struct IC1PSC_R(crate::FieldReader<u8, IC1PSC_A>);
impl IC1PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC1PSC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IC1PSC_A {
        match self.bits {
            0 => IC1PSC_A::OUTPUT,
            1 => IC1PSC_A::CAPTURE_2,
            2 => IC1PSC_A::CAPTURE_4,
            3 => IC1PSC_A::CAPTURE_8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `OUTPUT`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == IC1PSC_A::OUTPUT
    }
    ///Checks if the value of the field is `CAPTURE_2`
    #[inline(always)]
    pub fn is_capture_2(&self) -> bool {
        **self == IC1PSC_A::CAPTURE_2
    }
    ///Checks if the value of the field is `CAPTURE_4`
    #[inline(always)]
    pub fn is_capture_4(&self) -> bool {
        **self == IC1PSC_A::CAPTURE_4
    }
    ///Checks if the value of the field is `CAPTURE_8`
    #[inline(always)]
    pub fn is_capture_8(&self) -> bool {
        **self == IC1PSC_A::CAPTURE_8
    }
}
impl core::ops::Deref for IC1PSC_R {
    type Target = crate::FieldReader<u8, IC1PSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IC1PSC` writer - IC1PSC
pub struct IC1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1PSC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IC1PSC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(IC1PSC_A::OUTPUT)
    }
    ///Capture is done once every 2 events
    #[inline(always)]
    pub fn capture_2(self) -> &'a mut W {
        self.variant(IC1PSC_A::CAPTURE_2)
    }
    ///Capture is done once every 4 events
    #[inline(always)]
    pub fn capture_4(self) -> &'a mut W {
        self.variant(IC1PSC_A::CAPTURE_4)
    }
    ///Capture is done once every 8 events
    #[inline(always)]
    pub fn capture_8(self) -> &'a mut W {
        self.variant(IC1PSC_A::CAPTURE_8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///CC1S
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC1S_A {
    ///0: CCx channel is configured as output
    OUTPUT = 0,
    ///1: CCx channel is configured as input, ICx is mapped on TI1
    TI1 = 1,
    ///2: CCx channel is configured as input, ICx is mapped on TI2
    TI2 = 2,
    ///3: CCx channel is configured as input, ICx is mapped on TRC
    TRC = 3,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
///Field `CC1S` reader - CC1S
pub struct CC1S_R(crate::FieldReader<u8, CC1S_A>);
impl CC1S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC1S_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1S_A {
        match self.bits {
            0 => CC1S_A::OUTPUT,
            1 => CC1S_A::TI1,
            2 => CC1S_A::TI2,
            3 => CC1S_A::TRC,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `OUTPUT`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == CC1S_A::OUTPUT
    }
    ///Checks if the value of the field is `TI1`
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        **self == CC1S_A::TI1
    }
    ///Checks if the value of the field is `TI2`
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        **self == CC1S_A::TI2
    }
    ///Checks if the value of the field is `TRC`
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        **self == CC1S_A::TRC
    }
}
impl core::ops::Deref for CC1S_R {
    type Target = crate::FieldReader<u8, CC1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1S` writer - CC1S
pub struct CC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1S_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC1S_A::OUTPUT)
    }
    ///CCx channel is configured as input, ICx is mapped on TI1
    #[inline(always)]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC1S_A::TI1)
    }
    ///CCx channel is configured as input, ICx is mapped on TI2
    #[inline(always)]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC1S_A::TI2)
    }
    ///CCx channel is configured as input, ICx is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC1S_A::TRC)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 4:7 - IC1F
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 2:3 - IC1PSC
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - CC1S
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 4:7 - IC1F
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W {
        IC1F_W { w: self }
    }
    ///Bits 2:3 - IC1PSC
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W {
        IC1PSC_W { w: self }
    }
    ///Bits 0:1 - CC1S
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM16/TIM17 capture/compare mode register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1_input](index.html) module
pub struct CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for CCMR1_INPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr1_input::R](R) reader structure
impl crate::Readable for CCMR1_INPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr1_input::W](W) writer structure
impl crate::Writable for CCMR1_INPUT_SPEC {
    type Writer = W;
}
///`reset()` method sets CCMR1_Input to value 0
impl crate::Resettable for CCMR1_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
