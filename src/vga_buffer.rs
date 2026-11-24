use core::fmt;

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;
pub const VGA_BUFFER_ADDRESS: usize = 0xb8000;

type Buffer = [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT];
#[derive(Copy, Clone)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color_code: u8,
}

pub struct Writer {
    row_position: usize,
    column_position: usize,
    color_code: u8,
    buffer: &'static mut Buffer,
}
impl Writer {
    pub fn new() -> Self {
        let buffer_ptr = VGA_BUFFER_ADDRESS as *mut Buffer;

        Self {
            column_position: 0,
            row_position: 0,
            color_code: 0x0a,
            buffer: unsafe { &mut *buffer_ptr },
        }
    }
    pub fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            self.buffer.copy_within(1..BUFFER_HEIGHT, 0);
            for column in 0..BUFFER_WIDTH {
                self.buffer[BUFFER_HEIGHT - 1][column] = ScreenChar {
                    character: b' ',
                    color_code: self.color_code,
                };
            }
        }
        self.column_position = 0;
    }
    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
        } else {
            self.buffer[self.row_position][self.column_position] = ScreenChar {
                character: byte,
                color_code: self.color_code,
            };

            self.column_position += 1;
            if self.column_position == BUFFER_WIDTH {
                self.new_line()
            }
        }
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    if let Some(writer) = WRITER.lock().as_mut() {
        writer.write_fmt(args).unwrap();
    }
}
use spin::Mutex;
pub static WRITER: Mutex<Option<Writer>> = Mutex::new(None);