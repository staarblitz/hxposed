use crate::win::alloc::PoolAllocSized;
use crate::win::{InitializeObjectAttributes, Utf8ToUnicodeString};
use crate::{as_pvoid, get_data};
use alloc::format;
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

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Plugin {
    pub uuid: Uuid,
    pub permissions: PluginPermissions,
}
impl Plugin {
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

        let mut return_length = 0;
        let _ = unsafe {
            ZwQueryValueKey(
                key_handle,
                permissions.as_mut(),
                KeyValueFullInformation,
                Default::default(),
                0,
                &mut return_length,
            )
        };

        let mut info = KEY_VALUE_FULL_INFORMATION::alloc_sized(return_length as _);
        let status = unsafe {
            ZwQueryValueKey(
                key_handle,
                permissions.as_mut(),
                KeyValueFullInformation,
                as_pvoid!(info),
                return_length,
                &mut return_length,
            )
        };

        let permissions = unsafe { *get_data!(info, PluginPermissions) };

        if status != STATUS_SUCCESS {
            println!("Error querying key: {}", status);
            return None;
        }

        unsafe{ZwClose(key_handle)};

        Some(Self {
            uuid,
            permissions,
        })
    }
}
