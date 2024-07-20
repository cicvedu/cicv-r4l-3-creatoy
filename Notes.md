## 作业记录

[作业内容](https://docs.qq.com/doc/DSk5xTHRJY1FZVUdK)

> 因为我日常使用的是 Archlinux，这里没有使用 docker 而是直接使用主机作为开发环境。由于很多工具之前都已经安装好了，这里就没有记录。另外 Archlinux 上很多工具和库版本都很新，所以作业操作过程中出了不少问题...
> 
> <div align="center"> <img src=./docs/images/host_env.png width=80% /> </div>


### 环境配置问题
#### 1. 安装 bindgen 时出现错误：error: Usage of HTTP-based registries requires `-Z http-registry`  
改为使用下面的命令：
```
cargo +nig1htly install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen
```

环境配置完成后运行 make LLVM=1 rustavailable

<img src=./docs/images/linux_rust_available.png width=80% />


#### 2. 配置 BusyBox 时出现无法找到 ncurses 错误  
修改 scripts/kconfig/lxdialog/check-lxdialog.sh 第 48 行，main() {} 前加上 int，改为 int main() {}

配置完成后:

<img src=./docs/images/busybox_config.png width=80% />

#### 3. 编译时出现 TCA_CBQ_MAX TCA_CBQ_RATE 等未定义错误

<img src=./docs/images/busybox_error_TCA_CBQ_MAX.png width=80% />

[修复方法在这里](https://bugs.gentoo.org/926872)。或者直接使用[这个补丁文件](./busybox-1.36.1/networking_tc_c.patch)

编译完成:

<img src=./docs/images/busybox_compile_done.png width=80% />

### 作业1: 编译 Linux 内核

#### 操作流程:
```sh
make x86_64_defconfig
make LLVM=1 menuconfig
```
在菜单中选中第一条 **General setup --->**，进入后翻到最后选中 **Rust support**。(带菜单的选项使用 **Enter** 进入，前面带 **[ ]** 的选项使用 Y 或空格键选中。更多操作注意菜单顶端的帮助信息)
> 更方便的方法是使用搜索功能后使用数字键快速跳转到菜单项  

<img src=./docs/images/linux_rust_config.png width=80% />

接着进行内核的编译：
```sh
make LLVM=1 -j$(nproc)
```

<img src=./docs/images/linux_compile_done.png width=80% />

#### 问题记录

1. ~~编译时出现 anonymous union 无法解析的错误：~~ (后面使用 clang 14 可以正常编译)
> ~~解决办法 https://github.com/rust-lang/rust-bindgen/pull/2316~~

2. 编译时出现 __vdso_sgx_enter_enclave 未定义错误:
> 解决办法 https://lkml.org/lkml/2022/11/8/1236

3. **做到作业 2 时发现内核启动不了，最终发现是因为我使用的系统中的 clang 版本太新**
> 解决办法:
> 1. 安装 clang14 (系统包里刚好有)
> ```sh
> sudo pacman -S clang14 llvm14-libs llvm14
> ```
> 2. 系统包里没有 14 版本的 lld, 不过可以直接使用原来lld18的
> ```sh
> sudo ln -s /usr/bin/ld.lld /usr/lib/llvm14/bin/ld.lld
> ```
> 3. 重新编译 bindgen (如果问题 1 改过 bindgen 的代码记得改回来)
> ```sh
> export LLVM_CONFIG_PATH=/usr/lib/llvm14/bin
> export LIBCLANG_PATH=/usr/lib/llvm14/lib
> cargo +nightly install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen --force
> ```
> 4. 重新配置和编译内核
> ```sh
> make LLVM=/usr/lib/llvm14/bin/ clean
> make LLVM=/usr/lib/llvm14/bin/ x86_64_defconfig
> make LLVM=/usr/lib/llvm14/bin/ menuconfig
> make LLVM=/usr/lib/llvm14/bin/ -j$(nproc)
> ```

---

### 作业2: 对 Linux 内核进行一些配置

> NOTE: 按下 CTRL + A 后松开再按 X 可退出 QEMU

#### 操作流程:
1. 驱动编译:
```sh
cd src_e1000
make LLVM=1
```

<img src=./docs/images/e1000_compile.png width=80% />

2. 启动内核:
> 运行脚本前先把里面的 LLVM=1 也改成 LLVM=/usr/lib/llvm14/bin/  
```sh
./build_image.sh
```

<img src=./docs/images/kernel_launch.png width=80% />

3. 修改内核配置，禁用 e1000 网卡驱动:
```
Device Drivers --->
    [*] Network device support --->
        [*] Ethernet driver support --->
            <> Intel devices, Intel(R) PRO/1000 Gigabit Ethernet support
```

4. 重新编译内核后再次启动内核并加载驱动模块:
```sh
insmod r4l_e1000_demo.ko
ip link set eth0 up
ip addr add broadcast 10.0.2.255 dev eth0
ip addr add 10.0.2.15/255.255.255.0 dev eth0 
ip route add default via 10.0.2.1
ping 10.0.2.2
```

<img src=./docs/images/e1000_insmod.png width=80% />
<img src=./docs/images/e1000_ping.png width=80% />

#### 作业问题回答:
Q1、编译成内核模块，是在哪个文件中以哪条语句定义的？

A1: 编译成内核模块是在 **Kbuild** 中 **obj-m := r4l_e1000_demo.o** 这句定义的

Q2、该模块位于独立的文件夹内，却能编译成Linux内核模块，这叫做out-of-tree module，请分析它是如何与内核代码产生联系的？

A2: 在 Makefile 中使用 $(MAKE) -C $(KDIR) M=$$PWD 命令指定了内核和内核模块路径，它将引用内核模块相关的符号来构建树外模块。

---

### 作业3: 使用 rust 编写一个简单的内核模块并运行 

#### 操作流程:
1. 在 linux/samples/rust/ 目录下添加 rust_helloworld.rs 文件(文件内容见[作业内容]的作业3)
2. 修改 linux/samples/rust/ 目录下的 Makefile 和 Kconfig 文件
- 在 Makefile 第 2 行添加
```sh
obj-$(CONFIG_SAMPLE_RUST_HELLOWORLD)	+= rust_helloworld.o
```
- 在 Kconfig 第 13-21 行添加
```
config SAMPLE_RUST_HELLOWORLD
	tristate "Hello world module"
	help
	  This option builds the Rust hello world module sample.

	  To compile this as a module, choose M here:
	  the module will be called rust_helloworld.

	  If unsure, say N.
```
3. 修改内核模块配置
```sh
make LLVM=/usr/lib/llvm14/bin/ menuconfig
```
<img src=./docs/images/rust_helloworld_config.png width=80% />

4. 启动内核检查模块功能是否正常
```sh
cp samples/rust/rust_helloworld.ko ../src_e1000/rootfs/
cd ../src_e1000
./build_image.sh
```
<img src=./docs/images/rust_helloworld_module.png width=80% />

#### 修改后的文件:
[Makefile](./linux/samples/rust/Makefile)

[Kconfig](./linux/samples/rust/Kconfig)

---

### 作业4：为 e1000 网卡驱动添加 remove 代码
#### 操作流程:
见作业内容

#### 运行结果:
<img src=./docs/images/e1000_rmmod_op.png width=80% />
<img src=./docs/images/e1000_rmmod_ping.png width=80% />

#### 文件修改:
```diff
diff --git a/linux/rust/kernel/net.rs b/linux/rust/kernel/net.rs
index 0b432db74..ad74a0fd0 100644
--- a/linux/rust/kernel/net.rs
+++ b/linux/rust/kernel/net.rs
@@ -479,6 +479,14 @@ impl Napi {
         }
     }
 
+    /// Disable NAPI scheduling.
+    pub fn disable(&self) {
+        // SAFETY: The existence of a shared reference means `self.0` is valid.
+        unsafe {
+            bindings::napi_disable(self.0.get());
+        }
+    }
+
     /// Schedule NAPI poll routine to be called if it is not already running.
     pub fn schedule(&self) {
         // SAFETY: The existence of a shared reference means `self.0` is valid.
diff --git a/linux/rust/kernel/pci.rs b/linux/rust/kernel/pci.rs
index f10753105..1fbd79596 100644
--- a/linux/rust/kernel/pci.rs
+++ b/linux/rust/kernel/pci.rs
@@ -267,7 +267,7 @@ impl Device {
     ///
     /// `ptr` must be non-null and valid. It must remain valid for the lifetime of the returned
     /// instance.
-    unsafe fn from_ptr(ptr: *mut bindings::pci_dev) -> Self {
+    pub unsafe fn from_ptr(ptr: *mut bindings::pci_dev) -> Self {
         Self { ptr }
     }
 
@@ -277,6 +277,12 @@ impl Device {
         unsafe { bindings::pci_set_master(self.ptr) };
     }
 
+    /// disables bus-mastering for device
+    pub fn clear_master(&self) {
+        // SAFETY: By the type invariants, we know that `self.ptr` is non-null and valid.
+        unsafe { bindings::pci_clear_master(self.ptr) };
+    }
+
     /// get legacy irq number
     pub fn irq(&self) -> u32 {
         // SAFETY: By the type invariants, we know that `self.ptr` is non-null and valid.
@@ -294,6 +300,12 @@ impl Device {
         }
     }
 
+    /// disable device
+    pub fn disable_device(&mut self) {
+        // SAFETY: By the type invariants, we know that `self.ptr` is non-null and valid.
+        unsafe { bindings::pci_disable_device(self.ptr) };
+    }
+
     /// iter PCI Resouces
     pub fn iter_resource(&self) -> impl Iterator<Item = Resource> + '_ {
         // SAFETY: By the type invariants, we know that `self.ptr` is non-null and valid.
@@ -323,10 +335,21 @@ impl Device {
         }
     }
 
+    /// Release selected PCI I/O and memory resources
+    pub fn release_selected_regions(&mut self, bars: i32) {
+        // SAFETY: By the type invariants, we know that `self.ptr` is non-null and valid.
+        unsafe { bindings::pci_release_selected_regions(self.ptr, bars) };
+    }
+
     /// Get address for accessing the device
     pub fn map_resource(&self, resource: &Resource, len: usize) -> Result<MappedResource> {
         MappedResource::try_new(resource.start, len)
     }
+
+    /// Get pointer of the device
+    pub unsafe fn ptr(&self) -> *mut bindings::pci_dev {
+        self.ptr
+    }
 }
 
 unsafe impl device::RawDevice for Device {
diff --git a/src_e1000/r4l_e1000_demo.rs b/src_e1000/r4l_e1000_demo.rs
index 7c235cf59..ac052b737 100644
--- a/src_e1000/r4l_e1000_demo.rs
+++ b/src_e1000/r4l_e1000_demo.rs
@@ -189,8 +189,20 @@ impl net::DeviceOperations for NetDevice {
         Ok(())
     }
 
-    fn stop(_dev: &net::Device, _data: &NetDevicePrvData) -> Result {
+    fn stop(dev: &net::Device, data: &NetDevicePrvData) -> Result {
         pr_info!("Rust for linux e1000 driver demo (net device stop)\n");
+
+        dev.netif_carrier_off();
+        dev.netif_stop_queue();
+        data.napi.disable();
+
+        data.e1000_hw_ops.e1000_reset_hw();
+        data._irq_handler
+            .store(core::ptr::null_mut(), core::sync::atomic::Ordering::Relaxed);
+
+        let _ = data.tx_ring.lock_irqdisable().take();
+        let _ = data.rx_ring.lock_irqdisable().take();
+
         Ok(())
     }
 
@@ -293,6 +305,8 @@ impl kernel::irq::Handler for E1000InterruptHandler {
 /// the private data for the adapter
 struct E1000DrvPrvData {
     _netdev_reg: net::Registration<NetDevice>,
+    pci_dev_ptr: AtomicPtr<bindings::pci_dev>,
+    bars: i32,
 }
 
 impl driver::DeviceRemoval for E1000DrvPrvData {
@@ -462,12 +476,28 @@ impl pci::Driver for E1000Drv {
             E1000DrvPrvData{
                 // Must hold this registration, or the device will be removed.
                 _netdev_reg: netdev_reg,
+                pci_dev_ptr: AtomicPtr::new(unsafe { dev.ptr() }),
+                bars,
             }
         )?)
     }
 
     fn remove(data: &Self::Data) {
         pr_info!("Rust for linux e1000 driver demo (remove)\n");
+
+        let mut netdev_reg = &data._netdev_reg;
+        let netdev = netdev_reg.dev_get();
+        netdev.netif_carrier_off();
+        netdev.netif_stop_queue();
+        drop(netdev);
+        drop(netdev_reg);
+
+        let pci_dev_ptr = data.pci_dev_ptr.load(core::sync::atomic::Ordering::Relaxed);
+        let mut pci_dev = unsafe { pci::Device::from_ptr(pci_dev_ptr) };
+
+        pci_dev.release_selected_regions(data.bars);
+        pci_dev.clear_master();
+        pci_dev.disable_device();
     }
 }
 struct E1000KernelMod {
@@ -488,5 +518,6 @@ impl kernel::Module for E1000KernelMod {
 impl Drop for E1000KernelMod {
     fn drop(&mut self) {
         pr_info!("Rust for linux e1000 driver demo (exit)\n");
+        drop(&self._dev);
     }
 }
```

---

### 作业5: 注册字符设备
#### 操作流程:
1. 修改内核配置:

<img src=./docs/images/char_dev_config.png width=80% />

2. 修改 linux/samples/rust/rust_chrdev.rs 中的 write 和 read 方法:
```rust
fn write(this: &Self,_file: &file::File,reader: &mut impl kernel::io_buffer::IoBufferReader,offset:u64,) -> Result<usize> {
    let mut buf = this.inner.lock();
    let offset = offset as usize;

    let buf_cap_left = buf.len().saturating_sub(offset);
    let len = buf_cap_left.min(reader.len());
    reader.read_slice(&mut buf[offset..offset+len])?;

    Ok(len)
}

fn read(this: &Self,_file: &file::File,writer: &mut impl kernel::io_buffer::IoBufferWriter,offset:u64,) -> Result<usize> {
    let buf = this.inner.lock();
    let offset = offset as usize;

    let buf_cap_left = buf.len().saturating_sub(offset);
    let len = buf_cap_left.min(writer.len());
    writer.write_slice(&buf[offset..offset+len])?;

    Ok(len)
}
```

3. 重新编译并启动内核:
```sh
make LLVM=/usr/lib/llvm14/bin/ -j$(nproc)
cd ../src_e1000
./build_image.sh
```


#### 实验效果:
<img src=./docs/images/char_dev_test.png width=80% />

---

## 项目小试验
#### 环境配置
##### 1. 创建 initramfs 镜像

<img src=./docs/images/initramfs.png width=80% />

##### 2. 支持 NFS

<img src=./docs/images/nfs_server.png width=80% />

##### 3. 支持 Telnet server

<img src=./docs/images/telnet_server.png width=60% />

#### 项目实战:

