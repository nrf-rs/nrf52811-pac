#[doc = "Reader of register RAM"]
pub type R = crate::R<u32, super::RAM>;
#[doc = "RAM variant\n\nValue on reset: 24"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RAM_A {
    #[doc = "24: 24 kByte RAM"]
    K24 = 24,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<RAM_A> for u32 {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAM`"]
pub type RAM_R = crate::R<u32, RAM_A>;
impl RAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RAM_A> {
        use crate::Variant::*;
        match self.bits {
            24 => Val(RAM_A::K24),
            4294967295 => Val(RAM_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K24`"]
    #[inline(always)]
    pub fn is_k24(&self) -> bool {
        *self == RAM_A::K24
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == RAM_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - RAM variant"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
