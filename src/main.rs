#![no_std]
#![no_main]

mod vga_buffer;
use core::fmt::Write;
use vga_buffer::Writer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new();

    writeln!(writer, "Brainfuck Kernel").unwrap();
    writeln!(writer, "VGA Textmode Enabled").unwrap();

    loop {}
} 