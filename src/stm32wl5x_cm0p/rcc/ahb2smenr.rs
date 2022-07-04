#[doc = "Register `AHB2SMENR` reader"]
pub struct R(crate::R<AHB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2SMENR` writer"]
pub struct W(crate::W<AHB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2SMENR_SPEC>;
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
impl From<crate::W<AHB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOHSMEN` reader - IO port H clock enable during CPU1 CSleep mode."]
pub type GPIOHSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOHSMEN` writer - IO port H clock enable during CPU1 CSleep mode."]
pub type GPIOHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOCSMEN` reader - IO port C clock enable during CPU1 CSleep mode."]
pub type GPIOCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCSMEN` writer - IO port C clock enable during CPU1 CSleep mode."]
pub type GPIOCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOBSMEN` reader - IO port B clock enable during CPU1 CSleep mode."]
pub type GPIOBSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBSMEN` writer - IO port B clock enable during CPU1 CSleep mode."]
pub type GPIOBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOASMEN` reader - IO port A clock enable during CPU1 CSleep mode."]
pub type GPIOASMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOASMEN` writer - IO port A clock enable during CPU1 CSleep mode."]
pub type GPIOASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2SMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - IO port H clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IO port A clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - IO port H clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<7> {
        GPIOHSMEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<2> {
        GPIOCSMEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<1> {
        GPIOBSMEN_W::new(self)
    }
    #[doc = "Bit 0 - IO port A clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<0> {
        GPIOASMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clocks enable in Sleep modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2smenr](index.html) module"]
pub struct AHB2SMENR_SPEC;
impl crate::RegisterSpec for AHB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2smenr::R](R) reader structure"]
impl crate::Readable for AHB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2smenr::W](W) writer structure"]
impl crate::Writable for AHB2SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2SMENR to value 0x87"]
impl crate::Resettable for AHB2SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x87
    }
}
