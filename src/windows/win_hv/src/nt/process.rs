use crate::nt::context::ApcProcessContext;
use crate::nt::lock::pushlock::PushLock;
use crate::nt::object::NtObject;
use crate::nt::{EProcessField, EThreadField, get_eprocess_field, get_ethread_field};
use crate::objects::ObjectTracker;
use crate::objects::async_obj::AsyncState;
use crate::utils::danger::DangerPtr;
use crate::utils::handlebox::HandleBox;
use crate::win::unicode_string::UnicodeString;
use crate::win::{
    IoGetCurrentProcess, LIST_ENTRY, NtStatus, PACCESS_TOKEN, PEPROCESS, PETHREAD, PHANDLE_TABLE,
    PsGetProcessId, PsGetThreadId, PsLookupProcessByProcessId, PsTerminateProcess, UNICODE_STRING,
};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};
use hxposed_core::hxposed::requests::memory::Pa;
use hxposed_core::services::types::process_fields::{
    MitigationOptions, ProcessProtection, ProcessSignatureLevels,
};
///
/// # Kernel Process
///
/// Abstraction over [`PEPROCESS`] to make the life easier.
#[allow(dead_code)]
#[derive(Debug)]
pub struct NtProcess {
    pub nt_process: PEPROCESS,
    pub lock: PushLock,
    pub thread_list_head: *mut LIST_ENTRY,
    pub id: u32,
    pub owns: bool,
}
impl Hash for NtProcess {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.nt_process as _);
    }
}

impl PartialEq<Self> for NtProcess {
    fn eq(&self, other: &Self) -> bool {
        self.nt_process == other.nt_process
    }
}

impl Eq for NtProcess {}

impl Clone for NtProcess {
    fn clone(&self) -> Self {
        Self::from_ptr_owning(self.nt_process)
    }
}

unsafe impl Send for NtProcess {}
unsafe impl Sync for NtProcess {}

impl Drop for NtProcess {
    fn drop(&mut self) {
        if self.owns {
            unsafe {
                NtObject::<u64>::decrement_ref_count(self.nt_process as _);
            }
        }
    }
}

impl NtProcess {
    pub fn from_id(id: u32) -> Option<NtProcess> {
        let mut process = PEPROCESS::default();
        let status = unsafe { PsLookupProcessByProcessId(id as _, &mut process) };

        if status != NtStatus::Success {
            return None;
        }

        Some(Self::open_process(process, true))
    }

    pub fn current() -> NtProcess {
        Self::open_process(unsafe { IoGetCurrentProcess() }, false)
    }

    pub fn from_ptr(process: PEPROCESS) -> Self {
        Self::open_process(process, false)
    }

    pub fn from_ptr_owning(process: PEPROCESS) -> Self {
        let me = Self::open_process(process, true);
        unsafe { NtObject::<()>::increment_ref_count(me.nt_process as _) };
        me
    }

    fn open_process(ptr: PEPROCESS, owns: bool) -> Self {
        Self {
            nt_process: ptr,
            id: unsafe { PsGetProcessId(ptr) } as _,
            lock: unsafe {
                PushLock::from_ptr(get_eprocess_field::<u64>(EProcessField::Lock, ptr))
            },
            thread_list_head: unsafe {
                get_eprocess_field::<LIST_ENTRY>(EProcessField::ThreadListHead, ptr)
            },
            owns,
        }
    }

    pub fn free_hx_info(&self) {
        let state_ptr =
            unsafe { *get_eprocess_field::<*mut AsyncState>(EProcessField::Pad, self.nt_process) };

        if !state_ptr.is_null() {
            let state = unsafe { Box::from_raw(state_ptr) };
            drop(state);

            unsafe {
                (state_ptr as *mut u64).write(0);
            }
        }

        let tracker_ptr = unsafe {
            *get_eprocess_field::<*mut ObjectTracker>(EProcessField::Pad, self.nt_process)
                .byte_offset(8)
        };

        if !tracker_ptr.is_null() {
            let tracker = unsafe { Box::from_raw(tracker_ptr) };
            drop(tracker);

            unsafe {
                (tracker_ptr as *mut u64).write(0);
            }
        }
    }

