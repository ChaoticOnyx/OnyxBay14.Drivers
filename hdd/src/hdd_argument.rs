const ARGUMENTS_OFFSET: usize = 0x100;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HddArgument {
    Arg0 = 0,
    Arg1,
    Arg2,
    Arg3,
    Arg4,
    Arg5,
    Arg6,
    Arg7,
    Arg8,
    Arg9,
}

impl HddArgument {
    pub fn offset(&self) -> usize {
        *self as usize + ARGUMENTS_OFFSET
    }
}

impl From<u8> for HddArgument {
    fn from(value: u8) -> Self {
        if value >= Self::Arg0 as u8 && value <= Self::Arg9 as u8 {
            unsafe { core::mem::transmute(value) }
        } else {
            Self::Arg0
        }
    }
}
