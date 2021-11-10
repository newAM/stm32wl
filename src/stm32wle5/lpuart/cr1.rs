///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///RXFIFO Full interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFFIE_A {
    ///0: Interrupt inhibited
    DISABLED = 0,
    ///1: USART interrupt generated when RXFF = 1 in the USART_ISR register
    ENABLED = 1,
}
impl From<RXFFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFFIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFFIE` reader - RXFIFO Full interrupt enable
pub struct RXFFIE_R(crate::FieldReader<bool, RXFFIE_A>);
impl RXFFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFFIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXFFIE_A {
        match self.bits {
            false => RXFFIE_A::DISABLED,
            true => RXFFIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXFFIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXFFIE_A::ENABLED
    }
}
impl core::ops::Deref for RXFFIE_R {
    type Target = crate::FieldReader<bool, RXFFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXFFIE` writer - RXFIFO Full interrupt enable
pub struct RXFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXFFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXFFIE_A::DISABLED)
    }
    ///USART interrupt generated when RXFF = 1 in the USART_ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXFFIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
///TXFIFO empty interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFEIE_A {
    ///0: Interrupt inhibited
    DISABLED = 0,
    ///1: USART interrupt generated when TXFE = 1 in the USART_ISR register
    ENABLED = 1,
}
impl From<TXFEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFEIE` reader - TXFIFO empty interrupt enable
pub struct TXFEIE_R(crate::FieldReader<bool, TXFEIE_A>);
impl TXFEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFEIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXFEIE_A {
        match self.bits {
            false => TXFEIE_A::DISABLED,
            true => TXFEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXFEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXFEIE_A::ENABLED
    }
}
impl core::ops::Deref for TXFEIE_R {
    type Target = crate::FieldReader<bool, TXFEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXFEIE` writer - TXFIFO empty interrupt enable
pub struct TXFEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXFEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFEIE_A::DISABLED)
    }
    ///USART interrupt generated when TXFE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFEIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
///FIFO mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOEN_A {
    ///0: FIFO mode is disabled
    DISABLED = 0,
    ///1: FIFO mode is enabled
    ENABLED = 1,
}
impl From<FIFOEN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FIFOEN` reader - FIFO mode enable
pub struct FIFOEN_R(crate::FieldReader<bool, FIFOEN_A>);
impl FIFOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN_A {
        match self.bits {
            false => FIFOEN_A::DISABLED,
            true => FIFOEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FIFOEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FIFOEN_A::ENABLED
    }
}
impl core::ops::Deref for FIFOEN_R {
    type Target = crate::FieldReader<bool, FIFOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FIFOEN` writer - FIFO mode enable
pub struct FIFOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FIFOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///FIFO mode is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIFOEN_A::DISABLED)
    }
    ///FIFO mode is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIFOEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
///Word length
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1_A {
    ///0: Use M0 to set the data bits
    M0 = 0,
    ///1: 1 start bit, 7 data bits, n stop bits
    BIT7 = 1,
}
impl From<M1_A> for bool {
    #[inline(always)]
    fn from(variant: M1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `M1` reader - Word length
pub struct M1_R(crate::FieldReader<bool, M1_A>);
impl M1_R {
    pub(crate) fn new(bits: bool) -> Self {
        M1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> M1_A {
        match self.bits {
            false => M1_A::M0,
            true => M1_A::BIT7,
        }
    }
    ///Checks if the value of the field is `M0`
    #[inline(always)]
    pub fn is_m0(&self) -> bool {
        **self == M1_A::M0
    }
    ///Checks if the value of the field is `BIT7`
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        **self == M1_A::BIT7
    }
}
impl core::ops::Deref for M1_R {
    type Target = crate::FieldReader<bool, M1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `M1` writer - Word length
pub struct M1_W<'a> {
    w: &'a mut W,
}
impl<'a> M1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: M1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Use M0 to set the data bits
    #[inline(always)]
    pub fn m0(self) -> &'a mut W {
        self.variant(M1_A::M0)
    }
    ///1 start bit, 7 data bits, n stop bits
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(M1_A::BIT7)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///Field `DEAT` reader - DEAT
pub struct DEAT_R(crate::FieldReader<u8, u8>);
impl DEAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEAT` writer - DEAT
pub struct DEAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | ((value as u32 & 0x1f) << 21);
        self.w
    }
}
///Field `DEDT` reader - DEDT
pub struct DEDT_R(crate::FieldReader<u8, u8>);
impl DEDT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEDT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEDT` writer - DEDT
pub struct DEDT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEDT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
///Character match interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated when the CMF bit is set in the ISR register
    ENABLED = 1,
}
impl From<CMIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMIE` reader - Character match interrupt enable
pub struct CMIE_R(crate::FieldReader<bool, CMIE_A>);
impl CMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMIE_A {
        match self.bits {
            false => CMIE_A::DISABLED,
            true => CMIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMIE_A::ENABLED
    }
}
impl core::ops::Deref for CMIE_R {
    type Target = crate::FieldReader<bool, CMIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CMIE` writer - Character match interrupt enable
