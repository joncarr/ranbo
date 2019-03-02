extern crate clap;
extern crate color_thief;
extern crate dirs;
extern crate image;

use clap::{App, Arg};
use color_thief::ColorFormat;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufWriter};
use std::mem::drop;
use std::path::PathBuf;
use std::process;

mod color;

const VERSION: &str = "0.2.0";

fn main() {
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

    let split = path.split("/");

    let mut filename = "";
    for s in split {
        if s.contains("jpg") || s.contains("png") || s.contains("bmp") {
            filename = s;
        }
    }

    let split_filename: Vec<&str> = filename.split(".").into_iter().collect();
    let filename = split_filename[0];

    // Open the image to be read by color_thief
    let img = match image::open(path) {
        Ok(p) => p,
        Err(_) => {
            println!("Oh Snap! There was a problem opening that image.");
            process::exit(1);
        }
    };

    // color_thief::get_palette requires a vec!<u8> holding pixel data
    // palette_rgb is the vector holding the palette in rgb<r, g, b> format
    let img_pixels = img.to_rgb().into_vec();
    let palette_rgb =
        color_thief::get_palette(&img_pixels, ColorFormat::Rgb, 5, count + 1).unwrap();

    let mut rgb_colors = Vec::<color::RGB>::new();
    let mut hex_values = Vec::<String>::new();

    // println!();
    // println!("RGB Values");
    for val in &palette_rgb {
        rgb_colors.push(color::RGB::new(val.r, val.g, val.b));
        // println!("{}", val);
    }

    // println!();
    // println!("Hexadecimal Values");
    for val in &rgb_colors {
        hex_values.push(val.to_hex_string());
        // println!("{}", val.to_hex_string());
    }

    if matches.is_present("theme") {
        rgb_colors[0].darken(80);
        rgb_colors[7].lighten(65);
        rgb_colors[15].lighten(65);
    }

    let config_dir = dirs::config_dir().unwrap();

    let mut gimp_palette_path = PathBuf::new();
    gimp_palette_path.push(&config_dir.as_path());
    gimp_palette_path.push("GIMP/2.10/palettes/");
    gimp_palette_path.push(&filename);
    gimp_palette_path.set_extension("gpl");

    //Create GIMP Palette file and write it to GIMP palettes directory
    let gimp_gpl = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&gimp_palette_path)
        .unwrap();

    let mut writer = BufWriter::new(gimp_gpl);

    writer.write(b"GIMP Palette\n").unwrap();
    let fname = format!("Name: {}\n", filename);
    writer.write(fname.as_ref()).unwrap();
    writer.write(b"#\n").unwrap();

    for (i, c) in rgb_colors.into_iter().enumerate() {
        let color_string = format!("{}\t{}\t{}\t#{}\n", c.r, c.g, c.b, hex_values[i]);
        writer.write(color_string.as_ref()).unwrap();
    }

    drop(writer);

    //Copy the GIMP palette file just created to inkscape palettes directory
    let mut inkscape_palette = PathBuf::new();
    inkscape_palette.push(config_dir.as_path());
    inkscape_palette.push("inkscape/palettes/");
    inkscape_palette.push(filename);
    inkscape_palette.set_extension("gpl");

    let mut inkscape_gpl = File::create(inkscape_palette.as_path()).unwrap();
    let mut og_gpl = File::open(&gimp_palette_path).unwrap();
    io::copy(&mut og_gpl, &mut inkscape_gpl);

    println!("GIMP and Inkscape palettes successfully created.");
    println!("Palette Name: {}", filename);
    println!("If GIMP or Inkscape were open, restart them to import your new palette.");
}
