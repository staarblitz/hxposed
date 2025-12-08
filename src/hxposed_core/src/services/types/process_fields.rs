use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct ProcessProtection {
    #[bits(3)]
    pub protection_type: ProtectionType,
    #[bits(1)]
    pub audit: bool,
    #[bits(4)]
    pub signer: ProtectionSigner,
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct ProcessFlags {
    pub fail_fast_on_commit_fail: bool,
    pub break_on_termination: bool,
    pub primary_token_frozen: bool,
    pub restrict_set_thread_context: bool,
    pub auxiliary_process: bool,
    pub high_graphics_priority: bool,
    pub system_process: bool,
    pub high_memory_priority: bool,
    pub disallow_user_terminate: bool,
    pub memory_compression_process: bool,
}

#[bitfield(u64)]
pub struct MitigationOptions {
    #[bits(32)]
    pub options1: MitigationOptions1,
    #[bits(32)]
    pub options2: MitigationOptions2,
}

#[bitfield(u32)]
pub struct MitigationOptions1 {
    pub control_flow_guard_enabled: bool,
    pub control_flow_guard_export_suppression_enabled: bool,
    pub control_flow_guard_strict: bool,
    pub disallow_stripped_images: bool,
    pub force_relocate_images: bool,
    pub high_entropy_aslr_enabled: bool,
    pub stack_randomization_disabled: bool,
    pub extension_point_disable: bool,
    pub disable_dynamic_code: bool,
    pub disable_dynamic_code_allow_opt_out: bool,
    pub disable_dynamic_code_allow_remote_downgrade: bool,
    pub audit_disable_dynamic_code: bool,
    pub disallow_win32k_system_calls: bool,
    pub audit_disallow_win32k_system_calls: bool,
    pub enable_filtered_win32k_apis: bool,
    pub audit_filtered_win32k_apis: bool,
    pub disable_non_system_fonts: bool,
    pub audit_non_system_font_loading: bool,
    pub prefer_system32_images: bool,
    pub prohibit_remote_image_map: bool,
    pub audit_prohibit_remote_image_map: bool,
    pub prohibit_low_il_image_map: bool,
    pub audit_prohibit_low_il_image_map: bool,
    pub signature_mitigation_opt_in: bool,
    pub audit_block_non_microsoft_binaries: bool,
    pub audit_block_non_microsoft_binaries_allow_store: bool,
    pub loader_integrity_continuity_enabled: bool,
    pub audit_loader_integrity_continuity: bool,
    pub enable_module_tampering_protection: bool,
    pub enable_module_tampering_protection_no_inherit: bool,
    pub restrict_indirect_branch_prediction: bool,
    pub isolate_security_domain: bool,
}

#[bitfield(u32)]
pub struct MitigationOptions2 {
    pub enable_export_address_filter: bool,
    pub audit_export_address_filter: bool,
    pub enable_export_address_filter_plus: bool,
    pub audit_export_address_filter_plus: bool,
    pub enable_rop_stack_pivot: bool,
    pub audit_rop_stack_pivot: bool,
    pub enable_rop_caller_check: bool,
    pub audit_rop_caller_check: bool,
    pub enable_rop_sim_exec: bool,
    pub audit_rop_sim_exec: bool,
    pub enable_import_address_filter: bool,
    pub audit_import_address_filter: bool,
    pub disable_page_combine: bool,
    pub speculative_store_bypass_disable: bool,
    pub cet_user_shadow_stacks: bool,
    pub audit_cet_user_shadow_stacks: bool,
    pub audit_cet_user_shadow_stacks_logged: bool,
    pub user_cet_set_context_ip_validation: bool,
    pub audit_user_cet_set_context_ip_validation: bool,
    pub audit_user_cet_set_context_ip_validation_logged: bool,
    pub cet_user_shadow_stacks_strict_mode: bool,
    pub block_non_cet_binaries: bool,
    pub block_non_cet_binaries_non_ehcont: bool,
    pub audit_block_non_cet_binaries: bool,
    pub audit_block_non_cet_binaries_logged: bool,
    pub xtended_control_flow_guard_deprecated: bool,
    pub audit_xtended_control_flow_guard_deprecated: bool,
    pub pointer_auth_user_ip: bool,
    pub audit_pointer_auth_user_ip: bool,
    pub audit_pointer_auth_user_ip_logged: bool,
    pub cet_dynamic_apis_out_of_proc_only: bool,
    pub user_cet_set_context_ip_validation_relaxed_mode: bool,
}

/*#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct ProcessMitigationOptions {
    pub control_flow_guard: ControlFlowGuard,
    pub image_rules: ImageRules,
    pub aslr: AslrRules,
    pub dynamic_code: DynamicCode,
    pub win32k_rules: Win32kRules,
    pub export_address_filter: ExportAddressFilter,
    pub import_address_filter: ImportAddressFilter,
    pub misc_rules: MiscRules
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Win32kRules {
    pub disallow: bool,
    pub audit_disallow: bool,
    pub enable_filtered: bool,
    pub audit_enable_filtered: bool,
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct ExportAddressFilter {
    pub enable: bool,
    pub audit_enable: bool,
    pub enable_plus: bool,
    pub audit_enable_plus: bool,
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct ImportAddressFilter {
    pub enable: bool,
    pub audit_enable: bool,
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct MiscRules {
    pub stack_randomization_disabled: bool,
    pub restrict_indirect_branch_prediction: bool,
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct ModuleTamperingRules {
    pub enable: bool,
    pub no_inherit: bool,
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct DynamicCode {
    pub disabled: bool,
    pub disabled_opt_out: bool,
    pub disabled_allow_remote_downgrade: bool,
    pub audit: bool
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AslrRules{
    pub high_entropy: bool,
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct ImageRules {
    pub disallow_stripped_images: bool,
    pub force_relocate_images: bool,
    pub disable_non_system_fonts: bool,
    pub audit_non_system_fonts: bool,
    pub prefer_system32_images: bool,
    pub prohibit_remote_image_map: bool,
    pub audit_prohibit_remote_image_map: bool,
    pub prohibit_low_il_image_map: bool,
    pub audit_prohibit_low_il_image_map: bool,
    pub audit_block_non_microsoft_binaries: bool,
    pub audit_block_non_microsoft_binaries_allow_store: bool,
    pub loader_integrity_continuity_enabled: bool,
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct ControlFlowGuard{
    pub enabled: bool,
    pub export_suppression: bool,
    pub strict: bool,
}*/

#[derive(Copy, Clone, Default, Debug)]
#[repr(u8)]
pub enum ProtectionSigner {
    #[default]
    None = 0,
    Authenticode = 1,
    CodeGen = 2,
    AntiMalware = 3,
    Lsa = 4,
    Windows = 5,
    WinTcb = 6,
    Max = 7,
}

impl ProtectionSigner {
    pub const fn from_bits(value: u8) -> Self {
        match value {
            1 => ProtectionSigner::Authenticode,
            2 => ProtectionSigner::CodeGen,
            3 => ProtectionSigner::AntiMalware,
            4 => ProtectionSigner::Lsa,
            5 => ProtectionSigner::Windows,
            6 => ProtectionSigner::WinTcb,
            7 => ProtectionSigner::Max,
            _ => ProtectionSigner::None
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as _
    }
}

#[bitfield(u16)]
pub struct ProcessSignatureLevels {
    #[bits(8)]
    pub signature_level: ProcessSignatureLevel,
    #[bits(8)]
    pub section_signature_level: u8
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(u8)]
pub enum ProcessSignatureLevel {
    #[default]
    Unchecked = 0,
    Unsigned = 1,
    Enterprise = 2,
    Custom = 3,
    Authenticode = 4,
    Custom2 = 5,
    Store = 6,
    AntiMalware = 7,
    Microsoft = 8,
    Custom4 = 9,
    Custom5 = 10,
    DynamicCodeGen = 11,
    Windows = 12,
    WindowsPPL = 13,
    WindowsTcb = 14,
    Custom6 = 15
}

impl ProcessSignatureLevel {
    pub const fn from_bits(value: u8) -> Self {
        match value {
            1 => ProcessSignatureLevel::Unsigned,
            2 => ProcessSignatureLevel::Enterprise,
            3 => ProcessSignatureLevel::Custom,
            4 => ProcessSignatureLevel::Authenticode,
            5 => ProcessSignatureLevel::Custom2,
            6 => ProcessSignatureLevel::Store,
            7 => ProcessSignatureLevel::AntiMalware,
            8 => ProcessSignatureLevel::Microsoft,
            9 => ProcessSignatureLevel::Custom4,
            10 => ProcessSignatureLevel::Custom5,
            11 => ProcessSignatureLevel::DynamicCodeGen,
            12 => ProcessSignatureLevel::Windows,
            13 => ProcessSignatureLevel::WindowsPPL,
            14 => ProcessSignatureLevel::WindowsTcb,
            15 => ProcessSignatureLevel::Custom6,
            _ => ProcessSignatureLevel::Unchecked,
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as _
    }
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(u8)]
pub enum ProtectionType {
    #[default]
    None = 0,
    Light = 1,
    Protected = 2,
    Max = 3,
}

impl ProtectionType {
    pub const fn from_bits(value: u8) -> Self {
        match value {
            1 => ProtectionType::Light,
            2 => ProtectionType::Protected,
            3 => ProtectionType::Max,
            _ => ProtectionType::None,
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as _
    }
}