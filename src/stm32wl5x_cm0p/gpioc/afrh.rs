///Register `AFRH` reader
pub struct R(crate::R<AFRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRH_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AFRH` writer
pub struct W(crate::W<AFRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRH_SPEC>;
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
impl From<crate::W<AFRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRH_SPEC>) -> Self {
        W(writer)
    }
}
///Alternate function selection for port x bit y (y = 8..15)
pub type AFRH15_A = AFRH8_A;
///Field `AFRH15` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFRH15_R = AFRH8_R;
///Field `AFRH15` writer - Alternate function selection for port x bit y (y = 8..15)
pub struct AFRH15_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRH15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRH15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH15_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH15_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH15_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH15_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH15_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH15_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH15_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH15_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRH15_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRH15_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRH15_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRH15_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRH15_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRH15_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRH15_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRH15_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 8..15)
pub type AFRH14_A = AFRH8_A;
///Field `AFRH14` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFRH14_R = AFRH8_R;
///Field `AFRH14` writer - Alternate function selection for port x bit y (y = 8..15)
pub struct AFRH14_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRH14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRH14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH14_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH14_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH14_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH14_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH14_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH14_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH14_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH14_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRH14_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRH14_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRH14_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRH14_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRH14_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRH14_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRH14_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRH14_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 8..15)
pub type AFRH13_A = AFRH8_A;
///Field `AFRH13` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFRH13_R = AFRH8_R;
///Field `AFRH13` writer - Alternate function selection for port x bit y (y = 8..15)
pub struct AFRH13_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRH13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRH13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH13_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH13_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH13_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH13_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH13_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH13_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH13_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH13_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRH13_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRH13_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRH13_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRH13_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRH13_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRH13_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRH13_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRH13_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 8..15)
pub type AFRH12_A = AFRH8_A;
///Field `AFRH12` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFRH12_R = AFRH8_R;
///Field `AFRH12` writer - Alternate function selection for port x bit y (y = 8..15)
pub struct AFRH12_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRH12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRH12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH12_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH12_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH12_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH12_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH12_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH12_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH12_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH12_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRH12_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRH12_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRH12_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRH12_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRH12_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRH12_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRH12_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRH12_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 8..15)
pub type AFRH11_A = AFRH8_A;
///Field `AFRH11` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFRH11_R = AFRH8_R;
///Field `AFRH11` writer - Alternate function selection for port x bit y (y = 8..15)
pub struct AFRH11_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRH11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRH11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH11_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH11_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH11_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH11_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH11_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH11_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH11_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH11_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRH11_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRH11_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRH11_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRH11_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRH11_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRH11_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRH11_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRH11_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 8..15)
pub type AFRH10_A = AFRH8_A;
///Field `AFRH10` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFRH10_R = AFRH8_R;
///Field `AFRH10` writer - Alternate function selection for port x bit y (y = 8..15)
pub struct AFRH10_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRH10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRH10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH10_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH10_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH10_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH10_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH10_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH10_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH10_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH10_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRH10_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRH10_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRH10_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRH10_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRH10_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRH10_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRH10_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRH10_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 8..15)
pub type AFRH9_A = AFRH8_A;
///Field `AFRH9` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFRH9_R = AFRH8_R;
///Field `AFRH9` writer - Alternate function selection for port x bit y (y = 8..15)
pub struct AFRH9_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRH9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRH9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH9_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH9_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH9_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH9_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH9_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH9_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH9_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH9_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRH9_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRH9_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRH9_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRH9_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRH9_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRH9_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRH9_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRH9_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 8..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AFRH8_A {
    ///0: AF0
    AF0 = 0,
    ///1: AF1
    AF1 = 1,
    ///2: AF2
    AF2 = 2,
    ///3: AF3
    AF3 = 3,
    ///4: AF4
    AF4 = 4,
    ///5: AF5
    AF5 = 5,
    ///6: AF6
    AF6 = 6,
    ///7: AF7
    AF7 = 7,
    ///8: AF8
    AF8 = 8,
    ///9: AF9
    AF9 = 9,
    ///10: AF10
    AF10 = 10,
    ///11: AF11
    AF11 = 11,
    ///12: AF12
    AF12 = 12,
    ///13: AF13
    AF13 = 13,
    ///14: AF14
    AF14 = 14,
    ///15: AF15
    AF15 = 15,
}
impl From<AFRH8_A> for u8 {
    #[inline(always)]
    fn from(variant: AFRH8_A) -> Self {
        variant as _
    }
}
///Field `AFRH8` reader - Alternate function selection for port x bit y (y = 8..15)
pub struct AFRH8_R(crate::FieldReader<u8, AFRH8_A>);
impl AFRH8_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFRH8_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AFRH8_A {
        match self.bits {
            0 => AFRH8_A::AF0,
            1 => AFRH8_A::AF1,
            2 => AFRH8_A::AF2,
            3 => AFRH8_A::AF3,
            4 => AFRH8_A::AF4,
            5 => AFRH8_A::AF5,
            6 => AFRH8_A::AF6,
            7 => AFRH8_A::AF7,
            8 => AFRH8_A::AF8,
            9 => AFRH8_A::AF9,
            10 => AFRH8_A::AF10,
            11 => AFRH8_A::AF11,
            12 => AFRH8_A::AF12,
            13 => AFRH8_A::AF13,
            14 => AFRH8_A::AF14,
            15 => AFRH8_A::AF15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `AF0`
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        **self == AFRH8_A::AF0
    }
    ///Checks if the value of the field is `AF1`
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        **self == AFRH8_A::AF1
    }
    ///Checks if the value of the field is `AF2`
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        **self == AFRH8_A::AF2
    }
    ///Checks if the value of the field is `AF3`
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        **self == AFRH8_A::AF3
    }
    ///Checks if the value of the field is `AF4`
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        **self == AFRH8_A::AF4
    }
    ///Checks if the value of the field is `AF5`
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        **self == AFRH8_A::AF5
    }
    ///Checks if the value of the field is `AF6`
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        **self == AFRH8_A::AF6
    }
    ///Checks if the value of the field is `AF7`
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        **self == AFRH8_A::AF7
    }
    ///Checks if the value of the field is `AF8`
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        **self == AFRH8_A::AF8
    }
    ///Checks if the value of the field is `AF9`
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        **self == AFRH8_A::AF9
    }
    ///Checks if the value of the field is `AF10`
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        **self == AFRH8_A::AF10
    }
    ///Checks if the value of the field is `AF11`
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        **self == AFRH8_A::AF11
    }
    ///Checks if the value of the field is `AF12`
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        **self == AFRH8_A::AF12
    }
    ///Checks if the value of the field is `AF13`
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        **self == AFRH8_A::AF13
    }
    ///Checks if the value of the field is `AF14`
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        **self == AFRH8_A::AF14
    }
    ///Checks if the value of the field is `AF15`
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        **self == AFRH8_A::AF15
    }
}
impl core::ops::Deref for AFRH8_R {
    type Target = crate::FieldReader<u8, AFRH8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AFRH8` writer - Alternate function selection for port x bit y (y = 8..15)
pub struct AFRH8_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRH8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRH8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRH8_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRH8_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRH8_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRH8_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRH8_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRH8_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRH8_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRH8_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRH8_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRH8_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRH8_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRH8_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRH8_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRH8_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRH8_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRH8_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh15(&self) -> AFRH15_R {
        AFRH15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh14(&self) -> AFRH14_R {
        AFRH14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh13(&self) -> AFRH13_R {
        AFRH13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh12(&self) -> AFRH12_R {
        AFRH12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh11(&self) -> AFRH11_R {
        AFRH11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh10(&self) -> AFRH10_R {
        AFRH10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh9(&self) -> AFRH9_R {
        AFRH9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh8(&self) -> AFRH8_R {
        AFRH8_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh15(&mut self) -> AFRH15_W {
        AFRH15_W { w: self }
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh14(&mut self) -> AFRH14_W {
        AFRH14_W { w: self }
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh13(&mut self) -> AFRH13_W {
        AFRH13_W { w: self }
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh12(&mut self) -> AFRH12_W {
        AFRH12_W { w: self }
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh11(&mut self) -> AFRH11_W {
        AFRH11_W { w: self }
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh10(&mut self) -> AFRH10_W {
        AFRH10_W { w: self }
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh9(&mut self) -> AFRH9_W {
        AFRH9_W { w: self }
    }
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afrh8(&mut self) -> AFRH8_W {
        AFRH8_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO alternate function high register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [afrh](index.html) module
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
///`read()` method returns [afrh::R](R) reader structure
impl crate::Readable for AFRH_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [afrh::W](W) writer structure
impl crate::Writable for AFRH_SPEC {
    type Writer = W;
}
///`reset()` method sets AFRH to value 0
impl crate::Resettable for AFRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
