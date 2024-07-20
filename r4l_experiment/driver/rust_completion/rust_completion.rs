// SPDX-License-Identifier: GPL-2.0
//! Rust hello world module sample

use kernel::prelude::*;

module! {
    type: RustCompletion,
    name: "rust_helloworld",
    author: "creatoy",
    description: "Hello world module in rust",
    license: "GPL",
}

struct RustCompletion {}

impl kernel::Module for RustCompletion {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello, Rust completion!\n");
        Ok(RustCompletion {})
    }
}

impl Drop for RustCompletion {
    fn drop(&mut self) {
        pr_info!("Bye, Rust completion!\n");
    }
}
