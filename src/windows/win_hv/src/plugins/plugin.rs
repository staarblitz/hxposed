use crate::nt::process::NtProcess;
use crate::plugins::commands::AsyncCommand;
use crate::utils::alloc::PoolAllocSized;
use crate::{as_pvoid, get_data};
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::ptr::slice_from_raw_parts_mut;
use core::sync::atomic::Ordering;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use uuid::Uuid;
use wdk_sys::ntddk::{
    IoAllocateMdl, RtlCompareUnicodeString,
    ZwClose, ZwOpenKey, ZwQueryValueKey,
};
use wdk_sys::_KEY_VALUE_INFORMATION_CLASS::KeyValueFullInformation;
use wdk_sys::{FALSE, HANDLE, KEY_ALL_ACCESS, KEY_VALUE_FULL_INFORMATION, OBJECT_ATTRIBUTES, OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE, PETHREAD, PIRP, PMDL, STATUS_SUCCESS};
use wdk_sys::{MDL, PACCESS_TOKEN, PEPROCESS, PVOID, UNICODE_STRING};
use crate::utils::danger::DangerPtr;
use crate::win::utf_to_unicode::Utf8ToUnicodeString;
use crate::win::utils::InitializeObjectAttributes;

#[derive(Default)]
pub(crate) struct Plugin {
    pub uuid: Uuid,
    pub permissions: PluginPermissions,
    #[allow(dead_code)]
    pub authorized_permissions: PluginPermissions,
    pub plugin_path: Box<UNICODE_STRING>,
    pub process: PEPROCESS,
    pub object_table: PluginObjectTable,
    pub awaiting_commands: VecDeque<Box<dyn AsyncCommand>>,
}

#[derive(Default)]
pub(crate) struct PluginObjectTable {
    pub open_processes: Vec<PEPROCESS>,
    pub open_threads: Vec<PETHREAD>,
    pub open_tokens: Vec<PACCESS_TOKEN>,
    pub allocated_mdls: Vec<DangerPtr<MDL>>,
}

impl PluginObjectTable {

    ///
    /// # Allocate MDL
    ///
    /// Allocates an MDL object on behalf of the plugin.
    ///
    /// ## Arguments
    /// * `ptr` - Virtual address MDL describes.
    /// * `length` - Length of the virtual address to describe.
    ///
    /// ## Returns
    /// * [`MDL`] - Object
    pub fn allocate_mdl(&mut self, ptr: PVOID, length: u32) -> &DangerPtr<MDL> {
        // uses ExAllocatePool. so it's safe for us to wrap it in our Box.
        let mdl = unsafe {
            IoAllocateMdl(
                ptr,
                length,
                FALSE as _,
                FALSE as _,
                PIRP::default(),
            )
        };
        self.allocated_mdls.push(DangerPtr {
            ptr: mdl
        });

        self.allocated_mdls.last().unwrap()
    }

    pub fn add_open_process(&mut self, process: PEPROCESS) {
        self.open_processes.push(process);
    }

    pub fn add_open_thread(&mut self, process: PETHREAD) {
        self.open_threads.push(process);
    }

    pub fn add_open_token(&mut self, process: PACCESS_TOKEN) {
        self.open_tokens.push(process);
    }

    ///
    /// # Pop Allocated Mdl
    ///
    /// Gets an MDL opened by the plugin. Then "closes" it.
    ///
    /// ## Arguments
    /// * `mapped_system_va` - [`MappedSystemVa`] field of the MDL object.
    ///
    /// ## Returns
    /// * [`Some`] - MDL object.
    /// * [`None`] - MDL was not found.
    pub fn pop_allocated_mdl(&mut self, mapped_system_va: u64) -> Option<DangerPtr<MDL>> {
        if let Some(pos) = self
            .allocated_mdls
            .iter()
            .position(|m| m.MappedSystemVa as u64 == mapped_system_va)
        {
            Some(self.allocated_mdls.remove(pos))
        } else {
            None
        }
    }

    ///
    /// # Pop Open Thread
    ///
    /// Gets a thread opened by the plugin. Then "closes" it.
    ///
    /// ## Arguments
    /// * `addr` - Address of the thread object in NT kernel.
    ///
    /// ## Returns
    /// * [`Some`] - Pointer to thread object.
    /// * [`None`] - Thread was not found.
    pub fn pop_open_thread(&mut self, addr: PETHREAD) -> Option<PETHREAD> {
        if let Some(pos) = self
            .open_threads
            .iter()
            .position(|m| (m.addr() as u64) == (addr as u64))
        {
            Some(self.open_threads.remove(pos))
        } else {
            None
        }
    }

