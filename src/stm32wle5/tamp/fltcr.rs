///Register `FLTCR` reader
pub struct R(crate::R<FLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLTCR` writer
pub struct W(crate::W<FLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTCR_SPEC>;
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
impl From<crate::W<FLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTCR_SPEC>) -> Self {
        W(writer)
    }
}
///TAMPFREQ
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPFREQ_A {
    ///0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    HZ_1 = 0,
    ///1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    HZ_2 = 1,
    ///2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    HZ_4 = 2,
    ///3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    HZ_8 = 3,
    ///4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    HZ_16 = 4,
    ///5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    HZ_32 = 5,
    ///6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    HZ_64 = 6,
    ///7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    HZ_128 = 7,
}
impl From<TAMPFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ_A) -> Self {
        variant as _
    }
}
///Field `TAMPFREQ` reader - TAMPFREQ
pub struct TAMPFREQ_R(crate::FieldReader<u8, TAMPFREQ_A>);
impl TAMPFREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPFREQ_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPFREQ_A {
        match self.bits {
            0 => TAMPFREQ_A::HZ_1,
            1 => TAMPFREQ_A::HZ_2,
            2 => TAMPFREQ_A::HZ_4,
            3 => TAMPFREQ_A::HZ_8,
            4 => TAMPFREQ_A::HZ_16,
            5 => TAMPFREQ_A::HZ_32,
            6 => TAMPFREQ_A::HZ_64,
            7 => TAMPFREQ_A::HZ_128,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `HZ_1`
    #[inline(always)]
    pub fn is_hz_1(&self) -> bool {
        **self == TAMPFREQ_A::HZ_1
    }
    ///Checks if the value of the field is `HZ_2`
    #[inline(always)]
    pub fn is_hz_2(&self) -> bool {
        **self == TAMPFREQ_A::HZ_2
    }
    ///Checks if the value of the field is `HZ_4`
    #[inline(always)]
    pub fn is_hz_4(&self) -> bool {
        **self == TAMPFREQ_A::HZ_4
    }
    ///Checks if the value of the field is `HZ_8`
    #[inline(always)]
    pub fn is_hz_8(&self) -> bool {
        **self == TAMPFREQ_A::HZ_8
    }
    ///Checks if the value of the field is `HZ_16`
    #[inline(always)]
    pub fn is_hz_16(&self) -> bool {
        **self == TAMPFREQ_A::HZ_16
    }
    ///Checks if the value of the field is `HZ_32`
    #[inline(always)]
    pub fn is_hz_32(&self) -> bool {
        **self == TAMPFREQ_A::HZ_32
    }
    ///Checks if the value of the field is `HZ_64`
    #[inline(always)]
    pub fn is_hz_64(&self) -> bool {
        **self == TAMPFREQ_A::HZ_64
    }
    ///Checks if the value of the field is `HZ_128`
    #[inline(always)]
    pub fn is_hz_128(&self) -> bool {
        **self == TAMPFREQ_A::HZ_128
    }
}
impl core::ops::Deref for TAMPFREQ_R {
    type Target = crate::FieldReader<u8, TAMPFREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPFREQ` writer - TAMPFREQ
pub struct TAMPFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFREQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPFREQ_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_1(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::HZ_1)
    }
    ///RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_2(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::HZ_2)
    }
    ///RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_4(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::HZ_4)
    }
    ///RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_8(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::HZ_8)
    }
    ///RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_16(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::HZ_16)
    }
    ///RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_32(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::HZ_32)
    }
    ///RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_64(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::HZ_64)
    }
    ///RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_128(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::HZ_128)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
