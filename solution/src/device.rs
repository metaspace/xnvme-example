use super::Result;
use crate::ForeignType;

/// An xNVMe device.
///
/// #Invariants
///
/// * `self.0` must be a valid `xnvme_dev`.
///
#[repr(transparent)]
pub struct Device(ForeignType<xnvme_sys::xnvme_dev>);

/// An onwned unique reference to a `Device`.
///
/// This handle exclusively owns the underlying `Device`. When dropped, the
/// underlying `Device` is deallocated.
pub struct DeviceHandle {
    device: &'static mut Device,
}

impl DeviceHandle {
    /// # Safety
    ///
    /// * `device` must point to an initialized and valid [xnvme_sys::xnvme_dev]
    /// * `device` must be uniquely owned by the caller
    /// * Ownership of pointee passes to this function upon invocation
    pub unsafe fn new(device: *mut xnvme_sys::xnvme_dev) -> Self {
        Self {
            // SAFETY: Pointee of `device` has same layout as `Device`
            device: unsafe { &mut *(device.cast::<Device>()) },
        }
    }

    /// Return the internal raw pointer to the underlying [xnvme_sys::xnvme_dev] struct.
    ///
    /// The returned pointer will always point to a valid and initialized struct.
    pub fn as_raw(&self) -> *mut xnvme_sys::xnvme_dev {
        self.device.0.get()
    }

    pub fn open(device_uri: &str, options: &crate::options::Options) -> Result<Self> {
        let device_uri_c = std::ffi::CString::new(device_uri)?;

        // SAFETY: This call is safe because first argument is a pointer to a
        // valid null terminated c string and the second argument is null, thus
        // complying with C API.
        let dev =
            unsafe { xnvme_sys::xnvme_dev_open(device_uri_c.as_ptr(), &mut options.as_raw()) };

        // Todo: check non null

        // SAFETY: This invocation complies with callee safety requirement
        // because we just constructed dev above and it is therefore a valid
        // instance.
        let device = unsafe { Self::new(dev) };

        Ok(device)
    }
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        unsafe { xnvme_sys::xnvme_dev_close(self.device.0.get()) }
    }
}
