use crate::ast2pdft::catalog_converter::CatalogConverter;

pub struct PdfConverter {
    version: String,
}

impl PdfConverter {
    pub fn new() -> Self {
        Self {
            version: "1.4".to_string(),
        }
    }

    pub fn convert(&self, pdf_node: crate::parser::PdfNode, images: &[String]) -> crate::pdf_tree::PdfNode {
        let catalog_converter = CatalogConverter::new();
        let (catalog, total_obj) = catalog_converter.convert(pdf_node, images);

        crate::pdf_tree::PdfNode {
            version: self.version.clone(),
            total_obj,
            root: catalog,
        }
    }
}
