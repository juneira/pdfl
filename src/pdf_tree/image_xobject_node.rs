use ascii85::encode;
use image::io::Reader as ImageReader;

pub struct ImageXObjectNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub data: String,
}

impl ImageXObjectNode {
    pub fn new(obj_num: usize, gen_num: usize, path: &str, name: String) -> Self {
        let img = ImageReader::open(path)
            .expect("unable to open image")
            .decode()
            .expect("unable to decode image");
        let rgb = img.to_rgb8();
        let (width, height) = rgb.dimensions();
        let data = encode(&rgb.into_raw());
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
        self.to_obj().into_bytes()
    }

    fn to_obj(&self) -> String {
        format!(
            "{} {} obj\n<< /Type /XObject\n/Subtype /Image\n/Width {}\n/Height {}\n/ColorSpace /DeviceRGB\n/BitsPerComponent 8\n/Filter /ASCII85Decode\n/Length {}>>\nstream\n{}\nendstream\nendobj\n",
            self.obj_num,
            self.gen_num,
            self.width,
            self.height,
            self.data.len(),
            self.data,
        )
    }
}
