#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HxLoaderParameterBlock {
    pub base_address: u64,
    pub pe_size: u64,
    pub booted_from_hxloader: bool,
}

#[unsafe(link_section = ".hxprm")]
#[unsafe(no_mangle)]
#[used]
pub static HX_LOADER_PARAMETER_BLOCK: HxLoaderParameterBlock = HxLoaderParameterBlock {
    base_address: 0,
    pe_size: 0,
    booted_from_hxloader: false,
};
