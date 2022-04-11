#[doc = "Register `RF_MIXER_CTRL1_REG` reader"]
pub struct R(crate::R<RF_MIXER_CTRL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_MIXER_CTRL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_MIXER_CTRL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_MIXER_CTRL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_MIXER_CTRL1_REG` writer"]
pub struct W(crate::W<RF_MIXER_CTRL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_MIXER_CTRL1_REG_SPEC>;
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
impl From<crate::W<RF_MIXER_CTRL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_MIXER_CTRL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIXER_IP2_DAC_Q_TRIM` reader - "]
pub struct MIXER_IP2_DAC_Q_TRIM_R(crate::FieldReader<u16, u16>);
impl MIXER_IP2_DAC_Q_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MIXER_IP2_DAC_Q_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIXER_IP2_DAC_Q_TRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIXER_IP2_DAC_Q_TRIM` writer - "]
pub struct MIXER_IP2_DAC_Q_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MIXER_IP2_DAC_Q_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `MIXER_IP2_DAC_I_TRIM` reader - "]
pub struct MIXER_IP2_DAC_I_TRIM_R(crate::FieldReader<u16, u16>);
impl MIXER_IP2_DAC_I_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MIXER_IP2_DAC_I_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIXER_IP2_DAC_I_TRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIXER_IP2_DAC_I_TRIM` writer - "]
pub struct MIXER_IP2_DAC_I_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MIXER_IP2_DAC_I_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn mixer_ip2_dac_q_trim(&self) -> MIXER_IP2_DAC_Q_TRIM_R {
        MIXER_IP2_DAC_Q_TRIM_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn mixer_ip2_dac_i_trim(&self) -> MIXER_IP2_DAC_I_TRIM_R {
        MIXER_IP2_DAC_I_TRIM_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn mixer_ip2_dac_q_trim(&mut self) -> MIXER_IP2_DAC_Q_TRIM_W {
        MIXER_IP2_DAC_Q_TRIM_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn mixer_ip2_dac_i_trim(&mut self) -> MIXER_IP2_DAC_I_TRIM_W {
        MIXER_IP2_DAC_I_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_mixer_ctrl1_reg](index.html) module"]
pub struct RF_MIXER_CTRL1_REG_SPEC;
impl crate::RegisterSpec for RF_MIXER_CTRL1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_mixer_ctrl1_reg::R](R) reader structure"]
impl crate::Readable for RF_MIXER_CTRL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_mixer_ctrl1_reg::W](W) writer structure"]
impl crate::Writable for RF_MIXER_CTRL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_MIXER_CTRL1_REG to value 0x010f_010f"]
impl crate::Resettable for RF_MIXER_CTRL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x010f_010f
    }
}
