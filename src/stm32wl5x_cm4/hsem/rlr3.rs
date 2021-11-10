///Register `RLR3` reader
pub struct R(crate::R<RLR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLR3_SPEC>) -> Self {
        R(reader)
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
///HSEM Read lock register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rlr3](index.html) module
pub struct RLR3_SPEC;
impl crate::RegisterSpec for RLR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [rlr3::R](R) reader structure
impl crate::Readable for RLR3_SPEC {
    type Reader = R;
}
///`reset()` method sets RLR3 to value 0
impl crate::Resettable for RLR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}