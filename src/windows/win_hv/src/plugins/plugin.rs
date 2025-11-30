use alloc::boxed::Box;
use alloc::collections::VecDeque;
use crate::win::alloc::PoolAllocSized;
use crate::win::{InitializeObjectAttributes, Utf8ToUnicodeString};
use crate::{PLUGINS, as_pvoid, get_data};
use alloc::format;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicPtr, Ordering};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use uuid::Uuid;
use wdk::println;
use wdk_sys::_KEY_VALUE_INFORMATION_CLASS::KeyValueFullInformation;
use wdk_sys::ntddk::{IoGetCurrentProcess, PsGetProcessId, ZwClose, ZwOpenKey, ZwQueryValueKey};
use wdk_sys::{_KPROCESS, PEPROCESS, PVOID};
use wdk_sys::{
    HANDLE, KEY_ALL_ACCESS, KEY_VALUE_FULL_INFORMATION, OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE,
    OBJECT_ATTRIBUTES, STATUS_SUCCESS,
};
use crate::plugins::async_command::AsyncCommand;

#[derive(Default)]
pub(crate) struct Plugin {
    pub uuid: Uuid,
    pub permissions: PluginPermissions,
    pub authorized_permissions: PluginPermissions,
    pub process: AtomicPtr<_KPROCESS>,
    pub open_processes: Vec<AtomicPtr<_KPROCESS>>,
    pub awaiting_commands: VecDeque<Box<dyn AsyncCommand>>
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
            let eprocess = p.load(Ordering::Relaxed) as PEPROCESS;
            if let Some(id) = id {
                if unsafe { PsGetProcessId(eprocess) as u32 == id } {
                    return true;
                }
            }
            if let Some(addr) = addr {
                if eprocess.addr() == addr as u64 as usize {
                    return true;
                }
            }

            false
        });

        match ptr {
            None => None,
            Some(x) => Some(x.load(Ordering::Relaxed)),
        }
    }

    ///
    /// # Get
    ///
    /// Gets the plugin from PLUGINS global variable.
    ///
    /// ## Arguments
    /// * `uuid` - [`Uuid`] the plugin was saved to system with.
    ///
    /// ## Return
    /// * [`None`] - Plugin not found.
    /// * [`Some`] - Plugin.
    pub fn lookup(uuid: Uuid) -> Option<&'static mut Self> {
        let ptr = PLUGINS.load(Ordering::Acquire);
        if ptr.is_null() {
            return None;
        }
        let slice = unsafe { &mut *ptr };

        match slice.plugins.iter_mut().find(|p| p.uuid == uuid) {
            Some(p) => Some(*p),
            None => None,
        }
    }

    ///
    /// # Current
    ///
    /// Gets the current plugin from current process context
    ///
    /// ## Return
    /// * [`None`] - No plugin associated with current process context.
    /// * [`Some`] - Plugin.
    pub fn current() -> Option<&'static mut Self> {
        let ptr = PLUGINS.load(Ordering::Acquire);
        if ptr.is_null() {
            return None;
        }

        let slice = unsafe { &mut *ptr };

        match slice
            .plugins
            .iter_mut()
            .find(|p| p.process.load(Ordering::Relaxed) == unsafe { IoGetCurrentProcess() })
        {
            Some(p) => Some(*p),
            None => None,
        }
    }

    ///
    /// # Permission Check
    ///
    /// Quick permission check for [self.authorized_permissions]
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
        self.process.store(process, Ordering::Relaxed);
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
            process: AtomicPtr::default(),
            open_processes: Vec::new(),
            awaiting_commands: VecDeque::with_capacity(32),
        })
    }
}
