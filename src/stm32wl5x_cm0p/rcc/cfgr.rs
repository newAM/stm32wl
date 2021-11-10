///Register `CFGR` reader
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR` writer
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Microcontroller clock output prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCOPRE_A {
    ///0: No division
    DIV1 = 0,
    ///1: Division by 2
    DIV2 = 1,
    ///2: Division by 4
    DIV4 = 2,
    ///3: Division by 8
    DIV8 = 3,
    ///4: Division by 16
    DIV16 = 4,
}
impl From<MCOPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE_A) -> Self {
        variant as _
    }
}
///Field `MCOPRE` reader - Microcontroller clock output prescaler
pub struct MCOPRE_R(crate::FieldReader<u8, MCOPRE_A>);
impl MCOPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCOPRE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MCOPRE_A> {
        match self.bits {
            0 => Some(MCOPRE_A::DIV1),
            1 => Some(MCOPRE_A::DIV2),
            2 => Some(MCOPRE_A::DIV4),
            3 => Some(MCOPRE_A::DIV8),
            4 => Some(MCOPRE_A::DIV16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == MCOPRE_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == MCOPRE_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == MCOPRE_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == MCOPRE_A::DIV8
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == MCOPRE_A::DIV16
    }
}
impl core::ops::Deref for MCOPRE_R {
    type Target = crate::FieldReader<u8, MCOPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MCOPRE` writer - Microcontroller clock output prescaler
pub struct MCOPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOPRE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MCOPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No division
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV1)
    }
    ///Division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV2)
    }
    ///Division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV4)
    }
    ///Division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV8)
    }
    ///Division by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
