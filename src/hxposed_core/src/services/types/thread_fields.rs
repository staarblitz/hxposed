use core::mem;

#[derive(Debug, Clone,Default)]
#[repr(align(16))]
#[repr(C)]
#[allow(non_snake_case)]
pub struct Amd64Context {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5Home: u64,
    pub P6Home: u64,
    pub ContextFlags: u32,
    pub MxCsr: u32,
    pub SegCs: u16,
    pub SegDs: u16,
    pub SegEs: u16,
    pub SegFs: u16,
    pub SegGs: u16,
    pub SegSs: u16,
    pub EFlags: u32,
    pub Dr0: u64,
    pub Dr1: u64,
    pub Dr2: u64,
    pub Dr3: u64,
    pub Dr6: u64,
    pub Dr7: u64,
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub Rsp: u64,
    pub Rbp: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Rip: u64,
    pub __bindgen_anon_1: _CONTEXT__bindgen_ty_1,
    pub VectorRegister: [M128A; 26usize],
    pub VectorControl: u64,
    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default)]
#[allow(non_snake_case, non_camel_case_types)]
pub union _CONTEXT__bindgen_ty_1 {
    pub FltSave: _XSAVE_FORMAT,
    pub __bindgen_anon_1: _CONTEXT__bindgen_ty_1__bindgen_ty_1,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct _CONTEXT__bindgen_ty_1__bindgen_ty_1 {
    pub Header: [M128A; 2usize],
    pub Legacy: [M128A; 8usize],
    pub Xmm0: M128A,
    pub Xmm1: M128A,
    pub Xmm2: M128A,
    pub Xmm3: M128A,
    pub Xmm4: M128A,
    pub Xmm5: M128A,
    pub Xmm6: M128A,
    pub Xmm7: M128A,
    pub Xmm8: M128A,
    pub Xmm9: M128A,
    pub Xmm10: M128A,
    pub Xmm11: M128A,
    pub Xmm12: M128A,
    pub Xmm13: M128A,
    pub Xmm14: M128A,
    pub Xmm15: M128A,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct _XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u16,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u8,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [M128A; 8usize],
    pub XmmRegisters: [M128A; 16usize],
    pub Reserved4: [u8; 96usize],
}

impl Default for _XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[repr(align(16))]
#[allow(non_snake_case)]
#[derive(Debug, Default, Copy, Clone)]
pub struct M128A {
    pub Low: u64,
    pub High: i64,
}