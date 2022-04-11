#[doc = "Register `BLE_BASETIMECNTCORR_REG` reader"]
pub struct R(crate::R<BLE_BASETIMECNTCORR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_BASETIMECNTCORR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_BASETIMECNTCORR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_BASETIMECNTCORR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_BASETIMECNTCORR_REG` writer"]
pub struct W(crate::W<BLE_BASETIMECNTCORR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_BASETIMECNTCORR_REG_SPEC>;
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
impl From<crate::W<BLE_BASETIMECNTCORR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_BASETIMECNTCORR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASETIMECNTCORR` reader - Base Time Counter correction value."]
pub struct BASETIMECNTCORR_R(crate::FieldReader<u32, u32>);
impl BASETIMECNTCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BASETIMECNTCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASETIMECNTCORR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BASETIMECNTCORR` writer - Base Time Counter correction value."]
pub struct BASETIMECNTCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASETIMECNTCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff_ffff) | (value as u32 & 0x07ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:26 - Base Time Counter correction value."]
    #[inline(always)]
    pub fn basetimecntcorr(&self) -> BASETIMECNTCORR_R {
        BASETIMECNTCORR_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:26 - Base Time Counter correction value."]
    #[inline(always)]
    pub fn basetimecntcorr(&mut self) -> BASETIMECNTCORR_W {
        BASETIMECNTCORR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base Time Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_basetimecntcorr_reg](index.html) module"]
pub struct BLE_BASETIMECNTCORR_REG_SPEC;
impl crate::RegisterSpec for BLE_BASETIMECNTCORR_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_basetimecntcorr_reg::R](R) reader structure"]
impl crate::Readable for BLE_BASETIMECNTCORR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_basetimecntcorr_reg::W](W) writer structure"]
impl crate::Writable for BLE_BASETIMECNTCORR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_BASETIMECNTCORR_REG to value 0"]
impl crate::Resettable for BLE_BASETIMECNTCORR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
