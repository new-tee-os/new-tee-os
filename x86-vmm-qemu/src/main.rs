mod edge_call;

use std::path::{Path, PathBuf};

use async_std::process::Command;

pub async fn create_disk_images(kernel_binary_path: &Path) -> PathBuf {
    let bootloader_manifest_path = bootloader_locator::locate_bootloader("bootloader").unwrap();
    let kernel_manifest_path = locate_cargo_manifest::locate_manifest().unwrap();

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd
        .arg("--kernel-manifest")
        .arg(&kernel_manifest_path);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary_path);
    build_cmd
        .arg("--target-dir")
        .arg(kernel_manifest_path.parent().unwrap().join("target"));
    build_cmd
        .arg("--out-dir")
        .arg(kernel_binary_path.parent().unwrap());
    build_cmd.arg("--firmware").arg("bios");

    if !build_cmd.status().await.unwrap().success() {
        panic!("build failed");
    }

    let kernel_binary_name = kernel_binary_path.file_name().unwrap().to_str().unwrap();
    let disk_image = kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-bios-{}.img", kernel_binary_name));
    if !disk_image.exists() {
        panic!("disk image {} not found", disk_image.display());
    }

    disk_image
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let mut args = std::env::args().skip(1); // skip executable name

    // build BIOS boot image
    log::info!("Building BIOS boot image");
    let kernel_binary_path = {
        let path = PathBuf::from(args.next().unwrap());
        path.canonicalize().unwrap()
    };
    let disk_img = create_disk_images(&kernel_binary_path).await;

    // start edge call server
    let edge_call_server = edge_call::EdgeCallServer::new().await.unwrap();

    // run QEMU
    log::info!("Starting QEMU");
    let mut run_cmd = Command::new("qemu-system-x86_64");
    run_cmd
        .arg("-drive")
        .arg(format!("format=raw,file={}", disk_img.display()));

    tokio::select! {
        result = edge_call_server.listen() => {
            result.unwrap();
            log::info!("Edge call server closed, shutting down QEMU");
        }
        exit_status = run_cmd.status() => {
            let exit_status = exit_status.unwrap();
            if !exit_status.success() {
                log::warn!("QEMU exited with status {}", exit_status);
                std::process::exit(exit_status.code().unwrap_or(1));
            } else {
                log::info!("QEMU has shut down, closing edge call server");
            }
        }
    }
}
