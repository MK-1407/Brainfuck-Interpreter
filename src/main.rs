#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;
use vga_buffer::WRITER;
use vga_buffer::Writer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Kernel Panic!");
    println!("{}",_info);
    loop {}
}
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    *WRITER.lock() = Some(Writer::new());
    println!("Brainfuck Kernel");
    println!("VGA Textmode Enabled");
    println!("Integer test: {}", 42);
    loop {}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print(format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}
