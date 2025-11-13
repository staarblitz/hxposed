use crate::plugins::plugin::Plugin;
use crate::win::alloc::PoolAllocSized;
use crate::win::{InitializeObjectAttributes, Utf8ToUnicodeString, timing};
use crate::{PLUGINS_DB, as_pvoid};
use alloc::vec::Vec;
use core::ops::DerefMut;
use core::str::FromStr;
use uuid::{Error, Uuid};
use wdk::{dbg_break, println};
use wdk_sys::_KEY_INFORMATION_CLASS::KeyBasicInformation;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{KeDelayExecutionThread, RtlUnicodeToUTF8N, ZwEnumerateKey, ZwOpenKey};
use wdk_sys::{
    HANDLE, KEY_ALL_ACCESS, KEY_BASIC_INFORMATION, LARGE_INTEGER, OBJ_CASE_INSENSITIVE,
    OBJ_KERNEL_HANDLE, OBJECT_ATTRIBUTES, PVOID, STATUS_NO_MORE_ENTRIES, STATUS_SUCCESS, TRUE,
};

/// The reason we are using a timer is that we need to keep an in-memory track of the HxPosed key. Since we cannot use ZwCreateKey
/// in IRQL 255 (typical irql on vmexit), we HAVE to do this ugly hack.
///
/// We don't use timers because they will fire us in DPC IRQL, which won't do us any good either.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn registry_timer(_context: PVOID) {
    let mut interval = LARGE_INTEGER {
        QuadPart: timing::relative(timing::milliseconds(2500)),
    };
    dbg_break();

    let mut root = "\\Registry\\Machine\\Software\\HxPosed\\Plugins".to_unicode_string();
    let mut attributes = OBJECT_ATTRIBUTES::default();
    unsafe {
        InitializeObjectAttributes(
            &mut attributes,
            root.as_mut(),
            OBJ_KERNEL_HANDLE | OBJ_CASE_INSENSITIVE,
            Default::default(),
            Default::default(),
        );
    }

    let mut key = HANDLE::default();
    let status = unsafe { ZwOpenKey(&mut key, KEY_ALL_ACCESS, &mut attributes) };

    if status != STATUS_SUCCESS {
        panic!("ZwOpenKey failed with status {}", status);
    }

    loop {
        let mut plugins = PLUGINS_DB.lock();
        plugins.deref_mut().clear();
        let mut index = 0;
        loop {
            let mut return_length = 0;
            let status = unsafe {
                ZwEnumerateKey(
                    key,
                    index,
                    KeyBasicInformation,
                    Default::default(),
                    0,
                    &mut return_length,
                )
            };

            if status == STATUS_NO_MORE_ENTRIES {
                index = 0;
                break;
            }

            let mut info = KEY_BASIC_INFORMATION::alloc_sized(return_length as _);

            let status = unsafe {
                ZwEnumerateKey(
                    key,
                    index,
                    KeyBasicInformation,
                    as_pvoid!(info),
                    return_length,
                    &mut return_length,
                )
            };

            if status != STATUS_SUCCESS {
                println!("ZwEnumerateKey failed with status {}", status);
                index += 1;
                continue;
            }

            let mut actual_bytes = 0;
            let _ = unsafe {
                RtlUnicodeToUTF8N(
                    Default::default(),
                    0,
                    &mut actual_bytes,
                    info.as_mut().Name.as_mut_ptr(),
                    info.as_mut().NameLength,
                )
            };

            let mut name = Vec::<u8>::with_capacity(actual_bytes as usize);
            let status = unsafe {
                RtlUnicodeToUTF8N(
                    name.as_mut_ptr() as _,
                    actual_bytes,
                    &mut actual_bytes,
                    info.as_mut().Name.as_mut_ptr(),
                    info.as_mut().NameLength,
                )
            };

            unsafe{
                // the Vec doesn't know bytes have been written to it. let's make it know.
                name.set_len(actual_bytes as usize);
            }

            if status != STATUS_SUCCESS {
                println!("RtlUnicodeToUTF8 failed with status {}", status);
                continue;
            }

            let uuid = match Uuid::parse_str(str::from_utf8(name.as_slice()).unwrap()) {
                Ok(uuid) => uuid,
                Err(err) => {
                    println!("Error parsing uuid: {:?}", err);
                    continue;
                }
            };

            let plugin = Plugin::open(uuid);

            plugins.deref_mut().push(plugin.unwrap());

            index += 1;
        }

        drop(plugins);

        unsafe {
            let _ = KeDelayExecutionThread(KernelMode as _, TRUE as _, &mut interval);
        }
    }
}
