#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::all)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::missing_safety_doc)]

use std::{cell::UnsafeCell, mem::MaybeUninit, marker::PhantomPinned};

use thiserror::Error;

pub mod device;
pub mod enumerate;
pub mod options;

#[derive(Error, Debug)]
pub enum XnvmeError {
    #[error("xNVME error: {0}")]
    XError(i32),

    #[error("Invalid parameter")]
    InvalidParameter(#[from] std::ffi::NulError),
}

pub type Result<T> = std::result::Result<T, XnvmeError>;


#[repr(transparent)]
struct ForeignType<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    _p: PhantomPinned,
}

impl<T> ForeignType<T> {

    #[allow(dead_code)]
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(MaybeUninit::new(value)),
            _p: PhantomPinned,
        }
    }

    pub fn get(&self) -> *mut T {
        UnsafeCell::get(&self.value).cast::<T>()
    }
}
