## RISC-V Keystone 编译与测试流程

本项目需要用到 Rust 工具链及 Keystone 套件（可使用 Docker 镜像，或者从 [此处](https://github.com/new-tee-os/new-tee-os.github.io/releases/download/attachments/keystone-20210423.tar.gz) 下载）。首先安装 Rust nightly 版工具链，并启动 Keystone 套件中的 QEMU：

```sh
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

```sh
cd keystone-rt
# 执行构建
cargo build --release
# 将构建好的 ELF 文件转换成平坦二进制文件
riscv64-unknown-elf-objcopy -O binary target/riscv64gc-unknown-none-elf/release/keystone-rt keystone-rt.bin
# 将二进制文件推送到 QEMU VM 上
scp -P 8022 keystone.bin root@localhost:/root/
```

编译宿主程序（host application），即运行在 host OS 上的 enclave 加载器：

```sh
cd ..
cd keystone-rt-runner
# 执行构建
cargo build --release
# 将构建好的 ELF 文件推送到 QEMU VM 上
scp -P 8022 target/riscv64gc-unknown-linux-gnu/release/keystone-rt-runner root@localhost:/root/
```

构建用于测试的用户程序：

```sh
cd riscv-hello-world
cargo build --release
scp -P 8022 target/riscv64gc-unknown-none-elf/release/riscv-hello-world root@localhost:/root/keystone-init
```

然后，即可在 QEMU VM 上启动 enclave 进行测试：

```sh
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
