// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");


fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let value = args.remove(0);
            let value:f32 = value.parse().expect("Failed to parse a number");
            blur(infile, outfile, value);
        },

        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let value = args.remove(0);
            let value = value.parse().expect("Failed to parse a number");
            brighten(infile, outfile, value);
        },

        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x = args.remove(0);
            let x: u32 = x.parse().expect("Failed to parse a number");
            let y = args.remove(0);
            let y: u32 = y.parse().expect("Failed to parse a number");
            let width = args.remove(0);
            let width: u32 = width.parse().expect("Failed to parse a number");
            let height = args.remove(0);
            let height: u32 = height.parse().expect("Failed to parse a number");
            crop(infile, outfile, x, y, width, height);
        },

        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let value = args.remove(0);
            let rotvalue = match value.as_str()
            {
                "90" => Rotation::R90,
                "180" => Rotation::R180,
                "270" => Rotation::R270,
                _ => { print_usage_and_exit(); panic!("aaaa");}
            };
            rotate(infile, outfile, rotvalue);
        },

        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            invert(infile, outfile);
        },

        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale(infile, outfile);
        },

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        "generate" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            let value = args.remove(0);
            let value : Vec<u8> = value.trim().split(',')
                .map(|s| s.parse().expect("Failed to parse a number"))
                .collect();
            match value.len() == 3
            {
                true => generate(outfile, value),
                false => print_usage_and_exit()
            }
            ;
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        },
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("brighten INFILE OUTFILE VALUE");
    println!("brighten INFILE OUTFILE X Y WIDTH HEIGHT");
    println!("rotate INFILE OUTFILE 90|180|270");
    println!("invert INFILE OUTFILE");
    println!("grayscale INFILE OUTFILE");
    println!("fractal OUTFILE");
    println!("generate OUTFILE R,G,B");

    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, value: f32) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(value);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, value: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.brighten(value);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.crop(x, y, width, height);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

enum Rotation
{
    R90,
    R180,
    R270,
}

fn rotate(infile: String, outfile: String, rotation: Rotation) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = match rotation
    {
        Rotation::R90 => img.rotate90(),
        Rotation::R180 => img.rotate180(),
        Rotation::R270 => img.rotate270(),
    };
    
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.grayscale();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, color: Vec<u8>) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = ((color[0] as u32 * x) % 256) as u8;
        let green = ((color[1] as u32 * y) % 256) as u8;
        let blue = ((color[2] as u32 * (x+y)) % 256) as u8;

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
