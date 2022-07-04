#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Compare 6 interrupt flag"]
pub use CC1IF_A as CC6IF_A;
#[doc = "Compare 5 interrupt flag"]
pub use CC1IF_A as CC5IF_A;
#[doc = "Compare 6 interrupt flag"]
pub use CC1IF_AW as CC6IF_AW;
#[doc = "Compare 5 interrupt flag"]
pub use CC1IF_AW as CC5IF_AW;
#[doc = "Field `CC6IF` reader - Compare 6 interrupt flag"]
pub use CC1IF_R as CC6IF_R;
#[doc = "Field `CC5IF` reader - Compare 5 interrupt flag"]
pub use CC1IF_R as CC5IF_R;
#[doc = "Field `CC6IF` writer - Compare 6 interrupt flag"]
pub use CC1IF_W as CC6IF_W;
#[doc = "Field `CC5IF` writer - Compare 5 interrupt flag"]
pub use CC1IF_W as CC5IF_W;
#[doc = "System Break interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBIF_A {
    #[doc = "0: No break event occurred"]
    NoTrigger = 0,
    #[doc = "1: An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register"]
    Trigger = 1,
}
impl From<SBIF_A> for bool {
    #[inline(always)]
    fn from(variant: SBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBIF` reader - System Break interrupt flag"]
pub type SBIF_R = crate::BitReader<SBIF_A>;
impl SBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBIF_A {
        match self.bits {
            false => SBIF_A::NoTrigger,
            true => SBIF_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoTrigger`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SBIF_A::NoTrigger
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == SBIF_A::Trigger
    }
}
#[doc = "System Break interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBIF_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<SBIF_AW> for bool {
    #[inline(always)]
    fn from(variant: SBIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBIF` writer - System Break interrupt flag"]
pub type SBIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, SBIF_AW, O>;
impl<'a, const O: u8> SBIF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SBIF_AW::Clear)
    }
}
#[doc = "Capture/Compare 4 overcapture flag"]
pub use CC1OF_A as CC4OF_A;
#[doc = "Capture/Compare 3 overcapture flag"]
pub use CC1OF_A as CC3OF_A;
#[doc = "Capture/Compare 2 overcapture flag"]
pub use CC1OF_A as CC2OF_A;
#[doc = "Capture/Compare 4 overcapture flag"]
pub use CC1OF_AW as CC4OF_AW;
#[doc = "Capture/Compare 3 overcapture flag"]
pub use CC1OF_AW as CC3OF_AW;
#[doc = "Capture/Compare 2 overcapture flag"]
pub use CC1OF_AW as CC2OF_AW;
#[doc = "Field `CC4OF` reader - Capture/Compare 4 overcapture flag"]
pub use CC1OF_R as CC4OF_R;
#[doc = "Field `CC3OF` reader - Capture/Compare 3 overcapture flag"]
pub use CC1OF_R as CC3OF_R;
#[doc = "Field `CC2OF` reader - Capture/Compare 2 overcapture flag"]
pub use CC1OF_R as CC2OF_R;
#[doc = "Field `CC4OF` writer - Capture/Compare 4 overcapture flag"]
pub use CC1OF_W as CC4OF_W;
#[doc = "Field `CC3OF` writer - Capture/Compare 3 overcapture flag"]
pub use CC1OF_W as CC3OF_W;
#[doc = "Field `CC2OF` writer - Capture/Compare 2 overcapture flag"]
pub use CC1OF_W as CC2OF_W;
#[doc = "Capture/Compare 1 overcapture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1OF_A {
    #[doc = "0: No overcapture has been detected"]
    NoOvercapture = 0,
    #[doc = "1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set"]
    Overcapture = 1,
}
impl From<CC1OF_A> for bool {
    #[inline(always)]
    fn from(variant: CC1OF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1OF` reader - Capture/Compare 1 overcapture flag"]
pub type CC1OF_R = crate::BitReader<CC1OF_A>;
impl CC1OF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1OF_A {
        match self.bits {
            false => CC1OF_A::NoOvercapture,
            true => CC1OF_A::Overcapture,
        }
    }
    #[doc = "Checks if the value of the field is `NoOvercapture`"]
    #[inline(always)]
    pub fn is_no_overcapture(&self) -> bool {
        *self == CC1OF_A::NoOvercapture
    }
    #[doc = "Checks if the value of the field is `Overcapture`"]
    #[inline(always)]
    pub fn is_overcapture(&self) -> bool {
        *self == CC1OF_A::Overcapture
    }
}
#[doc = "Capture/Compare 1 overcapture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1OF_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<CC1OF_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1OF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1OF` writer - Capture/Compare 1 overcapture flag"]
pub type CC1OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, CC1OF_AW, O>;
impl<'a, const O: u8> CC1OF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC1OF_AW::Clear)
    }
}
#[doc = "Break 2 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B2IF_A {
    #[doc = "0: No break event occurred"]
    NoTrigger = 0,
    #[doc = "1: An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register"]
    Trigger = 1,
}
impl From<B2IF_A> for bool {
    #[inline(always)]
    fn from(variant: B2IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B2IF` reader - Break 2 interrupt flag"]
pub type B2IF_R = crate::BitReader<B2IF_A>;
impl B2IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B2IF_A {
        match self.bits {
            false => B2IF_A::NoTrigger,
            true => B2IF_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoTrigger`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == B2IF_A::NoTrigger
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == B2IF_A::Trigger
    }
}
#[doc = "Break 2 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B2IF_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<B2IF_AW> for bool {
    #[inline(always)]
    fn from(variant: B2IF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B2IF` writer - Break 2 interrupt flag"]
pub type B2IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, B2IF_AW, O>;
impl<'a, const O: u8> B2IF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(B2IF_AW::Clear)
    }
}
#[doc = "Break interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIF_A {
    #[doc = "0: No break event occurred"]
    NoTrigger = 0,
    #[doc = "1: An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register"]
    Trigger = 1,
}
impl From<BIF_A> for bool {
    #[inline(always)]
    fn from(variant: BIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIF` reader - Break interrupt flag"]
pub type BIF_R = crate::BitReader<BIF_A>;
impl BIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIF_A {
        match self.bits {
            false => BIF_A::NoTrigger,
            true => BIF_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoTrigger`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == BIF_A::NoTrigger
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == BIF_A::Trigger
    }
}
#[doc = "Break interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIF_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<BIF_AW> for bool {
    #[inline(always)]
    fn from(variant: BIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIF` writer - Break interrupt flag"]
pub type BIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, BIF_AW, O>;
impl<'a, const O: u8> BIF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BIF_AW::Clear)
    }
}
#[doc = "Trigger interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF_A {
    #[doc = "0: No trigger event occurred"]
    NoTrigger = 0,
    #[doc = "1: Trigger interrupt pending"]
    Trigger = 1,
}
impl From<TIF_A> for bool {
    #[inline(always)]
    fn from(variant: TIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` reader - Trigger interrupt flag"]
pub type TIF_R = crate::BitReader<TIF_A>;
impl TIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF_A {
        match self.bits {
            false => TIF_A::NoTrigger,
            true => TIF_A::Trigger,
        }
    }
    #[doc = "Checks if the value of the field is `NoTrigger`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == TIF_A::NoTrigger
    }
    #[doc = "Checks if the value of the field is `Trigger`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TIF_A::Trigger
    }
}
#[doc = "Trigger interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<TIF_AW> for bool {
    #[inline(always)]
    fn from(variant: TIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` writer - Trigger interrupt flag"]
pub type TIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, TIF_AW, O>;
impl<'a, const O: u8> TIF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIF_AW::Clear)
    }
}
#[doc = "Field `COMIF` reader - COM interrupt flag"]
pub type COMIF_R = crate::BitReader<bool>;
#[doc = "Field `COMIF` writer - COM interrupt flag"]
pub type COMIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Capture/Compare 4 interrupt flag"]
pub use CC1IF_A as CC4IF_A;
#[doc = "Capture/Compare 3 interrupt flag"]
pub use CC1IF_A as CC3IF_A;
#[doc = "Capture/Compare 2 interrupt flag"]
pub use CC1IF_A as CC2IF_A;
#[doc = "Capture/Compare 4 interrupt flag"]
pub use CC1IF_AW as CC4IF_AW;
#[doc = "Capture/Compare 3 interrupt flag"]
pub use CC1IF_AW as CC3IF_AW;
#[doc = "Capture/Compare 2 interrupt flag"]
pub use CC1IF_AW as CC2IF_AW;
#[doc = "Field `CC4IF` reader - Capture/Compare 4 interrupt flag"]
pub use CC1IF_R as CC4IF_R;
#[doc = "Field `CC3IF` reader - Capture/Compare 3 interrupt flag"]
pub use CC1IF_R as CC3IF_R;
#[doc = "Field `CC2IF` reader - Capture/Compare 2 interrupt flag"]
pub use CC1IF_R as CC2IF_R;
#[doc = "Field `CC4IF` writer - Capture/Compare 4 interrupt flag"]
pub use CC1IF_W as CC4IF_W;
#[doc = "Field `CC3IF` writer - Capture/Compare 3 interrupt flag"]
pub use CC1IF_W as CC3IF_W;
#[doc = "Field `CC2IF` writer - Capture/Compare 2 interrupt flag"]
pub use CC1IF_W as CC2IF_W;
#[doc = "Capture/Compare 1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IF_A {
    #[doc = "0: No campture/compare has been detected"]
    NoMatch = 0,
    #[doc = "1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register"]
    Match = 1,
}
impl From<CC1IF_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IF` reader - Capture/Compare 1 interrupt flag"]
pub type CC1IF_R = crate::BitReader<CC1IF_A>;
impl CC1IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC1IF_A {
        match self.bits {
            false => CC1IF_A::NoMatch,
            true => CC1IF_A::Match,
        }
    }
    #[doc = "Checks if the value of the field is `NoMatch`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CC1IF_A::NoMatch
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CC1IF_A::Match
    }
}
#[doc = "Capture/Compare 1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IF_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<CC1IF_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1IF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IF` writer - Capture/Compare 1 interrupt flag"]
pub type CC1IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, CC1IF_AW, O>;
impl<'a, const O: u8> CC1IF_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC1IF_AW::Clear)
    }
}
#[doc = "Update interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIF_A {
    #[doc = "0: No update occurred"]
    Clear = 0,
    #[doc = "1: Update interrupt pending."]
    UpdatePending = 1,
}
impl From<UIF_A> for bool {
    #[inline(always)]
    fn from(variant: UIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIF` reader - Update interrupt flag"]
pub type UIF_R = crate::BitReader<UIF_A>;
impl UIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIF_A {
        match self.bits {
            false => UIF_A::Clear,
            true => UIF_A::UpdatePending,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UIF_A::Clear
    }
    #[doc = "Checks if the value of the field is `UpdatePending`"]
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UIF_A::UpdatePending
    }
}
#[doc = "Field `UIF` writer - Update interrupt flag"]
pub type UIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, UIF_A, O>;
impl<'a, const O: u8> UIF_W<'a, O> {
    #[doc = "No update occurred"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UIF_A::Clear)
    }
    #[doc = "Update interrupt pending."]
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut W {
        self.variant(UIF_A::UpdatePending)
    }
}
impl R {
    #[doc = "Bit 17 - Compare 6 interrupt flag"]
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag"]
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13 - System Break interrupt flag"]
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag"]
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Compare 6 interrupt flag"]
    #[inline(always)]
    pub fn cc6if(&mut self) -> CC6IF_W<17> {
        CC6IF_W::new(self)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag"]
    #[inline(always)]
    pub fn cc5if(&mut self) -> CC5IF_W<16> {
        CC5IF_W::new(self)
    }
    #[doc = "Bit 13 - System Break interrupt flag"]
    #[inline(always)]
    pub fn sbif(&mut self) -> SBIF_W<13> {
        SBIF_W::new(self)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4of(&mut self) -> CC4OF_W<12> {
        CC4OF_W::new(self)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3of(&mut self) -> CC3OF_W<11> {
        CC3OF_W::new(self)
    }
    #[doc = "Bit 10 - Capture/Compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W<10> {
        CC2OF_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<9> {
        CC1OF_W::new(self)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag"]
    #[inline(always)]
    pub fn b2if(&mut self) -> B2IF_W<8> {
        B2IF_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W<7> {
        BIF_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W<6> {
        TIF_W::new(self)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W<5> {
        COMIF_W::new(self)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&mut self) -> CC4IF_W<4> {
        CC4IF_W::new(self)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&mut self) -> CC3IF_W<3> {
        CC3IF_W::new(self)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W<2> {
        CC2IF_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<1> {
        CC1IF_W::new(self)
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<0> {
        UIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
