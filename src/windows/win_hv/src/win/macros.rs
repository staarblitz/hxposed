use wdk_sys::OBJECT_ATTRIBUTES;
use wdk_sys::ntddk::RtlInitUnicodeString;

#[macro_export]
macro_rules! as_utf16 {
    ($str:expr) => {
        $str.encode_utf16()
            .chain(once(0))
            .collect::<Vec<u16>>()
            .as_ptr()
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

#[macro_export]
macro_rules! init_object_attributes {
    ($obj:expr, $name:expr, $attr:expr, $root:expr, $sec:expr) => {
        RtlInitUnicodeString($obj.ObjectName, as_utf16!($name));
        $obj.Length = size_of::<OBJECT_ATTRIBUTES>() as _;
        $obj.Attributes = $attr;
        $obj.RootDirectory = $root;
        $obj.SecurityDescriptor = $sec;
    };
}
