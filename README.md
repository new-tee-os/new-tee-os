## Build for RISC-V Keystone

Besides the nightly Rust toolchain, the Rust target `riscv64gc-unknown-none-elf` is required for RISC-V Keystone.

```sh
rustup toolchain install nightly
rustup target add riscv64gc-unknown-none-elf
```

After installing the proper Rust toolchain, simply use Cargo to build the Keystone runtime:

```sh
cd keystone-rt
cargo build
```

The output will be placed at `target/riscv64gc-unknown-none-elf/debug/keystone-rt`.

## Running the Keystone runtime with QEMU

In order to test the Keystone runtime, you need to have a working Keystone environment. If you have Keystone QEMU installed, you should have these 2 files in the guest OS's home directory:

```
keystone-driver.ko
tests.ke
```

`tests.ke` is actually a self-extracting file that contains the host application, the Eyrie runtime, and the test suite eapps. Use the following command to extract files from `tests.ke`:

```sh
./tests.ke --target tests --noexec --keep
```

Now we should have the host application in the `tests` directory, which can load and execute our Keystone runtime. Transfer `keystone-rt` to `/root/` in the guest VM via SSH:

```sh
scp -P 8022 target/riscv64gc-unknown-none-elf/debug/keystone-rt root@localhost:/root/
```

And then run in the guest OS:

```sh
insmod keystone-driver.ko
./tests/test-runner tests/long-nop keystone-rt
```
