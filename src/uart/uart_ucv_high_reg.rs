#[doc = "Register `UART_UCV_HIGH_REG` reader"]
pub struct R(crate::R<UART_UCV_HIGH_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_UCV_HIGH_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_UCV_HIGH_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_UCV_HIGH_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_UCV_HIGH_REG` writer"]
pub struct W(crate::W<UART_UCV_HIGH_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_UCV_HIGH_REG_SPEC>;
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
impl From<crate::W<UART_UCV_HIGH_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_UCV_HIGH_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCV` reader - Component Version"]
pub struct UCV_R(crate::FieldReader<u16, u16>);
impl UCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Component Version"]
    #[inline(always)]
    pub fn ucv(&self) -> UCV_R {
        UCV_R::new(self.bits)
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
#[doc = "Component Version\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ucv_high_reg](index.html) module"]
pub struct UART_UCV_HIGH_REG_SPEC;
impl crate::RegisterSpec for UART_UCV_HIGH_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart_ucv_high_reg::R](R) reader structure"]
impl crate::Readable for UART_UCV_HIGH_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_ucv_high_reg::W](W) writer structure"]
impl crate::Writable for UART_UCV_HIGH_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_UCV_HIGH_REG to value 0x3331"]
impl crate::Resettable for UART_UCV_HIGH_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3331
    }
}
