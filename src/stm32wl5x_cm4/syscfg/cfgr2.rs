#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM2 parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPF_A {
    #[doc = "0: No SRAM2 parity error detected"]
    Nominal = 0,
    #[doc = "1: SRAM2 parity error detected"]
    Error = 1,
}
impl From<SPF_A> for bool {
    #[inline(always)]
    fn from(variant: SPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPF` reader - SRAM2 parity error flag"]
pub type SPF_R = crate::BitReader<SPF_A>;
impl SPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPF_A {
        match self.bits {
            false => SPF_A::Nominal,
            true => SPF_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `Nominal`"]
    #[inline(always)]
    pub fn is_nominal(&self) -> bool {
        *self == SPF_A::Nominal
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SPF_A::Error
    }
}
#[doc = "SRAM2 parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPF_AW {
    #[doc = "1: Clear SRAM2 parity error flag"]
    Clear = 1,
}
impl From<SPF_AW> for bool {
    #[inline(always)]
    fn from(variant: SPF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPF` writer - SRAM2 parity error flag"]
pub type SPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SPF_AW, O>;
impl<'a, const O: u8> SPF_W<'a, O> {
    #[doc = "Clear SRAM2 parity error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SPF_AW::Clear)
    }
}
#[doc = "ECC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCL_A {
    #[doc = "0: ECC error disconnected from TIM1/16/17 break input"]
    Disconnected = 0,
    #[doc = "1: ECC error connected to TIM1/16/17 break input"]
    Connected = 1,
}
impl From<ECCL_A> for bool {
    #[inline(always)]
    fn from(variant: ECCL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` reader - ECC Lock"]
pub type ECCL_R = crate::BitReader<ECCL_A>;
impl ECCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCL_A {
        match self.bits {
            false => ECCL_A::Disconnected,
            true => ECCL_A::Connected,
        }
    }
    #[doc = "Checks if the value of the field is `Disconnected`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == ECCL_A::Disconnected
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == ECCL_A::Connected
    }
}
#[doc = "ECC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCL_AW {
    #[doc = "1: Connect ECC error to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<ECCL_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` writer - ECC Lock"]
pub type ECCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ECCL_AW, O>;
impl<'a, const O: u8> ECCL_W<'a, O> {
    #[doc = "Connect ECC error to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(ECCL_AW::Connect)
    }
}
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVDL_A {
    #[doc = "0: PVD interrupt disconnected from TIM1/16/17 break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application"]
    Disconnected = 0,
    #[doc = "1: PVD interrupt connected to TIM1/16/17 break input. PVDE and PLS\\[2:0\\]
bits are read only"]
    Connected = 1,
}
impl From<PVDL_A> for bool {
    #[inline(always)]
    fn from(variant: PVDL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` reader - PVD lock enable bit"]
pub type PVDL_R = crate::BitReader<PVDL_A>;
impl PVDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDL_A {
        match self.bits {
            false => PVDL_A::Disconnected,
            true => PVDL_A::Connected,
        }
    }
    #[doc = "Checks if the value of the field is `Disconnected`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PVDL_A::Disconnected
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == PVDL_A::Connected
    }
}
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVDL_AW {
    #[doc = "1: Connect PVD interretup to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<PVDL_AW> for bool {
    #[inline(always)]
    fn from(variant: PVDL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` writer - PVD lock enable bit"]
pub type PVDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, PVDL_AW, O>;
impl<'a, const O: u8> PVDL_W<'a, O> {
    #[doc = "Connect PVD interretup to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(PVDL_AW::Connect)
    }
}
#[doc = "SRAM2 parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPL_A {
    #[doc = "0: SRAM2 parity error signal disconnected from TIM1/16/17 break input"]
    Disconnected = 0,
    #[doc = "1: SRAM2 parity error signal connected to TIM1/16/17 break input"]
    Connected = 1,
}
impl From<SPL_A> for bool {
    #[inline(always)]
    fn from(variant: SPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPL` reader - SRAM2 parity lock bit"]
pub type SPL_R = crate::BitReader<SPL_A>;
impl SPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPL_A {
        match self.bits {
            false => SPL_A::Disconnected,
            true => SPL_A::Connected,
        }
    }
    #[doc = "Checks if the value of the field is `Disconnected`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == SPL_A::Disconnected
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == SPL_A::Connected
    }
}
#[doc = "SRAM2 parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPL_AW {
    #[doc = "1: Connect SRAM2 parity error signal to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<SPL_AW> for bool {
    #[inline(always)]
    fn from(variant: SPL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPL` writer - SRAM2 parity lock bit"]
pub type SPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SPL_AW, O>;
impl<'a, const O: u8> SPL_W<'a, O> {
    #[doc = "Connect SRAM2 parity error signal to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(SPL_AW::Connect)
    }
}
#[doc = "CPU1 LOCKUP (Hardfault) output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLL_A {
    #[doc = "0: CPU LOCKUP output disconnected from TIM1/16/17 break input"]
    Disconnected = 0,
    #[doc = "1: CPU LOCKUP output connected to TIM1/16/17 break input"]
    Connected = 1,
}
impl From<CLL_A> for bool {
    #[inline(always)]
    fn from(variant: CLL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` reader - CPU1 LOCKUP (Hardfault) output enable bit"]
pub type CLL_R = crate::BitReader<CLL_A>;
impl CLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLL_A {
        match self.bits {
            false => CLL_A::Disconnected,
            true => CLL_A::Connected,
        }
    }
    #[doc = "Checks if the value of the field is `Disconnected`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == CLL_A::Disconnected
    }
    #[doc = "Checks if the value of the field is `Connected`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CLL_A::Connected
    }
}
#[doc = "CPU1 LOCKUP (Hardfault) output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLL_AW {
    #[doc = "1: Connect CPU LOCKUP output to TIM1/16/17 break input"]
    Connect = 1,
}
impl From<CLL_AW> for bool {
    #[inline(always)]
    fn from(variant: CLL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` writer - CPU1 LOCKUP (Hardfault) output enable bit"]
pub type CLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, CLL_AW, O>;
impl<'a, const O: u8> CLL_W<'a, O> {
    #[doc = "Connect CPU LOCKUP output to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(CLL_AW::Connect)
    }
}
impl R {
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - CPU1 LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&mut self) -> SPF_W<8> {
        SPF_W::new(self)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    pub fn eccl(&mut self) -> ECCL_W<3> {
        ECCL_W::new(self)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<2> {
        PVDL_W::new(self)
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    pub fn spl(&mut self) -> SPL_W<1> {
        SPL_W::new(self)
    }
    #[doc = "Bit 0 - CPU1 LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    pub fn cll(&mut self) -> CLL_W<0> {
        CLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
