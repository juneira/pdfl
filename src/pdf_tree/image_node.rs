pub struct ImageNode {
    pub attributes: std::collections::HashMap<String, String>,
}

impl ImageNode {
    pub fn to_obj(&self) -> String {
        use flate2::write::ZlibEncoder;
        use flate2::Compression;
        use image::io::Reader as ImageReader;
        use std::io::Write;

        let src = self
            .attributes
            .get("src")
            .expect("src attribute missing");

        let img = ImageReader::open(src)
            .expect("unable to open image")
            .decode()
            .expect("unable to decode image");
        let rgb = img.to_rgb8();
        let (width, height) = rgb.dimensions();

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&rgb.into_raw())
            .expect("compression failed");
        let compressed = encoder.finish().expect("compression finish failed");

        let hex: String = compressed.iter().map(|b| format!("{:02X}", b)).collect();

        format!(
            "BI\n/Width {}\n/Height {}\n/ColorSpace /DeviceRGB\n/BitsPerComponent 8\n/Filter /FlateDecode\nID\n<{}>\nEI",
            width, height, hex
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::Read;

    #[test]
    fn test_to_obj_compression() {
        let dir = std::env::temp_dir();
        let path = dir.join("pdfl_test_img.png");

        let img = image::RgbImage::from_pixel(1, 1, image::Rgb([10, 20, 30]));
        img.save(&path).unwrap();

        let mut attrs = HashMap::new();
        attrs.insert("src".to_string(), path.to_str().unwrap().to_string());
        let image = ImageNode { attributes: attrs };

        let obj = image.to_obj();
        assert!(obj.contains("/Filter /FlateDecode"));

        let data_part = obj
            .splitn(2, "ID\n")
            .nth(1)
            .unwrap()
            .splitn(2, "\nEI")
            .next()
            .unwrap()
            .trim()
            .trim_start_matches('<')
            .trim_end_matches('>');

        let bytes: Vec<u8> = (0..data_part.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&data_part[i..i + 2], 16).unwrap())
            .collect();

        let mut decoder = flate2::read::ZlibDecoder::new(&bytes[..]);
        let mut decoded = Vec::new();
        decoder.read_to_end(&mut decoded).unwrap();

        assert_eq!(decoded, vec![10, 20, 30]);

        std::fs::remove_file(path).unwrap();
    }
}
