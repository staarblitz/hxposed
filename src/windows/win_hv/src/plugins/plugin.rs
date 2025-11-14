use crate::win::alloc::PoolAllocSized;
use crate::win::{InitializeObjectAttributes, Utf8ToUnicodeString};
use crate::{as_pvoid, get_data, PLUGINS};
use alloc::format;
use core::sync::atomic::Ordering;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use uuid::Uuid;
use wdk::println;
use wdk_sys::_KEY_VALUE_INFORMATION_CLASS::KeyValueFullInformation;
use wdk_sys::PVOID;
use wdk_sys::ntddk::{ZwClose, ZwOpenKey, ZwQueryValueKey};
use wdk_sys::{
    HANDLE, KEY_ALL_ACCESS, KEY_VALUE_FULL_INFORMATION, OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE,
    OBJECT_ATTRIBUTES, STATUS_SUCCESS,
};

#[derive(Debug, Default)]
pub(crate) struct Plugin {
    pub uuid: Uuid,
    pub permissions: PluginPermissions,
    pub process: u64
}
impl Plugin {
    pub fn get(uuid: Uuid) -> Option<&'static Plugin>{
        let ptr = PLUGINS.load(Ordering::Acquire);
        if ptr.is_null() { return None; }
        let slice = unsafe { &*ptr };

        // :skull:
        Some(*slice.plugins.iter().find(|p| p.uuid == uuid).unwrap())
    }

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
            process: 0,
        })
    }
}
