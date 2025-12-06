use crate::plugins::commands::memory::{
    ProtectProcessMemoryAsyncCommand, RWProcessMemoryAsyncCommand,
};
use crate::plugins::plugin::Plugin;
use crate::win::{MmCopyVirtualMemory, ZwProtectVirtualMemory};
use alloc::boxed::Box;
use core::ops::BitAnd;
use core::sync::atomic::Ordering;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::error::{NotAllowedReason, NotFoundReason};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::{
    ProcessMemoryOperation, ProtectProcessMemoryRequest, RWProcessMemoryRequest,
};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::process::{
    ProtectProcessMemoryResponse, RWProcessMemoryResponse,
};
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use hxposed_core::services::types::memory_fields::MemoryProtection;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::ObOpenObjectByPointer;
use wdk_sys::{
    HANDLE, OBJ_KERNEL_HANDLE, PROCESS_ALL_ACCESS, PsProcessType, SIZE_T,
    STATUS_INVALID_PAGE_PROTECTION, STATUS_SUCCESS, ULONG,
};

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

    match unsafe {
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
    }
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
