## 练习作业各阶段结果

### 作业 1

<img src=./docs/images/linux_compile_done.png width=100% />

### 作业 2

<img src=./docs/images/e1000_insmod.png width=100% />
<img src=./docs/images/e1000_ping.png width=100% />

#### 作业问题回答:
Q1、编译成内核模块，是在哪个文件中以哪条语句定义的？

A1: 编译成内核模块是在 **Kbuild** 中 **obj-m := r4l_e1000_demo.o** 这句定义的

Q2、该模块位于独立的文件夹内，却能编译成Linux内核模块，这叫做out-of-tree module，请分析它是如何与内核代码产生联系的？

A2: 在 Makefile 中使用 $(MAKE) -C $(KDIR) M=$$PWD 命令指定了内核和内核模块路径，它将引用内核模块相关的符号来构建树外模块。


### 作业 3

<img src=./docs/images/rust_helloworld_module.png width=100% />

### 作业 4

<img src=./docs/images/e1000_rmmod_op.png width=100% />
<img src=./docs/images/e1000_rmmod_ping.png width=100% />


### 作业 5

<img src=./docs/images/char_dev_test.png width=100% />

Q：作业5中的字符设备/dev/cicv是怎么创建的？它的设备号是多少？它是如何与我们写的字符设备驱动关联上的？
A: /dev/cicv 是通过 **mknod /dev/cicv c 248 0** 命令创建的，它的设备号是 0。设备在注册时需要指定设备号，两者匹配时就会关联上。

### 项目小试验

#### 环境:

- NFS: 

<img src=./docs/images/nfs_server.png width=100% />

- Telnet: 

<img src=./docs/images/telnet_server.png width=80% />

#### 实战:

<img src=./docs/images/rust_completion.png width=60% />

> 修改文件见 [Notes](Notes.md)