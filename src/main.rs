extern crate clap;
extern crate color_thief;
extern crate image;

use clap::{App, Arg};
use color_thief::ColorFormat;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod color;

const VERSION: &str = "0.1.0";

fn main() {
    // Add command line tool information
    let matches = App::new("Ranbo")
        .version(VERSION)
        .author("Jon Carr <jec@joncarr.xyz>")
        .about("A Rust pywal-like tool")
        .arg(
            Arg::with_name("Image")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("path to image to pull colors from"),
        )
        .get_matches();

    // Parse the arguments and store to variable
    let path = matches.value_of("Image").unwrap();

    // Open the image to be read by color_thief
    let img = image::open(path).unwrap();

    // color_thief::get_palette requires a vec!<u8> holding pixel data
    // palette_rgb is the vector holding the palette in rgb<x, y, z> format
    let img_pixels = img.to_rgb().into_vec();
    let palette_rgb = color_thief::get_palette(&img_pixels, ColorFormat::Rgb, 5, 17).unwrap();

    // Open the template files, read the contents to a buffer and write to a String
    let tmpl = File::open("templates/ranbo.xcolors").expect("Unable to open file");
    let mut buffer = BufReader::new(&tmpl);
    let mut contents = String::new();
    buffer
        .read_to_string(&mut contents)
        .expect("Unable to read buffer");

    let mut rgb_colors = Vec::<color::RGB>::new();
    for (i, color) in palette_rgb.into_iter().enumerate() {
        rgb_colors.push(color::RGB::new(color.r as f64, color.g as f64, color.b as f64));
        if i == 0 {
            rgb_colors[i].darken(80.0);
        }
        if i == 7 || i == 15 {
            rgb_colors[i].lighten(65.0);
        }
    }
    

    let mut hex_values = Vec::<String>::new();
    for val in &rgb_colors {
        hex_values.push(val.to_hex_string());
        println!("{}", val.to_hex_string());
    }

    //Open all preloaded template files to extractfilenames
    // and prepare for writing to the filesystem
    // if let Ok(entries) = fs::read_dir("templates") {
    //     for entry in entries {
    //         if let Ok(entry) = entry {
    //             // println!("{:?}", entry.file_name());
    //             // Do work writing output buffers and out to file
    //         }
    //     }
    // }
}
