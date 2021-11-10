///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Allow debug in SLEEP mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_SLEEP_A {
    ///0: Debug Sleep Mode Disabled
    DISABLED = 0,
    ///1: Debug Sleep Mode Enabled
    ENABLED = 1,
}
impl From<DBG_SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_SLEEP` reader - Allow debug in SLEEP mode
pub struct DBG_SLEEP_R(crate::FieldReader<bool, DBG_SLEEP_A>);
impl DBG_SLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_SLEEP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_SLEEP_A {
        match self.bits {
            false => DBG_SLEEP_A::DISABLED,
            true => DBG_SLEEP_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DBG_SLEEP_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DBG_SLEEP_A::ENABLED
    }
}
impl core::ops::Deref for DBG_SLEEP_R {
    type Target = crate::FieldReader<bool, DBG_SLEEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_SLEEP` writer - Allow debug in SLEEP mode
pub struct DBG_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_SLEEP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_SLEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Debug Sleep Mode Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBG_SLEEP_A::DISABLED)
    }
    ///Debug Sleep Mode Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBG_SLEEP_A::ENABLED)
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
///Allow debug in STOP mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_STOP_A {
    ///0: Debug Stop Mode Disabled
    DISABLED = 0,
    ///1: Debug Stop Mode Enabled
    ENABLED = 1,
}
impl From<DBG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_STOP` reader - Allow debug in STOP mode
pub struct DBG_STOP_R(crate::FieldReader<bool, DBG_STOP_A>);
impl DBG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_STOP_A {
        match self.bits {
            false => DBG_STOP_A::DISABLED,
            true => DBG_STOP_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DBG_STOP_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DBG_STOP_A::ENABLED
    }
}
impl core::ops::Deref for DBG_STOP_R {
    type Target = crate::FieldReader<bool, DBG_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_STOP` writer - Allow debug in STOP mode
pub struct DBG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Debug Stop Mode Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBG_STOP_A::DISABLED)
    }
    ///Debug Stop Mode Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBG_STOP_A::ENABLED)
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
///Allow debug in STANDBY mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_STANDBY_A {
    ///0: Debug Standby Mode Disabled
    DISABLED = 0,
    ///1: Debug Standby Mode Enabled
    ENABLED = 1,
}
impl From<DBG_STANDBY_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STANDBY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_STANDBY` reader - Allow debug in STANDBY mode
pub struct DBG_STANDBY_R(crate::FieldReader<bool, DBG_STANDBY_A>);
impl DBG_STANDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_STANDBY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_STANDBY_A {
        match self.bits {
            false => DBG_STANDBY_A::DISABLED,
            true => DBG_STANDBY_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DBG_STANDBY_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DBG_STANDBY_A::ENABLED
    }
}
impl core::ops::Deref for DBG_STANDBY_R {
    type Target = crate::FieldReader<bool, DBG_STANDBY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_STANDBY` writer - Allow debug in STANDBY mode
pub struct DBG_STANDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_STANDBY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_STANDBY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Debug Standby Mode Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBG_STANDBY_A::DISABLED)
    }
    ///Debug Standby Mode Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBG_STANDBY_A::ENABLED)
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
impl R {
    ///Bit 0 - Allow debug in SLEEP mode
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Allow debug in STOP mode
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Allow debug in STANDBY mode
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Allow debug in SLEEP mode
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W {
        DBG_SLEEP_W { w: self }
    }
    ///Bit 1 - Allow debug in STOP mode
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W {
        DBG_STOP_W { w: self }
    }
    ///Bit 2 - Allow debug in STANDBY mode
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W {
        DBG_STANDBY_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DBGMCU Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
