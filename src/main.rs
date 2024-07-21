use std::env;
use std::process;
use std::path::Path;
use image::GenericImageView;
use image::ImageBuffer; 
use image::Rgba;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_image> <output_image>", args[0]);
        process::exit(1);
    }
    
    let fin = &args[1];
    let fout = &args[2];

    let fmts = vec!["png", "gif", "tif", "tiff", "webp"];

    if !fmts.contains(&Path::new(fout).extension().unwrap().to_str().unwrap()) {
        eprintln!("Error: The output file must have a supported extension ({:?})", fmts);
        return;
    }

    let img = image::open(fin).expect("Failed to open input image");

    let (w, h) = img.dimensions();
    let mut res = ImageBuffer::new(w, h);

    for (x, y, px) in img.pixels() {
        if px.0[0] == 255 && px.0[1] == 255 && px.0[2] == 255 {
            res.put_pixel(x, y, Rgba([255, 255, 255, 0]));
        } else {
            res.put_pixel(x, y, px);
        }
    }

    res.save(fout).expect("Failed to save output image");
}
