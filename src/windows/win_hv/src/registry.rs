use crate::as_pvoid;
use crate::win::alloc::PoolAllocSized;
use crate::win::{InitializeObjectAttributes, Utf8ToUnicodeString, timing};
use wdk::println;
use wdk_sys::_KEY_INFORMATION_CLASS::{KeyBasicInformation, KeyFullInformation};
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{
    KeDelayExecutionThread, RtlAppendUnicodeStringToString, RtlAppendUnicodeToString,
    RtlDuplicateUnicodeString, RtlFreeUnicodeString, ZwClose, ZwEnumerateKey, ZwOpenKey,
    ZwQueryKey,
};
use wdk_sys::{
    HANDLE, KEY_ALL_ACCESS, KEY_BASIC_INFORMATION, KEY_FULL_INFORMATION, LARGE_INTEGER,
    OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE, OBJECT_ATTRIBUTES, PVOID, STATUS_NO_MORE_ENTRIES,
    STATUS_SUCCESS, TRUE, UNICODE_STRING,
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

            // we cannot use RtlInitUnicodeString since info.Name is NOT null terminated.
            let mut subkey_name = UNICODE_STRING::default();
            subkey_name.Length = info.as_mut().NameLength as _;
            subkey_name.MaximumLength = subkey_name.Length;
            subkey_name.Buffer = info.as_mut().Name.as_mut_ptr();

            let mut new_root = UNICODE_STRING::default();
            let status = unsafe { RtlDuplicateUnicodeString(0, root.as_ref(), &mut new_root) };
            if status != STATUS_SUCCESS {
                println!("RtlDuplicateUnicodeString failed with status {}", status);
                index += 1;
                continue;
            }

            if unsafe { RtlAppendUnicodeStringToString(&mut new_root, &subkey_name) }
                != STATUS_SUCCESS
            {
                println!(
                    "RtlAppendUnicodeStringToString failed with status {}",
                    status
                );
                unsafe { RtlFreeUnicodeString(&mut new_root) };

                index += 1;
                continue;
            }

            let mut new_key = HANDLE::default();
            let mut new_attributes = OBJECT_ATTRIBUTES::default();
            unsafe {
                InitializeObjectAttributes(
                    &mut new_attributes,
                    &mut new_root,
                    OBJ_KERNEL_HANDLE | OBJ_CASE_INSENSITIVE,
                    Default::default(),
                    Default::default(),
                )
            };

            let status = unsafe { ZwOpenKey(&mut new_key, KEY_ALL_ACCESS, &mut attributes) };
            if status != STATUS_SUCCESS {
                println!("ZwOpenKey failed with status {}", status);
            }

            index += 1;
        }

        unsafe {
            let _ = KeDelayExecutionThread(KernelMode as _, TRUE as _, &mut interval);
        }
    }
}
