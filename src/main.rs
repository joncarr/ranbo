use clap::{App, Arg, ArgMatches};
use color::RGB;
use color_thief::{self, ColorFormat};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::{path::Path, process};

mod color;

fn main() {
    let version = String::from("0.3.0");

    let matches = new_app(&version);

    // Parse the arguments and store to variable
    let path = match matches.value_of("image") {
        Some(path) => path,
        None => {
            println!("No path to the image was provided.");
            process::exit(1);
        }
    };

    let color_count = match matches.value_of("count") {
        Some(count) => count,
        None => "10",
    };

    let mut count = match color_count.parse::<u8>() {
        Ok(count) => count,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    if count < 4 || count > 20 {
        println!("Color count should be between 4 and 20");
        process::exit(1);
    } else if count == 4 || count == 5 || count == 6 {
        count -= 1;
    }

    let path = Path::new(path);
    let ext = match path.extension() {
        Some(extension) => extension,
        None => {
            println!("Image must have a valid file extension (JPG, PNG, or BMP).");
            process::exit(1);
        }
    };

    let filename = path.file_stem().unwrap();

    if ext.is_empty() {
        println!("Image requires a valid file extension to be an image");
        process::exit(1);
    }

    // Open the image to be read by color_thief
    let img = image::open(path).expect("Error opening image");
    let img = img.to_rgb8().into_vec();

    let palette = match color_thief::get_palette(&img, ColorFormat::Rgb, 5, count) {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    let mut rgb_colors = Vec::<color::RGB>::new();
    let mut hex_colors = Vec::<String>::new();

    for color in palette {
        rgb_colors.push(color::RGB::new(color.r, color.g, color.b))
    }

    for color in &rgb_colors {
        hex_colors.push(color.to_hex_string());
    }

    let config_dir = match dirs::config_dir() {
        Some(path) => path,
        None => {
            println!("Unable to locate configuration directory.");
            process::exit(1);
        }
    };

    let gimp_palette_path = construct_gimp_palette_path(&config_dir, &filename.to_str().unwrap());

    write_gimp_palette(
        &gimp_palette_path,
        filename.to_str().unwrap(),
        rgb_colors,
        hex_colors,
    );

    copy_gimp_palette_to_inkscape(config_dir, filename.to_str().unwrap(), &gimp_palette_path);

    println!("GIMP and Inkscape palettes successfully created.");
    println!("Palette Name: {}", filename.to_str().unwrap());
    println!("If GIMP or Inkscape were open, restart them to import your new palette.");
}

fn construct_gimp_palette_path(config_dir: &PathBuf, filename: &str) -> PathBuf {
    let mut gimp_palette_path = PathBuf::new();
    gimp_palette_path.push(&config_dir.as_path());
    gimp_palette_path.push("GIMP/2.10/palettes/");
    gimp_palette_path.push(&filename);
    gimp_palette_path.set_extension("gpl");
    gimp_palette_path
}

fn new_app<'a>(version: &'a String) -> ArgMatches<'a> {
    App::new("Ranbo")
        .version(version.as_str())
        .author("Jon Carr <jecarr33@gmail.com>")
        .about("Generate palettes for GIMP and Inkscape from an image at the command line.")
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .required(true)
                .takes_value(true)
                .help("path to image to pull colors from"),
        )
        .arg(
            Arg::with_name("count").short("c").takes_value(true).help(
                "Number of colors you want on your palette, between 4 and 20 (defaults to 5).",
            ),
        )
        .get_matches()
}

fn write_gimp_palette(
    gimp_palette_path: &PathBuf,
    filename: &str,
    rgb_colors: Vec<RGB>,
    hex_colors: Vec<String>,
) {
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
        let color_string = format!("{}\t{}\t{}\t#{}\n", c.r, c.g, c.b, hex_colors[i]);
        writer.write(color_string.as_ref()).unwrap();
    }
    writer.flush().expect("Writer flush failed.");
}

fn copy_gimp_palette_to_inkscape(config_dir: PathBuf, filename: &str, gimp_palette_path: &PathBuf) {
    //Copy the GIMP palette file just created to inkscape palettes directory
    let mut inkscape_palette = PathBuf::new();
    inkscape_palette.push(config_dir.as_path());
    inkscape_palette.push("inkscape/palettes/");
    inkscape_palette.push(filename);
    inkscape_palette.set_extension("gpl");

    let mut inkscape_gpl = File::create(inkscape_palette.as_path()).unwrap();
    let mut og_gpl = File::open(&gimp_palette_path).unwrap();
    io::copy(&mut og_gpl, &mut inkscape_gpl).expect("Unable to generate inkscape palette.");
}