///Microcontroller clock output
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCOSEL_A {
    ///0: No clock
    NOCLOCK = 0,
    ///1: SYSCLK clock selected
    SYSCLK = 1,
    ///2: MSI oscillator clock selected
    MSI = 2,
    ///3: HSI16 oscillator clock selected
    HSI16 = 3,
    ///4: HSE32 oscillator clock selected
    HSE32 = 4,
    ///5: Main PLLRCLK clock selected
    PLLR = 5,
    ///6: LSI oscillator clock selected
    LSI = 6,
    ///8: LSE oscillator clock selected
    LSE = 8,
    ///13: Main PLLPCLK clock selected
    PLLP = 13,
    ///14: Main PLLQCLK clock selected
    PLLQ = 14,
}
impl From<MCOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOSEL_A) -> Self {
        variant as _
    }
}
///Field `MCOSEL` reader - Microcontroller clock output
pub struct MCOSEL_R(crate::FieldReader<u8, MCOSEL_A>);
impl MCOSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCOSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MCOSEL_A> {
        match self.bits {
            0 => Some(MCOSEL_A::NOCLOCK),
            1 => Some(MCOSEL_A::SYSCLK),
            2 => Some(MCOSEL_A::MSI),
            3 => Some(MCOSEL_A::HSI16),
            4 => Some(MCOSEL_A::HSE32),
            5 => Some(MCOSEL_A::PLLR),
            6 => Some(MCOSEL_A::LSI),
            8 => Some(MCOSEL_A::LSE),
            13 => Some(MCOSEL_A::PLLP),
            14 => Some(MCOSEL_A::PLLQ),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NOCLOCK`
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        **self == MCOSEL_A::NOCLOCK
    }
    ///Checks if the value of the field is `SYSCLK`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == MCOSEL_A::SYSCLK
    }
    ///Checks if the value of the field is `MSI`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == MCOSEL_A::MSI
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == MCOSEL_A::HSI16
    }
    ///Checks if the value of the field is `HSE32`
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        **self == MCOSEL_A::HSE32
    }
    ///Checks if the value of the field is `PLLR`
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        **self == MCOSEL_A::PLLR
    }
    ///Checks if the value of the field is `LSI`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == MCOSEL_A::LSI
    }
    ///Checks if the value of the field is `LSE`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == MCOSEL_A::LSE
    }
    ///Checks if the value of the field is `PLLP`
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        **self == MCOSEL_A::PLLP
    }
    ///Checks if the value of the field is `PLLQ`
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        **self == MCOSEL_A::PLLQ
    }
}
impl core::ops::Deref for MCOSEL_R {
    type Target = crate::FieldReader<u8, MCOSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MCOSEL` writer - Microcontroller clock output
pub struct MCOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MCOSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(MCOSEL_A::NOCLOCK)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCOSEL_A::SYSCLK)
    }
    ///MSI oscillator clock selected
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(MCOSEL_A::MSI)
    }
    ///HSI16 oscillator clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(MCOSEL_A::HSI16)
    }
    ///HSE32 oscillator clock selected
    #[inline(always)]
    pub fn hse32(self) -> &'a mut W {
        self.variant(MCOSEL_A::HSE32)
    }
    ///Main PLLRCLK clock selected
    #[inline(always)]
    pub fn pllr(self) -> &'a mut W {
        self.variant(MCOSEL_A::PLLR)
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCOSEL_A::LSI)
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCOSEL_A::LSE)
    }
    ///Main PLLPCLK clock selected
    #[inline(always)]
    pub fn pllp(self) -> &'a mut W {
        self.variant(MCOSEL_A::PLLP)
    }
    ///Main PLLQCLK clock selected
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(MCOSEL_A::PLLQ)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
///PCLK2 prescaler flag (APB2)
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRE2F_A {
    ///0: PCLK2 prescaler value not yet applied
    NOTAPPLIED = 0,
    ///1: PCLK2 prescaler value applied
    APPLIED = 1,
}
impl From<PPRE2F_A> for bool {
    #[inline(always)]
    fn from(variant: PPRE2F_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PPRE2F` reader - PCLK2 prescaler flag (APB2)
pub struct PPRE2F_R(crate::FieldReader<bool, PPRE2F_A>);
impl PPRE2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPRE2F_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PPRE2F_A {
        match self.bits {
            false => PPRE2F_A::NOTAPPLIED,
            true => PPRE2F_A::APPLIED,
        }
    }
    ///Checks if the value of the field is `NOTAPPLIED`
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        **self == PPRE2F_A::NOTAPPLIED
    }
    ///Checks if the value of the field is `APPLIED`
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        **self == PPRE2F_A::APPLIED
    }
}
impl core::ops::Deref for PPRE2F_R {
    type Target = crate::FieldReader<bool, PPRE2F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///PCLK1 prescaler flag (APB1)
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRE1F_A {
    ///0: PCLK1 prescaler value not yet applied
    NOTAPPLIED = 0,
    ///1: PCLK1 prescaler value applied
    APPLIED = 1,
}
impl From<PPRE1F_A> for bool {
    #[inline(always)]
    fn from(variant: PPRE1F_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PPRE1F` reader - PCLK1 prescaler flag (APB1)
pub struct PPRE1F_R(crate::FieldReader<bool, PPRE1F_A>);
impl PPRE1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPRE1F_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PPRE1F_A {
        match self.bits {
            false => PPRE1F_A::NOTAPPLIED,
            true => PPRE1F_A::APPLIED,
        }
    }
    ///Checks if the value of the field is `NOTAPPLIED`
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        **self == PPRE1F_A::NOTAPPLIED
    }
    ///Checks if the value of the field is `APPLIED`
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        **self == PPRE1F_A::APPLIED
    }
}
impl core::ops::Deref for PPRE1F_R {
    type Target = crate::FieldReader<bool, PPRE1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPREF_A {
    ///0: HCLK1 prescaler value not yet applied
    NOTAPPLIED = 0,
    ///1: HCLK1 prescaler value applied
    APPLIED = 1,
}
impl From<HPREF_A> for bool {
    #[inline(always)]
    fn from(variant: HPREF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HPREF` reader - HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)
pub struct HPREF_R(crate::FieldReader<bool, HPREF_A>);
impl HPREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPREF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HPREF_A {
        match self.bits {
            false => HPREF_A::NOTAPPLIED,
            true => HPREF_A::APPLIED,
        }
    }
    ///Checks if the value of the field is `NOTAPPLIED`
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        **self == HPREF_A::NOTAPPLIED
    }
    ///Checks if the value of the field is `APPLIED`
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        **self == HPREF_A::APPLIED
    }
}
impl core::ops::Deref for HPREF_R {
    type Target = crate::FieldReader<bool, HPREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Wakeup from Stop and CSS backup clock selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPWUCK_A {
    ///0: MSI oscillator selected as wakeup from stop clock and CSS backup clock
    MSI = 0,
    ///1: HSI16 oscillator selected as wakeup from stop clock and CSS backup clock
    HSI16 = 1,
}
impl From<STOPWUCK_A> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPWUCK` reader - Wakeup from Stop and CSS backup clock selection
pub struct STOPWUCK_R(crate::FieldReader<bool, STOPWUCK_A>);
impl STOPWUCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPWUCK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPWUCK_A {
        match self.bits {
            false => STOPWUCK_A::MSI,
            true => STOPWUCK_A::HSI16,
        }
    }
    ///Checks if the value of the field is `MSI`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == STOPWUCK_A::MSI
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == STOPWUCK_A::HSI16
    }
}
impl core::ops::Deref for STOPWUCK_R {
    type Target = crate::FieldReader<bool, STOPWUCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `STOPWUCK` writer - Wakeup from Stop and CSS backup clock selection
pub struct STOPWUCK_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPWUCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STOPWUCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///MSI oscillator selected as wakeup from stop clock and CSS backup clock
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::MSI)
    }
    ///HSI16 oscillator selected as wakeup from stop clock and CSS backup clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(STOPWUCK_A::HSI16)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///PCLK2 high-speed prescaler (APB2)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PPRE2_A {
    ///0: HCLK not divided
    DIV1 = 0,
    ///4: HCLK divided by 2
    DIV2 = 4,
    ///5: HCLK divided by 4
    DIV4 = 5,
    ///6: HCLK divided by 8
    DIV8 = 6,
    ///7: HCLK divided by 16
    DIV16 = 7,
}
impl From<PPRE2_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE2_A) -> Self {
        variant as _
    }
}
///Field `PPRE2` reader - PCLK2 high-speed prescaler (APB2)
pub struct PPRE2_R(crate::FieldReader<u8, PPRE2_A>);
impl PPRE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPRE2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE2_A> {
        match self.bits {
            0 => Some(PPRE2_A::DIV1),
            4 => Some(PPRE2_A::DIV2),
            5 => Some(PPRE2_A::DIV4),
            6 => Some(PPRE2_A::DIV8),
            7 => Some(PPRE2_A::DIV16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PPRE2_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PPRE2_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PPRE2_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PPRE2_A::DIV8
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PPRE2_A::DIV16
    }
}
impl core::ops::Deref for PPRE2_R {
    type Target = crate::FieldReader<u8, PPRE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PPRE2` writer - PCLK2 high-speed prescaler (APB2)
pub struct PPRE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PPRE2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///HCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV1)
    }
    ///HCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV2)
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV4)
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV8)
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
///PCLK1 low-speed prescaler (APB1)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PPRE1_A {
    ///0: HCLK not divided
    DIV1 = 0,
    ///4: HCLK divided by 2
    DIV2 = 4,
    ///5: HCLK divided by 4
    DIV4 = 5,
    ///6: HCLK divided by 8
    DIV8 = 6,
    ///7: HCLK divided by 16
    DIV16 = 7,
}
impl From<PPRE1_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1_A) -> Self {
        variant as _
    }
}
///Field `PPRE1` reader - PCLK1 low-speed prescaler (APB1)
pub struct PPRE1_R(crate::FieldReader<u8, PPRE1_A>);
impl PPRE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPRE1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE1_A> {
        match self.bits {
            0 => Some(PPRE1_A::DIV1),
            4 => Some(PPRE1_A::DIV2),
            5 => Some(PPRE1_A::DIV4),
            6 => Some(PPRE1_A::DIV8),
            7 => Some(PPRE1_A::DIV16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PPRE1_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PPRE1_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PPRE1_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PPRE1_A::DIV8
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PPRE1_A::DIV16
    }
}
impl core::ops::Deref for PPRE1_R {
    type Target = crate::FieldReader<u8, PPRE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PPRE1` writer - PCLK1 low-speed prescaler (APB1)
pub struct PPRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PPRE1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///HCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV1)
    }
    ///HCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV2)
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV4)
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV8)
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
///HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HPRE_A {
    ///0: SYSCLK not divided
    DIV1 = 0,
    ///1: SYSCLK divided by 3
    DIV3 = 1,
    ///2: SYSCLK divided by 5
    DIV5 = 2,
    ///5: SYSCLK divided by 6
    DIV6 = 5,
    ///6: SYSCLK divided by 10
    DIV10 = 6,
    ///7: SYSCLK divided by 32
    DIV32 = 7,
    ///8: SYSCLK divided by 2
    DIV2 = 8,
    ///9: SYSCLK divided by 4
    DIV4 = 9,
    ///10: SYSCLK divided by 8
    DIV8 = 10,
    ///11: SYSCLK divided by 16
    DIV16 = 11,
    ///12: SYSCLK divided by 64
    DIV64 = 12,
    ///13: SYSCLK divided by 128
    DIV128 = 13,
    ///14: SYSCLK divided by 128
    DIV256 = 14,
    ///15: SYSCLK divided by 512
    DIV512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
