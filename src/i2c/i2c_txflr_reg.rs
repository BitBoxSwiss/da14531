#[doc = "Register `I2C_TXFLR_REG` reader"]
pub struct R(crate::R<I2C_TXFLR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TXFLR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TXFLR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TXFLR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TXFLR_REG` writer"]
pub struct W(crate::W<I2C_TXFLR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TXFLR_REG_SPEC>;
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
impl From<crate::W<I2C_TXFLR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TXFLR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFLR` reader - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO. Size is constrained by the TXFLR value"]
pub struct TXFLR_R(crate::FieldReader<u8, u8>);
impl TXFLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXFLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO. Size is constrained by the TXFLR value"]
    #[inline(always)]
    pub fn txflr(&self) -> TXFLR_R {
        TXFLR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Transmit FIFO Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_txflr_reg](index.html) module"]
pub struct I2C_TXFLR_REG_SPEC;
impl crate::RegisterSpec for I2C_TXFLR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_txflr_reg::R](R) reader structure"]
impl crate::Readable for I2C_TXFLR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_txflr_reg::W](W) writer structure"]
impl crate::Writable for I2C_TXFLR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_TXFLR_REG to value 0"]
impl crate::Resettable for I2C_TXFLR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
