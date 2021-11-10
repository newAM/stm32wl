///Register `CCIPR` reader
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCIPR` writer
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
///RNG clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RNGSEL_A {
    ///0: PLLQ clock selected
    PLLQ = 0,
    ///1: LSI clock selected
    LSI = 1,
    ///2: LSE clock selected
    LSE = 2,
    ///3: MSI clock selected
    MSI = 3,
}
impl From<RNGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RNGSEL_A) -> Self {
        variant as _
    }
}
///Field `RNGSEL` reader - RNG clock source selection
pub struct RNGSEL_R(crate::FieldReader<u8, RNGSEL_A>);
impl RNGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNGSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RNGSEL_A {
        match self.bits {
            0 => RNGSEL_A::PLLQ,
            1 => RNGSEL_A::LSI,
            2 => RNGSEL_A::LSE,
            3 => RNGSEL_A::MSI,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PLLQ`
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        **self == RNGSEL_A::PLLQ
    }
    ///Checks if the value of the field is `LSI`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == RNGSEL_A::LSI
    }
    ///Checks if the value of the field is `LSE`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == RNGSEL_A::LSE
    }
    ///Checks if the value of the field is `MSI`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == RNGSEL_A::MSI
    }
}
impl core::ops::Deref for RNGSEL_R {
    type Target = crate::FieldReader<u8, RNGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RNGSEL` writer - RNG clock source selection
pub struct RNGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RNGSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///PLLQ clock selected
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(RNGSEL_A::PLLQ)
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RNGSEL_A::LSI)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RNGSEL_A::LSE)
    }
    ///MSI clock selected
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(RNGSEL_A::MSI)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
///ADC clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCSEL_A {
    ///0: No clock selected
    NOCLOCK = 0,
    ///1: HSI16 clock selected
    HSI16 = 1,
    ///2: PLLP clock selected
    PLLP = 2,
    ///3: SYSCLK clock selected
    SYSCLK = 3,
}
impl From<ADCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSEL_A) -> Self {
        variant as _
    }
}
///Field `ADCSEL` reader - ADC clock source selection
pub struct ADCSEL_R(crate::FieldReader<u8, ADCSEL_A>);
impl ADCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADCSEL_A {
        match self.bits {
            0 => ADCSEL_A::NOCLOCK,
            1 => ADCSEL_A::HSI16,
            2 => ADCSEL_A::PLLP,
            3 => ADCSEL_A::SYSCLK,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOCLOCK`
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        **self == ADCSEL_A::NOCLOCK
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == ADCSEL_A::HSI16
    }
    ///Checks if the value of the field is `PLLP`
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        **self == ADCSEL_A::PLLP
    }
    ///Checks if the value of the field is `SYSCLK`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == ADCSEL_A::SYSCLK
    }
}
impl core::ops::Deref for ADCSEL_R {
    type Target = crate::FieldReader<u8, ADCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ADCSEL` writer - ADC clock source selection
pub struct ADCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No clock selected
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(ADCSEL_A::NOCLOCK)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(ADCSEL_A::HSI16)
    }
    ///PLLP clock selected
    #[inline(always)]
    pub fn pllp(self) -> &'a mut W {
        self.variant(ADCSEL_A::PLLP)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(ADCSEL_A::SYSCLK)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
///Low power timer 3 clock source selection
pub type LPTIM3SEL_A = LPTIM1SEL_A;
///Field `LPTIM3SEL` reader - Low power timer 3 clock source selection
pub type LPTIM3SEL_R = LPTIM1SEL_R;
///Field `LPTIM3SEL` writer - Low power timer 3 clock source selection
pub struct LPTIM3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPTIM3SEL_A::PCLK)
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM3SEL_A::LSI)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPTIM3SEL_A::HSI16)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM3SEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
///Low power timer 2 clock source selection
pub type LPTIM2SEL_A = LPTIM1SEL_A;
///Field `LPTIM2SEL` reader - Low power timer 2 clock source selection
pub type LPTIM2SEL_R = LPTIM1SEL_R;
///Field `LPTIM2SEL` writer - Low power timer 2 clock source selection
pub struct LPTIM2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::PCLK)
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::LSI)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::HSI16)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
///Low power timer 1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    ///0: PCLK clock selected
    PCLK = 0,
    ///1: LSI clock selected
    LSI = 1,
    ///2: HSI16 clock selected
    HSI16 = 2,
    ///3: LSE clock selected
    LSE = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
///Field `LPTIM1SEL` reader - Low power timer 1 clock source selection
pub struct LPTIM1SEL_R(crate::FieldReader<u8, LPTIM1SEL_A>);
impl LPTIM1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM1SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::PCLK,
            1 => LPTIM1SEL_A::LSI,
            2 => LPTIM1SEL_A::HSI16,
            3 => LPTIM1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PCLK`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == LPTIM1SEL_A::PCLK
    }
    ///Checks if the value of the field is `LSI`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == LPTIM1SEL_A::LSI
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == LPTIM1SEL_A::HSI16
    }
    ///Checks if the value of the field is `LSE`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == LPTIM1SEL_A::LSE
    }
}
impl core::ops::Deref for LPTIM1SEL_R {
    type Target = crate::FieldReader<u8, LPTIM1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPTIM1SEL` writer - Low power timer 1 clock source selection
pub struct LPTIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::PCLK)
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSI)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::HSI16)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
///I2C3 clock source selection
pub type I2C3SEL_A = I2C1SEL_A;
///Field `I2C3SEL` reader - I2C3 clock source selection
pub type I2C3SEL_R = I2C1SEL_R;
///Field `I2C3SEL` writer - I2C3 clock source selection
pub struct I2C3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C3SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C3SEL_A::PCLK)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C3SEL_A::SYSCLK)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C3SEL_A::HSI16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
///I2C2 clock source selection
pub type I2C2SEL_A = I2C1SEL_A;
///Field `I2C2SEL` reader - I2C2 clock source selection
pub type I2C2SEL_R = I2C1SEL_R;
///Field `I2C2SEL` writer - I2C2 clock source selection
pub struct I2C2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C2SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C2SEL_A::PCLK)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C2SEL_A::SYSCLK)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C2SEL_A::HSI16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
///I2C1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    ///0: PCLK clock selected
    PCLK = 0,
    ///1: SYSCLK clock selected
    SYSCLK = 1,
    ///2: HSI16 clock selected
    HSI16 = 2,
}
impl From<I2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL_A) -> Self {
        variant as _
    }
}
///Field `I2C1SEL` reader - I2C1 clock source selection
pub struct I2C1SEL_R(crate::FieldReader<u8, I2C1SEL_A>);
impl I2C1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C1SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1SEL_A> {
        match self.bits {
            0 => Some(I2C1SEL_A::PCLK),
            1 => Some(I2C1SEL_A::SYSCLK),
            2 => Some(I2C1SEL_A::HSI16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PCLK`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == I2C1SEL_A::PCLK
    }
    ///Checks if the value of the field is `SYSCLK`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == I2C1SEL_A::SYSCLK
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == I2C1SEL_A::HSI16
    }
}
impl core::ops::Deref for I2C1SEL_R {
    type Target = crate::FieldReader<u8, I2C1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C1SEL` writer - I2C1 clock source selection
pub struct I2C1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::PCLK)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::SYSCLK)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C1SEL_A::HSI16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
///LPUART1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART1SEL_A {
    ///0: PCLK clock selected
    PCLK = 0,
    ///1: SYSCLK clock selected
    SYSCLK = 1,
    ///2: HSI16 clock selected
    HSI16 = 2,
    ///3: LSE clock selected
    LSE = 3,
}
impl From<LPUART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL_A) -> Self {
        variant as _
    }
}
///Field `LPUART1SEL` reader - LPUART1 clock source selection
pub struct LPUART1SEL_R(crate::FieldReader<u8, LPUART1SEL_A>);
impl LPUART1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPUART1SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPUART1SEL_A {
        match self.bits {
            0 => LPUART1SEL_A::PCLK,
            1 => LPUART1SEL_A::SYSCLK,
            2 => LPUART1SEL_A::HSI16,
            3 => LPUART1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PCLK`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == LPUART1SEL_A::PCLK
    }
    ///Checks if the value of the field is `SYSCLK`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == LPUART1SEL_A::SYSCLK
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == LPUART1SEL_A::HSI16
    }
    ///Checks if the value of the field is `LSE`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == LPUART1SEL_A::LSE
    }
}
impl core::ops::Deref for LPUART1SEL_R {
    type Target = crate::FieldReader<u8, LPUART1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPUART1SEL` writer - LPUART1 clock source selection
pub struct LPUART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPUART1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::PCLK)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::SYSCLK)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::HSI16)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///SPI2S2 I2S clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI2S2SEL_A {
    ///1: PLLQ clock selected
    PLLQ = 1,
    ///2: HSI16 clock selected
    HSI16 = 2,
    ///3: External input I2S_CKIN selected
    I2S = 3,
}
impl From<SPI2S2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI2S2SEL_A) -> Self {
        variant as _
    }
}
///Field `SPI2S2SEL` reader - SPI2S2 I2S clock source selection
pub struct SPI2S2SEL_R(crate::FieldReader<u8, SPI2S2SEL_A>);
impl SPI2S2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI2S2SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI2S2SEL_A> {
        match self.bits {
            1 => Some(SPI2S2SEL_A::PLLQ),
            2 => Some(SPI2S2SEL_A::HSI16),
            3 => Some(SPI2S2SEL_A::I2S),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PLLQ`
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        **self == SPI2S2SEL_A::PLLQ
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == SPI2S2SEL_A::HSI16
    }
    ///Checks if the value of the field is `I2S`
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        **self == SPI2S2SEL_A::I2S
    }
}
impl core::ops::Deref for SPI2S2SEL_R {
    type Target = crate::FieldReader<u8, SPI2S2SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SPI2S2SEL` writer - SPI2S2 I2S clock source selection
pub struct SPI2S2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2S2SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI2S2SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///PLLQ clock selected
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(SPI2S2SEL_A::PLLQ)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(SPI2S2SEL_A::HSI16)
    }
    ///External input I2S_CKIN selected
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(SPI2S2SEL_A::I2S)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///USART2 clock source selection
pub type USART2SEL_A = USART1SEL_A;
///Field `USART2SEL` reader - USART2 clock source selection
pub type USART2SEL_R = USART1SEL_R;
///Field `USART2SEL` writer - USART2 clock source selection
pub struct USART2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART2SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART2SEL_A::PCLK)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART2SEL_A::SYSCLK)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(USART2SEL_A::HSI16)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///USART1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART1SEL_A {
    ///0: PCLK clock selected
    PCLK = 0,
    ///1: SYSCLK clock selected
    SYSCLK = 1,
    ///2: HSI16 clock selected
    HSI16 = 2,
    ///3: LSE clock selected
    LSE = 3,
}
impl From<USART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL_A) -> Self {
        variant as _
    }
}
///Field `USART1SEL` reader - USART1 clock source selection
pub struct USART1SEL_R(crate::FieldReader<u8, USART1SEL_A>);
impl USART1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART1SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USART1SEL_A {
        match self.bits {
            0 => USART1SEL_A::PCLK,
            1 => USART1SEL_A::SYSCLK,
            2 => USART1SEL_A::HSI16,
            3 => USART1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PCLK`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == USART1SEL_A::PCLK
    }
    ///Checks if the value of the field is `SYSCLK`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == USART1SEL_A::SYSCLK
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == USART1SEL_A::HSI16
    }
    ///Checks if the value of the field is `LSE`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == USART1SEL_A::LSE
    }
}
impl core::ops::Deref for USART1SEL_R {
    type Target = crate::FieldReader<u8, USART1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART1SEL` writer - USART1 clock source selection
pub struct USART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///PCLK clock selected
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART1SEL_A::PCLK)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART1SEL_A::SYSCLK)
    }
    ///HSI16 clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(USART1SEL_A::HSI16)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 30:31 - RNG clock source selection
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    ///Bits 28:29 - ADC clock source selection
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 22:23 - Low power timer 3 clock source selection
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    ///Bits 20:21 - Low power timer 2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bits 18:19 - Low power timer 1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bits 16:17 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 14:15 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bits 12:13 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bits 10:11 - LPUART1 clock source selection
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 8:9 - SPI2S2 I2S clock source selection
    #[inline(always)]
    pub fn spi2s2sel(&self) -> SPI2S2SEL_R {
        SPI2S2SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 2:3 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 30:31 - RNG clock source selection
    #[inline(always)]
    pub fn rngsel(&mut self) -> RNGSEL_W {
        RNGSEL_W { w: self }
    }
    ///Bits 28:29 - ADC clock source selection
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W {
        ADCSEL_W { w: self }
    }
    ///Bits 22:23 - Low power timer 3 clock source selection
    #[inline(always)]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W {
        LPTIM3SEL_W { w: self }
    }
    ///Bits 20:21 - Low power timer 2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W {
        LPTIM2SEL_W { w: self }
    }
    ///Bits 18:19 - Low power timer 1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
    ///Bits 16:17 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W {
        I2C3SEL_W { w: self }
    }
    ///Bits 14:15 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W {
        I2C2SEL_W { w: self }
    }
    ///Bits 12:13 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W {
        I2C1SEL_W { w: self }
    }
    ///Bits 10:11 - LPUART1 clock source selection
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W {
        LPUART1SEL_W { w: self }
    }
    ///Bits 8:9 - SPI2S2 I2S clock source selection
    #[inline(always)]
    pub fn spi2s2sel(&mut self) -> SPI2S2SEL_W {
        SPI2S2SEL_W { w: self }
    }
    ///Bits 2:3 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W {
        USART2SEL_W { w: self }
    }
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W {
        USART1SEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Peripherals independent clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr](index.html) module
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccipr::R](R) reader structure
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccipr::W](W) writer structure
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
}
///`reset()` method sets CCIPR to value 0
impl crate::Resettable for CCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
