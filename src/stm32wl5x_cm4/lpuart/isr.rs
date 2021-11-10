///Register `ISR` reader
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
///Field `TXFT` reader - TXFIFO threshold flag
pub struct TXFT_R(crate::FieldReader<bool, bool>);
impl TXFT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXFT` reader - RXFIFO threshold flag
pub struct RXFT_R(crate::FieldReader<bool, bool>);
impl RXFT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXFF` reader - RXFIFO Full
pub struct RXFF_R(crate::FieldReader<bool, bool>);
impl RXFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXFE` reader - TXFIFO Empty
pub struct TXFE_R(crate::FieldReader<bool, bool>);
impl TXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REACK` reader - REACK
pub struct REACK_R(crate::FieldReader<bool, bool>);
impl REACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEACK` reader - TEACK
pub struct TEACK_R(crate::FieldReader<bool, bool>);
impl TEACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUF` reader - WUF
pub struct WUF_R(crate::FieldReader<bool, bool>);
impl WUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RWU` reader - RWU
pub struct RWU_R(crate::FieldReader<bool, bool>);
impl RWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SBKF` reader - SBKF
pub struct SBKF_R(crate::FieldReader<bool, bool>);
impl SBKF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CMF` reader - CMF
pub struct CMF_R(crate::FieldReader<bool, bool>);
impl CMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BUSY` reader - BUSY
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTS` reader - CTS
pub struct CTS_R(crate::FieldReader<bool, bool>);
impl CTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTSIF` reader - CTSIF
pub struct CTSIF_R(crate::FieldReader<bool, bool>);
impl CTSIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXFNF` reader - TXFNF
pub struct TXFNF_R(crate::FieldReader<bool, bool>);
impl TXFNF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TC` reader - TC
pub struct TC_R(crate::FieldReader<bool, bool>);
impl TC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXFNE` reader - RXFNE
pub struct RXFNE_R(crate::FieldReader<bool, bool>);
impl RXFNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IDLE` reader - IDLE
pub struct IDLE_R(crate::FieldReader<bool, bool>);
impl IDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ORE` reader - ORE
pub struct ORE_R(crate::FieldReader<bool, bool>);
impl ORE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NE` reader - NE
pub struct NE_R(crate::FieldReader<bool, bool>);
impl NE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FE` reader - FE
pub struct FE_R(crate::FieldReader<bool, bool>);
impl FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PE` reader - PE
pub struct PE_R(crate::FieldReader<bool, bool>);
impl PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 27 - TXFIFO threshold flag
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - RXFIFO threshold flag
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 24 - RXFIFO Full
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 23 - TXFIFO Empty
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - REACK
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 21 - TEACK
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - WUF
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - RWU
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - SBKF
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - CMF
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 10 - CTS
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - CTSIF
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 7 - TXFNF
    #[inline(always)]
    pub fn txfnf(&self) -> TXFNF_R {
        TXFNF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - RXFNE
    #[inline(always)]
    pub fn rxfne(&self) -> RXFNE_R {
        RXFNE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - IDLE
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - ORE
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - NE
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - FE
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - PE
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0x01) != 0)
    }
}
///Interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](index.html) module
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [isr::R](R) reader structure
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets ISR to value 0x0080_00c0
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_00c0
    }
}
