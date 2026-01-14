use crate::nt::mdl::MemoryDescriptor;
use crate::nt::process::NtProcess;
use crate::objects::ObjectTracker;
use crate::services::commands::memory::*;
use crate::utils::pop_guard::PopGuard;
use crate::win::{MmCopyVirtualMemory, ZwProtectVirtualMemory};
use alloc::boxed::Box;
use core::ops::{BitAnd, Deref};
use hv::hypervisor::host::Guest;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::{NotAllowedReason, NotFoundReason};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::memory::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::services::types::memory_fields::MemoryProtection;
use wdk_sys::_MODE::{KernelMode, UserMode};
use wdk_sys::{SIZE_T, STATUS_INVALID_PAGE_PROTECTION, STATUS_SUCCESS, ULONG};

pub(crate) fn free_mdl_sync(request: &FreeMemoryAsyncCommand) -> HypervisorResponse {
    match ObjectTracker::get_allocated_mdl(request.command.mdl as _) {
        Some(mdl) => {
            mdl.take();
        }
        None => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
    };

    EmptyResponse::with_service(ServiceFunction::FreeMemory)
}

pub(crate) fn free_mdl_async(
    request: FreeMemoryRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    ObjectTracker::queue_command(Box::new(FreeMemoryAsyncCommand {
        command: request,
        async_info,
    }));
    EmptyResponse::with_service(ServiceFunction::MapMemory)
}

pub(crate) fn map_mdl_sync(request: &MapMemoryAsyncCommand) -> HypervisorResponse {
    let mut mdl = match ObjectTracker::get_allocated_mdl(request.command.mdl as _) {
        Some(mdl) => mdl,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
    };

    let process = match ObjectTracker::get_open_process(request.command.process as _) {
        None => PopGuard::no_src(NtProcess::from_ptr(request.async_info.process as _)),
        Some(x) => x,
    };

    let _ctx = process.begin_context();
    let map_address = if request.command.map_address == 0 {
        None
    } else {
        Some(request.command.map_address as _)
    };

    let result = match request.command.operation {
        MapMemoryOperation::Map => {
            let ptr = match mdl.map(map_address, UserMode as _) {
                Ok(ptr) => ptr,
                Err(err) => {
                    return HypervisorResponse::nt_error(err as _);
                }
            };

            MapMemoryResponse {
                mapped_address: ptr as _,
            }
            .into_raw()
        }
        MapMemoryOperation::Unmap => match mdl.unmap() {
            Ok(_) => EmptyResponse::with_service(ServiceFunction::MapMemory),
            Err(_) => HypervisorResponse::invalid_params(ServiceParameter::BufferByUser),
        },
    };

    // paranoid
    drop(_ctx);

    result
}

pub(crate) fn map_mdl_async(
    request: MapMemoryRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    ObjectTracker::queue_command(Box::new(MapMemoryAsyncCommand {
        command: request,
        async_info,
    }));
    EmptyResponse::with_service(ServiceFunction::MapMemory)
}

pub(crate) fn allocate_mdl_sync(request: &AllocateMemoryAsyncCommand) -> HypervisorResponse {
    let mdl = match request.command.underlying_pages {
        0 => MemoryDescriptor::new(request.command.size as _),
        addr => MemoryDescriptor::new_describe(addr as _, request.command.size as _),
    };

    let ret = AllocateMemoryResponse {
        mdl: mdl.mdl.ptr as _,
        bytes_allocated: mdl.mdl.ByteCount,
    }
    .into_raw();

    ObjectTracker::add_mdl(mdl);
    ret
}

pub(crate) fn allocate_mdl_async(
    request: AllocateMemoryRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    ObjectTracker::queue_command(Box::new(AllocateMemoryAsyncCommand {
        command: request,
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
    mut request: ProtectProcessMemoryRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
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

    ObjectTracker::queue_command(Box::new(ProtectProcessMemoryAsyncCommand {
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
    let process = match ObjectTracker::get_open_process(request.command.process as _) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };
    let mut base = request.command.address;
    let mut bytes_processed = 4096u64;
    let mut protection = ULONG::default();

    let handle = match process.open_handle() {
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
    request: RWProcessMemoryRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    ObjectTracker::queue_command(Box::new(RWProcessMemoryAsyncCommand {
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

    let process = match ObjectTracker::get_open_process(request.command.process as _) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };

    let mut return_size = SIZE_T::default();

    let (source, target) = match request.command.operation {
        ProcessMemoryOperation::Read => (
            process.deref(),
            &NtProcess::from_ptr(request.async_info.process as _),
        ),
        ProcessMemoryOperation::Write => (
            &NtProcess::from_ptr(request.async_info.process as _),
            process.deref(),
        ),
    };

    match unsafe {
        MmCopyVirtualMemory(
            source.nt_process,
            request.command.address as _,
            target.nt_process,
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
