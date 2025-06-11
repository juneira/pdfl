mod pdf_node;
mod font_node;
mod page_node;
mod pages_node;
mod catalog_node;
mod content_node;
mod text_node;

pub use pdf_node::PdfNode;
pub use font_node::FontNode;
pub use page_node::PageNode;
pub use pages_node::PagesNode;
pub use catalog_node::CatalogNode;
pub use content_node::ContentNode;
pub use text_node::TextNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        let font_node = FontNode {
            obj_num: 5,
            gen_num: 0,
            subtype: "Type1".to_string(),
            base_font: "Helvetica".to_string(),
        };

        let mut resources = std::collections::HashMap::new();
        resources.insert("F1".to_string(), font_node);

        let pdf_node = PdfNode {
            version: "1.4".to_string(),
            total_obj: 6,
            root: CatalogNode {
                obj_num: 1,
                gen_num: 0,
                pages: PagesNode {
                    obj_num: 2,
                    gen_num: 0,
                    count: 1,
                    kids: vec![
                        PageNode {
                            obj_num: 3,
                            gen_num: 0,
                            resources: resources,
                            contents: ContentNode {
                                obj_num: 4,
                                gen_num: 0,
                                contents: vec![TextNode {
                                    font: "F1".to_string(),
                                    font_size: 24,
                                    x_pos: 100,
                                    y_pos: 700,
                                    text: "Hello World".to_string(),
                                    color: (0, 0, 0),
                                }],
                            },
                        },
                    ],
                },
            },
        };

        let pdf_string = String::from_utf8(pdf_node.to_buffer()).unwrap();

        assert_eq!(pdf_string, "%PDF-1.4
1 0 obj
<< /Type /Catalog
/Pages 2 0 R
>>
endobj
2 0 obj
<< /Type /Pages
/Count 1
/Kids [3 0 R]
>>
endobj
3 0 obj
<< /Type /Page
/Resources << /Font << /F1 5 0 R >> >>
/Contents 4 0 R
>>
endobj
5 0 obj
<< /Type /Font
/Subtype /Type1
/BaseFont /Helvetica
>>
endobj
4 0 obj
<< /Length 52>>
stream
BT
/F1 24 Tf
0 0 0 rg
100 700 Td
(Hello World) Tj
ET
endstream
endobj
xref
0 6
0000000000 65535 f
0000000010 00000 n
0000000060 00000 n
0000000118 00000 n
0000000207 00000 n
0000000366 00000 n
trailer
<< /Size 6
/Root 1 0 R
>>
startxref
367
%%EOF");
    }
}
