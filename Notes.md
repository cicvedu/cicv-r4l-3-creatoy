## 作业记录

> 因为用的 Archlinux 作为主机系统，很多工具版本都很新，所以出了一些问题...

### 环境配置问题
- 安装 bindgen 时出现错误：error: Usage of HTTP-based registries requires `-Z http-registry`  
改为使用下面的命令：
```
cargo +nightly install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen
```

- 配置 BusyBox 时出现无法找到 ncurses 错误  
修改 scripts/kconfig/lxdialog/check-lxdialog.sh 第 48 行，main() {} 前加上 int，改为 int main() {}

### 作业1: 编译 Linux 内核

#### 操作流程:
```sh
make x86_64_defconfig
make LLVM=1 menuconfig
```
在菜单中选中第一条 **General setup --->**，进入后翻到最后选中 **Rust support**。(带菜单的选项使用 **Enter** 进入，前面带 **[ ]** 的选项使用 Y 或空格键选中。更多操作注意菜单顶端的帮助信息)
> 更方便的方法是使用搜索功能后使用数字键快速跳转到菜单项
接着进行内核的编译：
```sh
make LLVM=1 -j$(nproc)
```

#### 问题记录

1. 编译时出现 anonymous union 无法解析的错误：
> 解决办法 https://github.com/rust-lang/rust-bindgen/pull/2316

2. 编译时出现 __vdso_sgx_enter_enclave 未定义错误:
> 解决办法 https://lkml.org/lkml/2022/11/8/1236

### 作业2: 对 Linux 内核进行一些配置

#### 操作流程:
