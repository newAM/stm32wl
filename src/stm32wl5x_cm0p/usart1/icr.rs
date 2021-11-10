///Register `ICR` writer
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Wakeup from low-power mode clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUCF_AW {
    ///1: Clears the WUF flag in the ISR register
    CLEAR = 1,
}
impl From<WUCF_AW> for bool {
    #[inline(always)]
    fn from(variant: WUCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `WUCF` writer - Wakeup from low-power mode clear flag
pub struct WUCF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the WUF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///Character match clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMCF_AW {
    ///1: Clears the CMF flag in the ISR register
    CLEAR = 1,
}
impl From<CMCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CMCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMCF` writer - Character match clear flag
pub struct CMCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CMCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the CMF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///SPI slave underrun clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDRCF_AW {
    ///1: Clear the UDR flag in the ISR register
    CLEAR = 1,
}
impl From<UDRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: UDRCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `UDRCF` writer - SPI slave underrun clear flag
pub struct UDRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UDRCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the UDR flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UDRCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///End of block clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOBCF_AW {
    ///1: Clears the EOBF flag in the ISR register
    CLEAR = 1,
}
impl From<EOBCF_AW> for bool {
    #[inline(always)]
    fn from(variant: EOBCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOBCF` writer - End of block clear flag
pub struct EOBCF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOBCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOBCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the EOBF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOBCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///Receiver timeout clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTOCF_AW {
    ///1: Clears the RTOF flag in the ISR register
    CLEAR = 1,
}
impl From<RTOCF_AW> for bool {
    #[inline(always)]
    fn from(variant: RTOCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOCF` writer - Receiver timeout clear flag
pub struct RTOCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTOCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTOCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the RTOF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RTOCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///CTS clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSCF_AW {
    ///1: Clears the CTSIF flag in the ISR register
    CLEAR = 1,
}
impl From<CTSCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSCF` writer - CTS clear flag
pub struct CTSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTSCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the CTSIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///LIN break detection clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDCF_AW {
    ///1: Clears the LBDF flag in the ISR register
    CLEAR = 1,
}
impl From<LBDCF_AW> for bool {
    #[inline(always)]
    fn from(variant: LBDCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDCF` writer - LIN break detection clear flag
pub struct LBDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> LBDCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LBDCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the LBDF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LBDCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Transmission complete before Guard time clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCBGTCF_AW {
    ///1: Clear the TCBGT flag in the ISR register
    CLEAR = 1,
}
impl From<TCBGTCF_AW> for bool {
    #[inline(always)]
    fn from(variant: TCBGTCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TCBGTCF` writer - Transmission complete before Guard time clear flag
pub struct TCBGTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCBGTCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TCBGTCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the TCBGT flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCBGTCF_AW::CLEAR)
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
///Transmission complete clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCF_AW {
    ///1: Clears the TC flag in the ISR register
    CLEAR = 1,
}
impl From<TCCF_AW> for bool {
    #[inline(always)]
    fn from(variant: TCCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TCCF` writer - Transmission complete clear flag
pub struct TCCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TCCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TC flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///TXFIFO empty clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFECF_AW {
    ///1: Clear the TXFE flag in the ISR register
    CLEAR = 1,
}
impl From<TXFECF_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFECF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFECF` writer - TXFIFO empty clear flag
pub struct TXFECF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFECF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXFECF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the TXFE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFECF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Idle line detected clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECF_AW {
    ///1: Clears the IDLE flag in the ISR register
    CLEAR = 1,
}
impl From<IDLECF_AW> for bool {
    #[inline(always)]
    fn from(variant: IDLECF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLECF` writer - Idle line detected clear flag
pub struct IDLECF_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IDLECF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the IDLE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDLECF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Overrun error clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORECF_AW {
    ///1: Clears the ORE flag in the ISR register
    CLEAR = 1,
}
impl From<ORECF_AW> for bool {
    #[inline(always)]
    fn from(variant: ORECF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ORECF` writer - Overrun error clear flag
pub struct ORECF_W<'a> {
    w: &'a mut W,
}
impl<'a> ORECF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ORECF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the ORE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ORECF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Noise detected clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCF_AW {
    ///1: Clears the NF flag in the ISR register
    CLEAR = 1,
}
impl From<NCF_AW> for bool {
    #[inline(always)]
    fn from(variant: NCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `NCF` writer - Noise detected clear flag
pub struct NCF_W<'a> {
    w: &'a mut W,
}
impl<'a> NCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the NF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Framing error clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FECF_AW {
    ///1: Clears the FE flag in the ISR register
    CLEAR = 1,
}
impl From<FECF_AW> for bool {
    #[inline(always)]
    fn from(variant: FECF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FECF` writer - Framing error clear flag
pub struct FECF_W<'a> {
    w: &'a mut W,
}
impl<'a> FECF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FECF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the FE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FECF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Parity error clear flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECF_AW {
    ///1: Clears the PE flag in the ISR register
    CLEAR = 1,
}
impl From<PECF_AW> for bool {
    #[inline(always)]
    fn from(variant: PECF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PECF` writer - Parity error clear flag
pub struct PECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PECF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PECF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the PE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    ///Bit 20 - Wakeup from low-power mode clear flag
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W {
        WUCF_W { w: self }
    }
    ///Bit 17 - Character match clear flag
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W {
        CMCF_W { w: self }
    }
    ///Bit 13 - SPI slave underrun clear flag
    #[inline(always)]
    pub fn udrcf(&mut self) -> UDRCF_W {
        UDRCF_W { w: self }
    }
    ///Bit 12 - End of block clear flag
    #[inline(always)]
    pub fn eobcf(&mut self) -> EOBCF_W {
        EOBCF_W { w: self }
    }
    ///Bit 11 - Receiver timeout clear flag
    #[inline(always)]
    pub fn rtocf(&mut self) -> RTOCF_W {
        RTOCF_W { w: self }
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W {
        CTSCF_W { w: self }
    }
    ///Bit 8 - LIN break detection clear flag
    #[inline(always)]
    pub fn lbdcf(&mut self) -> LBDCF_W {
        LBDCF_W { w: self }
    }
    ///Bit 7 - Transmission complete before Guard time clear flag
    #[inline(always)]
    pub fn tcbgtcf(&mut self) -> TCBGTCF_W {
        TCBGTCF_W { w: self }
    }
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W {
        TCCF_W { w: self }
    }
    ///Bit 5 - TXFIFO empty clear flag
    #[inline(always)]
    pub fn txfecf(&mut self) -> TXFECF_W {
        TXFECF_W { w: self }
    }
    ///Bit 4 - Idle line detected clear flag
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W {
        IDLECF_W { w: self }
    }
    ///Bit 3 - Overrun error clear flag
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W {
        ORECF_W { w: self }
    }
    ///Bit 2 - Noise detected clear flag
    #[inline(always)]
    pub fn ncf(&mut self) -> NCF_W {
        NCF_W { w: self }
    }
    ///Bit 1 - Framing error clear flag
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W {
        FECF_W { w: self }
    }
    ///Bit 0 - Parity error clear flag
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W {
        PECF_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt flag clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [icr::W](W) writer structure
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
