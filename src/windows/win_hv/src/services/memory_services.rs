use crate::nt::context::ApcProcessContext;
use crate::plugins::PluginTable;
use crate::plugins::commands::memory::*;
use crate::plugins::plugin::Plugin;
use crate::utils::blanket::OpenHandle;
use crate::win::{MmCopyVirtualMemory, ZwProtectVirtualMemory};
use alloc::boxed::Box;
use core::ops::BitAnd;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::{NotAllowedReason, NotFoundReason};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::memory::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use hxposed_core::services::types::memory_fields::MemoryProtection;
use wdk_sys::_MEMORY_CACHING_TYPE::MmCached;
use wdk_sys::_MM_PAGE_PRIORITY::HighPagePriority;
use wdk_sys::_MODE::{KernelMode, UserMode};
use wdk_sys::ntddk::{
    IoFreeMdl, MmAllocateContiguousMemorySpecifyCache, MmBuildMdlForNonPagedPool,
    MmFreeContiguousMemory, MmMapLockedPagesSpecifyCache, MmUnmapLockedPages,
};
use wdk_sys::{
    FALSE, PHYSICAL_ADDRESS, SIZE_T, STATUS_INVALID_PAGE_PROTECTION, STATUS_MEMORY_NOT_ALLOCATED,
    STATUS_SUCCESS, ULONG,
};

pub(crate) fn free_mdl_sync(request: &FreeMemoryAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let mdl = match plugin
        .object_table
        .pop_allocated_mdl(request.command.original_system_va)
    {
        Some(mdl) => mdl,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
    };

    unsafe {
        MmFreeContiguousMemory(mdl.MappedSystemVa);
        IoFreeMdl(mdl.ptr);
    }

    EmptyResponse::with_service(ServiceFunction::FreeMemory)
}

pub(crate) fn free_mdl_async(
    _guest: &mut dyn Guest,
    request: FreeMemoryRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(
        PluginPermissions::MEMORY_PHYSICAL
            | PluginPermissions::MEMORY_VIRTUAL
            | PluginPermissions::MEMORY_ALLOCATION,
    ) {
        return HypervisorResponse::not_allowed_perms(
            PluginPermissions::MEMORY_PHYSICAL
                | PluginPermissions::MEMORY_VIRTUAL
                | PluginPermissions::MEMORY_ALLOCATION,
        );
    };

    plugin.queue_command(Box::new(FreeMemoryAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::MapMemory)
}

pub(crate) fn map_mdl_sync(request: &MapMemoryAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let mdl = match plugin
        .object_table
        .get_allocated_mdl(request.command.original_system_va)
    {
        Some(mdl) => mdl,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
    };

    let process = match plugin
        .object_table
        .get_open_process(request.command.process as _)
    {
        None => plugin.process,
        Some(x) => x,
    };

    let _ctx = ApcProcessContext::begin(process);

    let result = match request.command.operation {
        MapMemoryOperation::Map => {
            let ptr = unsafe {
                match microseh::try_seh(|| {
                    MmMapLockedPagesSpecifyCache(
                        mdl.ptr,
                        UserMode as _,
                        MmCached,
                        request.command.map_address as _,
                        FALSE,
                        HighPagePriority as _,
                    )
                }) {
                    Ok(ptr) => ptr,
                    Err(_) => {
                        return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser);
                    }
                }
            };

            if ptr.is_null() {
                return HypervisorResponse::nt_error(STATUS_MEMORY_NOT_ALLOCATED as _);
            }

            MapMemoryResponse {
                mapped_address: ptr as _,
            }
            .into_raw()
        }
        MapMemoryOperation::Unmap => {
            unsafe { MmUnmapLockedPages(request.command.map_address as _, mdl.ptr) }
            EmptyResponse::with_service(ServiceFunction::MapMemory)
        }
    };

    // paranoid
    drop(_ctx);

    result
}

