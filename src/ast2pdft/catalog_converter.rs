use crate::ast2pdft::pages_converter::PagesConverter;

pub struct CatalogConverter;

impl CatalogConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert(
        &self,
        ast_pdf: crate::parser::PdfNode,
        images: &[String],
        fonts: &[String],
    ) -> (crate::pdf_tree::CatalogNode, usize) {
        let pages_converter = PagesConverter::new();
        let (pages_node, total_obj) = pages_converter.convert(ast_pdf.child_page, images, fonts);

        let catalog = crate::pdf_tree::CatalogNode {
            obj_num: 1,
            gen_num: 0,
            pages: pages_node,
        };

        (catalog, total_obj + 1)
    }
}
