## 作业记录

[作业内容](https://docs.qq.com/doc/DSk5xTHRJY1FZVUdK)

> 因为我日常使用的是 Archlinux，这里没有使用 docker 而是直接使用主机作为开发环境。由于很多工具之前都已经安装好了，这里就没有记录。另外 Archlinux 上很多工具和库版本都很新，所以作业操作过程中出了不少问题...
> 
> <div align="center"> <img src=./docs/images/host_env.png width=80% /> </div>


### 环境配置问题
#### 1. 安装 bindgen 时出现错误：error: Usage of HTTP-based registries requires `-Z http-registry`  
改为使用下面的命令：
```
cargo +nightly install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen
```

环境配置完成后运行 make LLVM=1 rustavailable

<img src=./docs/images/linux_rust_available.png width=60% />


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

<img src=./docs/images/linux_compile_done.png width=60% />

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

#### 操作流程:
驱动编译:
```sh
cd src_e1000
make LLVM=1
```

<img src=./docs/images/e1000_compile.png width=80% />

启动内核:
```sh
./build_image.sh
```

<img src=./docs/images/kernel_launch.png width=80% />

修改内核配置，禁用 e1000 网卡驱动:
```
Device Drivers --->
    [*] Network device support --->
        [*] Ethernet driver support --->
            <> Intel devices, Intel(R) PRO/1000 Gigabit Ethernet support
```

重新编译内核后再次启动内核并加载驱动模块:
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

 