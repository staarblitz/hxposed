use crate::nt::context::ApcProcessContext;
use crate::plugins::commands::memory::*;
use crate::plugins::plugin::Plugin;
use crate::win::{MmCopyVirtualMemory, ZwProtectVirtualMemory};
use alloc::boxed::Box;
use core::ops::{BitAnd, DerefMut};
use core::sync::atomic::Ordering;
use hv::hypervisor::host::Guest;
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
use wdk_sys::ntddk::{IoFreeMdl, MmAllocateContiguousMemorySpecifyCache, MmBuildMdlForNonPagedPool, MmFreeContiguousMemory, MmMapLockedPagesSpecifyCache, MmUnmapLockedPages, ObOpenObjectByPointer, ZwClose};
use wdk_sys::{
    FALSE, HANDLE, OBJ_KERNEL_HANDLE, PHYSICAL_ADDRESS, PROCESS_ALL_ACCESS, PsProcessType, SIZE_T,
    STATUS_INVALID_PAGE_PROTECTION, STATUS_MEMORY_NOT_ALLOCATED, STATUS_SUCCESS, ULONG,
};

pub(crate) fn free_mdl_sync(request: &FreeMemoryAsyncCommand) -> HypervisorResponse {
    let plugin = match Plugin::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let process = plugin.process.load(Ordering::Relaxed);

    let mut mdl = match plugin.pop_allocated_mdl(request.command.mdl_address) {
        Some(mdl) => mdl,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
    };

    let _ctx = ApcProcessContext::begin(process);

    unsafe{
        MmFreeContiguousMemory(mdl.MappedSystemVa);
        IoFreeMdl(mdl.deref_mut());
    }

    drop(_ctx);

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
    let plugin = match Plugin::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let process = plugin.process.load(Ordering::Relaxed);

    let mdl = match plugin.get_allocated_mdl(request.command.mdl_address) {
        Some(mdl) => mdl,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
    };

    let _ctx = ApcProcessContext::begin(process);

    let result = match request.command.operation {
        MapMemoryOperation::Map => {
            let ptr = unsafe {
                match microseh::try_seh(|| {
                    MmMapLockedPagesSpecifyCache(
                        mdl.deref_mut(),
                        UserMode as _,
                        MmCached,
                        request.command.map_address as _,
                        FALSE,
                        HighPagePriority as _,
                    )
                }) {
                    Ok(ptr) => ptr,
                    Err(e) => return HypervisorResponse::nt_error(e.code() as _),
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
            unsafe { MmUnmapLockedPages(request.command.map_address as _, mdl.deref_mut()) }
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
    let plugin = match Plugin::lookup(request.uuid) {
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

    let mdl = plugin.allocate_mdl(alloc, request.command.size);

    unsafe { MmBuildMdlForNonPagedPool(mdl.deref_mut()) }

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
            | PluginPermissions::MEMORY_PROTECT,
    ) {
        return HypervisorResponse::not_allowed_perms(
            PluginPermissions::MEMORY_PHYSICAL
                | PluginPermissions::MEMORY_VIRTUAL
                | PluginPermissions::MEMORY_PROTECT,
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
/// # Process Virtual Memory
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

    if request.protection.bits() != MemoryProtection::NO_CACHE.bits() && // there must be a better way to do it tbh
        request.protection.bits() != MemoryProtection::READONLY.bits() &&
        request.protection.bits() != MemoryProtection::READWRITE.bits() &&
        request.protection.bits() != MemoryProtection::WRITECOPY.bits() &&
        request.protection.bits() != MemoryProtection::EXECUTE.bits() &&
        request.protection.bits() != MemoryProtection::EXECUTE_READ.bits() &&
        request.protection.bits() != MemoryProtection::EXECUTE_READWRITE.bits() &&
        request.protection.bits() != MemoryProtection::EXECUTE_WRITECOPY.bits() &&
        request.protection.bits() != MemoryProtection::READONLY.bits()
    {
        return HypervisorResponse::nt_error(STATUS_INVALID_PAGE_PROTECTION as _);
    }

    let process = match plugin.get_open_process(Some(request.id), None) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };

    plugin.queue_command(Box::new(ProtectProcessMemoryAsyncCommand {
        process,
        plugin_process: plugin.process.load(Ordering::Relaxed),
        command: request,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::ProtectProcessMemory)
}

///
/// # Protect Virtual Memory
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
    let mut base = request.command.address;
    let mut bytes_processed = 4096u64;
    let mut protection = ULONG::default();
    let mut handle = HANDLE::default();

    match unsafe {
        ObOpenObjectByPointer(
            request.process as _,
            OBJ_KERNEL_HANDLE,
            Default::default(),
            PROCESS_ALL_ACCESS,
            *PsProcessType,
            KernelMode as _,
            &mut handle,
        )
    } {
        STATUS_SUCCESS => {}
        err => return HypervisorResponse::nt_error(err as _),
    };

    let result = match unsafe {
        ZwProtectVirtualMemory(
            handle,
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

    let _ = unsafe { ZwClose(handle) };

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

    let process = match plugin.get_open_process(Some(request.id), None) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };

    plugin.queue_command(Box::new(RWProcessMemoryAsyncCommand {
        plugin_process: plugin.process.load(Ordering::Relaxed),
        process,
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

    let mut return_size = SIZE_T::default();

    // this is gross but also amazing.
    match match request.command.operation {
        ProcessMemoryOperation::Read => unsafe {
            MmCopyVirtualMemory(
                request.process,
                request.command.address as _,
                request.plugin_process,
                request.command.data as _,
                request.command.data_len as _,
                KernelMode as _,
                &mut return_size,
            )
        },
        ProcessMemoryOperation::Write => unsafe {
            MmCopyVirtualMemory(
                request.plugin_process,
                request.command.data as _,
                request.process,
                request.command.address as _,
                request.command.data_len as _,
                KernelMode as _,
                &mut return_size,
            )
        },
    } {
        STATUS_SUCCESS => RWProcessMemoryResponse {
            bytes_processed: return_size as _,
        }
        .into_raw(),
        err => HypervisorResponse::nt_error(err as _),
    }
}
