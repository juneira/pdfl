use flate2::{write::ZlibEncoder, Compression};
use image::io::Reader as ImageReader;
use std::io::Write;

pub struct ImageXObjectNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl ImageXObjectNode {
    pub fn new(obj_num: usize, gen_num: usize, path: &str, name: String) -> Self {
        let img = ImageReader::open(path)
            .expect("unable to open image")
            .decode()
            .expect("unable to decode image");
        let rgb = img.to_rgb8();
        let (width, height) = rgb.dimensions();
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&rgb.into_raw())
            .expect("unable to compress image");
        let data = encoder.finish().expect("unable to finish compression");
        Self {
            obj_num,
            gen_num,
            name,
            width,
            height,
            data,
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(
            format!(
                "{} {} obj\n<< /Type /XObject\n/Subtype /Image\n/Width {}\n/Height {}\n/ColorSpace /DeviceRGB\n/BitsPerComponent 8\n/Filter /FlateDecode\n/Length {}>>\nstream\n",
                self.obj_num,
                self.gen_num,
                self.width,
                self.height,
                self.data.len(),
            )
            .as_bytes(),
        );
        buffer.extend(&self.data);
        buffer.extend(b"\nendstream\nendobj\n");
        buffer
    }

}
