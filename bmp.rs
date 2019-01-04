// BMP-related data types based on Microsoft's own

// aliases for C/C++ primitive data types
// https://msdn.microsoft.com/en-us/library/cc230309.aspx
//typedef uint8_t  BYTE;
//typedef uint32_t DWORD;
//typedef int32_t  LONG;
//typedef uint16_t WORD;

#![allow(non_snake_case)]

// information about the type, size, and layout of a file
// https://msdn.microsoft.com/en-us/library/dd183374(v=vs.85).aspx
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct BitmapFileHeader {
    pub bfType: u16,
    pub bfSize: u32,
    pub bfReserved1: u16,
    pub bfReserved2: u16,
    pub bfOffBits: u32,
}

// information about the dimensions and color format
// https://msdn.microsoft.com/en-us/library/dd183376(v=vs.85).aspx
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct BitmapInfoHeader {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}

// relative intensities of red, green, and blue
// https://msdn.microsoft.com/en-us/library/dd162939(v=vs.85).aspx
#[derive(Debug, Copy, Clone)]
pub struct RGBTriple {
    pub rgbtBlue: u8,
    pub rgbtGreen: u8,
    pub rgbtRed: u8,
} 

// An additional struct to contain an entire bmp image
// This just seemed more logical to me
#[derive(Debug)]
pub struct BMPImg {
    pub BFHeader: BitmapFileHeader,
    pub BFInfo: BitmapInfoHeader,
//    Pixels: Vec<RGBTriple>,
}

