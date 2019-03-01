extern crate clap;
extern crate color_thief;
extern crate image;

use clap::{App, Arg};
use color_thief::ColorFormat;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

mod color;

const VERSION: &str = "0.1.0";

fn main() {
    // Add command line tool information
    let matches =
        App::new("Ranbo")
            .version(VERSION)
            .author("Jon Carr <jec@joncarr.xyz>")
            .about("A tool to generate color palettes from an image. The smallest color palette that can be created is 4 colors, while the biggest is 20 colors")
            .arg(
                Arg::with_name("image")
                    .short("i")
                    .long("image")
                    .required(true)
                    .takes_value(true)
                    .help("path to image to pull colors from"),
            )
            .arg(Arg::with_name("count").short("c").takes_value(true).help(
                "Number of colors you want on your palette, between 4 and 20 (defaults to 5).  Do not pass '-c' and '-t' together.",
            ))
            .arg(
                Arg::with_name("theme")
                    .short("t")
                    .long("theme")
                    .required(false)
                    .takes_value(false)
                    .help("Pass this flag to generate palette as a theme with theme templates. The color count will default to 16 to satisfy Base16 themes. So you may omit '-c' when passing the '-t' flag."),
            )
            .get_matches();

    // Parse the arguments and store to variable
    let path = matches.value_of("image").unwrap();
    let color_count = match matches.value_of("count") {
        Some(count) => count,
        None => "5",
    };

    let mut count = color_count.parse::<u8>().unwrap();

    if matches.is_present("theme") {
        count = 16;
    }

    if count < 4 || count > 20 {
        println!("Color count should be between 4 and 20");
        process::exit(1);
    } else if count == 4 || count == 5 || count == 6 {
        count -= 1;
    }

    // Open the image to be read by color_thief
    let img = match image::open(path) {
        Ok(p) => p,
        Err(_) => {
            println!("Oh Snap! There was a problem opening that image.");
            process::exit(1);
        }
    };

    // if img == "" {
    //     println!("Oh Snap! There was a problem opening that image.");
    //     process::exit(1);
    // }

    // color_thief::get_palette requires a vec!<u8> holding pixel data
    // palette_rgb is the vector holding the palette in rgb<x, y, z> format
    let img_pixels = img.to_rgb().into_vec();
    let palette_rgb =
        color_thief::get_palette(&img_pixels, ColorFormat::Rgb, 5, count + 1).unwrap();

    let mut rgb_colors = Vec::<color::RGB>::new();
    let mut hex_values = Vec::<String>::new();

    println!();
    println!("RGB Values");
    for val in &palette_rgb {
        rgb_colors.push(color::RGB::new(val.r, val.g, val.b));
        println!("{}", val);
    }

    println!();
    println!("Hexadecimal Values");
    for val in &rgb_colors {
        hex_values.push(val.to_hex_string());
        println!("{}", val.to_hex_string());
    }

    if matches.is_present("theme") {
        rgb_colors[0].darken(80);
        rgb_colors[7].lighten(65);
        rgb_colors[15].lighten(65);

        // Open the template files, read the contents to a buffer and write to a String
        // let tmpl = File::open("templates/ranbo.xcolors").expect("Unable to open file");
        // let mut buffer = BufReader::new(&tmpl);
        // let mut contents = String::new();
        // buffer
        //     .read_to_string(&mut contents)
        //     .expect("Unable to read buffer");

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
}
