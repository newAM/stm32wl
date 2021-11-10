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
///MODER15
pub type MODER15_A = MODER0_A;
///Field `MODER15` reader - MODER15
pub type MODER15_R = MODER0_R;
///Field `MODER15` writer - MODER15
pub struct MODER15_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER15_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER15_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER15_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER15_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
///MODER14
pub type MODER14_A = MODER0_A;
///Field `MODER14` reader - MODER14
pub type MODER14_R = MODER0_R;
///Field `MODER14` writer - MODER14
pub struct MODER14_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER14_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER14_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER14_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER14_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
///MODER13
pub type MODER13_A = MODER0_A;
///Field `MODER13` reader - MODER13
pub type MODER13_R = MODER0_R;
///Field `MODER13` writer - MODER13
pub struct MODER13_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER13_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER13_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER13_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER13_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
///MODER12
pub type MODER12_A = MODER0_A;
///Field `MODER12` reader - MODER12
pub type MODER12_R = MODER0_R;
///Field `MODER12` writer - MODER12
pub struct MODER12_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER12_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER12_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER12_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER12_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
///MODER11
pub type MODER11_A = MODER0_A;
///Field `MODER11` reader - MODER11
pub type MODER11_R = MODER0_R;
///Field `MODER11` writer - MODER11
pub struct MODER11_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER11_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER11_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER11_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER11_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
///MODER10
pub type MODER10_A = MODER0_A;
///Field `MODER10` reader - MODER10
pub type MODER10_R = MODER0_R;
///Field `MODER10` writer - MODER10
pub struct MODER10_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER10_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER10_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER10_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER10_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
///MODER9
pub type MODER9_A = MODER0_A;
///Field `MODER9` reader - MODER9
pub type MODER9_R = MODER0_R;
///Field `MODER9` writer - MODER9
pub struct MODER9_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER9_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER9_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER9_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER9_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
///MODER8
pub type MODER8_A = MODER0_A;
///Field `MODER8` reader - MODER8
pub type MODER8_R = MODER0_R;
///Field `MODER8` writer - MODER8
pub struct MODER8_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER8_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER8_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER8_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER8_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
///MODER7
pub type MODER7_A = MODER0_A;
///Field `MODER7` reader - MODER7
pub type MODER7_R = MODER0_R;
///Field `MODER7` writer - MODER7
pub struct MODER7_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER7_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER7_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER7_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER7_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
///MODER6
pub type MODER6_A = MODER0_A;
///Field `MODER6` reader - MODER6
pub type MODER6_R = MODER0_R;
///Field `MODER6` writer - MODER6
pub struct MODER6_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER6_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER6_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER6_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER6_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
///MODER5
pub type MODER5_A = MODER0_A;
///Field `MODER5` reader - MODER5
pub type MODER5_R = MODER0_R;
///Field `MODER5` writer - MODER5
pub struct MODER5_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER5_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER5_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER5_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER5_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///MODER4
pub type MODER4_A = MODER0_A;
///Field `MODER4` reader - MODER4
pub type MODER4_R = MODER0_R;
///Field `MODER4` writer - MODER4
pub struct MODER4_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER4_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER4_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER4_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER4_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///MODER3
pub type MODER3_A = MODER0_A;
///Field `MODER3` reader - MODER3
pub type MODER3_R = MODER0_R;
///Field `MODER3` writer - MODER3
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
///MODER2
pub type MODER2_A = MODER0_A;
///Field `MODER2` reader - MODER2
pub type MODER2_R = MODER0_R;
///Field `MODER2` writer - MODER2
pub struct MODER2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER2_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER2_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER2_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER2_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
///MODER1
pub type MODER1_A = MODER0_A;
///Field `MODER1` reader - MODER1
pub type MODER1_R = MODER0_R;
///Field `MODER1` writer - MODER1
pub struct MODER1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER1_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER1_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER1_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER1_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///MODER0
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODER0_A {
    ///0: Input mode (reset state)
    INPUT = 0,
    ///1: General purpose output mode
    OUTPUT = 1,
    ///2: Alternate function mode
    ALTERNATE = 2,
    ///3: Analog mode
    ANALOG = 3,
}
impl From<MODER0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODER0_A) -> Self {
        variant as _
    }
}
///Field `MODER0` reader - MODER0
pub struct MODER0_R(crate::FieldReader<u8, MODER0_A>);
impl MODER0_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODER0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODER0_A {
        match self.bits {
            0 => MODER0_A::INPUT,
            1 => MODER0_A::OUTPUT,
            2 => MODER0_A::ALTERNATE,
            3 => MODER0_A::ANALOG,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `INPUT`
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == MODER0_A::INPUT
    }
    ///Checks if the value of the field is `OUTPUT`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == MODER0_A::OUTPUT
    }
    ///Checks if the value of the field is `ALTERNATE`
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        **self == MODER0_A::ALTERNATE
    }
    ///Checks if the value of the field is `ANALOG`
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        **self == MODER0_A::ANALOG
    }
}
impl core::ops::Deref for MODER0_R {
    type Target = crate::FieldReader<u8, MODER0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MODER0` writer - MODER0
pub struct MODER0_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODER0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODER0_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODER0_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODER0_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODER0_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 30:31 - MODER15
    #[inline(always)]
    pub fn moder15(&self) -> MODER15_R {
        MODER15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    ///Bits 28:29 - MODER14
    #[inline(always)]
    pub fn moder14(&self) -> MODER14_R {
        MODER14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 26:27 - MODER13
    #[inline(always)]
    pub fn moder13(&self) -> MODER13_R {
        MODER13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    ///Bits 24:25 - MODER12
    #[inline(always)]
    pub fn moder12(&self) -> MODER12_R {
        MODER12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    ///Bits 22:23 - MODER11
    #[inline(always)]
    pub fn moder11(&self) -> MODER11_R {
        MODER11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    ///Bits 20:21 - MODER10
    #[inline(always)]
    pub fn moder10(&self) -> MODER10_R {
        MODER10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bits 18:19 - MODER9
    #[inline(always)]
    pub fn moder9(&self) -> MODER9_R {
        MODER9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bits 16:17 - MODER8
    #[inline(always)]
    pub fn moder8(&self) -> MODER8_R {
        MODER8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 14:15 - MODER7
    #[inline(always)]
    pub fn moder7(&self) -> MODER7_R {
        MODER7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bits 12:13 - MODER6
    #[inline(always)]
    pub fn moder6(&self) -> MODER6_R {
        MODER6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bits 10:11 - MODER5
    #[inline(always)]
    pub fn moder5(&self) -> MODER5_R {
        MODER5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 8:9 - MODER4
    #[inline(always)]
    pub fn moder4(&self) -> MODER4_R {
        MODER4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 6:7 - MODER3
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bits 4:5 - MODER2
    #[inline(always)]
    pub fn moder2(&self) -> MODER2_R {
        MODER2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bits 2:3 - MODER1
    #[inline(always)]
    pub fn moder1(&self) -> MODER1_R {
        MODER1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - MODER0
    #[inline(always)]
    pub fn moder0(&self) -> MODER0_R {
        MODER0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 30:31 - MODER15
    #[inline(always)]
    pub fn moder15(&mut self) -> MODER15_W {
        MODER15_W { w: self }
    }
    ///Bits 28:29 - MODER14
    #[inline(always)]
    pub fn moder14(&mut self) -> MODER14_W {
        MODER14_W { w: self }
    }
    ///Bits 26:27 - MODER13
    #[inline(always)]
    pub fn moder13(&mut self) -> MODER13_W {
        MODER13_W { w: self }
    }
    ///Bits 24:25 - MODER12
    #[inline(always)]
    pub fn moder12(&mut self) -> MODER12_W {
        MODER12_W { w: self }
    }
    ///Bits 22:23 - MODER11
    #[inline(always)]
    pub fn moder11(&mut self) -> MODER11_W {
        MODER11_W { w: self }
    }
    ///Bits 20:21 - MODER10
    #[inline(always)]
    pub fn moder10(&mut self) -> MODER10_W {
        MODER10_W { w: self }
    }
    ///Bits 18:19 - MODER9
    #[inline(always)]
    pub fn moder9(&mut self) -> MODER9_W {
        MODER9_W { w: self }
    }
    ///Bits 16:17 - MODER8
    #[inline(always)]
    pub fn moder8(&mut self) -> MODER8_W {
        MODER8_W { w: self }
    }
    ///Bits 14:15 - MODER7
    #[inline(always)]
    pub fn moder7(&mut self) -> MODER7_W {
        MODER7_W { w: self }
    }
    ///Bits 12:13 - MODER6
    #[inline(always)]
    pub fn moder6(&mut self) -> MODER6_W {
        MODER6_W { w: self }
    }
    ///Bits 10:11 - MODER5
    #[inline(always)]
    pub fn moder5(&mut self) -> MODER5_W {
        MODER5_W { w: self }
    }
    ///Bits 8:9 - MODER4
    #[inline(always)]
    pub fn moder4(&mut self) -> MODER4_W {
        MODER4_W { w: self }
    }
    ///Bits 6:7 - MODER3
    #[inline(always)]
    pub fn moder3(&mut self) -> MODER3_W {
        MODER3_W { w: self }
    }
    ///Bits 4:5 - MODER2
    #[inline(always)]
    pub fn moder2(&mut self) -> MODER2_W {
        MODER2_W { w: self }
    }
    ///Bits 2:3 - MODER1
    #[inline(always)]
    pub fn moder1(&mut self) -> MODER1_W {
        MODER1_W { w: self }
    }
    ///Bits 0:1 - MODER0
    #[inline(always)]
    pub fn moder0(&mut self) -> MODER0_W {
        MODER0_W { w: self }
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
///`reset()` method sets MODER to value 0xabff_ffff
impl crate::Resettable for MODER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xabff_ffff
    }
}
