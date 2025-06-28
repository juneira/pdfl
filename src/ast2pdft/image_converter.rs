pub struct ImageConverter;

impl ImageConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_xobject(
        &self,
        img_path: &str,
        obj_num: usize,
        gen_num: usize,
    ) -> (String, crate::pdf_tree::ImageXObjectNode) {
        let name = std::path::Path::new(img_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap()
            .to_string();

        let image_xobject = crate::pdf_tree::ImageXObjectNode::new(obj_num, gen_num, img_path);

        (name, image_xobject)
    }
}
