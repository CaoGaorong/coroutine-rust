
// #[repr(C, packed)]
pub struct Context {
    pub ret: usize,
    pub rbx: usize, 
    pub rcx: usize,
    pub rdx: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rsp: usize,
    pub rbp: usize,
    pub r8: usize,
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
}

impl  Context {
    
    pub fn new(ret: usize, rdi: usize, rsp: usize) -> Self {
        Self {
            ret,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi,
            rsp,
            rbp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
        }
    }

    pub fn empty() -> Self {
        Self::new(0, 0, 0)
    }
}
