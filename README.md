# TEE OS

本项目的目标是实现一个面向可信执行环境（Trusted Execution Environments，简称TEEs）的OS，提供与Linux兼容的系统调用，并且支持多种不同的TEE平台 ，使得用户能够在任何TEE上运行普通的Linux应用程序，降低TEE的开发及使用门槛。

## 项目背景

可信执行环境（Trusted Execution Environments，简称TEEs）是一种基于硬件的安全技术，通过隔离机制和内存加密等手段保护TEE内的敏感的代码和数据，抵御来自TEE外的特权级的软硬件攻击（包括宿主机上的OS或者Hypervisor）。

TEE是机密计算（Confidential Computing）的主要技术基础之一，后者赋能了可信公有云、保护隐私的机器学习、多方安全计算、密钥管理等应用场景。

## 项目挑战

任何计算环境都可以受益于OS。一个兼容Linux ABI的TEE OS可以将普通Linux应用程序运行于TEE之内，让用户轻松享受到TEE的高安全保护。

然而，为TEE设计和实现OS有着与传统环境不同的挑战。

首当其中的一个技术挑战是克服TEE的多样性。近年来，不同体系结构下、具备不同特点的TEE开始大量涌现，如Intel SGX，AMD SEV，Intel TDX、ARM CCA，RISC-V Keystone等等。这些TEE主要可分为两种类型：

* 基于进程的TEE（如Intel SGX）。基于进程的TEE是运行在用户态，因此无法访问直接操纵硬件资源（如页表、中断向量表、I/O 设备等），必须通过与（不可信的）宿主机OS交互，间接地访问这些资源。同时，基于进程的TEE中往往也无法操纵TEE的页表，内存管理功能受限。此类TEE OS实际上是Library OS，与应用程序在同一个地址空间。
* 基于虚拟机的TEE（如AMD SEV和RISC-V Keystone）。以Keystone为例，Keystone TEE 的物理内存空间与宿主机OS相互隔离，但Keystone允许开发者使用自定义的内核实现，并且该内核能够使用RISC-V S模式下的全部功能，比如修改页表。因此，Keystone TEE实际上相当于一个与宿主操作系统相互隔离的一个虚拟机环境。除了能够通过宿主调用（host call）利用宿主机OS的功能之外，TEE 上的内核与普通的操作系统在本质上并无区别。

除了上面分类所展示的TEE在是否拥有特权方面的差异，TEE还存在其他重要差异，包括但不限于：体系结构、线程模型、安全保证、I/O接口等等。显然，TEE的这种多样性为OS设计和实现带来显著的、额外的复杂性。

另一大挑战是安全性。这既包括OS由于其复杂性而普遍存在的安全漏洞（最常见的原因是内存安全问题），也包括TEE特殊的安全问题（比如，文件I/O需要加密，以及访问不可信的宿主机OS的服务时需要小心被利用）。

## 总体思路

为了应对向上两个挑战，我们主要采取了下面两种手段：

1. 针对TEE多样性的挑战，我们采用了各种软件工程手段统一不同TEE之间的差异。软件工程手段包括但不限于硬件抽象层（HAL）的引入、编译时的配置、和选择性的链接等。
2. 针对TEE安全性的挑战，我们选择使用Rust语言，一个内存安全的编程语言，来开发我们的TEE OS。

值得一提的是，**据我们所知，本项目是业界首个兼容多种TEE的OS**。

## 设计与实现

我们计划同时支持三种不同的TEE实现，RISC-V Keystone、Intel SGX、和AMD SEV。目前，我们实现的进展情况是：

- RISC-V Keystone：可以启动和运行简单的ELF程序；
- Intel SGX：支持TEE的创建和OS的启动，接近完成运行简单程序；
- AMD SEV：方案设计中。

### 代码结构

#### RISC-V Keystone

- `linux-abi`，是本项目的主体，提供进程管理，内存管理等 OS 功能，以及对 Linux ABI 的支持。所有兼容层最终都会将系统调用转发到该 crate 上。
- `keystone-hal`：是本项目针对 Keystone 的兼容层 / 硬件抽象层（HAL），提供实现 Linux ABI 所需的基础架构，如对边缘调用（edge call）和对页表操作的支持等。
- `keystone-cfg`：提供与对应平台相关的配置项，如内核内存基地址、边缘内存大小等。
- `kmalloc` / `elf-loader`：一些可替换的组件，提供特定的功能，如内存分配器，ELF 加载器等。

