pub(crate) use crate::plugins::plugin::Plugin;
use crate::utils::alloc::PoolAllocSized;
use crate::{as_pvoid};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, Ordering};
use uuid::Uuid;
use wdk_sys::_KEY_INFORMATION_CLASS::KeyBasicInformation;
use wdk_sys::ntddk::{IoGetCurrentProcess, RtlUnicodeToUTF8N, ZwEnumerateKey, ZwOpenKey};
use wdk_sys::{
    HANDLE, KEY_ALL_ACCESS, KEY_BASIC_INFORMATION, OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE,
    OBJECT_ATTRIBUTES, PVOID, STATUS_NO_MORE_ENTRIES, STATUS_SUCCESS,
};
use crate::win::utf_to_unicode::Utf8ToUnicodeString;
use crate::win::utils::InitializeObjectAttributes;

pub(crate) mod plugin;
pub(crate) mod commands;

pub static PLUGINS: AtomicPtr<PluginTable> = AtomicPtr::new(null_mut());


// TODO: Fix ownership semantics
pub(crate) struct PluginTable {
    pub plugins: &'static mut [&'static mut Plugin],
}

impl PluginTable {
    ///
    /// # Get
    ///
    /// Gets the plugin from PLUGINS global variable.
    ///
    /// ## Arguments
    /// * `uuid` - [`Uuid`] the plugin was saved to system with.
    ///
    /// ## Return
    /// * [`None`] - Plugin not found.
    /// * [`Some`] - Plugin.
    pub fn lookup(uuid: Uuid) -> Option<&'static mut Plugin> {
        let ptr = PLUGINS.load(Ordering::Acquire);
        if ptr.is_null() {
            return None;
        }
        let slice = unsafe { &mut *ptr };

        match slice.plugins.iter_mut().find(|p| p.uuid == uuid) {
            Some(p) => Some(*p),
            None => None,
        }
    }

    ///
    /// # Current
    ///
    /// Gets the current plugin from current process context
    ///
    /// ## Return
    /// * [`None`] - No plugin associated with current process context.
    /// * [`Some`] - Plugin.
    pub fn current() -> Option<&'static mut Plugin> {
        let ptr = PLUGINS.load(Ordering::Acquire);
        if ptr.is_null() {
            return None;
        }

        let slice = unsafe { &mut *ptr };

        match slice
            .plugins
            .iter_mut()
            .find(|p| p.process == unsafe { IoGetCurrentProcess() })
        {
            Some(p) => Some(*p),
            None => None,
        }
    }
}

///
/// # Load plugins
///
/// Reads the plugins from \REGISTRY\MACHINE\SOFTWARE\HxPosed\Plugins.
///
/// Uses tricky stuff to save them to PLUGINS global variable, too.
///
/// ## Warning
/// In case of a corrupted installation of HxPosed, where the registry keys are missing, this function will fail but driver will be loaded.
///
pub(crate) fn load_plugins() {
    log::trace!("Loading plugins");
    let mut list = Vec::<&mut Plugin>::new();
    list.clear();

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
        log::error!("Error opening registry key: {:?}", status);
        log::warn!("No plugins loaded.");
        return;
    }

    let mut index = 0;
    loop {
        let mut return_length = 0;
        match unsafe {
            ZwEnumerateKey(
                key,
                index,
                KeyBasicInformation,
                Default::default(),
                0,
                &mut return_length,
            )
        } {
            STATUS_NO_MORE_ENTRIES => {
                break;
            }
            _ => {}
        };

        let mut info = KEY_BASIC_INFORMATION::alloc_sized(return_length as _);

        match unsafe {
            ZwEnumerateKey(
                key,
                index,
                KeyBasicInformation,
                as_pvoid!(info),
                return_length,
                &mut return_length,
            )
        } {
            STATUS_SUCCESS => {}
            status => {
                log::warn!("ZwEnumerateKey failed with status {}", status);
                index += 1;
                continue;
            }
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
        match unsafe {
            RtlUnicodeToUTF8N(
                name.as_mut_ptr() as _,
                actual_bytes,
                &mut actual_bytes,
                info.as_mut().Name.as_mut_ptr(),
                info.as_mut().NameLength,
            )
        } {
            STATUS_SUCCESS => unsafe {
                // the Vec doesn't know bytes have been written to it. let's make it know.
                name.set_len(actual_bytes as usize);
            },
            _ => {
                log::warn!("Failed to convert Unicode to UTF8.");
                log::warn!("Plugin skipped.");
                continue;
            }
        }

        let uuid = match Uuid::parse_str(str::from_utf8(name.as_slice()).unwrap()) {
            Ok(uuid) => uuid,
            Err(err) => {
                log::warn!("Error parsing plugin GUID: {:?}", err);
                log::warn!("Plugin skipped.");
                continue;
            }
        };

        match Plugin::open(uuid) {
            Some(plugin) => list.push(Box::leak(Box::new(plugin))),
            None => log::error!("Queried plugin not found."),
        }

        index += 1;
    }

    // dark shady Rust evasion stuff
    let plugin_slice: &'static mut [&mut Plugin] = Box::leak(list.into_boxed_slice());

    log::trace!("Total of {} plugins loaded.", plugin_slice.len());

    let table = Box::leak(Box::new(PluginTable {
        plugins: plugin_slice,
    }));
    PLUGINS.store(table as *const _ as *mut _, Ordering::Release);
}
