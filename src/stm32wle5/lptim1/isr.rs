#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Repetition register update Ok\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPOK_A {
    #[doc = "1: Repetition register update OK"]
    Set = 1,
}
impl From<REPOK_A> for bool {
    #[inline(always)]
    fn from(variant: REPOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPOK` reader - Repetition register update Ok"]
pub type REPOK_R = crate::BitReader<REPOK_A>;
impl REPOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REPOK_A> {
        match self.bits {
            true => Some(REPOK_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == REPOK_A::Set
    }
}
#[doc = "LPTIM update event occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UE_A {
    #[doc = "1: LPTIM update event occurred"]
    Set = 1,
}
impl From<UE_A> for bool {
    #[inline(always)]
    fn from(variant: UE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UE` reader - LPTIM update event occurred"]
pub type UE_R = crate::BitReader<UE_A>;
impl UE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UE_A> {
        match self.bits {
            true => Some(UE_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UE_A::Set
    }
}
#[doc = "Counter direction change up to down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWN_A {
    #[doc = "1: Counter direction change up to down"]
    Set = 1,
}
impl From<DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: DOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWN` reader - Counter direction change up to down"]
pub type DOWN_R = crate::BitReader<DOWN_A>;
impl DOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DOWN_A> {
        match self.bits {
            true => Some(DOWN_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DOWN_A::Set
    }
}
#[doc = "Counter direction change down to up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UP_A {
    #[doc = "1: Counter direction change down to up"]
    Set = 1,
}
impl From<UP_A> for bool {
    #[inline(always)]
    fn from(variant: UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UP` reader - Counter direction change down to up"]
pub type UP_R = crate::BitReader<UP_A>;
impl UP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UP_A> {
        match self.bits {
            true => Some(UP_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UP_A::Set
    }
}
#[doc = "Autoreload register update OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARROK_A {
    #[doc = "1: Autoreload register update OK"]
    Set = 1,
}
impl From<ARROK_A> for bool {
    #[inline(always)]
    fn from(variant: ARROK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARROK` reader - Autoreload register update OK"]
pub type ARROK_R = crate::BitReader<ARROK_A>;
impl ARROK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARROK_A> {
        match self.bits {
            true => Some(ARROK_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ARROK_A::Set
    }
}
#[doc = "Compare register update OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPOK_A {
    #[doc = "1: Compare register update OK"]
    Set = 1,
}
impl From<CMPOK_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPOK` reader - Compare register update OK"]
pub type CMPOK_R = crate::BitReader<CMPOK_A>;
impl CMPOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPOK_A> {
        match self.bits {
            true => Some(CMPOK_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMPOK_A::Set
    }
}
#[doc = "External trigger edge event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTTRIG_A {
    #[doc = "1: External trigger edge event"]
    Set = 1,
}
impl From<EXTTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIG` reader - External trigger edge event"]
pub type EXTTRIG_R = crate::BitReader<EXTTRIG_A>;
impl EXTTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTTRIG_A> {
        match self.bits {
            true => Some(EXTTRIG_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EXTTRIG_A::Set
    }
}
#[doc = "Autoreload match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARRM_A {
    #[doc = "1: Autoreload match"]
    Set = 1,
}
impl From<ARRM_A> for bool {
    #[inline(always)]
    fn from(variant: ARRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARRM` reader - Autoreload match"]
pub type ARRM_R = crate::BitReader<ARRM_A>;
impl ARRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARRM_A> {
        match self.bits {
            true => Some(ARRM_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ARRM_A::Set
    }
}
#[doc = "Compare match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPM_A {
    #[doc = "1: Compare match"]
    Set = 1,
}
impl From<CMPM_A> for bool {
    #[inline(always)]
    fn from(variant: CMPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPM` reader - Compare match"]
pub type CMPM_R = crate::BitReader<CMPM_A>;
impl CMPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPM_A> {
        match self.bits {
            true => Some(CMPM_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMPM_A::Set
    }
}
impl R {
    #[doc = "Bit 8 - Repetition register update Ok"]
    #[inline(always)]
    pub fn repok(&self) -> REPOK_R {
        REPOK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - LPTIM update event occurred"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter direction change up to down"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK"]
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK"]
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match"]
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Compare match"]
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
