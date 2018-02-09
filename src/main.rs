extern crate image;
#[macro_use]
extern crate dump;

use std::fs::File;

use image::GenericImage;
use image::Pixel;
use image::imageops;

fn main() {
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let img = image::open("ashish.jpg").unwrap();
    let mut subs = vec!();
    for (x,y,pixel) in img.pixels() {
       if x < 10 && y < 10 {
           subs.push(pixel);
       }
    }

dump!(subs[0])
}
