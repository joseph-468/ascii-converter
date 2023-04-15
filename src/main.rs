use image::{DynamicImage, GenericImageView};
use std::fs;

const CHAR_SET: &str = " .:-=*#P@";

fn main() {
    // Get file name
    println!("Please enter the full file name of the image you want to convert");
    let mut file_name: String = String::new();
    std::io::stdin().read_line(&mut file_name).unwrap();

    // Load image
    let chars: Vec<char> = CHAR_SET.chars().collect();
    let mut output: Vec<String> = Vec::new();
    let mut values: Vec<u8> = Vec::new();
    let img: DynamicImage = image::open(file_name.trim()).unwrap().grayscale().resize(
        64,
        64,
        image::imageops::FilterType::Nearest,
    );
    for i in img.pixels() {
        values.push(i.2[0]);
    }

    // Convert image
    for (i, x) in values.iter().enumerate() {
        output.push(chars[(*x as usize / 29) as usize].to_string());
        if (i as u32 + 1) % img.width() == 0 {
            output.push("\n".to_string());
        }
    }
    fs::write("output.txt", output.join("")).expect("Unable to write to file");
    println!("Conversion complete! Result is contained in output.txt");
}
