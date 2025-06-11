use std::collections::HashMap;

pub fn to_pdft(pdf_node: crate::parser::PdfNode) -> crate::pdf_tree::PdfNode {
    let version = "1.4".to_string();
    let (catalog, total_obj) = catalog_node_from_ast(pdf_node);

    crate::pdf_tree::PdfNode {
        version,
        total_obj,
        root: catalog,
    }
}

fn catalog_node_from_ast(ast_pdf: crate::parser::PdfNode) -> (crate::pdf_tree::CatalogNode, usize) {
    let (pages_node, total_obj) = pages_node_from_ast(ast_pdf.child_page);

    let catalog = crate::pdf_tree::CatalogNode {
        obj_num: 1,
        gen_num: 0,
        pages: pages_node,
    };

    (catalog, total_obj + 1)
}

fn pages_node_from_ast(ast_page: crate::parser::PageNode) -> (crate::pdf_tree::PagesNode, usize) {
    let mut total_obj = 0;
    let mut kids: Vec<crate::pdf_tree::PageNode> = Vec::new();

    let mut obj_num = 3;

    let mut current_page = ast_page;

    loop {
        let page_node = page_node_from_ast(&current_page, obj_num, 0);
        total_obj += 3;
        obj_num += 3;

        kids.push(page_node);

        if current_page.child_page == None {
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

fn page_node_from_ast(
    ast_page: &crate::parser::PageNode,
    obj_num: usize,
    gen_num: usize,
) -> crate::pdf_tree::PageNode {
    let mut resources = HashMap::new();
    resources.insert("F1".to_string(), resource(obj_num + 1, gen_num));

    let content_node = content_node_from_ast(&ast_page.child_content, obj_num + 2, gen_num);

    crate::pdf_tree::PageNode {
        obj_num,
        gen_num,
        resources,
        contents: content_node,
    }
}

fn resource(obj_num: usize, gen_num: usize) -> crate::pdf_tree::FontNode {
    crate::pdf_tree::FontNode {
        obj_num: obj_num,
        gen_num: gen_num,
        subtype: "Type1".to_string(),
        base_font: "Helvetica".to_string(),
    }
}

fn content_node_from_ast(
    ast_content: &crate::parser::ContentNode,
    obj_num: usize,
    gen_num: usize,
) -> crate::pdf_tree::ContentNode {
    let text_nodes = ast_content
        .child_texts
        .iter()
        .map(|t| text_node_from_ast(t))
        .collect();

    crate::pdf_tree::ContentNode {
        obj_num: obj_num,
        gen_num: gen_num,
        contents: text_nodes,
    }
}

fn text_node_from_ast(ast_text: &crate::parser::TextNode) -> crate::pdf_tree::TextNode {
    let x_pos = ast_text
        .attributes
        .get("pos_x")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(100);
    let y_pos = ast_text
        .attributes
        .get("pos_y")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(700);
    let font_size = ast_text
        .attributes
        .get("font_size")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(24);

    crate::pdf_tree::TextNode {
        font: "F1".to_string(),
        font_size,
        x_pos,
        y_pos,
        text: ast_text.child_string.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pdft() {
        let code: &'static str = "
    <pdf>
        <page>
            <content>
                <text>
                    texto    mais    longoooo
                    pdf
                </text>
            </content>
        </page>
        <page>
            <content>
                <text>
                    Outro texto
                </text>
            </content>
        </page>
    </pdf>
    ";

    let node = crate::parser::parse(code).unwrap();
    let pdft = to_pdft(node);

    let buffer = pdft.to_buffer();
    let pdf_string = String::from_utf8(buffer).unwrap();

    assert_eq!(
        pdf_string,
        "%PDF-1.4
1 0 obj
<< /Type /Catalog
/Pages 2 0 R
>>
endobj
2 0 obj
<< /Type /Pages
/Count 2
/Kids [3 0 R 6 0 R]
>>
endobj
3 0 obj
<< /Type /Page
/Resources << /Font << /F1 4 0 R >> >>
/Contents 5 0 R
>>
endobj
4 0 obj
<< /Type /Font
/Subtype /Type1
/BaseFont /Helvetica
>>
endobj
5 0 obj
<< /Length 81>>
stream
BT
/F1 24 Tf
100 700 Td
(texto    mais    longoooo
                    pdf) Tj
ET
endstream
endobj
6 0 obj
<< /Type /Page
/Resources << /Font << /F1 7 0 R >> >>
/Contents 8 0 R
>>
endobj
7 0 obj
<< /Type /Font
/Subtype /Type1
/BaseFont /Helvetica
>>
endobj
8 0 obj
<< /Length 43>>
stream
BT
/F1 24 Tf
100 700 Td
(Outro texto) Tj
ET
endstream
endobj
xref
0 8
0000000000 65535 f
0000000010 00000 n
0000000060 00000 n
0000000124 00000 n
0000000213 00000 n
0000000372 00000 n
0000000724 00000 n
0000000813 00000 n
0000000972 00000 n
trailer
<< /Size 8
/Root 1 0 R
>>
startxref
973
%%EOF");

    }

    #[test]
    fn test_text_position_attributes() {
        let code = "<pdf><page><content><text pos_x=\"20\" pos_y=\"50\">a</text></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node);
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8(buffer).unwrap();
        assert!(pdf_string.contains("20 50 Td"));
    }

    #[test]
    fn test_text_font_size_attribute() {
        let code = "<pdf><page><content><text font_size=\"30\">a</text></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node);
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8(buffer).unwrap();
        assert!(pdf_string.contains("/F1 30 Tf"));
    }
}
