#![allow(dead_code)]
#![allow(unused_imports)]
use volatile::Volatile;

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
struct Buffer {
    char_buf: [[TextCharacter; TEXT_BUF_WIDTH]; TEXT_BUF_HEIGHT],
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
    
                self.buffer.char_buf[buf_row][buf_col] = TextCharacter{
                    ascii_char: byte, // write the byte to the current x, y position in the buffer.
                    color_code, // set the color code to whatever was given in.
                };
                self.col_position += 1; // increment column positon.
            }
        }
    }
    /// Writes a string to the VGA buffer.
    pub fn write_string(&mut self, string: &str){
        for byte in string.bytes(){
            match byte {
                // Get the inclusive range of all printable characters in ascii.
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // If we find an invalid character, print ■
            }
        }
    }

    fn write_newline(&mut self){
        todo!("Newline WIP");
    }
}

pub fn print_test(string: &str){
    use core::fmt::Write;
    let mut writer = &mut Writer{
        col_position: 0,
        color_code: ColorCoding::new(Color::Black, Color::LightCyan),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
        // Make a mutable reference by dereferencing a mutable pointer to a Buffer.
    };
    Writer::write_string(writer, string);
    
}