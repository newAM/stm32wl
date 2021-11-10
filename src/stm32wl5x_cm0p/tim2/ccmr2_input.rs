///Register `CCMR2_Input` reader
pub struct R(crate::R<CCMR2_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR2_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR2_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR2_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR2_Input` writer
pub struct W(crate::W<CCMR2_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR2_INPUT_SPEC>;
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
impl From<crate::W<CCMR2_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR2_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Input capture 4 filter
pub type IC4F_A = IC3F_A;
///Field `IC4F` reader - Input capture 4 filter
pub type IC4F_R = IC3F_R;
///Field `IC4F` writer - Input capture 4 filter
pub struct IC4F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4F_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IC4F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC4F_A::NOFILTER)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(IC4F_A::FCK_INT_N2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(IC4F_A::FCK_INT_N4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(IC4F_A::FCK_INT_N8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV2_N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV2_N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV4_N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV4_N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV8_N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV8_N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV16_N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV16_N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV16_N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV32_N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV32_N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(IC4F_A::FDTS_DIV32_N8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
///Input capture 4 prescaler
pub type IC4PSC_A = IC3PSC_A;
///Field `IC4PSC` reader - Input capture 4 prescaler
pub type IC4PSC_R = IC3PSC_R;
///Field `IC4PSC` writer - Input capture 4 prescaler
pub struct IC4PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4PSC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IC4PSC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(IC4PSC_A::OUTPUT)
    }
    ///Capture is done once every 2 events
    #[inline(always)]
    pub fn capture2(self) -> &'a mut W {
        self.variant(IC4PSC_A::CAPTURE2)
    }
    ///Capture is done once every 4 events
    #[inline(always)]
    pub fn capture4(self) -> &'a mut W {
        self.variant(IC4PSC_A::CAPTURE4)
    }
    ///Capture is done once every 8 events
    #[inline(always)]
    pub fn capture8(self) -> &'a mut W {
        self.variant(IC4PSC_A::CAPTURE8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///Capture/Compare 4 selection
pub type CC4S_A = CC3S_A;
///Field `CC4S` reader - Capture/Compare 4 selection
pub type CC4S_R = CC3S_R;
///Field `CC4S` writer - Capture/Compare 4 selection
pub struct CC4S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC4S_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC4S_A::OUTPUT)
    }
    ///CCx channel is configured as input, ICx is mapped on TI1
    #[inline(always)]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC4S_A::TI1)
    }
    ///CCx channel is configured as input, ICx is mapped on TI2
    #[inline(always)]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC4S_A::TI2)
    }
    ///CCx channel is configured as input, ICx is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC4S_A::TRC)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///Input capture 3 filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC3F_A {
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
impl From<IC3F_A> for u8 {
    #[inline(always)]
    fn from(variant: IC3F_A) -> Self {
        variant as _
    }
}
///Field `IC3F` reader - Input capture 3 filter
pub struct IC3F_R(crate::FieldReader<u8, IC3F_A>);
impl IC3F_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC3F_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IC3F_A {
        match self.bits {
            0 => IC3F_A::NOFILTER,
            1 => IC3F_A::FCK_INT_N2,
            2 => IC3F_A::FCK_INT_N4,
            3 => IC3F_A::FCK_INT_N8,
            4 => IC3F_A::FDTS_DIV2_N6,
            5 => IC3F_A::FDTS_DIV2_N8,
            6 => IC3F_A::FDTS_DIV4_N6,
            7 => IC3F_A::FDTS_DIV4_N8,
            8 => IC3F_A::FDTS_DIV8_N6,
            9 => IC3F_A::FDTS_DIV8_N8,
            10 => IC3F_A::FDTS_DIV16_N5,
            11 => IC3F_A::FDTS_DIV16_N6,
            12 => IC3F_A::FDTS_DIV16_N8,
            13 => IC3F_A::FDTS_DIV32_N5,
            14 => IC3F_A::FDTS_DIV32_N6,
            15 => IC3F_A::FDTS_DIV32_N8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOFILTER`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        **self == IC3F_A::NOFILTER
    }
    ///Checks if the value of the field is `FCK_INT_N2`
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        **self == IC3F_A::FCK_INT_N2
    }
    ///Checks if the value of the field is `FCK_INT_N4`
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        **self == IC3F_A::FCK_INT_N4
    }
    ///Checks if the value of the field is `FCK_INT_N8`
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        **self == IC3F_A::FCK_INT_N8
    }
    ///Checks if the value of the field is `FDTS_DIV2_N6`
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        **self == IC3F_A::FDTS_DIV2_N6
    }
    ///Checks if the value of the field is `FDTS_DIV2_N8`
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        **self == IC3F_A::FDTS_DIV2_N8
    }
    ///Checks if the value of the field is `FDTS_DIV4_N6`
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        **self == IC3F_A::FDTS_DIV4_N6
    }
    ///Checks if the value of the field is `FDTS_DIV4_N8`
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        **self == IC3F_A::FDTS_DIV4_N8
    }
    ///Checks if the value of the field is `FDTS_DIV8_N6`
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        **self == IC3F_A::FDTS_DIV8_N6
    }
    ///Checks if the value of the field is `FDTS_DIV8_N8`
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        **self == IC3F_A::FDTS_DIV8_N8
    }
    ///Checks if the value of the field is `FDTS_DIV16_N5`
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        **self == IC3F_A::FDTS_DIV16_N5
    }
    ///Checks if the value of the field is `FDTS_DIV16_N6`
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        **self == IC3F_A::FDTS_DIV16_N6
    }
    ///Checks if the value of the field is `FDTS_DIV16_N8`
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        **self == IC3F_A::FDTS_DIV16_N8
    }
    ///Checks if the value of the field is `FDTS_DIV32_N5`
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        **self == IC3F_A::FDTS_DIV32_N5
    }
    ///Checks if the value of the field is `FDTS_DIV32_N6`
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        **self == IC3F_A::FDTS_DIV32_N6
    }
    ///Checks if the value of the field is `FDTS_DIV32_N8`
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        **self == IC3F_A::FDTS_DIV32_N8
    }
}
impl core::ops::Deref for IC3F_R {
    type Target = crate::FieldReader<u8, IC3F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IC3F` writer - Input capture 3 filter
pub struct IC3F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3F_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IC3F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC3F_A::NOFILTER)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(IC3F_A::FCK_INT_N2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(IC3F_A::FCK_INT_N4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FCK_INT_N8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV2_N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV2_N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV4_N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV4_N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV8_N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV8_N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV16_N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV16_N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV16_N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV32_N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV32_N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FDTS_DIV32_N8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///Input capture 3 prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC3PSC_A {
    ///0: CCx channel is configured as output
    OUTPUT = 0,
    ///1: Capture is done once every 2 events
    CAPTURE2 = 1,
    ///2: Capture is done once every 4 events
    CAPTURE4 = 2,
    ///3: Capture is done once every 8 events
    CAPTURE8 = 3,
}
impl From<IC3PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: IC3PSC_A) -> Self {
        variant as _
    }
}
///Field `IC3PSC` reader - Input capture 3 prescaler
pub struct IC3PSC_R(crate::FieldReader<u8, IC3PSC_A>);
impl IC3PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC3PSC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IC3PSC_A {
        match self.bits {
            0 => IC3PSC_A::OUTPUT,
            1 => IC3PSC_A::CAPTURE2,
            2 => IC3PSC_A::CAPTURE4,
            3 => IC3PSC_A::CAPTURE8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `OUTPUT`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == IC3PSC_A::OUTPUT
    }
    ///Checks if the value of the field is `CAPTURE2`
    #[inline(always)]
    pub fn is_capture2(&self) -> bool {
        **self == IC3PSC_A::CAPTURE2
    }
    ///Checks if the value of the field is `CAPTURE4`
    #[inline(always)]
    pub fn is_capture4(&self) -> bool {
        **self == IC3PSC_A::CAPTURE4
    }
    ///Checks if the value of the field is `CAPTURE8`
    #[inline(always)]
    pub fn is_capture8(&self) -> bool {
        **self == IC3PSC_A::CAPTURE8
    }
}
impl core::ops::Deref for IC3PSC_R {
    type Target = crate::FieldReader<u8, IC3PSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IC3PSC` writer - Input capture 3 prescaler
pub struct IC3PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3PSC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IC3PSC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(IC3PSC_A::OUTPUT)
    }
    ///Capture is done once every 2 events
    #[inline(always)]
    pub fn capture2(self) -> &'a mut W {
        self.variant(IC3PSC_A::CAPTURE2)
    }
    ///Capture is done once every 4 events
    #[inline(always)]
    pub fn capture4(self) -> &'a mut W {
        self.variant(IC3PSC_A::CAPTURE4)
    }
    ///Capture is done once every 8 events
    #[inline(always)]
    pub fn capture8(self) -> &'a mut W {
        self.variant(IC3PSC_A::CAPTURE8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Capture/Compare 3 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC3S_A {
    ///0: CCx channel is configured as output
    OUTPUT = 0,
    ///1: CCx channel is configured as input, ICx is mapped on TI1
    TI1 = 1,
    ///2: CCx channel is configured as input, ICx is mapped on TI2
    TI2 = 2,
    ///3: CCx channel is configured as input, ICx is mapped on TRC
    TRC = 3,
}
impl From<CC3S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC3S_A) -> Self {
        variant as _
    }
}
///Field `CC3S` reader - Capture/Compare 3 selection
pub struct CC3S_R(crate::FieldReader<u8, CC3S_A>);
impl CC3S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC3S_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC3S_A {
        match self.bits {
            0 => CC3S_A::OUTPUT,
            1 => CC3S_A::TI1,
            2 => CC3S_A::TI2,
            3 => CC3S_A::TRC,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `OUTPUT`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == CC3S_A::OUTPUT
    }
    ///Checks if the value of the field is `TI1`
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        **self == CC3S_A::TI1
    }
    ///Checks if the value of the field is `TI2`
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        **self == CC3S_A::TI2
    }
    ///Checks if the value of the field is `TRC`
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        **self == CC3S_A::TRC
    }
}
impl core::ops::Deref for CC3S_R {
    type Target = crate::FieldReader<u8, CC3S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC3S` writer - Capture/Compare 3 selection
pub struct CC3S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC3S_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC3S_A::OUTPUT)
    }
    ///CCx channel is configured as input, ICx is mapped on TI1
    #[inline(always)]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC3S_A::TI1)
    }
    ///CCx channel is configured as input, ICx is mapped on TI2
    #[inline(always)]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC3S_A::TI2)
    }
    ///CCx channel is configured as input, ICx is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC3S_A::TRC)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W {
        IC4F_W { w: self }
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W {
        IC4PSC_W { w: self }
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W {
        CC4S_W { w: self }
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W {
        IC3F_W { w: self }
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W {
        IC3PSC_W { w: self }
    }
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W {
        CC3S_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register 2 (input mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr2_input](index.html) module
pub struct CCMR2_INPUT_SPEC;
impl crate::RegisterSpec for CCMR2_INPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr2_input::R](R) reader structure
impl crate::Readable for CCMR2_INPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr2_input::W](W) writer structure
impl crate::Writable for CCMR2_INPUT_SPEC {
    type Writer = W;
}
///`reset()` method sets CCMR2_Input to value 0
impl crate::Resettable for CCMR2_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
