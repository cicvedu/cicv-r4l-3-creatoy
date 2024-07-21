// SPDX-License-Identifier: GPL-2.0

//! Rust completion sample.

use kernel::prelude::*;
use kernel::{
    file::self,
    chrdev,
    bindings,
    sync::Completion,
    task::Task,
};

module! {
    type: RustCompletion,
    name: "rust_completion",
    author: "creatoy",
    description: "Rust completion sample",
    license: "GPL",
}

static SHARED_COMPLETION: Completion = unsafe { Completion::new() };

struct RustFile {
    #[allow(dead_code)]
    inner: &'static Completion,
}

#[vtable]
impl file::Operations for RustFile {
    type Data = Box<Self>;

    fn open(_shared: &(), _file: &file::File) -> Result<Box<Self>> {
        pr_warn!("open is invoked\n");

        Ok(
            Box::try_new(RustFile {
                inner: &SHARED_COMPLETION
            })?
        )
    }

    fn write(this: &Self,_file: &file::File,reader: &mut impl kernel::io_buffer::IoBufferReader,_offset:u64,) -> Result<usize> {
        pr_info!("write is invoked\n");

        let current = Task::current();

        pr_info!("process {} awakening the readers...\n", current.pid());
        this.inner.complete();

        Ok(reader.len())
    }

    fn read(this: &Self,_file: &file::File,_writer: &mut impl kernel::io_buffer::IoBufferWriter,_offset:u64,) -> Result<usize> {
        pr_info!("read is invoked\n");

        let current = Task::current();

        pr_info!("process {} is going to sleep\n", current.pid());
        this.inner.wait();
        pr_info!("awoken {}\n", current.pid());

        Ok(0)
    }
}

struct RustCompletion {
    _dev: Pin<Box<chrdev::Registration<1>>>,
}

impl kernel::Module for RustCompletion {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust completion sample (init)\n");

        unsafe {
            bindings::init_completion(SHARED_COMPLETION.completion());
        }

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustCompletion { _dev: chrdev_reg })
    }
}

impl Drop for RustCompletion {
    fn drop(&mut self) {
        pr_info!("Rust completion sample (exit)\n");
    }
}
