use crate::win::alloc::PoolAllocSized;
use crate::win::{InitializeObjectAttributes, Utf8ToUnicodeString};
use crate::{PLUGINS, as_pvoid, get_data};
use alloc::format;
use core::sync::atomic::{AtomicPtr, Ordering};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use uuid::Uuid;
use wdk::println;
use wdk_sys::_KEY_VALUE_INFORMATION_CLASS::KeyValueFullInformation;
use wdk_sys::ntddk::{ZwClose, ZwOpenKey, ZwQueryValueKey};
use wdk_sys::{_KPROCESS, PVOID, PEPROCESS};
use wdk_sys::{
    HANDLE, KEY_ALL_ACCESS, KEY_VALUE_FULL_INFORMATION, OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE,
    OBJECT_ATTRIBUTES, STATUS_SUCCESS,
};

#[derive(Debug, Default)]
pub(crate) struct Plugin {
    pub uuid: Uuid,
    pub permissions: PluginPermissions,
    pub authorized_permissions: PluginPermissions,
    pub process: AtomicPtr<_KPROCESS>,
}
impl Plugin {
    ///
    /// # Get
    ///
    /// Gets the plugin from PLUGINS global variable.
    ///
    /// ## Arguments
    /// uuid - GUID the plugin was saved to system with.
    ///
    /// ## Return
    /// Returns an [Option] containing static mutable reference to [Plugin].
    pub fn get(uuid: Uuid) -> Option<&'static mut Plugin> {
        let ptr = PLUGINS.load(Ordering::Acquire);
        if ptr.is_null() {
            return None;
        }
        let slice = unsafe { &mut *ptr };

        //:skull:
        Some(*slice.plugins.iter_mut().find(|p| p.uuid == uuid).unwrap())
    }

    ///
    /// # Integrate
    ///
    /// Integrates a plugin with process, and permissions that are allowed.
    ///
    /// ## Arguments
    /// process - Pointer to NT executive process object.
    ///
    /// permissions - Permission mask that plugin will utilize.
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
    /// uuid - GUID the plugin was saved to system with.
    ///
    /// ## Return
    /// Returns an [Option] containing [Plugin]. Some if plugin was found, None if not.
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
        })
    }
}
