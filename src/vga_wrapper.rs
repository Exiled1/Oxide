#![allow(dead_code)]
#![allow(unused_imports)]

// use core::{cell::UnsafeCell, ptr::{write_volatile, read_volatile}};
use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // Specify that the type representation is the representation of its only field. Since ColorCoding(u8) might not behave the same as u8.
struct ColorCoding(u8);
// Create a new u4 + u4 color code.
impl ColorCoding {
    fn new(background: Color, foreground: Color) -> ColorCoding{
        ColorCoding((background as u8) << 4 | (foreground as u8))
    }
    // 0 0 0 0 0 0 0 0
    // 1 1 1 1 0 0 0 0
    //         1 1 1 1
    // 0 0 0 0 0 0 0 0    
}   

// Width = 80;
// Height = 25;
// Text buffer ----------------------------

#[repr(C)] // Represented as a C struct to maintain struct layout. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TextCharacter { // C struct.
    ascii_char: u8,
    color_code: ColorCoding,
}


const TEXT_BUF_HEIGHT: usize = 25;
const TEXT_BUF_WIDTH: usize = 80;

#[repr(transparent)] // Should keep the same memory layout as the char_buffer itself. There should be no offset.
struct Buffer{
    char_buf: [[Volatile<TextCharacter>; TEXT_BUF_WIDTH]; TEXT_BUF_HEIGHT],
    // Create a 2d text buffer array for our buffer Height and width.
}

// --- Text Writer ----

pub struct Writer {
    col_position: usize,         // The position in the previous row.
    color_code: ColorCoding,     // Current bg/fg colors.
    buffer: &'static mut Buffer, // We only have one of these dammit, it's also a reference to the VGA buffer we have.
}

/**
 * Implementation for our writer, this should write bytes to the buffer, and when we find a newline, make a new line.
 * fn write_byte
*/

impl Writer {
    pub fn write_byte(&mut self, byte: u8){ // Take a u8 byte and write it to the buffer.
        match byte{
            b'\n' => self.write_newline(),
            _ => { 
                // Anything else, check for the col position if it's exceeded the buffer width.
                if self.col_position >= TEXT_BUF_WIDTH {
                    self.write_newline();
                }
                // set x, y position in buffer.
                let buf_row = TEXT_BUF_HEIGHT - 1; // We have a buffer size of 25, go up to 24, since we need the 25th spot to write a newline.
                let buf_col = self.col_position;
    
                let color_code = self.color_code; // Set our color to whatever was passed.
                
                self.buffer.char_buf[buf_row][buf_col].write(TextCharacter{
                    ascii_char: byte, // write the byte to the current x, y position in the buffer.
                    color_code, // set the color code to whatever was given in.
                });
                // Has the ability to overwrite text position.
                
                self.col_position += 1; // increment column positon.
            }
        }
    }
    /// Writes a string to the VGA buffer by bytes.
    pub fn write_string(&mut self, string: &str){
        for byte in string.bytes(){
            match byte {
                // Get the inclusive range of all printable characters in ascii.
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // If we find an invalid character, print ■
            }
        }
    }
    /// When this is called, go through and move all of the text up one line.
    fn write_newline(&mut self){
        
        for cur_row in 1..TEXT_BUF_HEIGHT{ // Non inclusive write.
            for cur_col in 0..TEXT_BUF_WIDTH {
                let new_char = self.buffer.char_buf[cur_row][cur_col].read();
                self.buffer.char_buf[cur_row - 1][cur_col].write(new_char); 
                // Write the old bytes one row up at a time. Kind of like dial up.
            }
        }
        self.clear_row(TEXT_BUF_HEIGHT - 1);
        self.col_position = 0;
    }

    /// Clear the row at the specified position.
    fn clear_row(&mut self, row_num: usize){
        let clear_char = TextCharacter{
            ascii_char: 0x20, // ' ', space.
            color_code: self.color_code,

        };
        for column in 0..TEXT_BUF_WIDTH {
            // For the width of the buffer, write a space for clearing.
            self.buffer.char_buf[row_num][column].write(clear_char);
        }
    }
}
// Create an implementation for formatted writing.
impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_string(string);
        Ok(())
    }
}

// Provide a standard way of writing to the VGA buffer, WIP.
lazy_static!{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_position: 0,
        color_code: ColorCoding::new(Color::Black, Color::Cyan),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
    });
}

/**
    These macros were ripped from the std library for the macros of the print functions
*/
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_wrapper::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    
    WRITER.lock().write_fmt(args).unwrap();
}