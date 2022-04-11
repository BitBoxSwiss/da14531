#[doc = "Register `I2C_INTR_STAT_REG` reader"]
pub struct R(crate::R<I2C_INTR_STAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_INTR_STAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_INTR_STAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_INTR_STAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_INTR_STAT_REG` writer"]
pub struct W(crate::W<I2C_INTR_STAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_INTR_STAT_REG_SPEC>;
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
impl From<crate::W<I2C_INTR_STAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_INTR_STAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R_GEN_CALL` reader - Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling controller or when the CPU reads bit 0 of the I2C_CLR_GEN_CALL register. The controller stores the received data in the Rx buffer."]
pub struct R_GEN_CALL_R(crate::FieldReader<bool, bool>);
impl R_GEN_CALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_GEN_CALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_GEN_CALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_START_DET` reader - Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether controller is operating in slave or master mode."]
pub struct R_START_DET_R(crate::FieldReader<bool, bool>);
impl R_START_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_START_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_START_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_STOP_DET` reader - Indicates whether a STOP condition has occurred on the I2C interface regardless of whether controller is operating in slave or master mode."]
pub struct R_STOP_DET_R(crate::FieldReader<bool, bool>);
impl R_STOP_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_STOP_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_STOP_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_ACTIVITY` reader - This bit captures I2C Ctrl activity and stays set until it is cleared. There are four ways to clear it: => Disabling the I2C Ctrl => Reading the IC_CLR_ACTIVITY register => Reading the IC_CLR_INTR register => System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the controller module is idle, this bit remains set until cleared, indicating that there was activity on the bus."]
pub struct R_ACTIVITY_R(crate::FieldReader<bool, bool>);
impl R_ACTIVITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_ACTIVITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_ACTIVITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_RX_DONE` reader - When the controller is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done."]
pub struct R_RX_DONE_R(crate::FieldReader<bool, bool>);
impl R_RX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_RX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_RX_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_TX_ABRT` reader - This bit indicates if the controller, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a \"transmit abort\". When this bit is set to 1, the I2C_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places. NOTE: The controller flushes/resets/empties the TX FIFO whenever this bit is set. The TX FIFO remains in this flushed state until the register I2C_CLR_TX_ABRT is read. Once this read is performed, the TX FIFO is then ready to accept more data bytes from the APB interface."]
pub struct R_TX_ABRT_R(crate::FieldReader<bool, bool>);
impl R_TX_ABRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_TX_ABRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_TX_ABRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_RD_REQ` reader - This bit is set to 1 when the controller is acting as a slave and another I2C master is attempting to read data from the controller. The controller holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the I2C_DATA_CMD register. This bit is set to 0 just after the processor reads the I2C_CLR_RD_REQ register"]
pub struct R_RD_REQ_R(crate::FieldReader<bool, bool>);
impl R_RD_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_RD_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_RD_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_TX_EMPTY` reader - This bit is set to 1 when the transmit buffer is at or below the threshold value set in the I2C_TX_TL register. It is automatically cleared by hardware when the buffer level goes above the threshold. When the IC_ENABLE bit 0 is 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer activity, then with ic_en=0, this bit is set to 0."]
pub struct R_TX_EMPTY_R(crate::FieldReader<bool, bool>);
impl R_TX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_TX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_TX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_TX_OVER` reader - Set during transmit if the transmit buffer is filled to 32 and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared"]
pub struct R_TX_OVER_R(crate::FieldReader<bool, bool>);
impl R_TX_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_TX_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_TX_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_RX_FULL` reader - Set when the receive buffer reaches or goes above the RX_TL threshold in the I2C_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (I2C_ENABLE\\[0\\]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the I2C_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues."]
pub struct R_RX_FULL_R(crate::FieldReader<bool, bool>);
impl R_RX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_RX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_RX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_RX_OVER` reader - Set if the receive buffer is completely filled to 32 and an additional byte is received from an external I2C device. The controller acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (I2C_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared."]
pub struct R_RX_OVER_R(crate::FieldReader<bool, bool>);
impl R_RX_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_RX_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_RX_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_RX_UNDER` reader - Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (I2C_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared."]
pub struct R_RX_UNDER_R(crate::FieldReader<bool, bool>);
impl R_RX_UNDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        R_RX_UNDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_RX_UNDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 11 - Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling controller or when the CPU reads bit 0 of the I2C_CLR_GEN_CALL register. The controller stores the received data in the Rx buffer."]
    #[inline(always)]
    pub fn r_gen_call(&self) -> R_GEN_CALL_R {
        R_GEN_CALL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether controller is operating in slave or master mode."]
    #[inline(always)]
    pub fn r_start_det(&self) -> R_START_DET_R {
        R_START_DET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates whether a STOP condition has occurred on the I2C interface regardless of whether controller is operating in slave or master mode."]
    #[inline(always)]
    pub fn r_stop_det(&self) -> R_STOP_DET_R {
        R_STOP_DET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit captures I2C Ctrl activity and stays set until it is cleared. There are four ways to clear it: => Disabling the I2C Ctrl => Reading the IC_CLR_ACTIVITY register => Reading the IC_CLR_INTR register => System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the controller module is idle, this bit remains set until cleared, indicating that there was activity on the bus."]
    #[inline(always)]
    pub fn r_activity(&self) -> R_ACTIVITY_R {
        R_ACTIVITY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - When the controller is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done."]
    #[inline(always)]
    pub fn r_rx_done(&self) -> R_RX_DONE_R {
        R_RX_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit indicates if the controller, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a \"transmit abort\". When this bit is set to 1, the I2C_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places. NOTE: The controller flushes/resets/empties the TX FIFO whenever this bit is set. The TX FIFO remains in this flushed state until the register I2C_CLR_TX_ABRT is read. Once this read is performed, the TX FIFO is then ready to accept more data bytes from the APB interface."]
    #[inline(always)]
    pub fn r_tx_abrt(&self) -> R_TX_ABRT_R {
        R_TX_ABRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set to 1 when the controller is acting as a slave and another I2C master is attempting to read data from the controller. The controller holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the I2C_DATA_CMD register. This bit is set to 0 just after the processor reads the I2C_CLR_RD_REQ register"]
    #[inline(always)]
    pub fn r_rd_req(&self) -> R_RD_REQ_R {
        R_RD_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is set to 1 when the transmit buffer is at or below the threshold value set in the I2C_TX_TL register. It is automatically cleared by hardware when the buffer level goes above the threshold. When the IC_ENABLE bit 0 is 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer activity, then with ic_en=0, this bit is set to 0."]
    #[inline(always)]
    pub fn r_tx_empty(&self) -> R_TX_EMPTY_R {
        R_TX_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Set during transmit if the transmit buffer is filled to 32 and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared"]
    #[inline(always)]
    pub fn r_tx_over(&self) -> R_TX_OVER_R {
        R_TX_OVER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Set when the receive buffer reaches or goes above the RX_TL threshold in the I2C_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (I2C_ENABLE\\[0\\]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the I2C_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues."]
    #[inline(always)]
    pub fn r_rx_full(&self) -> R_RX_FULL_R {
        R_RX_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Set if the receive buffer is completely filled to 32 and an additional byte is received from an external I2C device. The controller acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (I2C_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared."]
    #[inline(always)]
    pub fn r_rx_over(&self) -> R_RX_OVER_R {
        R_RX_OVER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (I2C_ENABLE\\[0\\]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared."]
    #[inline(always)]
    pub fn r_rx_under(&self) -> R_RX_UNDER_R {
        R_RX_UNDER_R::new((self.bits & 1) != 0)
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
#[doc = "I2C Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_intr_stat_reg](index.html) module"]
pub struct I2C_INTR_STAT_REG_SPEC;
impl crate::RegisterSpec for I2C_INTR_STAT_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_intr_stat_reg::R](R) reader structure"]
impl crate::Readable for I2C_INTR_STAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_intr_stat_reg::W](W) writer structure"]
impl crate::Writable for I2C_INTR_STAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_INTR_STAT_REG to value 0"]
impl crate::Resettable for I2C_INTR_STAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
