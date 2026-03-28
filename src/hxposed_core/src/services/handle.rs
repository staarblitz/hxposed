use crate::error::HxError;
use crate::hxposed::{Handle, HxObject};
use crate::hxposed::requests::handle::*;
use crate::hxposed::responses::handle::*;
use crate::hxposed::requests::Syscall;

/// # HxHandle
///
/// Represents a handle
///
/// ## Getting a handle with full access rights
///
/// To get a handle with full rights, do the steps below:
/// 1. Create a handle to current process with `PROCESS_QUERY_LIMITED_INFORMATION` (to be sure)
/// 2. Upgrade the handle via [`Self::upgrade`] to all access rights
/// 3. Change the underlying object via [`Self::set_object`] to desired object
/// 4. Use the handle as is.
pub struct HxHandle {
    pub handle: Handle
}

impl HxHandle {
    /// All access for all objects. Maximum possible value for `GrantedAccessRights` in `_HANDLE_TABLE_ENTRY` structure.
    pub const HANDLE_ALL_ACCESS: u32 = 0x1FFFFFF;

    ///
    /// # From Handle
    ///
    /// A new instance of HxHandle from handle object.
    pub fn from_handle(handle: u64) -> HxHandle {
        Self { handle }
    }

    ///
    /// # Upgrade
    ///
    /// Sets access rights of a handle.
    ///
    /// ## Arguments
    /// `access_right` - New access right to set
    ///
    /// ## Remarks
    /// - This has no object type checking.
    /// - To get all access rights for all objects, use [`Self::HANDLE_ALL_ACCESS`]
    /// - Remember that some objects (especially processes) might have extra protection mechanisms.
    pub fn upgrade(&mut self, access_rights: u32) -> Result<(), HxError> {
        UpgradeHandleRequest {
            handle: self.handle,
            access_rights,
            process: 0
        }.send().map(|_| ())
    }

    ///
    /// # Set Object
    ///
    /// Sets the underlying object type of the handle
    ///
    /// ## Arguments
    /// `new_object` - Address of the object in kernel
    ///
    /// ## Remarks
    /// - It is undefined what happens (probably BSOD) if you set it to an invalid object
    /// - There is no checking for that. So be careful and always take pointers from Hx structures
    /// - You still need to adjust the handle access rights to access the object
    /// - The handle stays valid even if you close the Hx object associated with it (e.g. [`HxProcess`])
    /// - The handle is still a normal handle object you can close with `CloseHandle`
    /// - This handles the kernel-mode reference counts. So don't worry and enjoy the handle
    pub fn set_object(&mut self, new_object: HxObject) -> Result<(), HxError> {
        SwapHandleObjectRequest {
            handle: self.handle,
            process: 0,
            object: new_object
        }.send().map(|_| ())
    }

    ///
    /// # Get Object
    ///
    /// Returns the underlying object type and access rights
    ///
    /// ## Returns
    /// [`GetHandleObjectResponse`] - Underlying object and access rights.
    pub fn get_object(&mut self) -> Result<GetHandleObjectResponse, HxError> {
        GetHandleObjectRequest {
            handle: self.handle,
            process: 0,
        }.send()
    }
}