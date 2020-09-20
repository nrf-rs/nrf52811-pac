#[doc = "Reader of register DFESTATUS"]
pub type R = crate::R<u32, super::DFESTATUS>;
#[doc = "Internal state of switching state machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWITCHINGSTATE_A {
    #[doc = "0: Switching state Idle"]
    IDLE = 0,
    #[doc = "1: Switching state Offset"]
    OFFSET = 1,
    #[doc = "2: Switching state Guard"]
    GUARD = 2,
    #[doc = "3: Switching state Ref"]
    REF = 3,
    #[doc = "4: Switching state Switching"]
    SWITCHING = 4,
    #[doc = "5: Switching state Ending"]
    ENDING = 5,
}
impl From<SWITCHINGSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: SWITCHINGSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWITCHINGSTATE`"]
pub type SWITCHINGSTATE_R = crate::R<u8, SWITCHINGSTATE_A>;
impl SWITCHINGSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWITCHINGSTATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWITCHINGSTATE_A::IDLE),
            1 => Val(SWITCHINGSTATE_A::OFFSET),
            2 => Val(SWITCHINGSTATE_A::GUARD),
            3 => Val(SWITCHINGSTATE_A::REF),
            4 => Val(SWITCHINGSTATE_A::SWITCHING),
            5 => Val(SWITCHINGSTATE_A::ENDING),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SWITCHINGSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `OFFSET`"]
    #[inline(always)]
    pub fn is_offset(&self) -> bool {
        *self == SWITCHINGSTATE_A::OFFSET
    }
    #[doc = "Checks if the value of the field is `GUARD`"]
    #[inline(always)]
    pub fn is_guard(&self) -> bool {
        *self == SWITCHINGSTATE_A::GUARD
    }
    #[doc = "Checks if the value of the field is `REF`"]
    #[inline(always)]
    pub fn is_ref_(&self) -> bool {
        *self == SWITCHINGSTATE_A::REF
    }
    #[doc = "Checks if the value of the field is `SWITCHING`"]
    #[inline(always)]
    pub fn is_switching(&self) -> bool {
        *self == SWITCHINGSTATE_A::SWITCHING
    }
    #[doc = "Checks if the value of the field is `ENDING`"]
    #[inline(always)]
    pub fn is_ending(&self) -> bool {
        *self == SWITCHINGSTATE_A::ENDING
    }
}
#[doc = "Internal state of sampling state machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLINGSTATE_A {
    #[doc = "0: Sampling state Idle"]
    IDLE = 0,
    #[doc = "1: Sampling state Sampling"]
    SAMPLING = 1,
}
impl From<SAMPLINGSTATE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLINGSTATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAMPLINGSTATE`"]
pub type SAMPLINGSTATE_R = crate::R<bool, SAMPLINGSTATE_A>;
impl SAMPLINGSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLINGSTATE_A {
        match self.bits {
            false => SAMPLINGSTATE_A::IDLE,
            true => SAMPLINGSTATE_A::SAMPLING,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SAMPLINGSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `SAMPLING`"]
    #[inline(always)]
    pub fn is_sampling(&self) -> bool {
        *self == SAMPLINGSTATE_A::SAMPLING
    }
}
impl R {
    #[doc = "Bits 0:2 - Internal state of switching state machine"]
    #[inline(always)]
    pub fn switchingstate(&self) -> SWITCHINGSTATE_R {
        SWITCHINGSTATE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Internal state of sampling state machine"]
    #[inline(always)]
    pub fn samplingstate(&self) -> SAMPLINGSTATE_R {
        SAMPLINGSTATE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
