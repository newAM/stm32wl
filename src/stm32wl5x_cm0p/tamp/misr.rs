///Register `MISR` reader
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
///TAMP1MF:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1MF_A {
    ///0: No tamper detected - Masked
    IDLE = 0,
    ///1: Tamper detected - Masked
    TAMPER = 1,
}
impl From<TAMP1MF_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1MF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1MF` reader - TAMP1MF:
pub struct TAMP1MF_R(crate::FieldReader<bool, TAMP1MF_A>);
impl TAMP1MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1MF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP1MF_A {
        match self.bits {
            false => TAMP1MF_A::IDLE,
            true => TAMP1MF_A::TAMPER,
        }
    }
    ///Checks if the value of the field is `IDLE`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == TAMP1MF_A::IDLE
    }
    ///Checks if the value of the field is `TAMPER`
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        **self == TAMP1MF_A::TAMPER
    }
}
impl core::ops::Deref for TAMP1MF_R {
    type Target = crate::FieldReader<bool, TAMP1MF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///TAMP2MF
pub type TAMP2MF_A = TAMP1MF_A;
///Field `TAMP2MF` reader - TAMP2MF
pub type TAMP2MF_R = TAMP1MF_R;
///TAMP3MF
pub type TAMP3MF_A = TAMP1MF_A;
///Field `TAMP3MF` reader - TAMP3MF
pub type TAMP3MF_R = TAMP1MF_R;
///ITAMP3MF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP3MF_A {
    ///0: No tamper detected - Masked
    IDLE = 0,
    ///1: Internal tamper detected - Masked
    TAMPER = 1,
}
impl From<ITAMP3MF_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3MF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP3MF` reader - ITAMP3MF
pub struct ITAMP3MF_R(crate::FieldReader<bool, ITAMP3MF_A>);
impl ITAMP3MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3MF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3MF_A {
        match self.bits {
            false => ITAMP3MF_A::IDLE,
            true => ITAMP3MF_A::TAMPER,
        }
    }
    ///Checks if the value of the field is `IDLE`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == ITAMP3MF_A::IDLE
    }
    ///Checks if the value of the field is `TAMPER`
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        **self == ITAMP3MF_A::TAMPER
    }
}
impl core::ops::Deref for ITAMP3MF_R {
    type Target = crate::FieldReader<bool, ITAMP3MF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///ITAMP5MF
pub type ITAMP5MF_A = ITAMP3MF_A;
///Field `ITAMP5MF` reader - ITAMP5MF
pub type ITAMP5MF_R = ITAMP3MF_R;
///ITAMP6MF
pub type ITAMP6MF_A = ITAMP3MF_A;
///Field `ITAMP6MF` reader - ITAMP6MF
pub type ITAMP6MF_R = ITAMP3MF_R;
///ITAMP8MF
pub type ITAMP8MF_A = ITAMP3MF_A;
///Field `ITAMP8MF` reader - ITAMP8MF
pub type ITAMP8MF_R = ITAMP3MF_R;
impl R {
    ///Bit 0 - TAMP1MF:
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TAMP2MF
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - TAMP3MF
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 18 - ITAMP3MF
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 20 - ITAMP5MF
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - ITAMP6MF
    #[inline(always)]
    pub fn itamp6mf(&self) -> ITAMP6MF_R {
        ITAMP6MF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 23 - ITAMP8MF
    #[inline(always)]
    pub fn itamp8mf(&self) -> ITAMP8MF_R {
        ITAMP8MF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
///TAMP masked interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [misr](index.html) module
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [misr::R](R) reader structure
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