    ///
    /// # Pop Open Process
    ///
    /// Gets a process opened by the plugin. Then "closes" it.
    ///
    /// ## Arguments
    /// * `addr` - Address of the process object in NT kernel.
    ///
    /// ## Returns
    /// * [`Some`] - Pointer to process object.
    /// * [`None`] - Process was not found.
    pub fn pop_open_process(&mut self, addr: PEPROCESS) -> Option<PEPROCESS> {
        if let Some(pos) = self
            .open_processes
            .iter()
            .position(|m| (m.addr() as u64) == (addr as u64))
        {
            Some(self.open_processes.remove(pos))
        } else {
            None
        }
    }

    ///
    /// # Pop Open Token
    ///
    /// Gets a token opened by the plugin. Then "closes" it.
    ///
    /// ## Arguments
    /// * `addr` - Address of the token object in NT kernel.
    ///
    /// ## Returns
    /// * [`Some`] - Pointer to token object.
    /// * [`None`] - Token was not found.
    pub fn pop_open_token(&mut self, addr: PACCESS_TOKEN) -> Option<PACCESS_TOKEN> {
        if let Some(pos) = self
            .open_tokens
            .iter()
            .position(|m| (m.addr() as u64) == (addr as u64))
        {
            Some(self.open_tokens.remove(pos))
        } else {
            None
        }
    }

    ///
    /// # Get Open Token
    ///
    /// Gets a token opened by the plugin.
    ///
    /// ## Arguments
    /// * `addr` - Address of the token object in NT kernel.
    ///
    /// ## Returns
    /// * [`Some`] - Pointer to token object.
    /// * [`None`] - Token was not found.
    pub fn get_open_token(&self, addr: PACCESS_TOKEN) -> Option<PACCESS_TOKEN> {
        let ptr = self.open_tokens.iter().find(|p| {
            if (**p).addr() == addr as u64 as usize {
                return true;
            }

            false
        });

        match ptr {
            None => None,
            Some(p) => Some(*p),
        }
    }

    ///
    /// # Get Open Thread
    ///
    /// Gets a thread opened by the plugin.
    ///
    /// ## Arguments
    /// * `addr` - Address of the thread object in NT kernel.
    ///
    /// ## Returns
    /// * [`Some`] - Pointer to thread object.
    /// * [`None`] - Thread was not found.
    pub fn get_open_thread(&self, addr: PETHREAD) -> Option<PETHREAD> {
        let ptr = self.open_threads.iter().find(|p| {
            if (**p).addr() == addr as u64 as usize {
                return true;
            }

            false
        });

        match ptr {
            None => None,
            Some(x) => Some(*x),
        }
    }

    ///
    /// # Get Open Process
    ///
    /// Gets a process open by the plugin.
    ///
    /// ## Arguments
    /// * `id` - If [`Some`], the id is compared for result.
    /// * `addr` - If [Some], the addr is compared for result.
    ///
    /// ## Warning
    /// - Arguments are *not* compared together.
    ///
    /// ## Returns
    /// * [`Some`] - Pointer to process object.
    /// * [`None`] - Process was not found.
    pub fn get_open_process(&self, addr: PEPROCESS) -> Option<PEPROCESS> {
        let ptr = self.open_processes.iter().find(|p| {
            if (**p).addr() == addr as u64 as usize {
                return true;
            }

            false
        });

        match ptr {
            None => None,
            Some(x) => Some(*x),
        }
    }

    ///
    /// # Get Allocated MDL
    ///
    /// Gets an MDL opened by the plugin.
    ///
    /// ## Arguments
    /// * `mapped_system_va` - [`MappedSystemVa`] field of the MDL object.
    ///
    /// ## Returns
    /// * [`Some`] - MDL object.
    /// * [`None`] - MDL was not found.
    pub fn get_allocated_mdl(&self, mapped_system_va: u64) -> Option<&DangerPtr<MDL>> {
        self.allocated_mdls
            .iter()
            .find(|m| m.MappedSystemVa as u64 == mapped_system_va)
    }
}

impl Plugin {
    ///
    /// # Queue Command
    ///
    /// Queues a command for later execution by the worker thread on PASSIVE_LEVEL.
    ///
    /// ## Arguments
    /// * `command` - Well... See [`AsyncCommand`]
    pub fn queue_command(&mut self, command: Box<dyn AsyncCommand>) {
        self.awaiting_commands.push_back(command);
    }