pub struct CMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CMIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMIE_A::DISABLED)
    }
    ///Interrupt is generated when the CMF bit is set in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Mute mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MME_A {
    ///0: Receiver in active mode permanently
    DISABLED = 0,
    ///1: Receiver can switch between mute mode and active mode
    ENABLED = 1,
}
impl From<MME_A> for bool {
    #[inline(always)]
    fn from(variant: MME_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MME` reader - Mute mode enable
pub struct MME_R(crate::FieldReader<bool, MME_A>);
impl MME_R {
    pub(crate) fn new(bits: bool) -> Self {
        MME_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MME_A {
        match self.bits {
            false => MME_A::DISABLED,
            true => MME_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MME_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MME_A::ENABLED
    }
}
impl core::ops::Deref for MME_R {
    type Target = crate::FieldReader<bool, MME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MME` writer - Mute mode enable
pub struct MME_W<'a> {
    w: &'a mut W,
}
impl<'a> MME_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Receiver in active mode permanently
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MME_A::DISABLED)
    }
    ///Receiver can switch between mute mode and active mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MME_A::ENABLED)
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
///Word length
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0_A {
    ///0: 1 start bit, 8 data bits, n stop bits
    BIT8 = 0,
    ///1: 1 start bit, 9 data bits, n stop bits
    BIT9 = 1,
}
impl From<M0_A> for bool {
    #[inline(always)]
    fn from(variant: M0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `M0` reader - Word length
pub struct M0_R(crate::FieldReader<bool, M0_A>);
impl M0_R {
    pub(crate) fn new(bits: bool) -> Self {
        M0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> M0_A {
        match self.bits {
            false => M0_A::BIT8,
            true => M0_A::BIT9,
        }
    }
    ///Checks if the value of the field is `BIT8`
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        **self == M0_A::BIT8
    }
    ///Checks if the value of the field is `BIT9`
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        **self == M0_A::BIT9
    }
}
impl core::ops::Deref for M0_R {
    type Target = crate::FieldReader<bool, M0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `M0` writer - Word length
pub struct M0_W<'a> {
    w: &'a mut W,
}
impl<'a> M0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: M0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///1 start bit, 8 data bits, n stop bits
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(M0_A::BIT8)
    }
    ///1 start bit, 9 data bits, n stop bits
    #[inline(always)]
    pub fn bit9(self) -> &'a mut W {
        self.variant(M0_A::BIT9)
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
///Receiver wakeup method
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_A {
    ///0: Idle line
    IDLE = 0,
    ///1: Address mask
    ADDRESS = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WAKE` reader - Receiver wakeup method
pub struct WAKE_R(crate::FieldReader<bool, WAKE_A>);
impl WAKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::IDLE,
            true => WAKE_A::ADDRESS,
        }
    }
    ///Checks if the value of the field is `IDLE`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == WAKE_A::IDLE
    }
    ///Checks if the value of the field is `ADDRESS`
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        **self == WAKE_A::ADDRESS
    }
}
impl core::ops::Deref for WAKE_R {
    type Target = crate::FieldReader<bool, WAKE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAKE` writer - Receiver wakeup method
pub struct WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WAKE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Idle line
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(WAKE_A::IDLE)
    }
    ///Address mask
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WAKE_A::ADDRESS)
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
///Parity control enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCE_A {
    ///0: Parity control disabled
    DISABLED = 0,
    ///1: Parity control enabled
    ENABLED = 1,
}
impl From<PCE_A> for bool {
    #[inline(always)]
    fn from(variant: PCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PCE` reader - Parity control enable
pub struct PCE_R(crate::FieldReader<bool, PCE_A>);
impl PCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCE_A {
        match self.bits {
            false => PCE_A::DISABLED,
            true => PCE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PCE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PCE_A::ENABLED
    }
}
impl core::ops::Deref for PCE_R {
    type Target = crate::FieldReader<bool, PCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PCE` writer - Parity control enable
pub struct PCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Parity control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PCE_A::DISABLED)
    }
    ///Parity control enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PCE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Parity selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    ///0: Even parity
    EVEN = 0,
    ///1: Odd parity
    ODD = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PS` reader - Parity selection
pub struct PS_R(crate::FieldReader<bool, PS_A>);
impl PS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::EVEN,
            true => PS_A::ODD,
        }
    }
    ///Checks if the value of the field is `EVEN`
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == PS_A::EVEN
    }
    ///Checks if the value of the field is `ODD`
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        **self == PS_A::ODD
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<bool, PS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PS` writer - Parity selection
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Even parity
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PS_A::EVEN)
    }
    ///Odd parity
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PS_A::ODD)
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
///PE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated whenever PE=1 in the ISR register
    ENABLED = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PEIE` reader - PE interrupt enable
pub struct PEIE_R(crate::FieldReader<bool, PEIE_A>);
impl PEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::DISABLED,
            true => PEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PEIE_A::ENABLED
    }
}
impl core::ops::Deref for PEIE_R {
    type Target = crate::FieldReader<bool, PEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PEIE` writer - PE interrupt enable
