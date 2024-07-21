use super::{LockClassKey, NeedsLockClass};
use crate::{bindings, str::CStr, Opaque};
use core::{marker::PhantomPinned, pin::Pin};


/// Safely initialises a [`Completion`] with the given name, generating a new lock class.
#[macro_export]
macro_rules! completion_init {
    ($completion:expr, $name:literal) => {
        $crate::init_with_lockdep!($completion, $name)
    };
}

/// A wrapper around a kernel completion object.
pub struct Completion {
    completion: Opaque<bindings::completion>,

    _pin: PhantomPinned,
}

unsafe impl Send for Completion {}
unsafe impl Sync for Completion {}

impl Completion {
    /// The caller must call `completion_init!` before using the conditional variable.
    pub const unsafe fn new() -> Self {
        Self {
            completion: Opaque::uninit(),
            _pin: PhantomPinned,
        }
    }

    /// Wait for the completion to complete.
    pub fn wait(&self) {
        unsafe { bindings::wait_for_completion(self.completion.get()) }
        // Task::current().signal_pending()
    }

    /// Complete the completion.
    pub fn complete(&self) {
        unsafe { bindings::complete(self.completion.get()) }
    }

    /// Get the completion pointer.
    pub fn completion(&self) -> *mut bindings::completion {
        self.completion.get()
    }
}

impl NeedsLockClass for Completion {
    fn init(
        self: Pin<&mut Self>,
        _name: &'static CStr,
        _key: &'static LockClassKey,
        _: &'static LockClassKey,
    ) {
        unsafe {
            bindings::init_completion(self.completion.get())
        };
    }
}
