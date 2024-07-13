#![no_std]
#![no_main]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn success() -> ! {
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn failed() {
    exit_qemu(QemuExitCode::Failed);
}

pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
