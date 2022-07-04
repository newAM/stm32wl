#[doc = "Register `ICSR` reader"]
pub struct R(crate::R<ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSR` writer"]
pub struct W(crate::W<ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR_SPEC>;
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
impl From<crate::W<ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Recalibration pending Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECALPF_A {
    #[doc = "1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
    Pending = 1,
}
impl From<RECALPF_A> for bool {
    #[inline(always)]
    fn from(variant: RECALPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECALPF` reader - Recalibration pending Flag"]
pub type RECALPF_R = crate::BitReader<RECALPF_A>;
impl RECALPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RECALPF_A> {
        match self.bits {
            true => Some(RECALPF_A::Pending),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECALPF_A::Pending
    }
}
#[doc = "BCD update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BCDU_A {
    #[doc = "0: 1s increment each time SS\\[7:0\\]=0"]
    Bit7 = 0,
    #[doc = "1: 1s increment each time SS\\[8:0\\]=0"]
    Bit8 = 1,
    #[doc = "2: 1s increment each time SS\\[9:0\\]=0"]
    Bit9 = 2,
    #[doc = "3: 1s increment each time SS\\[10:0\\]=0"]
    Bit10 = 3,
    #[doc = "4: 1s increment each time SS\\[11:0\\]=0"]
    Bit11 = 4,
    #[doc = "5: 1s increment each time SS\\[12:0\\]=0"]
    Bit12 = 5,
    #[doc = "6: 1s increment each time SS\\[13:0\\]=0"]
    Bit13 = 6,
    #[doc = "7: 1s increment each time SS\\[14:0\\]=0"]
    Bit14 = 7,
}
impl From<BCDU_A> for u8 {
    #[inline(always)]
    fn from(variant: BCDU_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BCDU` reader - BCD update"]
pub type BCDU_R = crate::FieldReader<u8, BCDU_A>;
impl BCDU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCDU_A {
        match self.bits {
            0 => BCDU_A::Bit7,
            1 => BCDU_A::Bit8,
            2 => BCDU_A::Bit9,
            3 => BCDU_A::Bit10,
            4 => BCDU_A::Bit11,
            5 => BCDU_A::Bit12,
            6 => BCDU_A::Bit13,
            7 => BCDU_A::Bit14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Bit7`"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == BCDU_A::Bit7
    }
    #[doc = "Checks if the value of the field is `Bit8`"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == BCDU_A::Bit8
    }
    #[doc = "Checks if the value of the field is `Bit9`"]
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == BCDU_A::Bit9
    }
    #[doc = "Checks if the value of the field is `Bit10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == BCDU_A::Bit10
    }
    #[doc = "Checks if the value of the field is `Bit11`"]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == BCDU_A::Bit11
    }
    #[doc = "Checks if the value of the field is `Bit12`"]
    #[inline(always)]
    pub fn is_bit12(&self) -> bool {
        *self == BCDU_A::Bit12
    }
    #[doc = "Checks if the value of the field is `Bit13`"]
    #[inline(always)]
    pub fn is_bit13(&self) -> bool {
        *self == BCDU_A::Bit13
    }
    #[doc = "Checks if the value of the field is `Bit14`"]
    #[inline(always)]
    pub fn is_bit14(&self) -> bool {
        *self == BCDU_A::Bit14
    }
}
#[doc = "Field `BCDU` writer - BCD update"]
pub type BCDU_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICSR_SPEC, u8, BCDU_A, 3, O>;
impl<'a, const O: u8> BCDU_W<'a, O> {
    #[doc = "1s increment each time SS\\[7:0\\]=0"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(BCDU_A::Bit7)
    }
    #[doc = "1s increment each time SS\\[8:0\\]=0"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(BCDU_A::Bit8)
    }
    #[doc = "1s increment each time SS\\[9:0\\]=0"]
    #[inline(always)]
    pub fn bit9(self) -> &'a mut W {
        self.variant(BCDU_A::Bit9)
    }
    #[doc = "1s increment each time SS\\[10:0\\]=0"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(BCDU_A::Bit10)
    }
    #[doc = "1s increment each time SS\\[11:0\\]=0"]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut W {
        self.variant(BCDU_A::Bit11)
    }
    #[doc = "1s increment each time SS\\[12:0\\]=0"]
    #[inline(always)]
    pub fn bit12(self) -> &'a mut W {
        self.variant(BCDU_A::Bit12)
    }
    #[doc = "1s increment each time SS\\[13:0\\]=0"]
    #[inline(always)]
    pub fn bit13(self) -> &'a mut W {
        self.variant(BCDU_A::Bit13)
    }
    #[doc = "1s increment each time SS\\[14:0\\]=0"]
    #[inline(always)]
    pub fn bit14(self) -> &'a mut W {
        self.variant(BCDU_A::Bit14)
    }
}
#[doc = "Binary mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIN_A {
    #[doc = "0: Free running BCD calendar mode (Binary mode disabled)"]
    Bcd = 0,
    #[doc = "1: Free running Binary mode (BCD mode disabled)"]
    Binary = 1,
    #[doc = "2: Free running BCD calendar and Binary modes"]
    BinBcd = 2,
    #[doc = "3: Free running BCD calendar and Binary modes"]
    BinBcd2 = 3,
}
impl From<BIN_A> for u8 {
    #[inline(always)]
    fn from(variant: BIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BIN` reader - Binary mode"]
pub type BIN_R = crate::FieldReader<u8, BIN_A>;
impl BIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIN_A {
        match self.bits {
            0 => BIN_A::Bcd,
            1 => BIN_A::Binary,
            2 => BIN_A::BinBcd,
            3 => BIN_A::BinBcd2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Bcd`"]
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        *self == BIN_A::Bcd
    }
    #[doc = "Checks if the value of the field is `Binary`"]
    #[inline(always)]
    pub fn is_binary(&self) -> bool {
        *self == BIN_A::Binary
    }
    #[doc = "Checks if the value of the field is `BinBcd`"]
    #[inline(always)]
    pub fn is_bin_bcd(&self) -> bool {
        *self == BIN_A::BinBcd
    }
    #[doc = "Checks if the value of the field is `BinBcd2`"]
    #[inline(always)]
    pub fn is_bin_bcd2(&self) -> bool {
        *self == BIN_A::BinBcd2
    }
}
#[doc = "Field `BIN` writer - Binary mode"]
pub type BIN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICSR_SPEC, u8, BIN_A, 2, O>;
impl<'a, const O: u8> BIN_W<'a, O> {
    #[doc = "Free running BCD calendar mode (Binary mode disabled)"]
    #[inline(always)]
    pub fn bcd(self) -> &'a mut W {
        self.variant(BIN_A::Bcd)
    }
    #[doc = "Free running Binary mode (BCD mode disabled)"]
    #[inline(always)]
    pub fn binary(self) -> &'a mut W {
        self.variant(BIN_A::Binary)
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn bin_bcd(self) -> &'a mut W {
        self.variant(BIN_A::BinBcd)
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn bin_bcd2(self) -> &'a mut W {
        self.variant(BIN_A::BinBcd2)
    }
}
#[doc = "Initialization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Free running mode"]
    FreeRunningMode = 0,
    #[doc = "1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    InitMode = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization mode"]
