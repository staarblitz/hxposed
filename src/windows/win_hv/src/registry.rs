use crate::plugins::plugin::Plugin;
use crate::win::alloc::PoolAllocSized;
use crate::win::{timing, InitializeObjectAttributes, Utf8ToUnicodeString};
use crate::{as_pvoid, PLUGINS_DB};
use core::ops::DerefMut;
use core::str::FromStr;
use uuid::Uuid;
use wdk::println;
use wdk_sys::ntddk::{KeDelayExecutionThread, RtlUnicodeToUTF8N, ZwEnumerateKey, ZwOpenKey};
use wdk_sys::_KEY_INFORMATION_CLASS::KeyBasicInformation;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::{
    HANDLE, KEY_ALL_ACCESS, KEY_BASIC_INFORMATION, LARGE_INTEGER, OBJECT_ATTRIBUTES,
    OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE, PVOID, STATUS_NO_MORE_ENTRIES, STATUS_SUCCESS, TRUE,
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

            // a guid is max 38 bytes in its string representation
            let mut name = [0u8; 38];
            let mut actual_bytes = 0;
            let status = unsafe {
                RtlUnicodeToUTF8N(
                    name.as_mut_ptr() as _,
                    38,
                    &mut actual_bytes,
                    info.as_mut().Name.as_mut_ptr(),
                    info.as_mut().NameLength,
                )
            };

            if status != STATUS_SUCCESS {
                println!("RtlUnicodeToUTF8 failed with status {}", status);
            }

            let plugin = Plugin::open(Uuid::from_str(str::from_utf8(&name).unwrap()).unwrap());
            plugins.deref_mut().push(plugin.unwrap());

            index += 1;
        }

        drop(plugins);

        unsafe {
            let _ = KeDelayExecutionThread(KernelMode as _, TRUE as _, &mut interval);
        }
    }
}
