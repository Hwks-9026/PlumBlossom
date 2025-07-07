use image::{ImageBuffer, RgbaImage};

pub(crate) fn blit(buffer: Vec<u8>) {
    let width = 640;
    let height = 480;

    let img: RgbaImage = ImageBuffer::from_raw(width, height, buffer).expect("buffer size must match image size");
    img.save("render.png").expect("failed to save image");
}
