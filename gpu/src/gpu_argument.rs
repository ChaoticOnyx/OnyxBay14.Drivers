const ARGUMENTS_OFFSET: usize = 0x100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GpuArgument {
    Arg0,
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

impl GpuArgument {
    pub fn offset(&self) -> usize {
        let offset = match self {
            GpuArgument::Arg0 => 0,
            GpuArgument::Arg1 => 1,
            GpuArgument::Arg2 => 2,
            GpuArgument::Arg3 => 3,
            GpuArgument::Arg4 => 4,
            GpuArgument::Arg5 => 5,
            GpuArgument::Arg6 => 6,
            GpuArgument::Arg7 => 7,
            GpuArgument::Arg8 => 8,
            GpuArgument::Arg9 => 9,
        };

        offset + ARGUMENTS_OFFSET
    }
}
