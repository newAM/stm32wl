///Register `R15` reader
pub struct R(crate::R<R15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R15_SPEC>) -> Self {
        R(reader)
    }
}
///Register `R15` writer
pub struct W(crate::W<R15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R15_SPEC>;
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
impl From<crate::W<R15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R15_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PROCID` reader - Semaphore ProcessID
pub struct PROCID_R(crate::FieldReader<u8, u8>);
impl PROCID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PROCID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PROCID` writer - Semaphore ProcessID
pub struct PROCID_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `COREID` reader - COREID
pub struct COREID_R(crate::FieldReader<u8, u8>);
impl COREID_R {
    pub(crate) fn new(bits: u8) -> Self {
        COREID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COREID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `COREID` writer - COREID
pub struct COREID_W<'a> {
    w: &'a mut W,
}
impl<'a> COREID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Lock indication
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    ///0: Semaphore is free
    FREE = 0,
    ///1: Semaphore is locked
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` reader - Lock indication
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::FREE,
            true => LOCK_A::LOCKED,
        }
    }
    ///Checks if the value of the field is `FREE`
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        **self == LOCK_A::FREE
    }
    ///Checks if the value of the field is `LOCKED`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LOCK_A::LOCKED
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Lock indication
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_AW {
    ///0: Free semaphore
    FREE = 0,
    ///1: Try to lock semaphore
    TRYLOCK = 1,
}
impl From<LOCK_AW> for bool {
    #[inline(always)]
    fn from(variant: LOCK_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` writer - Lock indication
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LOCK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Free semaphore
    #[inline(always)]
    pub fn free(self) -> &'a mut W {
        self.variant(LOCK_AW::FREE)
    }
    ///Try to lock semaphore
    #[inline(always)]
    pub fn try_lock(self) -> &'a mut W {
        self.variant(LOCK_AW::TRYLOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Semaphore ProcessID
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - COREID
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 31 - Lock indication
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:7 - Semaphore ProcessID
    #[inline(always)]
    pub fn procid(&mut self) -> PROCID_W {
        PROCID_W { w: self }
    }
    ///Bits 8:11 - COREID
    #[inline(always)]
    pub fn coreid(&mut self) -> COREID_W {
        COREID_W { w: self }
    }
    ///Bit 31 - Lock indication
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HSEM register HSEM_R0 HSEM_R31
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [r15](index.html) module
pub struct R15_SPEC;
impl crate::RegisterSpec for R15_SPEC {
    type Ux = u32;
}
///`read()` method returns [r15::R](R) reader structure
impl crate::Readable for R15_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [r15::W](W) writer structure
impl crate::Writable for R15_SPEC {
    type Writer = W;
}
///`reset()` method sets R15 to value 0
impl crate::Resettable for R15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
