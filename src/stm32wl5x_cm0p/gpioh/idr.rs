///Register `IDR` reader
pub struct R(crate::R<IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDR_SPEC>) -> Self {
        R(reader)
    }
}
///Port input data (y = 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDR3_A {
    ///1: Input is logic high
    HIGH = 1,
    ///0: Input is logic low
    LOW = 0,
}
impl From<IDR3_A> for bool {
    #[inline(always)]
    fn from(variant: IDR3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDR3` reader - Port input data (y = 0..15)
pub struct IDR3_R(crate::FieldReader<bool, IDR3_A>);
impl IDR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDR3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDR3_A {
        match self.bits {
            true => IDR3_A::HIGH,
            false => IDR3_A::LOW,
        }
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == IDR3_A::HIGH
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == IDR3_A::LOW
    }
}
impl core::ops::Deref for IDR3_R {
    type Target = crate::FieldReader<bool, IDR3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 3 - Port input data (y = 0..15)
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
///GPIO port input data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idr](index.html) module
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [idr::R](R) reader structure
impl crate::Readable for IDR_SPEC {
    type Reader = R;
}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
