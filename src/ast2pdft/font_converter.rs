pub struct FontConverter;

impl FontConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert(
        &self,
        font: &crate::parser::FontNode,
        obj_num: usize,
        gen_num: usize,
    ) -> (String, crate::pdf_tree::FontNode, usize) {
        let key = font
            .attributes
            .get("key")
            .expect("font key is required")
            .to_string();
        if let Some(src) = font.attributes.get("src") {
            let raw = std::fs::read(src).expect("unable to read font file");
            let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
            use std::io::Write;
            encoder.write_all(&raw).expect("unable to compress font");
            let data = encoder.finish().expect("compression failed");
            let font_node = crate::pdf_tree::FontNode {
                obj_num,
                gen_num,
                subtype: "TrueType".to_string(),
                base_font: key.clone(),
                file_obj_num: Some(obj_num + 1),
                data: Some(data),
                length1: Some(raw.len()),
            };
            return (key, font_node, 2);
        }
        let subtype = font
            .attributes
            .get("subtype")
            .cloned()
            .unwrap_or_else(|| "Type1".to_string());
        let base_font = font
            .attributes
            .get("base_font")
            .cloned()
            .unwrap_or_else(|| "Helvetica".to_string());

        let font_node = crate::pdf_tree::FontNode {
            obj_num,
            gen_num,
            subtype,
            base_font,
            file_obj_num: None,
            data: None,
            length1: None,
        };

        (key, font_node, 1)
    }

    pub fn convert_file(
        &self,
        path: &str,
        obj_num: usize,
        gen_num: usize,
    ) -> (String, crate::pdf_tree::FontNode, usize) {
        let name = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap()
            .to_string();
        let raw = std::fs::read(path).expect("unable to read font file");
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        use std::io::Write;
        encoder.write_all(&raw).expect("unable to compress font");
        let data = encoder.finish().expect("compression failed");

        let font_node = crate::pdf_tree::FontNode {
            obj_num,
            gen_num,
            subtype: "TrueType".to_string(),
            base_font: name.clone(),
            file_obj_num: Some(obj_num + 1),
            data: Some(data),
            length1: Some(raw.len()),
        };

        (name, font_node, 2)
    }

    pub fn create_default(&self, obj_num: usize, gen_num: usize) -> crate::pdf_tree::FontNode {
        crate::pdf_tree::FontNode {
            obj_num,
            gen_num,
            subtype: "Type1".to_string(),
            base_font: "Helvetica".to_string(),
            file_obj_num: None,
            data: None,
            length1: None,
        }
    }
}
