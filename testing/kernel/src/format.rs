#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use log::info;

entry_point!(kernel_main);

fn kernel_main(_info: &'static mut BootInfo) -> ! {
    com_logger::builder()
        .formatter(|buf, record| writeln!(buf, "**** {} ****", record.args()))
        .setup();

    info!("Hello world!");

    kernel::success()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kernel::panic(info)
}
