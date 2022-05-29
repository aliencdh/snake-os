use volatile::Volatile;
use core::fmt;
use spin::Mutex;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga_buffer::_err_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _err_print(args: fmt::Arguments) {
    use core::fmt::Write;
    ERR_WRITER.lock().write_fmt(args).unwrap();
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buf: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

lazy_static! {
    pub static ref ERR_WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightRed, Color::Black),
        buf: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buf: &'static mut Buffer,
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUF_WIDTH {
                    self.new_line();
                }

                let row = BUF_HEIGHT - 1;
                let col = self.column_position;

                self.buf.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUF_HEIGHT {
            for col in 0..BUF_WIDTH {
                let character = self.buf.chars[row][col].read();
                self.buf.chars[row - 1][col].write(character);
            }
        }

        self.clear_row(BUF_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUF_WIDTH {
            self.buf.chars[row][col].write(blank);
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUF_HEIGHT: usize = 25;
const BUF_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUF_WIDTH]; BUF_HEIGHT],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_println_many() {
        for _ in 0..500 {
            println!("test_println_many");
        }
    }

    #[test_case]
    fn test_println_out() {
        let input = "test_println_out";
        println!("{}", input);
        for (i, c) in input.chars().enumerate() {
            let result_char = WRITER.lock().buf.chars[BUF_HEIGHT - 2][i].read();
            assert_eq!(result_char.ascii_char as char, c);
        }
    }
}
