use sdl_isometric::ascii::AsciiImage;

fn main() {
    let dog_image = image::io::Reader::open("assets/hacker.png")
        .unwrap()
        .decode()
        .unwrap();
    let ascii_image = AsciiImage::new(&dog_image, 20, 40);

    let width = ascii_image.width;
    let height = ascii_image.height;
    let txt_image = ascii_image.ascii_image;

    for y in 0..height {
        println!(
            "{}",
            txt_image
                .get((y * width) as usize..((y + 1) * width) as usize)
                .unwrap()
        );
    }
}
