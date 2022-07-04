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
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRDY_A {
    #[doc = "0: ADC not yet ready to start conversion"]
    NotReady = 0,
    #[doc = "1: ADC ready to start conversion"]
    Ready = 1,
}
impl From<ADRDY_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY` reader - ADRDY"]
pub type ADRDY_R = crate::BitReader<ADRDY_A>;
impl ADRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRDY_A {
        match self.bits {
            false => ADRDY_A::NotReady,
            true => ADRDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDY_A::Ready
    }
}
#[doc = "ADRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRDY_AW {
    #[doc = "1: Clear the ADC ready flag"]
    Clear = 1,
}
impl From<ADRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY` writer - ADRDY"]
pub type ADRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, ADRDY_AW, O>;
impl<'a, const O: u8> ADRDY_W<'a, O> {
    #[doc = "Clear the ADC ready flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADRDY_AW::Clear)
    }
}
#[doc = "EOSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSMP_A {
    #[doc = "0: Not at the end of the samplings phase"]
    NotAtEnd = 0,
    #[doc = "1: End of sampling phase reached"]
    AtEnd = 1,
}
impl From<EOSMP_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP` reader - EOSMP"]
pub type EOSMP_R = crate::BitReader<EOSMP_A>;
impl EOSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSMP_A {
        match self.bits {
            false => EOSMP_A::NotAtEnd,
            true => EOSMP_A::AtEnd,
        }
    }
    #[doc = "Checks if the value of the field is `NotAtEnd`"]
    #[inline(always)]
    pub fn is_not_at_end(&self) -> bool {
        *self == EOSMP_A::NotAtEnd
    }
    #[doc = "Checks if the value of the field is `AtEnd`"]
    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        *self == EOSMP_A::AtEnd
    }
}
#[doc = "EOSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSMP_AW {
    #[doc = "1: Clear the sampling phase flag"]
    Clear = 1,
}
impl From<EOSMP_AW> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP` writer - EOSMP"]
pub type EOSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOSMP_AW, O>;
impl<'a, const O: u8> EOSMP_W<'a, O> {
    #[doc = "Clear the sampling phase flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOSMP_AW::Clear)
    }
}
#[doc = "EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_A {
    #[doc = "0: Channel conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Channel conversion complete"]
    Complete = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` reader - EOC"]
pub type EOC_R = crate::BitReader<EOC_A>;
impl EOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::NotComplete,
            true => EOC_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC_A::Complete
    }
}
#[doc = "EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_AW {
    #[doc = "1: Clear the channel conversion flag"]
    Clear = 1,
}
impl From<EOC_AW> for bool {
    #[inline(always)]
    fn from(variant: EOC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` writer - EOC"]
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOC_AW, O>;
impl<'a, const O: u8> EOC_W<'a, O> {
    #[doc = "Clear the channel conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOC_AW::Clear)
    }
}
#[doc = "EOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOS_A {
    #[doc = "0: Conversion sequence is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion sequence complete"]
    Complete = 1,
}
impl From<EOS_A> for bool {
    #[inline(always)]
    fn from(variant: EOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS` reader - EOS"]
pub type EOS_R = crate::BitReader<EOS_A>;
impl EOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOS_A {
        match self.bits {
            false => EOS_A::NotComplete,
            true => EOS_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOS_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOS_A::Complete
    }
}
#[doc = "EOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOS_AW {
    #[doc = "1: Clear the conversion sequence flag"]
    Clear = 1,
}
impl From<EOS_AW> for bool {
    #[inline(always)]
    fn from(variant: EOS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS` writer - EOS"]
pub type EOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOS_AW, O>;
impl<'a, const O: u8> EOS_W<'a, O> {
    #[doc = "Clear the conversion sequence flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOS_AW::Clear)
    }
}
#[doc = "OVR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    #[doc = "0: No overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun occurred"]
    Overrun = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` reader - OVR"]
pub type OVR_R = crate::BitReader<OVR_A>;
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NoOverrun,
            true => OVR_A::Overrun,
        }
    }
    #[doc = "Checks if the value of the field is `NoOverrun`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NoOverrun
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::Overrun
    }
}
#[doc = "OVR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_AW {
    #[doc = "1: Clear the overrun flag"]
    Clear = 1,
}
impl From<OVR_AW> for bool {
    #[inline(always)]
    fn from(variant: OVR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` writer - OVR"]
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, OVR_AW, O>;
impl<'a, const O: u8> OVR_W<'a, O> {
    #[doc = "Clear the overrun flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVR_AW::Clear)
    }
}
#[doc = "AWD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1_A {
    #[doc = "0: No analog watchdog event occurred"]
    NoEvent = 0,
    #[doc = "1: Analog watchdog event occurred"]
    Event = 1,
}
impl From<AWD1_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` reader - AWD1"]
pub type AWD1_R = crate::BitReader<AWD1_A>;
impl AWD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1_A {
        match self.bits {
            false => AWD1_A::NoEvent,
            true => AWD1_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1_A::Event
    }
}
#[doc = "AWD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1_AW {
    #[doc = "1: Clear the analog watchdog event flag"]
    Clear = 1,
}
impl From<AWD1_AW> for bool {
    #[inline(always)]
    fn from(variant: AWD1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` writer - AWD1"]