///TAMPFLT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPFLT_A {
    ///0: Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)"
    NOFILTER = 0,
    ///1: Tamper event is activated after 2 consecutive samples at the active level"
    FILTER2 = 1,
    ///2: Tamper event is activated after 4 consecutive samples at the active level"
    FILTER4 = 2,
    ///3: Tamper event is activated after 8 consecutive samples at the active level"
    FILTER8 = 3,
}
impl From<TAMPFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT_A) -> Self {
        variant as _
    }
}
///Field `TAMPFLT` reader - TAMPFLT
pub struct TAMPFLT_R(crate::FieldReader<u8, TAMPFLT_A>);
impl TAMPFLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPFLT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPFLT_A {
        match self.bits {
            0 => TAMPFLT_A::NOFILTER,
            1 => TAMPFLT_A::FILTER2,
            2 => TAMPFLT_A::FILTER4,
            3 => TAMPFLT_A::FILTER8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOFILTER`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        **self == TAMPFLT_A::NOFILTER
    }
    ///Checks if the value of the field is `FILTER2`
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        **self == TAMPFLT_A::FILTER2
    }
    ///Checks if the value of the field is `FILTER4`
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        **self == TAMPFLT_A::FILTER4
    }
    ///Checks if the value of the field is `FILTER8`
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        **self == TAMPFLT_A::FILTER8
    }
}
impl core::ops::Deref for TAMPFLT_R {
    type Target = crate::FieldReader<u8, TAMPFLT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPFLT` writer - TAMPFLT
pub struct TAMPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFLT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPFLT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)"
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(TAMPFLT_A::NOFILTER)
    }
    ///Tamper event is activated after 2 consecutive samples at the active level"
    #[inline(always)]
    pub fn filter2(self) -> &'a mut W {
        self.variant(TAMPFLT_A::FILTER2)
    }
    ///Tamper event is activated after 4 consecutive samples at the active level"
    #[inline(always)]
    pub fn filter4(self) -> &'a mut W {
        self.variant(TAMPFLT_A::FILTER4)
    }
    ///Tamper event is activated after 8 consecutive samples at the active level"
    #[inline(always)]
    pub fn filter8(self) -> &'a mut W {
        self.variant(TAMPFLT_A::FILTER8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
///TAMPPRCH
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPPRCH_A {
    ///0: 1 RTCCLK cycle
    CYCLES1 = 0,
    ///1: 2 RTCCLK cycles
    CYCLES2 = 1,
    ///2: 4 RTCCLK cycles
    CYCLES4 = 2,
    ///3: 8 RTCCLK cycles
    CYCLES8 = 3,
}
impl From<TAMPPRCH_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH_A) -> Self {
        variant as _
    }
}
///Field `TAMPPRCH` reader - TAMPPRCH
pub struct TAMPPRCH_R(crate::FieldReader<u8, TAMPPRCH_A>);
impl TAMPPRCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPPRCH_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPPRCH_A {
        match self.bits {
            0 => TAMPPRCH_A::CYCLES1,
            1 => TAMPPRCH_A::CYCLES2,
            2 => TAMPPRCH_A::CYCLES4,
            3 => TAMPPRCH_A::CYCLES8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `CYCLES1`
    #[inline(always)]
    pub fn is_cycles1(&self) -> bool {
        **self == TAMPPRCH_A::CYCLES1
    }
    ///Checks if the value of the field is `CYCLES2`
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        **self == TAMPPRCH_A::CYCLES2
    }
    ///Checks if the value of the field is `CYCLES4`
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        **self == TAMPPRCH_A::CYCLES4
    }
    ///Checks if the value of the field is `CYCLES8`
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        **self == TAMPPRCH_A::CYCLES8
    }
}
impl core::ops::Deref for TAMPPRCH_R {
    type Target = crate::FieldReader<u8, TAMPPRCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPPRCH` writer - TAMPPRCH
pub struct TAMPPRCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPRCH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPPRCH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///1 RTCCLK cycle
    #[inline(always)]
    pub fn cycles1(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::CYCLES1)
    }
    ///2 RTCCLK cycles
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::CYCLES2)
    }
    ///4 RTCCLK cycles
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::CYCLES4)
    }
    ///8 RTCCLK cycles
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::CYCLES8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
///TAMPPUDIS
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPPUDIS_A {
    ///0: Precharge TAMP_INx pins before sampling (enable internal pull-up)
    ENABLED = 0,
    ///1: Disable precharge of TAMP_INx pins
    DISABLED = 1,
}
impl From<TAMPPUDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPPUDIS` reader - TAMPPUDIS
pub struct TAMPPUDIS_R(crate::FieldReader<bool, TAMPPUDIS_A>);
impl TAMPPUDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPPUDIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPPUDIS_A {
        match self.bits {
            false => TAMPPUDIS_A::ENABLED,
            true => TAMPPUDIS_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TAMPPUDIS_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TAMPPUDIS_A::DISABLED
    }
}
impl core::ops::Deref for TAMPPUDIS_R {
    type Target = crate::FieldReader<bool, TAMPPUDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPPUDIS` writer - TAMPPUDIS
pub struct TAMPPUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPUDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPPUDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Precharge TAMP_INx pins before sampling (enable internal pull-up)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::ENABLED)
    }
    ///Disable precharge of TAMP_INx pins
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    ///Bits 0:2 - TAMPFREQ
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 3:4 - TAMPFLT
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    ///Bits 5:6 - TAMPPRCH
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    ///Bit 7 - TAMPPUDIS
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:2 - TAMPFREQ
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W {
        TAMPFREQ_W { w: self }
    }
    ///Bits 3:4 - TAMPFLT
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W {
        TAMPFLT_W { w: self }
    }
    ///Bits 5:6 - TAMPPRCH
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W {
        TAMPPRCH_W { w: self }
    }
    ///Bit 7 - TAMPPUDIS
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W {
        TAMPPUDIS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltcr](index.html) module
pub struct FLTCR_SPEC;
impl crate::RegisterSpec for FLTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fltcr::R](R) reader structure
impl crate::Readable for FLTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fltcr::W](W) writer structure
impl crate::Writable for FLTCR_SPEC {
    type Writer = W;
}
///`reset()` method sets FLTCR to value 0
impl crate::Resettable for FLTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
