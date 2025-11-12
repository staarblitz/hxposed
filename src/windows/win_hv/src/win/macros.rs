use wdk_sys::OBJECT_ATTRIBUTES;
use wdk_sys::ntddk::RtlInitUnicodeString;

#[macro_export]
macro_rules! as_utf16 {
    ($str:expr) => {
       WString::from($str).as_mut_wstr().as_mut()
    };
}

#[macro_export]
macro_rules! as_pvoid {
    ($t:ident) => {
        $t.as_mut() as *mut _ as PVOID
    };
}

#[macro_export]
macro_rules! get_data {
    ($k:ident, $t:ident) => {
        ($k.as_mut() as *mut _ as PVOID).byte_offset($k.DataOffset as _) as *mut $t
    };
}