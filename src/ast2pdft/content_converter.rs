use crate::ast2pdft::element_converter::ElementConverter;

pub struct ContentConverter;

impl ContentConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert(
        &self,
        ast_content: &crate::parser::ContentNode,
        obj_num: usize,
        gen_num: usize,
    ) -> crate::pdf_tree::ContentNode {
        let element_converter = ElementConverter::new();

        let elements = ast_content
            .children
            .iter()
            .filter_map(|el| element_converter.convert(el))
            .collect();

        crate::pdf_tree::ContentNode {
            obj_num,
            gen_num,
            contents: elements,
        }
    }
}
