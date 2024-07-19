// SPDX-License-Identifier: GPL-2.0
//! Rust hello world module sample

use kernel::prelude::*;

module! {
    type: RustHelloWorld,
    name: "rust_helloworld",
    author: "creatoy",
    description: "Hello world module in rust",
    license: "GPL",
}

struct RustHelloWorld {}

impl kernel::Module for RustHelloWorld {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello, Rust module!\n");
        Ok(RustHelloWorld {})
    }
}

impl Drop for RustHelloWorld {
    fn drop(&mut self) {
        pr_info!("Bye, Rust module!\n");
    }
}
