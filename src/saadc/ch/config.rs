#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0x0002_0000"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0000
    }
}
#[doc = "Positive channel resistor control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESP_A {
    #[doc = "0: Bypass resistor ladder"]
    BYPASS = 0,
    #[doc = "1: Pull-down to GND"]
    PULLDOWN = 1,
    #[doc = "2: Pull-up to VDD"]
    PULLUP = 2,
    #[doc = "3: Set input at VDD/2"]
    VDD1_2 = 3,
}
impl From<RESP_A> for u8 {
    #[inline(always)]
    fn from(variant: RESP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESP`"]
pub type RESP_R = crate::R<u8, RESP_A>;
impl RESP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESP_A {
        match self.bits {
            0 => RESP_A::BYPASS,
            1 => RESP_A::PULLDOWN,
            2 => RESP_A::PULLUP,
            3 => RESP_A::VDD1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == RESP_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == RESP_A::PULLDOWN
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == RESP_A::PULLUP
    }
    #[doc = "Checks if the value of the field is `VDD1_2`"]
    #[inline(always)]
    pub fn is_vdd1_2(&self) -> bool {
        *self == RESP_A::VDD1_2
    }
}
#[doc = "Write proxy for field `RESP`"]
pub struct RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bypass resistor ladder"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(RESP_A::BYPASS)
    }
    #[doc = "Pull-down to GND"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(RESP_A::PULLDOWN)
    }
    #[doc = "Pull-up to VDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(RESP_A::PULLUP)
    }
    #[doc = "Set input at VDD/2"]
    #[inline(always)]
    pub fn vdd1_2(self) -> &'a mut W {
        self.variant(RESP_A::VDD1_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Negative channel resistor control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESN_A {
    #[doc = "0: Bypass resistor ladder"]
    BYPASS = 0,
    #[doc = "1: Pull-down to GND"]
    PULLDOWN = 1,
    #[doc = "2: Pull-up to VDD"]
    PULLUP = 2,
    #[doc = "3: Set input at VDD/2"]
    VDD1_2 = 3,
}
impl From<RESN_A> for u8 {
    #[inline(always)]
    fn from(variant: RESN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESN`"]
pub type RESN_R = crate::R<u8, RESN_A>;
impl RESN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESN_A {
        match self.bits {
            0 => RESN_A::BYPASS,
            1 => RESN_A::PULLDOWN,
            2 => RESN_A::PULLUP,
            3 => RESN_A::VDD1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == RESN_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == RESN_A::PULLDOWN
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == RESN_A::PULLUP
    }
    #[doc = "Checks if the value of the field is `VDD1_2`"]
    #[inline(always)]
    pub fn is_vdd1_2(&self) -> bool {
        *self == RESN_A::VDD1_2
    }
}
#[doc = "Write proxy for field `RESN`"]
pub struct RESN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bypass resistor ladder"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(RESN_A::BYPASS)
    }
    #[doc = "Pull-down to GND"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(RESN_A::PULLDOWN)
    }
    #[doc = "Pull-up to VDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(RESN_A::PULLUP)
    }
    #[doc = "Set input at VDD/2"]
    #[inline(always)]
    pub fn vdd1_2(self) -> &'a mut W {
        self.variant(RESN_A::VDD1_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Gain control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN_A {
    #[doc = "0: 1/6"]
    GAIN1_6 = 0,
    #[doc = "1: 1/5"]
    GAIN1_5 = 1,
    #[doc = "2: 1/4"]
    GAIN1_4 = 2,
    #[doc = "3: 1/3"]
    GAIN1_3 = 3,
    #[doc = "4: 1/2"]
    GAIN1_2 = 4,
    #[doc = "5: 1"]
    GAIN1 = 5,
    #[doc = "6: 2"]
    GAIN2 = 6,
    #[doc = "7: 4"]
    GAIN4 = 7,
}
impl From<GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN`"]
pub type GAIN_R = crate::R<u8, GAIN_A>;
impl GAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN_A {
        match self.bits {
            0 => GAIN_A::GAIN1_6,
            1 => GAIN_A::GAIN1_5,
            2 => GAIN_A::GAIN1_4,
            3 => GAIN_A::GAIN1_3,
            4 => GAIN_A::GAIN1_2,
            5 => GAIN_A::GAIN1,
            6 => GAIN_A::GAIN2,
            7 => GAIN_A::GAIN4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GAIN1_6`"]
    #[inline(always)]
    pub fn is_gain1_6(&self) -> bool {
        *self == GAIN_A::GAIN1_6
    }
    #[doc = "Checks if the value of the field is `GAIN1_5`"]
    #[inline(always)]
    pub fn is_gain1_5(&self) -> bool {
        *self == GAIN_A::GAIN1_5
    }
    #[doc = "Checks if the value of the field is `GAIN1_4`"]
    #[inline(always)]
    pub fn is_gain1_4(&self) -> bool {
        *self == GAIN_A::GAIN1_4
    }
    #[doc = "Checks if the value of the field is `GAIN1_3`"]
    #[inline(always)]
    pub fn is_gain1_3(&self) -> bool {
        *self == GAIN_A::GAIN1_3
    }
    #[doc = "Checks if the value of the field is `GAIN1_2`"]
    #[inline(always)]
    pub fn is_gain1_2(&self) -> bool {
        *self == GAIN_A::GAIN1_2
    }
    #[doc = "Checks if the value of the field is `GAIN1`"]
    #[inline(always)]
    pub fn is_gain1(&self) -> bool {
        *self == GAIN_A::GAIN1
    }
    #[doc = "Checks if the value of the field is `GAIN2`"]
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == GAIN_A::GAIN2
    }
    #[doc = "Checks if the value of the field is `GAIN4`"]
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == GAIN_A::GAIN4
    }
}
#[doc = "Write proxy for field `GAIN`"]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1/6"]
    #[inline(always)]
    pub fn gain1_6(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN1_6)
    }
    #[doc = "1/5"]
    #[inline(always)]
    pub fn gain1_5(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN1_5)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn gain1_4(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN1_4)
    }
    #[doc = "1/3"]
    #[inline(always)]
    pub fn gain1_3(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN1_3)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn gain1_2(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN1_2)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn gain1(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn gain2(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn gain4(self) -> &'a mut W {
        self.variant(GAIN_A::GAIN4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reference control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSEL_A {
    #[doc = "0: Internal reference (0.6 V)"]
    INTERNAL = 0,
    #[doc = "1: VDD/4 as reference"]
    VDD1_4 = 1,
}
impl From<REFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<bool, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            false => REFSEL_A::INTERNAL,
            true => REFSEL_A::VDD1_4,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFSEL_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `VDD1_4`"]
    #[inline(always)]
    pub fn is_vdd1_4(&self) -> bool {
        *self == REFSEL_A::VDD1_4
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal reference (0.6 V)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(REFSEL_A::INTERNAL)
    }
    #[doc = "VDD/4 as reference"]
    #[inline(always)]
    pub fn vdd1_4(self) -> &'a mut W {
        self.variant(REFSEL_A::VDD1_4)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Acquisition time, the time the ADC uses to sample the input voltage\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TACQ_A {
    #[doc = "0: 3 us"]
    _3US = 0,
    #[doc = "1: 5 us"]
    _5US = 1,
    #[doc = "2: 10 us"]
    _10US = 2,
    #[doc = "3: 15 us"]
    _15US = 3,
    #[doc = "4: 20 us"]
    _20US = 4,
    #[doc = "5: 40 us"]
    _40US = 5,
}
impl From<TACQ_A> for u8 {
    #[inline(always)]
    fn from(variant: TACQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TACQ`"]
pub type TACQ_R = crate::R<u8, TACQ_A>;
impl TACQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TACQ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TACQ_A::_3US),
            1 => Val(TACQ_A::_5US),
            2 => Val(TACQ_A::_10US),
            3 => Val(TACQ_A::_15US),
            4 => Val(TACQ_A::_20US),
            5 => Val(TACQ_A::_40US),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_3US`"]
    #[inline(always)]
    pub fn is_3us(&self) -> bool {
        *self == TACQ_A::_3US
    }
    #[doc = "Checks if the value of the field is `_5US`"]
    #[inline(always)]
    pub fn is_5us(&self) -> bool {
        *self == TACQ_A::_5US
    }
    #[doc = "Checks if the value of the field is `_10US`"]
    #[inline(always)]
    pub fn is_10us(&self) -> bool {
        *self == TACQ_A::_10US
    }
    #[doc = "Checks if the value of the field is `_15US`"]
    #[inline(always)]
    pub fn is_15us(&self) -> bool {
        *self == TACQ_A::_15US
    }
    #[doc = "Checks if the value of the field is `_20US`"]
    #[inline(always)]
    pub fn is_20us(&self) -> bool {
        *self == TACQ_A::_20US
    }
    #[doc = "Checks if the value of the field is `_40US`"]
    #[inline(always)]
    pub fn is_40us(&self) -> bool {
        *self == TACQ_A::_40US
    }
}
#[doc = "Write proxy for field `TACQ`"]
pub struct TACQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TACQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TACQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "3 us"]
    #[inline(always)]
    pub fn _3us(self) -> &'a mut W {
        self.variant(TACQ_A::_3US)
    }
    #[doc = "5 us"]
    #[inline(always)]
    pub fn _5us(self) -> &'a mut W {
        self.variant(TACQ_A::_5US)
    }
    #[doc = "10 us"]
    #[inline(always)]
    pub fn _10us(self) -> &'a mut W {
        self.variant(TACQ_A::_10US)
    }
    #[doc = "15 us"]
    #[inline(always)]
    pub fn _15us(self) -> &'a mut W {
        self.variant(TACQ_A::_15US)
    }
    #[doc = "20 us"]
    #[inline(always)]
    pub fn _20us(self) -> &'a mut W {
        self.variant(TACQ_A::_20US)
    }
    #[doc = "40 us"]
    #[inline(always)]
    pub fn _40us(self) -> &'a mut W {
        self.variant(TACQ_A::_40US)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Enable differential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Single ended, PSELN will be ignored, negative input to ADC shorted to GND"]
    SE = 0,
    #[doc = "1: Differential"]
    DIFF = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::SE,
            true => MODE_A::DIFF,
        }
    }
    #[doc = "Checks if the value of the field is `SE`"]
    #[inline(always)]
    pub fn is_se(&self) -> bool {
        *self == MODE_A::SE
    }
    #[doc = "Checks if the value of the field is `DIFF`"]
    #[inline(always)]
    pub fn is_diff(&self) -> bool {
        *self == MODE_A::DIFF
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single ended, PSELN will be ignored, negative input to ADC shorted to GND"]
    #[inline(always)]
    pub fn se(self) -> &'a mut W {
        self.variant(MODE_A::SE)
    }
    #[doc = "Differential"]
    #[inline(always)]
    pub fn diff(self) -> &'a mut W {
        self.variant(MODE_A::DIFF)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Enable burst mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURST_A {
    #[doc = "0: Burst mode is disabled (normal operation)"]
    DISABLED = 0,
    #[doc = "1: Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    ENABLED = 1,
}
impl From<BURST_A> for bool {
    #[inline(always)]
    fn from(variant: BURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BURST`"]
pub type BURST_R = crate::R<bool, BURST_A>;
impl BURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURST_A {
        match self.bits {
            false => BURST_A::DISABLED,
            true => BURST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BURST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BURST_A::ENABLED
    }
}
#[doc = "Write proxy for field `BURST`"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Burst mode is disabled (normal operation)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BURST_A::DISABLED)
    }
    #[doc = "Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BURST_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Positive channel resistor control"]
    #[inline(always)]
    pub fn resp(&self) -> RESP_R {
        RESP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Negative channel resistor control"]
    #[inline(always)]
    pub fn resn(&self) -> RESN_R {
        RESN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Gain control"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Reference control"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Acquisition time, the time the ADC uses to sample the input voltage"]
    #[inline(always)]
    pub fn tacq(&self) -> TACQ_R {
        TACQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Enable differential mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable burst mode"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Positive channel resistor control"]
    #[inline(always)]
    pub fn resp(&mut self) -> RESP_W {
        RESP_W { w: self }
    }
    #[doc = "Bits 4:5 - Negative channel resistor control"]
    #[inline(always)]
    pub fn resn(&mut self) -> RESN_W {
        RESN_W { w: self }
    }
    #[doc = "Bits 8:10 - Gain control"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Bit 12 - Reference control"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bits 16:18 - Acquisition time, the time the ADC uses to sample the input voltage"]
    #[inline(always)]
    pub fn tacq(&mut self) -> TACQ_W {
        TACQ_W { w: self }
    }
    #[doc = "Bit 20 - Enable differential mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 24 - Enable burst mode"]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
}
