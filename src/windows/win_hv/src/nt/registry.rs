use crate::utils::alloc::PoolAllocSized;
use crate::utils::handlebox::HandleBox;
use crate::win::rtl_utils::init_object_attributes;
use crate::win::unicode_string::UnicodeString;
use crate::win::{
    HANDLE, KEY_FULL_INFORMATION, KEY_VALUE_FULL_INFORMATION, KeyAccessRights, KeyInformationClass,
    KeyValueInformationClass, NtStatus, ObjectAttributes, ZwOpenKey, ZwQueryKey, ZwQueryValueKey,
};
use crate::{as_pvoid, get_data};
use alloc::boxed::Box;
use alloc::string::String;
use core::ffi::c_void;
use core::ptr::null_mut;
use core::str::FromStr;

pub struct NtKey {
    pub path: String,
    pub num_values: usize,
    handle: HandleBox,
}

impl NtKey {
    pub fn open(path: &str) -> Result<NtKey, NtStatus> {
        let mut unicode_string = UnicodeString::new(path);
        let mut str = unicode_string.to_unicode_string();
        let mut attr = init_object_attributes(
            &mut str,
            ObjectAttributes::KernelHandle,
            Default::default(),
            Default::default(),
        );
        let mut handle = HANDLE::default();

        match unsafe { ZwOpenKey(&mut handle, KeyAccessRights::All, &mut attr) } {
            NtStatus::Success => {}
            err => return Err(err),
        }

        let mut return_size = u32::default();
        let mut info = KEY_FULL_INFORMATION::alloc_sized(512);

        match unsafe {
            ZwQueryKey(
                handle,
                KeyInformationClass::KeyFullInformation,
                as_pvoid!(info),
                512,
                &mut return_size,
            )
        } {
            NtStatus::Success => {}
            err => return Err(err),
        }

        Ok(Self {
            path: String::from_str(path).unwrap(),
            handle: HandleBox::new(handle),
            num_values: info.Values as _,
        })
    }

    pub fn get_value_string(&self, value: &str) -> Result<UnicodeString, NtStatus> {
        let value_ptr = self.get_value::<u16>(value)?;
        Ok(UnicodeString::from_raw(value_ptr))
    }

    pub fn get_value<T>(&self, value: &str) -> Result<&mut T, NtStatus> {
        let mut return_size = u32::default();
        let mut info: Box<KEY_VALUE_FULL_INFORMATION>;
        let mut string = UnicodeString::new(value);
        let mut value_name = string.to_unicode_string();

        match unsafe {
            ZwQueryValueKey(
                self.handle.get_danger(),
                &mut value_name,
                KeyValueInformationClass::KeyValueFullInformation,
                null_mut(),
                0,
                &mut return_size,
            )
        } {
            NtStatus::BufferTooSmall => {}
            err => return Err(err),
        };

        info = KEY_VALUE_FULL_INFORMATION::alloc_sized(return_size as usize);

        match unsafe {
            ZwQueryValueKey(
                self.handle.get_danger(),
                &mut value_name,
                KeyValueInformationClass::KeyValueFullInformation,
                as_pvoid!(info),
                return_size,
                &mut return_size,
            )
        } {
            NtStatus::Success => {}
            err => return Err(err),
        };

        Ok(unsafe { &mut *get_data!(info, T) })
    }
}