///Field `HPRE` reader - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)
pub struct HPRE_R(crate::FieldReader<u8, HPRE_A>);
impl HPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        HPRE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            0 => Some(HPRE_A::DIV1),
            1 => Some(HPRE_A::DIV3),
            2 => Some(HPRE_A::DIV5),
            5 => Some(HPRE_A::DIV6),
            6 => Some(HPRE_A::DIV10),
            7 => Some(HPRE_A::DIV32),
            8 => Some(HPRE_A::DIV2),
            9 => Some(HPRE_A::DIV4),
            10 => Some(HPRE_A::DIV8),
            11 => Some(HPRE_A::DIV16),
            12 => Some(HPRE_A::DIV64),
            13 => Some(HPRE_A::DIV128),
            14 => Some(HPRE_A::DIV256),
            15 => Some(HPRE_A::DIV512),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == HPRE_A::DIV1
    }
    ///Checks if the value of the field is `DIV3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        **self == HPRE_A::DIV3
    }
    ///Checks if the value of the field is `DIV5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        **self == HPRE_A::DIV5
    }
    ///Checks if the value of the field is `DIV6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == HPRE_A::DIV6
    }
    ///Checks if the value of the field is `DIV10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        **self == HPRE_A::DIV10
    }
    ///Checks if the value of the field is `DIV32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == HPRE_A::DIV32
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == HPRE_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == HPRE_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == HPRE_A::DIV8
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == HPRE_A::DIV16
    }
    ///Checks if the value of the field is `DIV64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == HPRE_A::DIV64
    }
    ///Checks if the value of the field is `DIV128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == HPRE_A::DIV128
    }
    ///Checks if the value of the field is `DIV256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == HPRE_A::DIV256
    }
    ///Checks if the value of the field is `DIV512`
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        **self == HPRE_A::DIV512
    }
}
impl core::ops::Deref for HPRE_R {
    type Target = crate::FieldReader<u8, HPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HPRE` writer - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)
pub struct HPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HPRE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///SYSCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::DIV1)
    }
    ///SYSCLK divided by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(HPRE_A::DIV3)
    }
    ///SYSCLK divided by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(HPRE_A::DIV5)
    }
    ///SYSCLK divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(HPRE_A::DIV6)
    }
    ///SYSCLK divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(HPRE_A::DIV10)
    }
    ///SYSCLK divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(HPRE_A::DIV32)
    }
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::DIV2)
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::DIV4)
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::DIV8)
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::DIV16)
    }
    ///SYSCLK divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::DIV64)
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::DIV128)
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::DIV256)
    }
    ///SYSCLK divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::DIV512)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///System clock switch status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWS_A {
    ///0: MSI oscillator used as system clock
    MSI = 0,
    ///1: HSI16 oscillator used as system clock
    HSI16 = 1,
    ///2: HSE32 oscillator used as system clock
    HSE32 = 2,
    ///3: PLLRCLK used as system clock
    PLLR = 3,
}
impl From<SWS_A> for u8 {
    #[inline(always)]
    fn from(variant: SWS_A) -> Self {
        variant as _
    }
}
///Field `SWS` reader - System clock switch status
pub struct SWS_R(crate::FieldReader<u8, SWS_A>);
impl SWS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SWS_A {
        match self.bits {
            0 => SWS_A::MSI,
            1 => SWS_A::HSI16,
            2 => SWS_A::HSE32,
            3 => SWS_A::PLLR,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `MSI`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == SWS_A::MSI
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == SWS_A::HSI16
    }
    ///Checks if the value of the field is `HSE32`
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        **self == SWS_A::HSE32
    }
    ///Checks if the value of the field is `PLLR`
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        **self == SWS_A::PLLR
    }
}
impl core::ops::Deref for SWS_R {
    type Target = crate::FieldReader<u8, SWS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///System clock switch
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SW_A {
    ///0: MSI oscillator used as system clock
    MSI = 0,
    ///1: HSI16 oscillator used as system clock
    HSI16 = 1,
    ///2: HSE32 oscillator used as system clock
    HSE32 = 2,
    ///3: PLLRCLK used as system clock
    PLLR = 3,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
///Field `SW` reader - System clock switch
pub struct SW_R(crate::FieldReader<u8, SW_A>);
impl SW_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            0 => SW_A::MSI,
            1 => SW_A::HSI16,
            2 => SW_A::HSE32,
            3 => SW_A::PLLR,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `MSI`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == SW_A::MSI
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == SW_A::HSI16
    }
    ///Checks if the value of the field is `HSE32`
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        **self == SW_A::HSE32
    }
    ///Checks if the value of the field is `PLLR`
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        **self == SW_A::PLLR
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<u8, SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SW` writer - System clock switch
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SW_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///MSI oscillator used as system clock
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(SW_A::MSI)
    }
    ///HSI16 oscillator used as system clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(SW_A::HSI16)
    }
    ///HSE32 oscillator used as system clock
    #[inline(always)]
    pub fn hse32(self) -> &'a mut W {
        self.variant(SW_A::HSE32)
    }
    ///PLLRCLK used as system clock
    #[inline(always)]
    pub fn pllr(self) -> &'a mut W {
        self.variant(SW_A::PLLR)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 28:30 - Microcontroller clock output prescaler
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    ///Bits 24:27 - Microcontroller clock output
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 18 - PCLK2 prescaler flag (APB2)
    #[inline(always)]
    pub fn ppre2f(&self) -> PPRE2F_R {
        PPRE2F_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - PCLK1 prescaler flag (APB1)
    #[inline(always)]
    pub fn ppre1f(&self) -> PPRE1F_R {
        PPRE1F_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - HCLK1 prescaler flag (CPU1, AHB1, AHB2, and SRAM1)
    #[inline(always)]
    pub fn hpref(&self) -> HPREF_R {
        HPREF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Wakeup from Stop and CSS backup clock selection
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 11:13 - PCLK2 high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    ///Bits 8:10 - PCLK1 low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bits 4:7 - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 2:3 - System clock switch status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - System clock switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 28:30 - Microcontroller clock output prescaler
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W {
        MCOPRE_W { w: self }
    }
    ///Bits 24:27 - Microcontroller clock output
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W {
        MCOSEL_W { w: self }
    }
    ///Bit 15 - Wakeup from Stop and CSS backup clock selection
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W {
        STOPWUCK_W { w: self }
    }
    ///Bits 11:13 - PCLK2 high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W {
        PPRE2_W { w: self }
    }
    ///Bits 8:10 - PCLK1 low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W {
        PPRE1_W { w: self }
    }
    ///Bits 4:7 - HCLK1 prescaler (CPU1, AHB1, AHB2, and SRAM1.)
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W {
        HPRE_W { w: self }
    }
    ///Bits 0:1 - System clock switch
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr](index.html) module
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr::R](R) reader structure
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr::W](W) writer structure
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR to value 0x0007_0000
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_0000
    }
}
