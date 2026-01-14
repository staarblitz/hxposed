use crate::utils::alloc::PoolAllocSized;
use crate::utils::handlebox::HandleBox;
use crate::win::utf_to_unicode::Utf8ToUnicodeString;
use crate::win::utils::init_object_attributes;
use crate::{as_pvoid, get_data};
use alloc::boxed::Box;
use alloc::string::ToString;
use core::ptr::null_mut;
use wdk_sys::_KEY_INFORMATION_CLASS::KeyFullInformation;
use wdk_sys::_KEY_VALUE_INFORMATION_CLASS::KeyValueFullInformation;
use wdk_sys::ntddk::{ZwOpenKey, ZwQueryKey, ZwQueryValueKey};
use wdk_sys::{HANDLE, KEY_ALL_ACCESS, KEY_FULL_INFORMATION, KEY_VALUE_FULL_INFORMATION, NTSTATUS, OBJ_KERNEL_HANDLE, PVOID, STATUS_BUFFER_TOO_SMALL, STATUS_SUCCESS, UNICODE_STRING};

pub struct NtKey {
    pub path: Box<UNICODE_STRING>,
    pub num_values: usize,
    handle: HandleBox,
}

impl NtKey {
    pub fn open(path: &str) -> Result<NtKey, NTSTATUS> {
        let path = path.to_string();
        let mut attr = init_object_attributes(
            path.clone(),
            OBJ_KERNEL_HANDLE,
            Default::default(),
            Default::default(),
        );
        let mut handle = HANDLE::default();

        match unsafe { ZwOpenKey(&mut handle, KEY_ALL_ACCESS, attr.as_mut()) } {
            STATUS_SUCCESS => {}
            err => return Err(err),
        }

        let mut return_size = u32::default();
        let mut info = KEY_FULL_INFORMATION::alloc_sized(512);

        match unsafe {
            ZwQueryKey(
                handle,
                KeyFullInformation,
                as_pvoid!(info),
                512,
                &mut return_size,
            )
        } {
            STATUS_SUCCESS => {}
            err => return Err(err),
        }

        Ok(Self {
            path: path.to_unicode_string(),
            handle: HandleBox::new(handle),
            num_values: info.Values as _,
        })
    }

    pub fn get_value<T>(&self, value: &str) -> Result<&mut T, NTSTATUS> {
        let mut return_size = u32::default();
        let mut info: Box<KEY_VALUE_FULL_INFORMATION>;

        match unsafe {
            ZwQueryValueKey(
                self.handle.get_danger(),
                value.to_unicode_string().as_mut(),
                KeyValueFullInformation,
                null_mut(),
                0,
                &mut return_size,
            )
        } {
            STATUS_BUFFER_TOO_SMALL => {}
            err => return Err(err),
        };

        info = KEY_VALUE_FULL_INFORMATION::alloc_sized(return_size as usize);

        match unsafe {
            ZwQueryValueKey(
                self.handle.get_danger(),
                value.to_unicode_string().as_mut(),
                KeyValueFullInformation,
                as_pvoid!(info),
                return_size,
                &mut return_size,
            )
        } {
            STATUS_SUCCESS => {}
            err => return Err(err),
        };

        Ok(unsafe { &mut *get_data!(info, T) })
    }
}