pub struct PEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEIE_A::DISABLED)
    }
    ///Interrupt is generated whenever PE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEIE_A::ENABLED)
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
///interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated whenever TXE=1 in the ISR register
    ENABLED = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXEIE` reader - interrupt enable
pub struct TXEIE_R(crate::FieldReader<bool, TXEIE_A>);
impl TXEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::DISABLED,
            true => TXEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXEIE_A::ENABLED
    }
}
impl core::ops::Deref for TXEIE_R {
    type Target = crate::FieldReader<bool, TXEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXEIE` writer - interrupt enable
pub struct TXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEIE_A::DISABLED)
    }
    ///Interrupt is generated whenever TXE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXEIE_A::ENABLED)
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
///Transmission complete interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated whenever TC=1 in the ISR register
    ENABLED = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - Transmission complete interrupt enable
pub struct TCIE_R(crate::FieldReader<bool, TCIE_A>);
impl TCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::DISABLED,
            true => TCIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TCIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TCIE_A::ENABLED
    }
}
impl core::ops::Deref for TCIE_R {
    type Target = crate::FieldReader<bool, TCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TCIE` writer - Transmission complete interrupt enable
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::DISABLED)
    }
    ///Interrupt is generated whenever TC=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::ENABLED)
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
///RXNE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register
    ENABLED = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNEIE` reader - RXNE interrupt enable
pub struct RXNEIE_R(crate::FieldReader<bool, RXNEIE_A>);
impl RXNEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNEIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::DISABLED,
            true => RXNEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXNEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXNEIE_A::ENABLED
    }
}
impl core::ops::Deref for RXNEIE_R {
    type Target = crate::FieldReader<bool, RXNEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXNEIE` writer - RXNE interrupt enable
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXNEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::DISABLED)
    }
    ///Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::ENABLED)
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
///IDLE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated whenever IDLE=1 in the ISR register
    ENABLED = 1,
}
impl From<IDLEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLEIE` reader - IDLE interrupt enable
pub struct IDLEIE_R(crate::FieldReader<bool, IDLEIE_A>);
impl IDLEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLEIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDLEIE_A {
        match self.bits {
            false => IDLEIE_A::DISABLED,
            true => IDLEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IDLEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IDLEIE_A::ENABLED
    }
}
impl core::ops::Deref for IDLEIE_R {
    type Target = crate::FieldReader<bool, IDLEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IDLEIE` writer - IDLE interrupt enable
pub struct IDLEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IDLEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::DISABLED)
    }
    ///Interrupt is generated whenever IDLE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::ENABLED)
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
///Transmitter enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE_A {
    ///0: Transmitter is disabled
    DISABLED = 0,
    ///1: Transmitter is enabled
    ENABLED = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TE` reader - Transmitter enable
pub struct TE_R(crate::FieldReader<bool, TE_A>);
impl TE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::DISABLED,
            true => TE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TE_A::ENABLED
    }
}
impl core::ops::Deref for TE_R {
    type Target = crate::FieldReader<bool, TE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TE` writer - Transmitter enable
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Transmitter is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TE_A::DISABLED)
    }
    ///Transmitter is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TE_A::ENABLED)
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
///Receiver enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    ///0: Receiver is disabled
    DISABLED = 0,
    ///1: Receiver is enabled
    ENABLED = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RE` reader - Receiver enable
pub struct RE_R(crate::FieldReader<bool, RE_A>);
impl RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::DISABLED,
            true => RE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RE_A::ENABLED
    }
}
impl core::ops::Deref for RE_R {
    type Target = crate::FieldReader<bool, RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RE` writer - Receiver enable
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Receiver is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RE_A::DISABLED)
    }
    ///Receiver is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RE_A::ENABLED)
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
///USART enable in Stop mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UESM_A {
    ///0: USART not able to wake up the MCU from Stop mode
    DISABLED = 0,
    ///1: USART able to wake up the MCU from Stop mode
    ENABLED = 1,
}
impl From<UESM_A> for bool {
    #[inline(always)]
    fn from(variant: UESM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UESM` reader - USART enable in Stop mode
pub struct UESM_R(crate::FieldReader<bool, UESM_A>);
impl UESM_R {
    pub(crate) fn new(bits: bool) -> Self {
        UESM_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UESM_A {
        match self.bits {
            false => UESM_A::DISABLED,
            true => UESM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == UESM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == UESM_A::ENABLED
    }
}
impl core::ops::Deref for UESM_R {
    type Target = crate::FieldReader<bool, UESM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UESM` writer - USART enable in Stop mode
pub struct UESM_W<'a> {
    w: &'a mut W,
}
impl<'a> UESM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UESM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///USART not able to wake up the MCU from Stop mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UESM_A::DISABLED)
    }
    ///USART able to wake up the MCU from Stop mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UESM_A::ENABLED)
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
///USART enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UE_A {
    ///0: UART is disabled
    DISABLED = 0,
    ///1: UART is enabled
    ENABLED = 1,
}
impl From<UE_A> for bool {
    #[inline(always)]
    fn from(variant: UE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UE` reader - USART enable
pub struct UE_R(crate::FieldReader<bool, UE_A>);
impl UE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UE_A {
        match self.bits {
            false => UE_A::DISABLED,
            true => UE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == UE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == UE_A::ENABLED
    }
}
impl core::ops::Deref for UE_R {
    type Target = crate::FieldReader<bool, UE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UE` writer - USART enable
pub struct UE_W<'a> {
    w: &'a mut W,
}
impl<'a> UE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///UART is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UE_A::DISABLED)
    }
    ///UART is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UE_A::ENABLED)
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
impl R {
    ///Bit 31 - RXFIFO Full interrupt enable
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - TXFIFO empty interrupt enable
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 29 - FIFO mode enable
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bits 21:25 - DEAT
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bits 16:20 - DEDT
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 31 - RXFIFO Full interrupt enable
    #[inline(always)]
    pub fn rxffie(&mut self) -> RXFFIE_W {
        RXFFIE_W { w: self }
    }
    ///Bit 30 - TXFIFO empty interrupt enable
    #[inline(always)]
    pub fn txfeie(&mut self) -> TXFEIE_W {
        TXFEIE_W { w: self }
    }
    ///Bit 29 - FIFO mode enable
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W {
        FIFOEN_W { w: self }
    }
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W {
        M1_W { w: self }
    }
    ///Bits 21:25 - DEAT
    #[inline(always)]
    pub fn deat(&mut self) -> DEAT_W {
        DEAT_W { w: self }
    }
    ///Bits 16:20 - DEDT
    #[inline(always)]
    pub fn dedt(&mut self) -> DEDT_W {
        DEDT_W { w: self }
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W {
        CMIE_W { w: self }
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W {
        MME_W { w: self }
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W {
        M0_W { w: self }
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W {
        WAKE_W { w: self }
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W {
        PCE_W { w: self }
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W {
        PEIE_W { w: self }
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W {
        TXEIE_W { w: self }
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W {
        IDLEIE_W { w: self }
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W {
        UESM_W { w: self }
    }
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W {
        UE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}