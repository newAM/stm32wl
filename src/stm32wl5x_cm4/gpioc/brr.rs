///Register `BRR` reader
pub struct R(crate::R<BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BRR` writer
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
///Port Reset bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BR0_A {
    ///0: No action on the corresponding ODx bit
    NOACTION = 0,
    ///1: Reset the ODx bit
    RESET = 1,
}
impl From<BR0_A> for bool {
    #[inline(always)]
    fn from(variant: BR0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BR0` reader - Port Reset bit
pub struct BR0_R(crate::FieldReader<bool, BR0_A>);
impl BR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BR0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BR0_A {
        match self.bits {
            false => BR0_A::NOACTION,
            true => BR0_A::RESET,
        }
    }
    ///Checks if the value of the field is `NOACTION`
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        **self == BR0_A::NOACTION
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == BR0_A::RESET
    }
}
impl core::ops::Deref for BR0_R {
    type Target = crate::FieldReader<bool, BR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BR0` writer - Port Reset bit
pub struct BR0_W<'a> {
    w: &'a mut W,
}
impl<'a> BR0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0_A::NOACTION)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0_A::RESET)
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
///Port Reset bit
pub type BR1_A = BR0_A;
///Field `BR1` reader - Port Reset bit
pub type BR1_R = BR0_R;
///Field `BR1` writer - Port Reset bit
pub struct BR1_W<'a> {
    w: &'a mut W,
}
impl<'a> BR1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR1_A::NOACTION)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR1_A::RESET)
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
///Port Reset bit
pub type BR2_A = BR0_A;
///Field `BR2` reader - Port Reset bit
pub type BR2_R = BR0_R;
///Field `BR2` writer - Port Reset bit
pub struct BR2_W<'a> {
    w: &'a mut W,
}
impl<'a> BR2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR2_A::NOACTION)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR2_A::RESET)
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
///Port Reset bit
pub type BR3_A = BR0_A;
///Field `BR3` reader - Port Reset bit
pub type BR3_R = BR0_R;
///Field `BR3` writer - Port Reset bit
pub struct BR3_W<'a> {
    w: &'a mut W,
}
impl<'a> BR3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR3_A::NOACTION)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR3_A::RESET)
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
///Port Reset bit
pub type BR4_A = BR0_A;
///Field `BR4` reader - Port Reset bit
pub type BR4_R = BR0_R;
///Field `BR4` writer - Port Reset bit
pub struct BR4_W<'a> {
    w: &'a mut W,
}
impl<'a> BR4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR4_A::NOACTION)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR4_A::RESET)
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
///Port Reset bit
pub type BR5_A = BR0_A;
///Field `BR5` reader - Port Reset bit
pub type BR5_R = BR0_R;
///Field `BR5` writer - Port Reset bit
pub struct BR5_W<'a> {
    w: &'a mut W,
}
impl<'a> BR5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR5_A::NOACTION)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR5_A::RESET)
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
///Port Reset bit
pub type BR6_A = BR0_A;
///Field `BR6` reader - Port Reset bit
pub type BR6_R = BR0_R;
///Field `BR6` writer - Port Reset bit
pub struct BR6_W<'a> {
    w: &'a mut W,
}
impl<'a> BR6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR6_A::NOACTION)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR6_A::RESET)
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
///Port Reset bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BR13_A {
    ///0: No action on the corresponding ODx bit
    NOACTION = 0,
    ///1: Reset the ODx bit
    RESET = 1,
}
impl From<BR13_A> for bool {
    #[inline(always)]
    fn from(variant: BR13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BR13` reader - Port Reset bit
pub struct BR13_R(crate::FieldReader<bool, BR13_A>);
impl BR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        BR13_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BR13_A {
        match self.bits {
            false => BR13_A::NOACTION,
            true => BR13_A::RESET,
        }
    }
    ///Checks if the value of the field is `NOACTION`
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        **self == BR13_A::NOACTION
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == BR13_A::RESET
    }
}
impl core::ops::Deref for BR13_R {
    type Target = crate::FieldReader<bool, BR13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BR13` writer - Port Reset bit
pub struct BR13_W<'a> {
    w: &'a mut W,
}
impl<'a> BR13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR13_A::NOACTION)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR13_A::RESET)
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
///Port Reset bit
pub type BR14_A = BR13_A;
///Field `BR14` reader - Port Reset bit
pub type BR14_R = BR13_R;
///Field `BR14` writer - Port Reset bit
pub struct BR14_W<'a> {
    w: &'a mut W,
}
impl<'a> BR14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR14_A::NOACTION)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR14_A::RESET)
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
///Port Reset bit
pub type BR15_A = BR13_A;
///Field `BR15` reader - Port Reset bit
pub type BR15_R = BR13_R;
///Field `BR15` writer - Port Reset bit
pub struct BR15_W<'a> {
    w: &'a mut W,
}
impl<'a> BR15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR15_A::NOACTION)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR15_A::RESET)
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
impl R {
    ///Bit 0 - Port Reset bit
    #[inline(always)]
    pub fn br0(&self) -> BR0_R {
        BR0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Port Reset bit
    #[inline(always)]
    pub fn br1(&self) -> BR1_R {
        BR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Port Reset bit
    #[inline(always)]
    pub fn br2(&self) -> BR2_R {
        BR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    pub fn br3(&self) -> BR3_R {
        BR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Port Reset bit
    #[inline(always)]
    pub fn br4(&self) -> BR4_R {
        BR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Port Reset bit
    #[inline(always)]
    pub fn br5(&self) -> BR5_R {
        BR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Port Reset bit
    #[inline(always)]
    pub fn br6(&self) -> BR6_R {
        BR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 13 - Port Reset bit
    #[inline(always)]
    pub fn br13(&self) -> BR13_R {
        BR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Port Reset bit
    #[inline(always)]
    pub fn br14(&self) -> BR14_R {
        BR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Port Reset bit
    #[inline(always)]
    pub fn br15(&self) -> BR15_R {
        BR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Port Reset bit
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W {
        BR0_W { w: self }
    }
    ///Bit 1 - Port Reset bit
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W {
        BR1_W { w: self }
    }
    ///Bit 2 - Port Reset bit
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W {
        BR2_W { w: self }
    }
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W {
        BR3_W { w: self }
    }
    ///Bit 4 - Port Reset bit
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W {
        BR4_W { w: self }
    }
    ///Bit 5 - Port Reset bit
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W {
        BR5_W { w: self }
    }
    ///Bit 6 - Port Reset bit
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W {
        BR6_W { w: self }
    }
    ///Bit 13 - Port Reset bit
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W {
        BR13_W { w: self }
    }
    ///Bit 14 - Port Reset bit
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W {
        BR14_W { w: self }
    }
    ///Bit 15 - Port Reset bit
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W {
        BR15_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port bit reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [brr](index.html) module
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [brr::R](R) reader structure
impl crate::Readable for BRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [brr::W](W) writer structure
impl crate::Writable for BRR_SPEC {
    type Writer = W;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
