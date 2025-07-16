use crate::ast2pdft::page_converter::PageConverter;

pub struct PagesConverter;

impl PagesConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert(
        &self,
        ast_page: crate::parser::PageNode,
        images: &[String],
        fonts: &[String],
    ) -> (crate::pdf_tree::PagesNode, usize) {
        let mut total_obj = 0;
        let mut kids: Vec<crate::pdf_tree::PageNode> = Vec::new();
        let mut obj_num = 3;
        let mut current_page = ast_page;

        let page_converter = PageConverter::new();

        loop {
            let (page_node, used_obj) =
                page_converter.convert(&current_page, obj_num, 0, images, fonts);
            total_obj += used_obj;
            obj_num += used_obj;

            kids.push(page_node);

            if current_page.child_page.is_none() {
                break;
            }

            current_page = *current_page.child_page.unwrap();
        }

        let pages_node = crate::pdf_tree::PagesNode {
            obj_num: 2,
            gen_num: 0,
            count: kids.len(),
            kids,
        };

        (pages_node, total_obj + 1)
    }
}
