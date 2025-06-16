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
        let (page_node, used_obj) = page_node_from_ast(&current_page, obj_num, 0);
        total_obj += used_obj;
        obj_num += used_obj;

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
) -> (crate::pdf_tree::PageNode, usize) {
    let mut resources = HashMap::new();
    let mut next_obj = obj_num + 1;

    if let Some(res) = &ast_page.resources {
        for font in &res.fonts {
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

            resources.insert(
                key,
                crate::pdf_tree::FontNode {
                    obj_num: next_obj,
                    gen_num,
                    subtype,
                    base_font,
                },
            );
            next_obj += 1;
        }
    } else {
        resources.insert("F1".to_string(), resource(next_obj, gen_num));
        next_obj += 1;
    }

    let content_node = content_node_from_ast(&ast_page.child_content, next_obj, gen_num);
    next_obj += 1;

    (
        crate::pdf_tree::PageNode {
            obj_num,
            gen_num,
            resources,
            contents: content_node,
        },
        next_obj - obj_num,
    )
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
    let elements = ast_content
        .children
        .iter()
        .map(|el| match el {
            crate::parser::ContentElement::Text(t) => {
                crate::pdf_tree::ContentItem::Text(text_node_from_ast(t))
            }
            crate::parser::ContentElement::Rectangle(r) => {
                crate::pdf_tree::ContentItem::Rectangle(rect_node_from_ast(r))
            }
        })
        .collect();

    crate::pdf_tree::ContentNode {
        obj_num: obj_num,
        gen_num: gen_num,
        contents: elements,
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

    let font = ast_text
        .attributes
        .get("font")
        .and_then(|v| v.parse::<String>().ok())
        .unwrap_or("F1".to_string())
        .to_string();

    let color = ast_text
        .attributes
        .get("color")
        .map(|v| v.trim_start_matches('#'))
        .and_then(|v| u32::from_str_radix(v, 16).ok())
        .map(|rgb| (((rgb >> 16) & 0xff) as u8, ((rgb >> 8) & 0xff) as u8, (rgb & 0xff) as u8))
        .unwrap_or((0, 0, 0));

    crate::pdf_tree::TextNode {
        font,
        font_size,
        x_pos,
        y_pos,
        text: ast_text.child_string.clone(),
        color,
    }
}

fn rect_node_from_ast(ast_rect: &crate::parser::RectangleNode) -> crate::pdf_tree::RectangleNode {
    let x_pos = ast_rect
        .attributes
        .get("pos_x")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let y_pos = ast_rect
        .attributes
        .get("pos_y")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let width = ast_rect
        .attributes
        .get("width")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let height = ast_rect
        .attributes
        .get("height")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let color = ast_rect
        .attributes
        .get("color")
        .map(|v| v.trim_start_matches('#'))
        .and_then(|v| u32::from_str_radix(v, 16).ok())
        .map(|rgb| (((rgb >> 16) & 0xff) as u8, ((rgb >> 8) & 0xff) as u8, (rgb & 0xff) as u8))
        .unwrap_or((0, 0, 0));

    crate::pdf_tree::RectangleNode {
        x_pos,
        y_pos,
        width,
        height,
        color,
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
            <resource>
                <font key=\"F1\" />
            </resource>
            <content>
                <text font=\"F1\">
                    texto    mais    longoooo
                    pdf
                </text>
            </content>
        </page>
        <page>
            <resource>
                <font key=\"F1\" />
            </resource>
            <content>
                <text font=\"F1\">
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
<< /Length 90>>
stream
BT
/F1 24 Tf
0 0 0 rg
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
<< /Length 52>>
stream
BT
/F1 24 Tf
0 0 0 rg
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
0000000733 00000 n
0000000822 00000 n
0000000981 00000 n
trailer
<< /Size 8
/Root 1 0 R
>>
startxref
982
%%EOF");

    }

    #[test]
    fn test_text_position_attributes() {
        let code = "<pdf><page><resource><font key=\"F1\" /></resource><content><text font=\"F1\" pos_x=\"20\" pos_y=\"50\">a</text></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node);
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8(buffer).unwrap();
        assert!(pdf_string.contains("20 50 Td"));
    }

    #[test]
    fn test_text_font_size_attribute() {
        let code = "<pdf><page><resource><font key=\"F1\" /></resource><content><text font=\"F1\" font_size=\"30\">a</text></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node);
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8(buffer).unwrap();
        assert!(pdf_string.contains("/F1 30 Tf"));
    }

    #[test]
    fn test_rectangle_generation() {
        let code = "<pdf><page><content><rectangle pos_x=\"10\" pos_y=\"20\" width=\"30\" height=\"40\" color=\"#FF0000\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node);
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8(buffer).unwrap();
        assert!(pdf_string.contains("10 20 30 40 re"));
    }
}