#### Intel SGX

- `sgxrunner` ，用于加载我们的操作系统，提供enclave创建、初始化及退出、销毁以及外部一些系统调用等功能。
- `enclave` ，是我们操作体统的内核部分，主要包含
  - `linux-abi` ，提供了相应的系统调用实现。
  - `sgx_rt` ，编译入口点，在其中通过 `Cargo.toml` 引用 `linux-abi` 并开启`sgx`平台的feature。
  - `elfloader`，可替换的组件，用于提供特定的功能，`elfloader`负责提供ELF加载器。

### 兼容多TEE

本项目中采用了以下措施，以实现在多平台支持和软件架构之间的平衡。

- 在 `linux-abi` crate 中，将会自动根据编译时选择的 features 来自动导入对应平台的 HAL，并重命名为 `crate::hal` 以供平台无关的代码使用。这很类似于 Rust 实现 `std` crate 的机制：不同平台提供对应的 `imp` 库，封装在 `sys` 模块中，在编译时按需导入。这要求不同平台的 HAL 所提供的接口必须是相同的，否则在某些平台上将无法通过编译。
- 我们对于不同的平台分别提供对应的编译入口点（即类型为「可执行文件」的 crate），如 `keystone-rt`。在这些入口点中，通过 `Cargo.toml` 引用 `linux-abi` 并开启对应平台的 feature。
- 此外，我们考虑使用 Rust 的高抽象性（闭包，trait）结合 Linux 中现有的一些措施（如 `vm_operations`）来实现在底层的硬件操作上抽象出高层且安全的 API。

## 编译和测试

### RISC-V Keystone

