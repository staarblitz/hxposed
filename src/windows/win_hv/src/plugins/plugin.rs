use crate::plugins::commands::AsyncCommand;
use crate::win::alloc::PoolAllocSized;
use crate::win::{InitializeObjectAttributes, Utf8ToUnicodeString};
use crate::{as_pvoid, get_data};
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::format;
use alloc::vec::Vec;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use uuid::Uuid;
use wdk::println;
use wdk_sys::_KEY_VALUE_INFORMATION_CLASS::KeyValueFullInformation;
use wdk_sys::ntddk::{IoAllocateMdl, PsGetProcessId, PsGetThreadId, ZwClose, ZwOpenKey, ZwQueryValueKey};
use wdk_sys::{_KPROCESS, MDL, PEPROCESS, PVOID, PACCESS_TOKEN};
use wdk_sys::{
    FALSE, HANDLE, KEY_ALL_ACCESS, KEY_VALUE_FULL_INFORMATION, OBJ_CASE_INSENSITIVE,
    OBJ_KERNEL_HANDLE, OBJECT_ATTRIBUTES, PETHREAD, PIRP, STATUS_SUCCESS,
};

#[derive(Default)]
pub(crate) struct Plugin {
    pub uuid: Uuid,
    pub permissions: PluginPermissions,
    pub authorized_permissions: PluginPermissions,
    pub process: PEPROCESS,
    pub object_table: PluginObjectTable,
    pub awaiting_commands: VecDeque<Box<dyn AsyncCommand>>,
}

#[derive(Default)]
pub(crate) struct PluginObjectTable {
    pub open_processes: Vec<PEPROCESS>,
    pub open_threads: Vec<PETHREAD>,
    pub open_tokens: Vec<PACCESS_TOKEN>,
    pub allocated_mdls: Vec<Box<MDL>>,
}

impl PluginObjectTable {
    pub fn allocate_mdl(&mut self, ptr: PVOID, length: u32) -> &mut Box<MDL> {
        // uses ExAllocatePool. so it's safe for us to wrap it in our Box.
        let mdl = unsafe {
            Box::from_raw(IoAllocateMdl(
                ptr,
                length,
                FALSE as _,
                FALSE as _,
                PIRP::default(),
            ))
        };
        self.allocated_mdls.push(mdl);

        self.allocated_mdls.last_mut().unwrap()
    }

    pub fn get_allocated_mdl(&mut self, mapped_system_va: u64) -> Option<&mut Box<MDL>> {
        self.allocated_mdls
            .iter_mut()
            .find(|m| m.MappedSystemVa as u64 == mapped_system_va)
    }

    pub fn pop_allocated_mdl(&mut self, mapped_system_va: u64) -> Option<Box<MDL>> {
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

    pub fn get_open_thread(
        &self,
        id: Option<u32>,
        addr: Option<PETHREAD>,
    ) -> Option<PETHREAD> {
        let ptr = self.open_threads.iter().find(|p| {
            if let Some(id) = id {
                if unsafe { PsGetThreadId(**p) as u32 == id } {
                    return true;
                }
            }
            if let Some(addr) = addr {
                if (**p).addr() == addr as u64 as usize {
                    return true;
                }
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
    /// Gets a process open in the [self.open_processes]
    ///
    /// ## Arguments
    /// * `id` - If [`Some`], the id is compared for result.
    /// * `addr` - If [Some], the addr is compared for result.
    ///
    /// ## Warning
    /// - Arguments are *not* compared together.
    ///
    /// ## Returns
    /// * [`Some`] - Pointer to [`_KPROCESS`].
    /// * [`None`] - Process was not found.
    pub fn get_open_process(
        &self,
        id: Option<u32>,
        addr: Option<PEPROCESS>,
    ) -> Option<*mut _KPROCESS> {
        let ptr = self.open_processes.iter().find(|p| {
            if let Some(id) = id {
                if unsafe { PsGetProcessId(**p) as u32 == id } {
                    return true;
                }
            }
            if let Some(addr) = addr {
                if (**p).addr() == addr as u64 as usize {
                    return true;
                }
            }

            false
        });

        match ptr {
            None => None,
            Some(x) => Some(*x),
        }
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
    pub fn integrate(&mut self, process: PEPROCESS, permissions: PluginPermissions) {
        self.process = process;
        self.permissions = permissions;
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

        let status = unsafe { ZwOpenKey(&mut key_handle, KEY_ALL_ACCESS, &mut attributes) };

        if status != STATUS_SUCCESS {
            println!("Error opening key: {}", status);
            return None;
        }

        let mut permissions = "Permissions".to_unicode_string();
        let mut return_length = 0; // dummy
        let mut info = KEY_VALUE_FULL_INFORMATION::alloc_sized(64);
        let status = unsafe {
            ZwQueryValueKey(
                key_handle,
                permissions.as_mut(),
                KeyValueFullInformation,
                as_pvoid!(info),
                64,
                &mut return_length,
            )
        };

        if status != STATUS_SUCCESS {
            println!("Error querying key: {}", status);
            return None;
        }

        let permissions = unsafe { *get_data!(info, PluginPermissions) };

        let _ = unsafe { ZwClose(key_handle) };

        Some(Self {
            uuid,
            permissions,
            authorized_permissions: permissions,
            awaiting_commands: VecDeque::with_capacity(32),

            ..Default::default()
        })
    }
}