pub type INIT_R = crate::BitReader<INIT_A>;
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::FreeRunningMode,
            true => INIT_A::InitMode,
        }
    }
    #[doc = "Checks if the value of the field is `FreeRunningMode`"]
    #[inline(always)]
    pub fn is_free_running_mode(&self) -> bool {
        *self == INIT_A::FreeRunningMode
    }
    #[doc = "Checks if the value of the field is `InitMode`"]
    #[inline(always)]
    pub fn is_init_mode(&self) -> bool {
        *self == INIT_A::InitMode
    }
}
#[doc = "Field `INIT` writer - Initialization mode"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, INIT_A, O>;
impl<'a, const O: u8> INIT_W<'a, O> {
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn free_running_mode(self) -> &'a mut W {
        self.variant(INIT_A::FreeRunningMode)
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn init_mode(self) -> &'a mut W {
        self.variant(INIT_A::InitMode)
    }
}
#[doc = "Initialization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITF_A {
    #[doc = "0: Calendar registers update is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Calendar registers update is allowed"]
    Allowed = 1,
}
impl From<INITF_A> for bool {
    #[inline(always)]
    fn from(variant: INITF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITF` reader - Initialization flag"]
pub type INITF_R = crate::BitReader<INITF_A>;
impl INITF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITF_A {
        match self.bits {
            false => INITF_A::NotAllowed,
            true => INITF_A::Allowed,
        }
    }
    #[doc = "Checks if the value of the field is `NotAllowed`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == INITF_A::NotAllowed
    }
    #[doc = "Checks if the value of the field is `Allowed`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == INITF_A::Allowed
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_A {
    #[doc = "0: Calendar shadow registers not yet synchronized"]
    NotSynced = 0,
    #[doc = "1: Calendar shadow registers synchronized"]
    Synced = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Registers synchronization flag"]
pub type RSF_R = crate::BitReader<RSF_A>;
impl RSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::NotSynced,
            true => RSF_A::Synced,
        }
    }
    #[doc = "Checks if the value of the field is `NotSynced`"]
    #[inline(always)]
    pub fn is_not_synced(&self) -> bool {
        *self == RSF_A::NotSynced
    }
    #[doc = "Checks if the value of the field is `Synced`"]
    #[inline(always)]
    pub fn is_synced(&self) -> bool {
        *self == RSF_A::Synced
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<RSF_AW> for bool {
    #[inline(always)]
    fn from(variant: RSF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` writer - Registers synchronization flag"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, RSF_AW, O>;
impl<'a, const O: u8> RSF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSF_AW::Clear)
    }
}
#[doc = "Initialization status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITS_A {
    #[doc = "0: Calendar has not been initialized"]
    NotInitalized = 0,
    #[doc = "1: Calendar has been initialized"]
    Initalized = 1,
}
impl From<INITS_A> for bool {
    #[inline(always)]
    fn from(variant: INITS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITS` reader - Initialization status flag"]
pub type INITS_R = crate::BitReader<INITS_A>;
impl INITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITS_A {
        match self.bits {
            false => INITS_A::NotInitalized,
            true => INITS_A::Initalized,
        }
    }
    #[doc = "Checks if the value of the field is `NotInitalized`"]
    #[inline(always)]
    pub fn is_not_initalized(&self) -> bool {
        *self == INITS_A::NotInitalized
    }
    #[doc = "Checks if the value of the field is `Initalized`"]
    #[inline(always)]
    pub fn is_initalized(&self) -> bool {
        *self == INITS_A::Initalized
    }
}
#[doc = "Shift operation pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHPF_A {
    #[doc = "0: No shift operation is pending"]
    NoShiftPending = 0,
    #[doc = "1: A shift operation is pending"]
    ShiftPending = 1,
}
impl From<SHPF_A> for bool {
    #[inline(always)]
    fn from(variant: SHPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHPF` reader - Shift operation pending"]
pub type SHPF_R = crate::BitReader<SHPF_A>;
impl SHPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHPF_A {
        match self.bits {
            false => SHPF_A::NoShiftPending,
            true => SHPF_A::ShiftPending,
        }
    }
    #[doc = "Checks if the value of the field is `NoShiftPending`"]
    #[inline(always)]
    pub fn is_no_shift_pending(&self) -> bool {
        *self == SHPF_A::NoShiftPending
    }
    #[doc = "Checks if the value of the field is `ShiftPending`"]
    #[inline(always)]
    pub fn is_shift_pending(&self) -> bool {
        *self == SHPF_A::ShiftPending
    }
}
#[doc = "Wakeup timer write flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTWF_A {
    #[doc = "0: Wakeup timer configuration update not allowed"]
    UpdateNotAllowed = 0,
    #[doc = "1: Wakeup timer configuration update allowed"]
    UpdateAllowed = 1,
}
impl From<WUTWF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTWF` reader - Wakeup timer write flag"]
pub type WUTWF_R = crate::BitReader<WUTWF_A>;
impl WUTWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTWF_A {
        match self.bits {
            false => WUTWF_A::UpdateNotAllowed,
            true => WUTWF_A::UpdateAllowed,
        }
    }
    #[doc = "Checks if the value of the field is `UpdateNotAllowed`"]
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == WUTWF_A::UpdateNotAllowed
    }
    #[doc = "Checks if the value of the field is `UpdateAllowed`"]
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == WUTWF_A::UpdateAllowed
    }
}
impl R {
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 10:12 - BCD update"]
    #[inline(always)]
    pub fn bcdu(&self) -> BCDU_R {
        BCDU_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Binary mode"]
    #[inline(always)]
    pub fn bin(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 10:12 - BCD update"]
    #[inline(always)]
    pub fn bcdu(&mut self) -> BCDU_W<10> {
        BCDU_W::new(self)
    }
    #[doc = "Bits 8:9 - Binary mode"]
    #[inline(always)]
    pub fn bin(&mut self) -> BIN_W<8> {
        BIN_W::new(self)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<7> {
        INIT_W::new(self)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<5> {
        RSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initialization control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](index.html) module"]
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icsr::R](R) reader structure"]
impl crate::Readable for ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icsr::W](W) writer structure"]
impl crate::Writable for ICSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICSR to value 0x07"]
impl crate::Resettable for ICSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
