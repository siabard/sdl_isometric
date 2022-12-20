//! 아스키 텍스트로 이미지를 랜더링하기

const ASCII_TEXTURE: &'static str = " .:-+*=%@#";

/// 아스키 코드로 랜더링
pub struct AsciiImage {
    width: u32,
    height: u32,
    ascii_image: String,
}

impl AsciiImage {
    pub fn new(image: &image::DynamicImage) -> Self {
        AsciiImage {
            width: 0,
            height: 0,
            ascii_image: "".to_owned(),
        }
    }
}
