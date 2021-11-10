///Register `C1SCR` reader
pub struct R(crate::R<C1SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1SCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C1SCR` writer
pub struct W(crate::W<C1SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1SCR_SPEC>;
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
impl From<crate::W<C1SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1SCR_SPEC>) -> Self {
        W(writer)
    }
}
///CH1C
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1C_A {
    ///1: Processor receive channel n status bit clear
    CLEAR = 1,
    ///0: No action
    NOACTION = 0,
}
impl From<CH1C_A> for bool {
    #[inline(always)]
    fn from(variant: CH1C_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1C` reader - CH1C
pub struct CH1C_R(crate::FieldReader<bool, CH1C_A>);
impl CH1C_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1C_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CH1C_A {
        match self.bits {
            true => CH1C_A::CLEAR,
            false => CH1C_A::NOACTION,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CH1C_A::CLEAR
    }
    ///Checks if the value of the field is `NOACTION`
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        **self == CH1C_A::NOACTION
    }
}
impl core::ops::Deref for CH1C_R {
    type Target = crate::FieldReader<bool, CH1C_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CH1C` writer - CH1C
pub struct CH1C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1C_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH1C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor receive channel n status bit clear
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH1C_A::CLEAR)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH1C_A::NOACTION)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///CH2C
pub type CH2C_A = CH1C_A;
///Field `CH2C` reader - CH2C
pub type CH2C_R = CH1C_R;
///Field `CH2C` writer - CH2C
pub struct CH2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2C_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH2C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor receive channel n status bit clear
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH2C_A::CLEAR)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH2C_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///CH3C
pub type CH3C_A = CH1C_A;
///Field `CH3C` reader - CH3C
pub type CH3C_R = CH1C_R;
///Field `CH3C` writer - CH3C
pub struct CH3C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3C_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH3C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor receive channel n status bit clear
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH3C_A::CLEAR)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH3C_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///CH4C
pub type CH4C_A = CH1C_A;
///Field `CH4C` reader - CH4C
pub type CH4C_R = CH1C_R;
///Field `CH4C` writer - CH4C
pub struct CH4C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4C_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH4C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor receive channel n status bit clear
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH4C_A::CLEAR)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH4C_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///CH5C
pub type CH5C_A = CH1C_A;
///Field `CH5C` reader - CH5C
pub type CH5C_R = CH1C_R;
///Field `CH5C` writer - CH5C
pub struct CH5C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5C_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH5C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor receive channel n status bit clear
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH5C_A::CLEAR)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH5C_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///CH6C
pub type CH6C_A = CH1C_A;
///Field `CH6C` reader - CH6C
pub type CH6C_R = CH1C_R;
///Field `CH6C` writer - CH6C
pub struct CH6C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6C_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH6C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor receive channel n status bit clear
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH6C_A::CLEAR)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH6C_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///CH1S
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1S_A {
    ///1: Processor transmit channel n status bit set
    SET = 1,
    ///0: No action
    NOACTION = 0,
}
impl From<CH1S_A> for bool {
    #[inline(always)]
    fn from(variant: CH1S_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1S` reader - CH1S
pub struct CH1S_R(crate::FieldReader<bool, CH1S_A>);
impl CH1S_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1S_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CH1S_A {
        match self.bits {
            true => CH1S_A::SET,
            false => CH1S_A::NOACTION,
        }
    }
    ///Checks if the value of the field is `SET`
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == CH1S_A::SET
    }
    ///Checks if the value of the field is `NOACTION`
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        **self == CH1S_A::NOACTION
    }
}
impl core::ops::Deref for CH1S_R {
    type Target = crate::FieldReader<bool, CH1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CH1S` writer - CH1S
pub struct CH1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH1S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor transmit channel n status bit set
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH1S_A::SET)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH1S_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///CH2S
pub type CH2S_A = CH1S_A;
///Field `CH2S` reader - CH2S
pub type CH2S_R = CH1S_R;
///Field `CH2S` writer - CH2S
pub struct CH2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH2S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor transmit channel n status bit set
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH2S_A::SET)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH2S_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///CH3S
pub type CH3S_A = CH1S_A;
///Field `CH3S` reader - CH3S
pub type CH3S_R = CH1S_R;
///Field `CH3S` writer - CH3S
pub struct CH3S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH3S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor transmit channel n status bit set
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH3S_A::SET)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH3S_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///CH4S
pub type CH4S_A = CH1S_A;
///Field `CH4S` reader - CH4S
pub type CH4S_R = CH1S_R;
///Field `CH4S` writer - CH4S
pub struct CH4S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH4S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor transmit channel n status bit set
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH4S_A::SET)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH4S_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///CH5S
pub type CH5S_A = CH1S_A;
///Field `CH5S` reader - CH5S
pub type CH5S_R = CH1S_R;
///Field `CH5S` writer - CH5S
pub struct CH5S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH5S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor transmit channel n status bit set
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH5S_A::SET)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH5S_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///CH6S
pub type CH6S_A = CH1S_A;
///Field `CH6S` reader - CH6S
pub type CH6S_R = CH1S_R;
///Field `CH6S` writer - CH6S
pub struct CH6S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CH6S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Processor transmit channel n status bit set
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH6S_A::SET)
    }
    ///No action
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CH6S_A::NOACTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    ///Bit 0 - CH1C
    #[inline(always)]
    pub fn ch1c(&self) -> CH1C_R {
        CH1C_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - CH2C
    #[inline(always)]
    pub fn ch2c(&self) -> CH2C_R {
        CH2C_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - CH3C
    #[inline(always)]
    pub fn ch3c(&self) -> CH3C_R {
        CH3C_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - CH4C
    #[inline(always)]
    pub fn ch4c(&self) -> CH4C_R {
        CH4C_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - CH5C
    #[inline(always)]
    pub fn ch5c(&self) -> CH5C_R {
        CH5C_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - CH6C
    #[inline(always)]
    pub fn ch6c(&self) -> CH6C_R {
        CH6C_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 16 - CH1S
    #[inline(always)]
    pub fn ch1s(&self) -> CH1S_R {
        CH1S_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - CH2S
    #[inline(always)]
    pub fn ch2s(&self) -> CH2S_R {
        CH2S_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - CH3S
    #[inline(always)]
    pub fn ch3s(&self) -> CH3S_R {
        CH3S_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - CH4S
    #[inline(always)]
    pub fn ch4s(&self) -> CH4S_R {
        CH4S_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - CH5S
    #[inline(always)]
    pub fn ch5s(&self) -> CH5S_R {
        CH5S_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - CH6S
    #[inline(always)]
    pub fn ch6s(&self) -> CH6S_R {
        CH6S_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - CH1C
    #[inline(always)]
    pub fn ch1c(&mut self) -> CH1C_W {
        CH1C_W { w: self }
    }
    ///Bit 1 - CH2C
    #[inline(always)]
    pub fn ch2c(&mut self) -> CH2C_W {
        CH2C_W { w: self }
    }
    ///Bit 2 - CH3C
    #[inline(always)]
    pub fn ch3c(&mut self) -> CH3C_W {
        CH3C_W { w: self }
    }
    ///Bit 3 - CH4C
    #[inline(always)]
    pub fn ch4c(&mut self) -> CH4C_W {
        CH4C_W { w: self }
    }
    ///Bit 4 - CH5C
    #[inline(always)]
    pub fn ch5c(&mut self) -> CH5C_W {
        CH5C_W { w: self }
    }
    ///Bit 5 - CH6C
    #[inline(always)]
    pub fn ch6c(&mut self) -> CH6C_W {
        CH6C_W { w: self }
    }
    ///Bit 16 - CH1S
    #[inline(always)]
    pub fn ch1s(&mut self) -> CH1S_W {
        CH1S_W { w: self }
    }
    ///Bit 17 - CH2S
    #[inline(always)]
    pub fn ch2s(&mut self) -> CH2S_W {
        CH2S_W { w: self }
    }
    ///Bit 18 - CH3S
    #[inline(always)]
    pub fn ch3s(&mut self) -> CH3S_W {
        CH3S_W { w: self }
    }
    ///Bit 19 - CH4S
    #[inline(always)]
    pub fn ch4s(&mut self) -> CH4S_W {
        CH4S_W { w: self }
    }
    ///Bit 20 - CH5S
    #[inline(always)]
    pub fn ch5s(&mut self) -> CH5S_W {
        CH5S_W { w: self }
    }
    ///Bit 21 - CH6S
    #[inline(always)]
    pub fn ch6s(&mut self) -> CH6S_W {
        CH6S_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Reading this register will always return 0x0000 0000.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1scr](index.html) module
pub struct C1SCR_SPEC;
impl crate::RegisterSpec for C1SCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1scr::R](R) reader structure
impl crate::Readable for C1SCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c1scr::W](W) writer structure
impl crate::Writable for C1SCR_SPEC {
    type Writer = W;
}
///`reset()` method sets C1SCR to value 0
impl crate::Resettable for C1SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
