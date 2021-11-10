///Register `RGSR` reader
pub struct R(crate::R<RGSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGSR_SPEC>) -> Self {
        R(reader)
    }
}
///Trigger overrun event flag
pub type OF3_A = OF0_A;
///Field `OF3` reader - Trigger overrun event flag
pub type OF3_R = OF0_R;
///OF2
pub type OF2_A = OF0_A;
///Field `OF2` reader - OF2
pub type OF2_R = OF0_R;
///OF1
pub type OF1_A = OF0_A;
///Field `OF1` reader - OF1
pub type OF1_R = OF0_R;
///OF0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OF0_A {
    ///0: No new trigger event occured on DMA request generator channel x, before the request counter underrun
    NOTRIGGER = 0,
    ///1: New trigger event occured on DMA request generator channel x, before the request counter underrun
    TRIGGER = 1,
}
impl From<OF0_A> for bool {
    #[inline(always)]
    fn from(variant: OF0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OF0` reader - OF0
pub struct OF0_R(crate::FieldReader<bool, OF0_A>);
impl OF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OF0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OF0_A {
        match self.bits {
            false => OF0_A::NOTRIGGER,
            true => OF0_A::TRIGGER,
        }
    }
    ///Checks if the value of the field is `NOTRIGGER`
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        **self == OF0_A::NOTRIGGER
    }
    ///Checks if the value of the field is `TRIGGER`
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == OF0_A::TRIGGER
    }
}
impl core::ops::Deref for OF0_R {
    type Target = crate::FieldReader<bool, OF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 3 - Trigger overrun event flag
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - OF2
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - OF1
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - OF0
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 0x01) != 0)
    }
}
///request generator interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rgsr](index.html) module
pub struct RGSR_SPEC;
impl crate::RegisterSpec for RGSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rgsr::R](R) reader structure
impl crate::Readable for RGSR_SPEC {
    type Reader = R;
}
///`reset()` method sets RGSR to value 0
impl crate::Resettable for RGSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