    pub fn is_hx_info_present(&self) -> bool {
        unsafe { *get_eprocess_field::<*mut AsyncState>(EProcessField::Pad, self.nt_process) }
            .is_null()
            == false
    }

    pub fn setup_hx_info(&self) -> Result<(), NtStatus> {
        if self.is_hx_info_present() {
            return Err(NtStatus::Unsuccessful);
        }
        let state = match AsyncState::alloc_new(NtProcess::from_ptr_owning(self.nt_process)) {
            Ok(x) => x,
            Err(err) => {
                log::error!("Failed to allocate state for process.");
                return Err(err);
            }
        };

        let state_ptr =
            unsafe { get_eprocess_field::<*mut AsyncState>(EProcessField::Pad, self.nt_process) };

        let tracker_ptr = unsafe {
            get_eprocess_field::<*mut ObjectTracker>(EProcessField::Pad, self.nt_process)
                .byte_offset(8)
        };

        let state = Box::into_raw(state);

        unsafe { (state_ptr as *mut u64).write(state.addr() as _) };
        unsafe { (tracker_ptr as *mut u64).write(ObjectTracker::alloc_new() as _) };

        Ok(())
    }

    pub fn get_directory_table_base(&self) -> Pa {
        let dtb = unsafe {
            *get_eprocess_field::<u64>(EProcessField::DirectoryTableBase, self.nt_process)
        };
        Pa::from(dtb)
    }

    pub fn get_user_directory_table_base(&self) -> Pa {
        let dtb = unsafe {
            *get_eprocess_field::<u64>(EProcessField::DirectoryTableBase, self.nt_process)
        };
        Pa::from(dtb)
    }

    pub fn get_hx_async_state(&self) -> Option<&mut AsyncState> {
        if !self.is_hx_info_present() {
            return None;
        }

        let state_ptr =
            unsafe { get_eprocess_field::<*mut AsyncState>(EProcessField::Pad, self.nt_process) };

        Some(unsafe { &mut **state_ptr })
    }

    pub fn get_object_tracker_unchecked(&self) -> &mut ObjectTracker {
        Self::get_object_tracker(self).unwrap()
    }

    pub fn get_hx_async_state_unchecked(&self) -> &mut AsyncState {
        Self::get_hx_async_state(self).unwrap()
    }

    pub fn get_object_tracker(&self) -> Option<&mut ObjectTracker> {
        if !self.is_hx_info_present() {
            return None;
        }

        let tracker_ptr = unsafe {
            get_eprocess_field::<*mut ObjectTracker>(EProcessField::Pad, self.nt_process)
                .byte_offset(8)
        };

        Some(unsafe { &mut **tracker_ptr })
    }

    pub fn get_nt_path(&self) -> UnicodeString {
        let nt_path = unsafe {
            &mut **get_eprocess_field::<*mut UNICODE_STRING>(
                EProcessField::SeAuditProcessCreationInfo,
                self.nt_process,
            )
        };

        UnicodeString::from_unicode_string(nt_path)
    }

    // not ImagePathHash.
    // TODO: Cache this
    pub fn get_path_hash(&self) -> u64 {
        let path = self.get_nt_path();
        wyhash::wyhash(
            unsafe { core::slice::from_raw_parts(path.as_ptr() as *const u8, path.len() * 2) },
            0x2009,
        )
    }

    pub fn open_handle(&self) -> Result<HandleBox, ()> {
        match NtObject::create_handle(self.nt_process as _, self.get_handle_table()) {
            Ok(x) => Ok(HandleBox::new(x)),
            Err(_) => Err(()),
        }
    }

    pub fn begin_context(&self) -> ApcProcessContext {
        ApcProcessContext::begin(self.nt_process as _)
    }

