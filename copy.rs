// Copies a BMP file
// Commented out lines from C code and replaced with a
// close representation in Rust

#![allow(non_snake_case)]

// #include <stdio.h>
// #include <stdlib.
use std::env;
use std::mem;
use std::process;
use std::fs::File;
use std::io::prelude::*;

//#include "bmp.
mod bmp;

// int main(int argc, char *argv[])
fn main() {
    // Args will be easier to used in a container
    // Using Vec just for the sake of ease
    let argv: Vec<String> = env::args().skip(1).collect();

    // ensure proper usage
    // Instead of argc, lets use the length of vec
    // Which should be 2
    if argv.len() != 2 {
        println!("Usage: copy infile outfile\n");
        process::exit(1);
    }

    // I guess I will go ahead and remember filenames
    // I probably would just use argv in this type of simple program though
    // remember filenames. 
    // char *infile = argv[1];
    // char *outfile = argv[2];
    let infile = &argv[0];
    let outfile = &argv[1];

    // open input file
    /////////////////////////////////////////////////
    // FILE *inptr = fopen(infile, "r");           //
    // if (inptr == NULL)                          //
    // {                                           //
    //     printf("Could not open %s.\n", infile); //
    //     return 2;                               //
    // }                                           //
    /////////////////////////////////////////////////
    let mut in_file = File::open(infile).expect("Failed to open file");


    // open output file
    ////////////////////////////////////////////////////
    // FILE *outptr = fopen(outfile, "w");            //
    // if (outptr == NULL)                            //
    // {                                              //
    //     fclose(inptr);                             //
    //     printf("Could not create %s.\n", outfile); //
    //     return 3;                                  //
    // }                                              //
    ////////////////////////////////////////////////////
    let mut out_file = File::create(outfile).expect("Failed to create file");

    /////////////////////////////////////////////////////
    // // read infile's BITMAPFILEHEADER               //
    // BITMAPFILEHEADER bf;                            //
    // fread(&bf, sizeof(BITMAPFILEHEADER), 1, inptr); //
    //                                                 //
    // // read infile's BITMAPINFOHEADER               //
    // BITMAPINFOHEADER bi;                            //
    // fread(&bi, sizeof(BITMAPINFOHEADER), 1, inptr); //
    /////////////////////////////////////////////////////
    // Rather than read in to structs this way
    // We will read into one struct and initialize the struct
    // There is a little fenagling though because of how Rust works
    // I chose to show one way of initializing the struct, and one
    // way where you don't have to. Just so you know
    // This does generate a warning for fileHead, but you can figure
    // out how to make that go away
    let copy_bmp = bmp::BMPImg {
        BFHeader: {
            let mut fileHead = bmp::BitmapFileHeader {
                bfType: 0,
                bfSize: 0,
                bfReserved1: 0,
                bfReserved2: 0,
                bfOffBits: 0,
            };
            let mut buf = [0u8; mem::size_of::<bmp::BitmapFileHeader>()];
            in_file.read_exact(&mut buf).expect("Failed to read File!");
            unsafe {
                fileHead = std::mem::transmute::<[u8; mem::size_of::<bmp::BitmapFileHeader>()], bmp::BitmapFileHeader>(buf);
            }
            fileHead // return fileHead to BFHeader
        },
        BFInfo: {
            let fileInfo: bmp::BitmapInfoHeader;
            let mut buf =  [0u8; mem::size_of::<bmp::BitmapInfoHeader>()];
            in_file.read_exact(&mut buf).expect("Failed to read file!");
            unsafe { fileInfo = std::mem::transmute::<[u8; mem::size_of::<bmp::BitmapInfoHeader>()], bmp::BitmapInfoHeader>(buf); }
            fileInfo
        },            
//        Pixels: Vec::new();
    };

    
    /////////////////////////////////////////////////////////////////////////
    // // ensure infile is (likely) a 24-bit uncompressed BMP 4.0          //
    // if (bf.bfType != 0x4d42 || bf.bfOffBits != 54 || bi.biSize != 40 || //
    //     bi.biBitCount != 24 || bi.biCompression != 0)                   //
    // {                                                                   //
    //     fclose(outptr);                                                 //
    //     fclose(inptr);                                                  //
    //     printf("Unsupported file format.\n");                           //
    //     return 4;                                                       //
    // }                                                                   //
    /////////////////////////////////////////////////////////////////////////
    if copy_bmp.BFHeader.bfType != 0x4d42 || copy_bmp.BFHeader.bfOffBits != 54 ||
        copy_bmp.BFInfo.biSize != 40 || copy_bmp.BFInfo.biBitCount != 24 ||
        copy_bmp.BFInfo.biCompression != 0 {
            println!("Unsupported file format.");
            process::exit(4);
        }

    ///////////////////////////////////////////////////////
    // // write outfile's BITMAPFILEHEADER               //
    // fwrite(&bf, sizeof(BITMAPFILEHEADER), 1, outptr); //
    ///////////////////////////////////////////////////////
    {
        let bytes: [u8; mem::size_of::<bmp::BitmapFileHeader>()] = unsafe {std::mem::transmute(copy_bmp.BFHeader) };
        out_file.write(&bytes).expect("Failed to write out FileHeader");
    }
    
    ///////////////////////////////////////////////////////
    // // write outfile's BITMAPINFOHEADER               //
    // fwrite(&bi, sizeof(BITMAPINFOHEADER), 1, outptr); //
    ///////////////////////////////////////////////////////
    {
        let bytes: [u8; mem::size_of::<bmp::BitmapInfoHeader>()] = unsafe {std::mem::transmute(copy_bmp.BFInfo) };
        out_file.write(&bytes).expect("Failed to write out InfoHeader");
    }
    
    ///////////////////////////////////////////////////////////////////
    // // determine padding for scanlines                            //
    // int padding = (4 - (bi.biWidth * sizeof(RGBTRIPLE)) % 4) % 4; //
    ///////////////////////////////////////////////////////////////////
    let padding = std::num::Wrapping((4 - (copy_bmp.BFInfo.biWidth * mem::size_of::<bmp::RGBTriple> as i32)) % 4 % 4).0 as i64;    
    
    /////////////////////////////////////////////////////////////////////
    // // iterate over infile's scanlines                              //
    // for (int i = 0, biHeight = abs(bi.biHeight); i < biHeight; i++) //
    // {                                                               //
    //     // iterate over pixels in scanline                          //
    //     for (int j = 0; j < bi.biWidth; j++)                        //
    //     {                                                           //
    //         // temporary storage                                    //
    //         RGBTRIPLE triple;                                       //
    //                                                                 //
    //         // read RGB triple from infile                          //
    //         fread(&triple, sizeof(RGBTRIPLE), 1, inptr);            //
    //                                                                 //
    //         // write RGB triple to outfile                          //
    //         fwrite(&triple, sizeof(RGBTRIPLE), 1, outptr);          //
    //     }                                                           //
    //                                                                 //
    //     // skip over padding, if any                                //
    //     fseek(inptr, padding, SEEK_CUR);                            //
    //                                                                 //
    //     // then add it back (to demonstrate how)                    //
    //     for (int k = 0; k < padding; k++)                           //
    //     {                                                           //
    //         fputc(0x00, outptr);                                    //
    //     }                                                           //
    // }                                                               //
    /////////////////////////////////////////////////////////////////////
    for _ in 0..copy_bmp.BFInfo.biHeight {
        for _ in 0..copy_bmp.BFInfo.biWidth {
            // Read in a pixel
            let mut bytes = [0u8; 3];
            in_file.read(&mut bytes).expect("Didn't work");
            let pixel = bmp::RGBTriple {
                rgbtBlue: bytes[0],
                rgbtGreen: bytes[1],
                rgbtRed: bytes[2],
            };
            // Now write it out
            out_file.write(&[pixel.rgbtBlue]).expect("Failed to write out a pixel");
            out_file.write(&[pixel.rgbtGreen]).expect("Failed to write out a pixel");
            out_file.write(&[pixel.rgbtRed]).expect("Failed to write out a pixel");
        }

        // Do the padding skip stuff
        in_file.seek(std::io::SeekFrom::Current(padding)).expect("Well that didn't work");

        for _ in 0..padding {
            out_file.write(&[0x00u8]).expect("Well that didn't work either");;
        }
    }

    //These last lines are just unnecessary
    //////////////////////
    // // close infile  //
    // fclose(inptr);   //
    //                  //
    // // close outfile //
    // fclose(outptr);  //
    //                  //
    // // success       //
    // return 0;        //
    //////////////////////
}
