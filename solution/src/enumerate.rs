use xnvme_sys;

use super::Result;
use super::XnvmeError;

/// Implement this trait to add logic to an enumeration pass.
pub trait EnumerateCallback {
    /// Called for each enumerated device
    fn callback(&mut self, device: &mut crate::device::DeviceHandle) -> Result<()>;
}

pub struct Enumerator<C: EnumerateCallback> {
    pub(crate) _c: std::marker::PhantomData<C>,
}

impl<C> Enumerator<C>
where
    C: EnumerateCallback,
{
    /// Enumerate devices on the system.
    ///
    /// Calls `callback()` method of the passed `EnumerateCallback` for each
    /// enumerated device.
    pub fn enumerate(arg: &mut C) -> Result<()> {
        let pointer: *mut C = arg;

        // SAFETY: By API contract C code does not hold on to `pointer` after
        // return of this function.
        let err = unsafe {
            xnvme_sys::xnvme_enumerate(
                std::ptr::null(),
                std::ptr::null_mut(),
                Some(Self::list_cb),
                pointer as *mut std::ffi::c_void,
            )
        };

        match err {
            0 => Ok(()),
            _ => Err(XnvmeError::XError(err)),
        }
    }

    /// # SAFETY
    ///
    /// * This function must only be called by `xnvme_enumerate` in C
    ///   when `xnvme_enumearte` was invoked by `enumerate()`.
    /// * `dev` must be pointing to a valid `xnvme_dev`.
    pub(crate) unsafe extern "C" fn list_cb(
        dev: *mut xnvme_sys::xnvme_dev,
        cb_args: *mut std::ffi::c_void,
    ) -> i32 {
        let pointer: *mut C = cb_args as *mut C;

        // SAFETY: By function safety requirements, pointer is a &mut C that we
        // cast to raw pointer ourselves. It must not leak from here. Note that
        // lifetime of `r: &mut C` is unbounded. This is OK because `r` never
        // leave this function.
        let r: &mut C = unsafe { &mut *pointer };

        // SAFETY: This call satisfies callees safety requirements because `dev`
        // is guaranteed to be valid by the safety requirements of this function.
        let mut device: crate::device::DeviceHandle =
            unsafe { crate::device::DeviceHandle::new(dev) };

        // TODO: Use return code
        let _ = r.callback(&mut device);

        // When `device` goes out of scope it is closed, so don't ask xnvme to
        // close it.
        xnvme_sys::xnvme_enumerate_action_XNVME_ENUMERATE_DEV_KEEP_OPEN as i32
    }
}

pub mod list;
