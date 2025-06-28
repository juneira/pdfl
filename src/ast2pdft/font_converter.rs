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
    ) -> (String, crate::pdf_tree::FontNode) {
        let key = font
            .attributes
            .get("key")
            .expect("font key is required")
            .to_string();
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
        };

        (key, font_node)
    }

    pub fn create_default(&self, obj_num: usize, gen_num: usize) -> crate::pdf_tree::FontNode {
        crate::pdf_tree::FontNode {
            obj_num,
            gen_num,
            subtype: "Type1".to_string(),
            base_font: "Helvetica".to_string(),
        }
    }
}
