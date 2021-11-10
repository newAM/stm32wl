///Register `SRRVR` reader
pub struct R(crate::R<SRRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRRVR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SRRVR` writer
pub struct W(crate::W<SRRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRRVR_SPEC>;
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
impl From<crate::W<SRRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRRVR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SBRV` reader - CPU2 boot reset vector
pub struct SBRV_R(crate::FieldReader<u16, u16>);
impl SBRV_R {
    pub(crate) fn new(bits: u16) -> Self {
        SBRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBRV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SBRV` writer - CPU2 boot reset vector
pub struct SBRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SBRV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
///Field `SBRSA` reader - Secure backup SRAM2 start address
pub struct SBRSA_R(crate::FieldReader<u8, u8>);
impl SBRSA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SBRSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBRSA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SBRSA` writer - Secure backup SRAM2 start address
pub struct SBRSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SBRSA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
        self.w
    }
}
///backup SRAM2 security disable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSD_A {
    ///0: SRAM2 is secure. SNBRSA\[4:0\]
    ///contains the start address of the first 1-Kbyte page of the secure backup SRAM2 area
    SECURE = 0,
    ///1: SRAM2 is non-secure
    NONSECURE = 1,
}
impl From<BRSD_A> for bool {
    #[inline(always)]
    fn from(variant: BRSD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BRSD` reader - backup SRAM2 security disable
pub struct BRSD_R(crate::FieldReader<bool, BRSD_A>);
impl BRSD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRSD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BRSD_A {
        match self.bits {
            false => BRSD_A::SECURE,
            true => BRSD_A::NONSECURE,
        }
    }
    ///Checks if the value of the field is `SECURE`
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == BRSD_A::SECURE
    }
    ///Checks if the value of the field is `NONSECURE`
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == BRSD_A::NONSECURE
    }
}
impl core::ops::Deref for BRSD_R {
    type Target = crate::FieldReader<bool, BRSD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BRSD` writer - backup SRAM2 security disable
pub struct BRSD_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BRSD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SRAM2 is secure. SNBRSA\[4:0\]
    ///contains the start address of the first 1-Kbyte page of the secure backup SRAM2 area
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(BRSD_A::SECURE)
    }
    ///SRAM2 is non-secure
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(BRSD_A::NONSECURE)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///Field `SNBRSA` reader - Secure non-backup SRAM1 start address
pub struct SNBRSA_R(crate::FieldReader<u8, u8>);
impl SNBRSA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SNBRSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNBRSA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SNBRSA` writer - Secure non-backup SRAM1 start address
pub struct SNBRSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SNBRSA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
///NBRSD
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBRSD_A {
    ///0: SRAM1 is secure. SNBRSA\[4:0\]
    ///contains the start address of the first 1-Kbyte page of the secure non-backup SRAM1 area
    SECURE = 0,
    ///1: SRAM1 is non-secure
    NONSECURE = 1,
}
impl From<NBRSD_A> for bool {
    #[inline(always)]
    fn from(variant: NBRSD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NBRSD` reader - NBRSD
pub struct NBRSD_R(crate::FieldReader<bool, NBRSD_A>);
impl NBRSD_R {
    pub(crate) fn new(bits: bool) -> Self {
        NBRSD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NBRSD_A {
        match self.bits {
            false => NBRSD_A::SECURE,
            true => NBRSD_A::NONSECURE,
        }
    }
    ///Checks if the value of the field is `SECURE`
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        **self == NBRSD_A::SECURE
    }
    ///Checks if the value of the field is `NONSECURE`
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        **self == NBRSD_A::NONSECURE
    }
}
impl core::ops::Deref for NBRSD_R {
    type Target = crate::FieldReader<bool, NBRSD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NBRSD` writer - NBRSD
pub struct NBRSD_W<'a> {
    w: &'a mut W,
}
impl<'a> NBRSD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NBRSD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SRAM1 is secure. SNBRSA\[4:0\]
    ///contains the start address of the first 1-Kbyte page of the secure non-backup SRAM1 area
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(NBRSD_A::SECURE)
    }
    ///SRAM1 is non-secure
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(NBRSD_A::NONSECURE)
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
///C2OPT
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C2OPT_A {
    ///0: SBRV offset addresses SRAM1 or SRAM2, from start address 0x2000_0000 + SBRV
    SRAM = 0,
    ///1: SBRV offset addresses the Flash memory, from start address 0x0800_0000 + SBRV
    FLASH = 1,
}
impl From<C2OPT_A> for bool {
    #[inline(always)]
    fn from(variant: C2OPT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C2OPT` reader - C2OPT
pub struct C2OPT_R(crate::FieldReader<bool, C2OPT_A>);
impl C2OPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2OPT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> C2OPT_A {
        match self.bits {
            false => C2OPT_A::SRAM,
            true => C2OPT_A::FLASH,
        }
    }
    ///Checks if the value of the field is `SRAM`
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        **self == C2OPT_A::SRAM
    }
    ///Checks if the value of the field is `FLASH`
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        **self == C2OPT_A::FLASH
    }
}
impl core::ops::Deref for C2OPT_R {
    type Target = crate::FieldReader<bool, C2OPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `C2OPT` writer - C2OPT
pub struct C2OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> C2OPT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: C2OPT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SBRV offset addresses SRAM1 or SRAM2, from start address 0x2000_0000 + SBRV
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(C2OPT_A::SRAM)
    }
    ///SBRV offset addresses the Flash memory, from start address 0x0800_0000 + SBRV
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(C2OPT_A::FLASH)
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
impl R {
    ///Bits 0:15 - CPU2 boot reset vector
    #[inline(always)]
    pub fn sbrv(&self) -> SBRV_R {
        SBRV_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 18:22 - Secure backup SRAM2 start address
    #[inline(always)]
    pub fn sbrsa(&self) -> SBRSA_R {
        SBRSA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bit 23 - backup SRAM2 security disable
    #[inline(always)]
    pub fn brsd(&self) -> BRSD_R {
        BRSD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bits 25:29 - Secure non-backup SRAM1 start address
    #[inline(always)]
    pub fn snbrsa(&self) -> SNBRSA_R {
        SNBRSA_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    ///Bit 30 - NBRSD
    #[inline(always)]
    pub fn nbrsd(&self) -> NBRSD_R {
        NBRSD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - C2OPT
    #[inline(always)]
    pub fn c2opt(&self) -> C2OPT_R {
        C2OPT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:15 - CPU2 boot reset vector
    #[inline(always)]
    pub fn sbrv(&mut self) -> SBRV_W {
        SBRV_W { w: self }
    }
    ///Bits 18:22 - Secure backup SRAM2 start address
    #[inline(always)]
    pub fn sbrsa(&mut self) -> SBRSA_W {
        SBRSA_W { w: self }
    }
    ///Bit 23 - backup SRAM2 security disable
    #[inline(always)]
    pub fn brsd(&mut self) -> BRSD_W {
        BRSD_W { w: self }
    }
    ///Bits 25:29 - Secure non-backup SRAM1 start address
    #[inline(always)]
    pub fn snbrsa(&mut self) -> SNBRSA_W {
        SNBRSA_W { w: self }
    }
    ///Bit 30 - NBRSD
    #[inline(always)]
    pub fn nbrsd(&mut self) -> NBRSD_W {
        NBRSD_W { w: self }
    }
    ///Bit 31 - C2OPT
    #[inline(always)]
    pub fn c2opt(&mut self) -> C2OPT_W {
        C2OPT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash secure SRAM start address and CPU2 reset vector register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [srrvr](index.html) module
pub struct SRRVR_SPEC;
impl crate::RegisterSpec for SRRVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [srrvr::R](R) reader structure
impl crate::Readable for SRRVR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [srrvr::W](W) writer structure
impl crate::Writable for SRRVR_SPEC {
    type Writer = W;
}
///`reset()` method sets SRRVR to value 0xffff_8000
impl crate::Resettable for SRRVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_8000
    }
}
