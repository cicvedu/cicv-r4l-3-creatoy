## 作业记录

[作业内容](https://docs.qq.com/doc/DSk5xTHRJY1FZVUdK)

> 因为我日常使用的是 Archlinux，这里没有使用 docker 而是直接使用主机作为开发环境。由于很多工具之前都已经安装好了，这里就没有记录。另外 Archlinux 上很多工具和库版本都很新，所以作业操作过程中出了不少问题...
> 
> <div align="center"> <img src=./docs/images/host_env.png width=80% /> </div>


### 环境配置问题
#### 安装 bindgen 时出现错误：error: Usage of HTTP-based registries requires `-Z http-registry`  
改为使用下面的命令：
```
cargo +nightly install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen
```

环境配置完成后运行 make LLVM=1 rustavailable

<img src=./docs/images/linux_rust_available.png width=60% />


#### 配置 BusyBox 时出现无法找到 ncurses 错误  
修改 scripts/kconfig/lxdialog/check-lxdialog.sh 第 48 行，main() {} 前加上 int，改为 int main() {}

配置完成后:

<img src=./docs/images/busybox_config.png width=80% />

#### 编译时出现 TCA_CBQ_MAX TCA_CBQ_RATE 等未定义错误

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

1. 编译时出现 anonymous union 无法解析的错误：
> 解决办法 https://github.com/rust-lang/rust-bindgen/pull/2316

2. 编译时出现 __vdso_sgx_enter_enclave 未定义错误:
> 解决办法 https://lkml.org/lkml/2022/11/8/1236

---

### 作业2: 对 Linux 内核进行一些配置

#### 操作流程:
驱动编译:

<img src=./docs/images/e1000_compile.png width=80% />