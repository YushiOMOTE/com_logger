use std::{io::Read, path::PathBuf, process::Stdio, time::Duration};
use wait_timeout::ChildExt;

#[macro_export]
macro_rules! test_kernel {
    ($test_name:expr, $mode:expr) => {
        $crate::test_kernel_internal(
            env!(concat!("CARGO_BIN_FILE_KERNEL_", $test_name)),
            env!("CARGO_TARGET_TMPDIR"),
            $mode,
        )
    };
}

pub enum Mode {
    Uefi,
    Bios,
}

pub fn test_kernel_internal(kernel_path: &str, tmp_dir: &str, mode: Mode) -> Vec<String> {
    let out_dir = PathBuf::from(tmp_dir);

    println!("Found test kernels: {}", kernel_path);

    let kernel = PathBuf::from(kernel_path);

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    match mode {
        Mode::Uefi => {
            // create an UEFI disk image (optional)
            let uefi_path = out_dir.join("uefi.img");
            bootloader::UefiBoot::new(&kernel)
                .create_disk_image(&uefi_path)
                .unwrap();

            cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
            cmd.arg("-drive")
                .arg(format!("format=raw,file={}", uefi_path.display()));
        }
        Mode::Bios => {
            // create a BIOS disk image
            let bios_path = out_dir.join("bios.img");
            bootloader::BiosBoot::new(&kernel)
                .create_disk_image(&bios_path)
                .unwrap();

            cmd.arg("-drive")
                .arg(format!("format=raw,file={}", bios_path.display()));
        }
    }
    cmd.arg("-device")
        .arg("isa-debug-exit,iobase=0xf4,iosize=0x04");
    cmd.arg("-serial").arg("stdio");
    cmd.arg("-display").arg("none");
    cmd.stdout(Stdio::piped());
    let mut child = cmd.spawn().unwrap();

    match child.wait_timeout(Duration::from_secs(30)).unwrap() {
        Some(status) => assert_eq!(status.code(), Some(33)),
        None => panic!("Test timed out"),
    }

    println!("Finish");

    let stdout = child.stdout.as_mut().unwrap();
    let mut output = String::default();

    stdout.read_to_string(&mut output).unwrap();

    println!("Finish: {}", output);

    let mut vec: Vec<_> = output
        .trim_end_matches('\n')
        .split('\n')
        .map(|s| s.into())
        .collect();
    vec.reverse();
    vec
}
