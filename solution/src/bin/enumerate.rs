pub use xnvme_sys::*;

fn main() {
    unsafe {
        let mut list: *mut xnvme_cli_enumeration = std::ptr::null_mut();
        let err = xnvme_cli_enumeration_alloc(&mut list, 100);

        let err = xnvme_enumerate(
            std::ptr::null(),
            std::ptr::null_mut(),
            Some(list_cb),
            list as *mut std::ffi::c_void,
        );

        let err = xnvme_cli_enumeration_pp(list, xnvme_pr_XNVME_PR_DEF as i32);
    }
}

unsafe extern "C" fn list_cb(dev: *mut xnvme_dev, cb_args: *mut std::ffi::c_void) -> i32 {
    let list: *mut xnvme_cli_enumeration = cb_args as *mut xnvme_cli_enumeration;
    let ident: *const xnvme_ident = xnvme_dev_get_ident(dev);
    let err = xnvme_cli_enumeration_append(list, ident);
    return xnvme_enumerate_action_XNVME_ENUMERATE_DEV_CLOSE as i32;
}