本项目需要用到 Rust 工具链及 Keystone 套件（可使用 Docker 镜像，或者从 [此处](https://github.com/new-tee-os/new-tee-os.github.io/releases/download/attachments/keystone-20210423.tar.gz) 下载）。首先安装 Rust nightly 版工具链，并启动 Keystone 套件中的 QEMU：

```
rustup toolchain add nightly

keystone/qemu/riscv64-softmmu/qemu-system-riscv64 -m 2G -nographic \
    -machine virt -bios keystone/build/bootrom.build/bootrom.bin \
    -kernel keystone/build/sm.build/platform/generic/firmware/fw_payload.elf \
    -append "console=ttyS0 ro root=/dev/vda" \
    -drive file=keystone/build/buildroot.build/images/rootfs.ext2,format=raw,id=hd0 \
    -device virtio-blk-device,drive=hd0 \
    -netdev user,id=net0,net=192.168.100.1/24,dhcpstart=192.168.100.128,hostfwd=tcp::8022-:22 \
    -device virtio-net-device,netdev=net0 -device virtio-rng-pci -smp 1
```

然后，编译 Keystone runtime，即 enclave 中的操作系统部分：

```
cd keystone-rt
# 执行构建
cargo build --release
# 将构建好的 ELF 文件转换成平坦二进制文件
riscv64-unknown-elf-objcopy -O binary target/riscv64gc-unknown-none-elf/release/keystone-rt keystone-rt.bin
# 将二进制文件推送到 QEMU VM 上
scp -P 8022 keystone.bin root@localhost:/root/
```

编译宿主程序（host application），即运行在 host OS 上的 enclave 加载器：

```
cd ..
cd keystone-rt-runner
# 执行构建
cargo build --release
# 将构建好的 ELF 文件推送到 QEMU VM 上
scp -P 8022 target/riscv64gc-unknown-linux-gnu/release/keystone-rt-runner root@localhost:/root/
```

构建用于测试的用户程序：

```
cd riscv-hello-world
cargo build --release
scp -P 8022 target/riscv64gc-unknown-none-elf/release/riscv-hello-world root@localhost:/root/keystone-init
```

然后，即可在 QEMU VM 上启动 enclave 进行测试：

```
# insmod keystone-driver.ko
[  136.863119] keystone_driver: loading out-of-tree module taints kernel.
[  136.879536] keystone_enclave: keystone enclave v1.0.0
# ./keystone-rt-runner
Base: 0xB8100000
Krnl: 0xB8104000
User: 0xB8149000
Free: 0xB8149000
End:  0xB8200000
UTM:  0xB8B26000
-------------------------
[DEBUG] It did not crash!
[DEBUG] ELF loader: mapping (0xffffffff0004e000 + 0x1000) -> 0x400000
[DEBUG] Allocated 0xFFFFFFFF0004F000 for page table
[DEBUG] Allocated 0xFFFFFFFF00050000 for page table
[DEBUG] ELF loader: mapping (0xffffffff00051000 + 0x1000) -> 0x401000
[DEBUG] Page fault at address 0x402FF8
Hello, world!
[DEBUG] U-mode program exited with status 0
Enclave exited with status 0
```

### Intel SGX

本项目在 `ubuntu20.04` 操作系统上调试、运行，其它的平台上暂时还没有尝试。

1. 首先配置 `Intel SGX` ，根据以下教程可以分别安装
   - [linux-sgx-driver](https://github.com/intel/linux-sgx-driver)
   - [linux-sgx](https://github.com/intel/linux-sgx)（包括 `SGX SDK` 以及 `SGX PSW` ）
   - 注：如果您的 `CPU` 或者 `BIOS` 并不支持 `Intel SGX` ，那么只需安装 `SGX SDK` 即可。
2. 由于本项目使用了 `Rust SGX SDK` ，因此您还需要将该项目克隆到本地。
   - [incubator-teaclave-sgx-sdk](https://github.com/apache/incubator-teaclave-sgx-sdk)
3. 将 `new-tee-os-sgx` 拷贝到目录 `incubator-teaclave-sgx-sdk/samplecode` 下，然后 `make` 编译。
4. 在 `bin` 目录下就会有可执行文件 `app`。

## 其他困难及解决方法

### RISC-V Keystone

类似于 RISC-V 体系结构，Keystone 是一种学术性的 RISC-V TEE 实现。相对于商业化的 TEE 来说，Keystone 的发展较缓慢，其 1.0 版本是今年 2 月 22 日才发布的。因此，其官方文档也非常不完整，大多数页面仍然是 TODO 状态。这就给我们了解 Keystone 的工作机制造成了困难。

幸运的是，Keystone 的实现是完全开源的，并且其架构设计和代码风格都相对清晰。举例来说，Keystone 对其代码进行了模块分割，每个模块位于一个 GitHub repo 下，不同模块之间的交叉引用几乎为零。这使得定位问题根源非常方便，当遇到问题时，几乎能够 100% 确定引发问题的是哪个 Keystone 模块，再结合源代码搜索功能即可方便地定位出问题根源。这为我们节省下了大量时间精力。

在开发过程中，我们确实遇到了一些与 Keystone 内部机制有关的问题，如：内核的启动页表无法通过安全检查；Keystone SM 一直返回「参数错误」等。Keystone 的日志机制很不完善，并且这些错误在 Keystone 的官方文档中均没有详细说明。最终我们是通过阅读 Keystone 对应的源代码，或者对 Keystone 的核心代码进行调试，才定位到了错误的具体原因。

因此，我们认为 Keystone 对于我们的项目最大的优势在于：不同于闭源的 TEE 实现，Keystone 的实现是完全与 Linux，QEMU，gdb 等开源生态系统兼容的，因此所有与 QEMU，与 Linux 有关的调试技巧都可以直接在 Keystone 上使用。这使得通过调试来定位错误比商用 TEE 要简单得多。虽然 Keystone 这样的学术性、开源的 TEE 的生态系统相对于商用的 TEE 来说要逊色一些，但由于其开源的特性，对其进行扩展和除错都是相对更简单的。这也是我们选择将 Keystone 作为首选 target 的原因之一。

## 项目指导老师

* 田洪亮，目前就职于蚂蚁集团
* Yanyan Shen, 目前就职于Cog Systems
* 申文博，目前为浙江大学研究员