    ///
    /// # Dequeue Command
    ///
    /// Pops a command from queue for execution by the worker thread.
    ///
    /// ## Returns
    /// - Whatever [`VecDeque::pop_back`] returns.
    pub fn dequeue_command(&mut self) -> Option<Box<dyn AsyncCommand>> {
        self.awaiting_commands.pop_front()
    }

    ///
    /// # Permission Check
    ///
    /// Quick permission check for [self.authorized_permissions]
    #[cfg(debug_assertions)]
    #[allow(unused_variables)]
    pub fn perm_check(&self, permissions: PluginPermissions) -> bool {
        true
    }

    ///
    /// # Permission Check
    ///
    /// Quick permission check for [self.authorized_permissions]
    #[cfg(not(debug_assertions))]
    pub fn perm_check(&self, permissions: PluginPermissions) -> bool {
        self.authorized_permissions.contains(permissions)
    }

    ///
    /// # Integrate
    ///
    /// Integrates a plugin with process, and permissions that are allowed.
    ///
    /// ## Arguments
    /// * `process` - Pointer to NT executive process object.
    /// * `permissions` - Permission mask that plugin will utilize.
    ///
    /// ## Return
    /// * [`None`] - Plugin does not meet specifications.
    /// * [`Some`] - Ok.
    pub fn integrate(&mut self, process: PEPROCESS, permissions: PluginPermissions) -> Option<()> {
        let kprocess = NtProcess::from_ptr(process);

        match unsafe {
            RtlCompareUnicodeString(
                kprocess.nt_path.load(Ordering::Relaxed),
                self.plugin_path.as_mut(),
                FALSE as _,
            )
        } {
            0 => {
                self.process = process;
                self.permissions = permissions;
                Some(())
            }
            _ => None,
        }
    }

    ///
    /// # Open
    ///
    /// Opens (creates instance that represents) a plugin from registry.
    ///
    /// ## Arguments
    /// * `uuid` - [`Uuid`] the plugin was saved to system with.
    ///
    /// ## Return
    /// * [`None`] - Plugin not found.
    /// * [`Some`] - Plugin.
    pub fn open(uuid: Uuid) -> Option<Self> {
        let mut full_path = format!("\\Registry\\Machine\\Software\\HxPosed\\Plugins\\{}", uuid)
            .as_str()
            .to_unicode_string();

        let mut key_handle = HANDLE::default();
        let mut attributes = OBJECT_ATTRIBUTES::default();
        unsafe {
            InitializeObjectAttributes(
                &mut attributes,
                full_path.as_mut(),
                OBJ_KERNEL_HANDLE | OBJ_CASE_INSENSITIVE,
                Default::default(),
                Default::default(),
            )
        };

        match unsafe { ZwOpenKey(&mut key_handle, KEY_ALL_ACCESS, &mut attributes) } {
            STATUS_SUCCESS => {},
            err => {
                log::error!("Error opening key: {}", err);
                return None;
            }
        }

        let mut permissions = "Permissions".to_unicode_string();
        let mut path = "Path".to_unicode_string();
        let mut return_length = 0; // dummy
        let mut info = KEY_VALUE_FULL_INFORMATION::alloc_sized(64);

        let permissions = match unsafe {
            ZwQueryValueKey(
                key_handle,
                permissions.as_mut(),
                KeyValueFullInformation,
                as_pvoid!(info),
                64,
                &mut return_length,
            )
        } {
            STATUS_SUCCESS => unsafe { *get_data!(info, PluginPermissions) },
            err => {
                log::error!("Error querying key: {}", err);
                return None;
            }
        };

        let mut info = KEY_VALUE_FULL_INFORMATION::alloc_sized(64 + 500); // 500 for path

        let path = match unsafe {
            ZwQueryValueKey(
                key_handle,
                path.as_mut(),
                KeyValueFullInformation,
                as_pvoid!(info),
                64 + 500,
                &mut return_length,
            )
        } {
            STATUS_SUCCESS => unsafe {
                &mut *slice_from_raw_parts_mut::<u16>(
                    (info.as_mut() as *mut KEY_VALUE_FULL_INFORMATION)
                        .byte_offset(info.DataOffset as _) as *mut _,
                    info.DataLength as _,
                )
            },
            err => {
                log::error!("Error querying key: {}", err);
                return None;
            }
        };
        let _ = unsafe { ZwClose(key_handle) };

        Some(Self {
            uuid,
            permissions,
            authorized_permissions: permissions,
            awaiting_commands: VecDeque::with_capacity(32),
            plugin_path: String::from_utf16_lossy(path).to_unicode_string(),
            ..Default::default()
        })
    }
}
