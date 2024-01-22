use crate::ForeignType;

use super::EnumerateCallback;
use xnvme_sys;

use super::Result;
use super::XnvmeError;

/// A list of enumerated devices.
///
/// # Invariants
///
/// * self.0 must be allocated and initialized by `xnvme_cli_enumeration_alloc()`.
/// 
#[repr(transparent)]
pub struct List(ForeignType<xnvme_sys::xnvme_cli_enumeration>);

impl List {
    /// Create a mutable reference from a pointer
    ///
    /// # SAFETY
    ///
    /// * Caller must ensure that `ptr` is a valid and initialized
    /// `xnvme_cli_enumeration`
    unsafe fn from_ptr<'a>(ptr: *mut xnvme_sys::xnvme_cli_enumeration) -> &'a mut Self {
        // SAFETY: By safety requirements of this function, pointee of `ptr` has
        // same layout as `List`
        unsafe { &mut *(ptr.cast::<List>()) }
    }
}

/// An owned unique reference to a `List`.
///
/// This handle uniquely owns the underlying `List`. When this type is dropped,
/// the underlying List is deallocated.
pub struct ListHandle {
    list: &'static mut List,
}

impl ListHandle {
    /// Create a new instance with spcace for `capacity` devices.
    ///
    /// Calls [xnvme_sys::xnvme_enumeration_alloc].
    pub fn new(capacity: u32) -> Result<Self> {
        let mut list: *mut xnvme_sys::xnvme_cli_enumeration = std::ptr::null_mut();

        // SAFETY: This function writes through the first argument. This is OK
        // since `&mut list` points to a pointer on the stack.
        let err = unsafe { xnvme_sys::xnvme_cli_enumeration_alloc(&mut list, capacity) };

        if err != 0 {
            return Err(XnvmeError::XError(err));
        }

        Ok(Self {
            // SAFETY: pointee of `list` was allocated and initialized above
            list: unsafe { List::from_ptr(list) },
        })
    }

    /// Pretty prints the contents of the contained list to stdout by calling
    /// `xnvme_sys::xnvme_enumeration_pp`
    pub fn pp(&self) -> Result<()>{
        // SAFETY: By type invariant of `List`, `self.list` is backed by a valid
        // `xnvme_cli_enumeration`
        let written = unsafe {
            xnvme_sys::xnvme_cli_enumeration_pp(
                self.list.0.get(),
                xnvme_sys::xnvme_pr_XNVME_PR_DEF as i32,
            )
        };

        if written < 0 {
            return Err(XnvmeError::XError(written));
        }

        Ok(())
    }
}

impl Drop for ListHandle {
    /// Calls [xnvme_sys::xnvme_cli_enumeration_free].
    fn drop(&mut self) {
        // SAFETY: By type invariant of `self.list`, backing value is valid and
        // was allocated by `xnvme_cli_enumeration_alloc()`.
        unsafe {
            xnvme_sys::xnvme_cli_enumeration_free(self.list.0.get());
        }
    }
}

/// A enumerator callback that adds enumerated devices to an internal list.
/// Wrapper for [xnvme_sys::xnvme_enumeration].
impl EnumerateCallback for ListHandle {
    /// Called for each enumerated device
    fn callback(&mut self, device: &mut crate::device::DeviceHandle) -> Result<()> {
        // SAFETY: By C API contract this call will return a valid `xnvme_ident`
        // when called with a valid `xnvme_dev` pointer. The lifetime of the
        // returned value is equal to the lifetime of the `xnvme_dev` argument.
        let ident: *const xnvme_sys::xnvme_ident =
            unsafe { xnvme_sys::xnvme_dev_get_ident(device.as_raw()) };

        // SAFETY: This call is safe when the passed arguments point to valid and initialized structures.
        let err = unsafe { xnvme_sys::xnvme_cli_enumeration_append(self.list.0.get(), ident) };

        match err {
            0 => Ok(()),
            _ => Err(XnvmeError::XError(err)),
        }
    }
}