pub(crate) fn map_mdl_async(
    _guest: &mut dyn Guest,
    request: MapMemoryRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(
        PluginPermissions::MEMORY_PHYSICAL
            | PluginPermissions::MEMORY_VIRTUAL
            | PluginPermissions::MEMORY_ALLOCATION,
    ) {
        return HypervisorResponse::not_allowed_perms(
            PluginPermissions::MEMORY_PHYSICAL
                | PluginPermissions::MEMORY_VIRTUAL
                | PluginPermissions::MEMORY_ALLOCATION,
        );
    };

    plugin.queue_command(Box::new(MapMemoryAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::MapMemory)
}

pub(crate) fn allocate_mdl_sync(request: &AllocateMemoryAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let alloc = unsafe {
        MmAllocateContiguousMemorySpecifyCache(
            request.command.size as _,
            PHYSICAL_ADDRESS::default(),
            PHYSICAL_ADDRESS {
                QuadPart: u64::MAX as _,
            },
            PHYSICAL_ADDRESS::default(),
            MmCached as _,
        )
    };

    if alloc.is_null() {
        return HypervisorResponse::nt_error(STATUS_MEMORY_NOT_ALLOCATED as _);
    }

    let mdl = plugin
        .object_table
        .allocate_mdl(alloc, request.command.size);

    // this function maps the mdl. so MappedSystemVa is a valid field now.
    unsafe { MmBuildMdlForNonPagedPool(mdl.ptr) }

    AllocateMemoryResponse {
        address: mdl.MappedSystemVa as _,
        bytes_allocated: mdl.ByteCount,
    }
    .into_raw()
}

pub(crate) fn allocate_mdl_async(
    _guest: &mut dyn Guest,
    request: AllocateMemoryRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(
        PluginPermissions::MEMORY_PHYSICAL
            | PluginPermissions::MEMORY_VIRTUAL
            | PluginPermissions::MEMORY_ALLOCATION,
    ) {
        return HypervisorResponse::not_allowed_perms(
            PluginPermissions::MEMORY_PHYSICAL
                | PluginPermissions::MEMORY_VIRTUAL
                | PluginPermissions::MEMORY_ALLOCATION,
        );
    }

    plugin.queue_command(Box::new(AllocateMemoryAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::AllocateMemory)
}

///
/// # Protect Virtual Memory
///
/// Queues command for [`ProtectProcessMemoryAsyncCommand`] on specified plugin.
///
/// ## Arguments
/// * `guest` - Currently unused.
/// * `request` - Identifies the target process and the exit code to use. See [`ProtectProcessMemoryRequest`].
/// * `plugin` - The plugin requesting the operation. See [`Plugin`].
/// * `async_handle` - Handle object plugin created.
///
/// ## Warning
/// - This function only enqueues the request; success does **not** imply the process was actually terminated.
///
/// ## Return
/// * [`HypervisorResponse::not_found`] - The specified process does not exist.
/// * [`HypervisorResponse::not_allowed_perms`] - The plugin lacks the required permissions.
/// * [`HypervisorResponse::ok`] - The request was successfully enqueued.
/// * [`HypervisorResponse::nt_error`] - Invalid page protection specified.
pub(crate) fn protect_vm_async(
    _guest: &mut dyn Guest,
    mut request: ProtectProcessMemoryRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(
        PluginPermissions::PROCESS_MEMORY
            | PluginPermissions::MEMORY_VIRTUAL
            | PluginPermissions::MEMORY_PROTECT,
    ) {
        return HypervisorResponse::not_allowed_perms(
            PluginPermissions::PROCESS_MEMORY
                | PluginPermissions::MEMORY_VIRTUAL
                | PluginPermissions::MEMORY_PROTECT,
        );
    }

    request.protection = request
        .protection
        .bitand(!(MemoryProtection::GUARD | MemoryProtection::NO_CACHE));

    let prot = request.protection.bits();

    if !matches!(
        prot,
        x if x == MemoryProtection::NO_CACHE.bits()
            || x == MemoryProtection::READONLY.bits()
            || x == MemoryProtection::READWRITE.bits()
            || x == MemoryProtection::WRITECOPY.bits()
            || x == MemoryProtection::EXECUTE.bits()
            || x == MemoryProtection::EXECUTE_READ.bits()
            || x == MemoryProtection::EXECUTE_READWRITE.bits()
            || x == MemoryProtection::EXECUTE_WRITECOPY.bits()
    ) {
        return HypervisorResponse::nt_error(STATUS_INVALID_PAGE_PROTECTION as _);
    }

    plugin.queue_command(Box::new(ProtectProcessMemoryAsyncCommand {
        uuid: plugin.uuid,
        command: request,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::ProtectProcessMemory)
}

///
/// # Protect Virtual Memory (sync)
///
/// Sets the protection of virtual memory
///
/// ## Arguments
/// * `request` - Arguments for the request. See [`ProtectProcessMemoryAsyncCommand`].
///
/// ## Warning
/// - Caller must signal the request *after* calling this function.
///
/// ## Return
/// * [`HypervisorResponse::nt_error`] - An error occurred in `ZwProtectVirtualMemory`.
/// * [`ProtectProcessMemoryResponse`] - Number of bytes processed.
pub(crate) fn protect_vm_sync(request: &ProtectProcessMemoryAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let process = match plugin
        .object_table
        .get_open_process(request.command.process as _)
    {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };
    let mut base = request.command.address;
    let mut bytes_processed = 4096u64;
    let mut protection = ULONG::default();

    let handle = match process.get_handle() {
        Ok(handle) => handle,
        Err(err) => return HypervisorResponse::nt_error(err as _),
    };

    let result = match unsafe {
        ZwProtectVirtualMemory(
            handle.get_danger(),
            &mut base as *mut _ as u64 as _,
            &mut bytes_processed as *mut _ as _,
            request.command.protection.bits(),
            &mut protection,
        )
    } {
        STATUS_SUCCESS => ProtectProcessMemoryResponse {
            bytes_processed: bytes_processed as _,
            old_protection: MemoryProtection::from_bits(protection).unwrap(),
            base_address: base as _,
        }
        .into_raw(),
        err => HypervisorResponse::nt_error(err as _),
    };

    result
}

///
/// # Process VM (virtual memory, not virtual machine) Operation
///
/// Queues command for [`ProcessMemoryOperation`] on specified plugin.
///
/// ## Arguments
/// * `guest` - Currently unused.
/// * `request` - Identifies the target process and the exit code to use. See [`RWProcessMemoryRequest`].
/// * `plugin` - The plugin requesting the operation. See [`Plugin`].
/// * `async_handle` - Handle object plugin created.
///
/// ## Warning
/// - This function only enqueues the request; success does **not** imply the process was actually terminated.
///
/// ## Return
/// * [`HypervisorResponse::not_found`] - The specified process does not exist.
/// * [`HypervisorResponse::not_allowed_perms`] - The plugin lacks the required permissions.
/// * [`HypervisorResponse::ok`] - The request was successfully enqueued.
pub(crate) fn process_vm_operation_async(
    _guest: &mut dyn Guest,
    request: RWProcessMemoryRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::PROCESS_MEMORY | PluginPermissions::MEMORY_VIRTUAL) {
        return HypervisorResponse::not_allowed_perms(
            PluginPermissions::PROCESS_MEMORY | PluginPermissions::MEMORY_VIRTUAL,
        );
    }

    plugin.queue_command(Box::new(RWProcessMemoryAsyncCommand {
        uuid: plugin.uuid,
        command: request,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::ProcessVMOperation)
}

///
/// # Process VM Operation (sync)
///
/// Reads or writes to process memory
///
/// ## Arguments
/// * `request` - Arguments for the request. See [`RWProcessMemoryAsyncCommand`].
///
/// ## Warning
/// - Caller must signal the request *after* calling this function.
///
/// ## Return
/// * [`HypervisorResponse::nt_error`] - An error occurred in `MmCopyVirtualMemory`. Most likely page protection error.
/// * [`RWProcessMemoryResponse`] - Number of bytes processed.
pub(crate) fn process_vm_operation_sync(
    request: &RWProcessMemoryAsyncCommand,
) -> HypervisorResponse {
    if request.command.data_len > 4096 * 4 {
        return HypervisorResponse::not_allowed(NotAllowedReason::Unknown);
    }

    let plugin = match PluginTable::lookup(request.uuid) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let process = match plugin
        .object_table
        .get_open_process(request.command.process as _)
    {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };

    let mut return_size = SIZE_T::default();

    let (source, target) = match request.command.operation {
        ProcessMemoryOperation::Read => (process, plugin.process),
        ProcessMemoryOperation::Write => (plugin.process, process),
    };

    match unsafe {
        MmCopyVirtualMemory(
            source,
            request.command.address as _,
            target,
            request.command.data as _,
            request.command.data_len as _,
            KernelMode as _,
            &mut return_size,
        )
    } {
        STATUS_SUCCESS => RWProcessMemoryResponse {
            bytes_processed: return_size as _,
        }
        .into_raw(),
        err => HypervisorResponse::nt_error(err as _),
    }
}