    pub fn get_handle_table(&self) -> PHANDLE_TABLE {
        unsafe { *get_eprocess_field::<PHANDLE_TABLE>(EProcessField::ObjectTable, self.nt_process) }
    }

    pub fn get_protection(&self) -> ProcessProtection {
        unsafe {
            *get_eprocess_field::<ProcessProtection>(EProcessField::Protection, self.nt_process)
        }
    }

    pub fn get_signers(&self) -> ProcessSignatureLevels {
        unsafe {
            *get_eprocess_field::<ProcessSignatureLevels>(
                EProcessField::SignatureLevels,
                self.nt_process,
            )
        }
    }

    pub fn get_mitigations(&self) -> MitigationOptions {
        unsafe {
            *get_eprocess_field::<MitigationOptions>(
                EProcessField::MitigationFlags1,
                self.nt_process,
            )
        }
    }

    pub fn get_token(&self) -> PACCESS_TOKEN {
        unsafe { *get_eprocess_field::<u64>(EProcessField::Token, self.nt_process) as _ }
    }

    pub fn set_protection(&mut self, protection: ProcessProtection) {
        let ptr = unsafe {
            get_eprocess_field::<ProcessProtection>(EProcessField::Protection, self.nt_process)
        };

        unsafe { ptr.write(protection) }
    }

    pub fn set_signers(&mut self, signers: ProcessSignatureLevels) {
        let ptr = unsafe {
            get_eprocess_field::<ProcessSignatureLevels>(
                EProcessField::SignatureLevels,
                self.nt_process,
            )
        };

        unsafe { ptr.write(signers) }
    }

    pub fn set_mitigations(&mut self, mitigations: MitigationOptions) {
        let ptr = unsafe {
            get_eprocess_field::<MitigationOptions>(
                EProcessField::MitigationFlags1,
                self.nt_process,
            )
        };

        unsafe { ptr.write(mitigations) }
    }

    pub fn set_token(&mut self, token: PACCESS_TOKEN) {
        let ptr = unsafe { get_eprocess_field::<u64>(EProcessField::Token, self.nt_process) };

        unsafe {
            NtObject::<u64>::decrement_ref_count(*ptr as _);
            NtObject::<u64>::increment_ref_count(token);
        }

        unsafe { ptr.write(token as _) }
    }

    pub fn get_threads(&self) -> Vec<u32> {
        // actually, we have to lock ThreadListLock.

        self.lock.acquire_shared();

        let threads = DangerPtr {
            ptr: self.thread_list_head,
        };

        let first_entry = DangerPtr::<LIST_ENTRY> { ptr: threads.Blink };
        let mut current_entry = DangerPtr::<LIST_ENTRY> { ptr: threads.ptr };

        let mut thread_numbers = Vec::<u32>::new();

        while current_entry != first_entry {
            current_entry = DangerPtr::<LIST_ENTRY> {
                ptr: current_entry.Flink,
            };

            // now it gets tricky. let me explain.

            // the ThreadListHead field of _EPROCESS holds a LIST_ENTRY. what makes it deserve its own comments in this source is
            // that the list header for items are not the first field of the item.
            // for example, in 25h2, _ETHREAD's ThreadListEntry structure resides on offset 0x578.
            // so we have to go back exactly that many bytes to get the head of _ETHREAD object.

            // gets the real ETHREAD
            let thread = unsafe {
                get_ethread_field::<u64>(EThreadField::OffsetFromListEntry, current_entry.ptr as _)
                    as PETHREAD
            };

            thread_numbers.push(unsafe { PsGetThreadId(thread) as _ });
        }

        thread_numbers
    }

    pub fn kill(self, exit_code: NtStatus) -> Result<(), NtStatus> {
        match unsafe { PsTerminateProcess(self.nt_process, exit_code) } {
            NtStatus::Success => Ok(()),
            err => Err(err),
        }
    }
}
