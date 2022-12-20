//! 아스키 텍스트로 이미지를 랜더링하기

const ASCII_TEXTURE: &'static str = " .:-+*=%@#";

/// 아스키 코드로 랜더링
#[derive(Clone, Debug)]
pub struct AsciiImage {
    pub width: u32,
    pub height: u32,
    pub ascii_image: String,
}

impl AsciiImage {
    /// 가로 w픽셀, 세로 h 픽셀만큼을 한 아스키 코드로 만든다.

    pub fn new(image: &image::DynamicImage, w: u32, h: u32) -> Self {
        let buffer = image.to_luma8();
        let mut mozaic: Vec<u32> = vec![];
        let mut ascii_image: String = "".to_owned();
        let width = buffer.width() / w;
        let height = buffer.height() / h;

        mozaic.resize_with(width as usize * height as usize, Default::default);

        for y in 0..(height * h) {
            for x in 0..(width * w) {
                // 픽셀값을 평균으로 환산
                let pixel = buffer.get_pixel(x, y).0[0];
                let x_ = x / w;
                let y_ = y / h;

                let r = mozaic[(y_ * width + x_) as usize] + (pixel as u32);
                mozaic[(y_ * width + x_) as usize] = r;
            }
        }

        // 일정 구간의 값을 합산(wxh)하여 0~100 까지의 값(n)으로 하고
        for y in 0..height {
            for x in 0..width {
                let n = mozaic[(y * width + x) as usize] * 100 / ((w * h * 255) as u32);
                let p = (n as usize * ASCII_TEXTURE.len() / 100) as usize;
                ascii_image = ascii_image + ASCII_TEXTURE.get(p..p + 1).unwrap();
            }
        }

        // 이를 ASCII_TEXTURE와의 길이(l)에 대한 비례식으로 값을 구한다.
        // x:n = l:100 x = n * l / 100

        AsciiImage {
            width,
            height,
            ascii_image,
        }
    }
}
