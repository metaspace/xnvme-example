use const_str::cstr;
use derive_builder::Builder;

#[derive(Builder)]
pub struct Options {
    backend: Backend,
    rdonly: bool,
}

#[derive(Copy, Clone)]
pub enum Backend {
    Linux,
    Spdk,
    Fbsd,
    Posix,
    Macos,
    Ramdisk,
    Vfio,
}

impl From<Backend> for &std::ffi::CStr {
    fn from(value: Backend) -> Self {
        use Backend::*;
        match value {
            Linux => cstr!("linux"),
            Spdk => cstr!("spdk"),
            Posix => cstr!("posix"),
            Ramdisk => cstr!("ramdisk"),
            Vfio => cstr!("vfio"),
            _ => cstr!("not supported"),
        }
    }
}

impl Options {
    pub fn as_raw(&self) -> xnvme_sys::xnvme_opts {
        let mut c_opts: xnvme_sys::xnvme_opts = unsafe { xnvme_sys::xnvme_opts_default() };

        let backend: &std::ffi::CStr = self.backend.into();

        //TODO: Check lifetime of string. Should be 'static, but need to check.
        c_opts.be = backend.as_ptr();

        // SAFETY: Access to union is unsafe. In this case it is safe because C
        // uses the union to access bitfield as u32.
        c_opts.rdonly = self.rdonly.into();

        c_opts
    }
}