pub type AWD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, AWD1_AW, O>;
impl<'a, const O: u8> AWD1_W<'a, O> {
    #[doc = "Clear the analog watchdog event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWD1_AW::Clear)
    }
}
#[doc = "AWD2"]
pub use AWD1_A as AWD2_A;
#[doc = "AWD3"]
pub use AWD1_A as AWD3_A;
#[doc = "AWD2"]
pub use AWD1_AW as AWD2_AW;
#[doc = "AWD3"]
pub use AWD1_AW as AWD3_AW;
#[doc = "Field `AWD2` reader - AWD2"]
pub use AWD1_R as AWD2_R;
#[doc = "Field `AWD3` reader - AWD3"]
pub use AWD1_R as AWD3_R;
#[doc = "Field `AWD2` writer - AWD2"]
pub use AWD1_W as AWD2_W;
#[doc = "Field `AWD3` writer - AWD3"]
pub use AWD1_W as AWD3_W;
#[doc = "EOCAL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCAL_A {
    #[doc = "0: Calibration is not complete"]
    NotComplete = 0,
    #[doc = "1: Calibration complete"]
    Complete = 1,
}
impl From<EOCAL_A> for bool {
    #[inline(always)]
    fn from(variant: EOCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCAL` reader - EOCAL"]
pub type EOCAL_R = crate::BitReader<EOCAL_A>;
impl EOCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCAL_A {
        match self.bits {
            false => EOCAL_A::NotComplete,
            true => EOCAL_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCAL_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCAL_A::Complete
    }
}
#[doc = "EOCAL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCAL_AW {
    #[doc = "1: Clear the calibration flag"]
    Clear = 1,
}
impl From<EOCAL_AW> for bool {
    #[inline(always)]
    fn from(variant: EOCAL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCAL` writer - EOCAL"]
pub type EOCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, EOCAL_AW, O>;
impl<'a, const O: u8> EOCAL_W<'a, O> {
    #[doc = "Clear the calibration flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOCAL_AW::Clear)
    }
}
#[doc = "CCRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCRDY_A {
    #[doc = "0: Channel configuration update not applied"]
    NotComplete = 0,
    #[doc = "1: Channel configuration update is applied"]
    Complete = 1,
}
impl From<CCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRDY` reader - CCRDY"]
pub type CCRDY_R = crate::BitReader<CCRDY_A>;
impl CCRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCRDY_A {
        match self.bits {
            false => CCRDY_A::NotComplete,
            true => CCRDY_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CCRDY_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CCRDY_A::Complete
    }
}
#[doc = "CCRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCRDY_AW {
    #[doc = "1: Clear the channel configuration flag"]
    Clear = 1,
}
impl From<CCRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: CCRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRDY` writer - CCRDY"]
pub type CCRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, CCRDY_AW, O>;
impl<'a, const O: u8> CCRDY_W<'a, O> {
    #[doc = "Clear the channel configuration flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRDY_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - EOCAL"]
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - CCRDY"]
    #[inline(always)]
    pub fn ccrdy(&self) -> CCRDY_R {
        CCRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W<0> {
        ADRDY_W::new(self)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W<1> {
        EOSMP_W::new(self)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<2> {
        EOC_W::new(self)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W<3> {
        EOS_W::new(self)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<4> {
        OVR_W::new(self)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    pub fn awd1(&mut self) -> AWD1_W<7> {
        AWD1_W::new(self)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    pub fn awd2(&mut self) -> AWD2_W<8> {
        AWD2_W::new(self)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    pub fn awd3(&mut self) -> AWD3_W<9> {
        AWD3_W::new(self)
    }
    #[doc = "Bit 11 - EOCAL"]
    #[inline(always)]
    pub fn eocal(&mut self) -> EOCAL_W<11> {
        EOCAL_W::new(self)
    }
    #[doc = "Bit 13 - CCRDY"]
    #[inline(always)]
    pub fn ccrdy(&mut self) -> CCRDY_W<13> {
        CCRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
