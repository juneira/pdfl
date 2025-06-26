use std::collections::HashMap;

pub fn to_pdft(pdf_node: crate::parser::PdfNode, images: &[String]) -> crate::pdf_tree::PdfNode {
    let version = "1.4".to_string();
    let (catalog, total_obj) = catalog_node_from_ast(pdf_node, images);

    crate::pdf_tree::PdfNode {
        version,
        total_obj,
        root: catalog,
    }
}

fn catalog_node_from_ast(ast_pdf: crate::parser::PdfNode, images: &[String]) -> (crate::pdf_tree::CatalogNode, usize) {
    let (pages_node, total_obj) = pages_node_from_ast(ast_pdf.child_page, images);

    let catalog = crate::pdf_tree::CatalogNode {
        obj_num: 1,
        gen_num: 0,
        pages: pages_node,
    };

    (catalog, total_obj + 1)
}

fn pages_node_from_ast(ast_page: crate::parser::PageNode, images: &[String]) -> (crate::pdf_tree::PagesNode, usize) {
    let mut total_obj = 0;
    let mut kids: Vec<crate::pdf_tree::PageNode> = Vec::new();

    let mut obj_num = 3;

    let mut current_page = ast_page;

    loop {
        let (page_node, used_obj) = page_node_from_ast(&current_page, obj_num, 0, images);
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
    images: &[String],
) -> (crate::pdf_tree::PageNode, usize) {
    let mut fonts = HashMap::new();
    let mut images_map = HashMap::new();
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

            fonts.insert(
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
        fonts.insert("F1".to_string(), resource(next_obj, gen_num));
        next_obj += 1;
    }

    for img_path in images {
        let name = std::path::Path::new(img_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap()
            .to_string();
        images_map.insert(
            name.clone(),
            crate::pdf_tree::ImageXObjectNode::new(next_obj, gen_num, img_path),
        );
        next_obj += 1;
    }

    let content_node = content_node_from_ast(&ast_page.child_content, next_obj, gen_num);
    next_obj += 1;

    (
        crate::pdf_tree::PageNode {
            obj_num,
            gen_num,
            fonts,
            images: images_map,
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
        .filter_map(|el| match el {
            crate::parser::ContentElement::Text(t) => {
                Some(crate::pdf_tree::ContentItem::Text(text_node_from_ast(t)))
            }
            crate::parser::ContentElement::Rectangle(r) => {
                Some(crate::pdf_tree::ContentItem::Rectangle(rect_node_from_ast(r)))
            }
            crate::parser::ContentElement::Line(l) => {
                Some(crate::pdf_tree::ContentItem::Line(line_node_from_ast(l)))
            }
            crate::parser::ContentElement::Circle(c) => {
                Some(crate::pdf_tree::ContentItem::Circle(circle_node_from_ast(c)))
            }
            crate::parser::ContentElement::Image(i) => {
                Some(crate::pdf_tree::ContentItem::Image(image_node_from_ast(i)))
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

fn line_node_from_ast(ast_line: &crate::parser::LineNode) -> crate::pdf_tree::LineNode {
    let x_pos = ast_line
        .attributes
        .get("pos_x")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let y_pos = ast_line
        .attributes
        .get("pos_y")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let width = ast_line
        .attributes
        .get("width")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let rotation = ast_line
        .attributes
        .get("rotation")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(0.0);
    let color = ast_line
        .attributes
        .get("color")
        .map(|v| v.trim_start_matches('#'))
        .and_then(|v| u32::from_str_radix(v, 16).ok())
        .map(|rgb| (((rgb >> 16) & 0xff) as u8, ((rgb >> 8) & 0xff) as u8, (rgb & 0xff) as u8))
        .unwrap_or((0, 0, 0));

    crate::pdf_tree::LineNode {
        x_pos,
        y_pos,
        width,
        color,
        rotation,
    }
}

fn circle_node_from_ast(ast_circle: &crate::parser::CircleNode) -> crate::pdf_tree::CircleNode {
    let x_pos = ast_circle
        .attributes
        .get("pos_x")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let y_pos = ast_circle
        .attributes
        .get("pos_y")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let width = ast_circle
        .attributes
        .get("width")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let height = ast_circle
        .attributes
        .get("height")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let color = ast_circle
        .attributes
        .get("color")
        .map(|v| v.trim_start_matches('#'))
        .and_then(|v| u32::from_str_radix(v, 16).ok())
        .map(|rgb| (((rgb >> 16) & 0xff) as u8, ((rgb >> 8) & 0xff) as u8, (rgb & 0xff) as u8))
        .unwrap_or((0, 0, 0));

    crate::pdf_tree::CircleNode {
        x_pos,
        y_pos,
        width,
        height,
        color,
    }
}

fn image_node_from_ast(ast_image: &crate::parser::ImageNode) -> crate::pdf_tree::ImageNode {
    let name = ast_image
        .attributes
        .get("src")
        .expect("src attribute missing")
        .to_string();
    let x_pos = ast_image
        .attributes
        .get("pos_x")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let y_pos = ast_image
        .attributes
        .get("pos_y")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let width = ast_image
        .attributes
        .get("width")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let height = ast_image
        .attributes
        .get("height")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);
    let rotation = ast_image
        .attributes
        .get("rotation")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(0.0)
        .clamp(0.0, 360.0);

    crate::pdf_tree::ImageNode {
        name,
        x_pos,
        y_pos,
        width,
        height,
        rotation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image;

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
    let pdft = to_pdft(node, &Vec::new());

    let buffer = pdft.to_buffer();
    let pdf_string = String::from_utf8_lossy(&buffer);

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
        let pdft = to_pdft(node, &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("20 50 Td"));
    }

    #[test]
    fn test_text_font_size_attribute() {
        let code = "<pdf><page><resource><font key=\"F1\" /></resource><content><text font=\"F1\" font_size=\"30\">a</text></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("/F1 30 Tf"));
    }

    #[test]
    fn test_rectangle_generation() {
        let code = "<pdf><page><content><rectangle pos_x=\"10\" pos_y=\"20\" width=\"30\" height=\"40\" color=\"#FF0000\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("10 20 30 40 re"));
    }

    #[test]
    fn test_line_generation() {
        let code = "<pdf><page><content><line pos_x=\"5\" pos_y=\"15\" width=\"25\" color=\"#00FF00\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("5 15 cm"));
    }

    #[test]
    fn test_line_rotation() {
        let code = "<pdf><page><content><line pos_x=\"5\" pos_y=\"15\" width=\"25\" rotation=\"45\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("0.707"));
    }

    #[test]
    fn test_circle_generation() {
        let code = "<pdf><page><content><circle pos_x=\"10\" pos_y=\"20\" width=\"30\" height=\"40\" color=\"#FF0000\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("f"));
    }

    #[test]
    fn test_image_generation() {
        let dir = std::env::temp_dir();
        let path = dir.join("pdfl_test_img.png");
        let img = image::RgbImage::from_pixel(1, 1, image::Rgb([10, 20, 30]));
        img.save(&path).unwrap();

        let code = "<pdf><page><content><image src=\"pdfl_test_img.png\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let images = vec![path.to_str().unwrap().to_string()];
        let pdft = to_pdft(node, &images);
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("/pdfl_test_img.png Do"));

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_image_rotation() {
        let dir = std::env::temp_dir();
        let path = dir.join("pdfl_test_img2.png");
        let img = image::RgbImage::from_pixel(1, 1, image::Rgb([10, 20, 30]));
        img.save(&path).unwrap();

        let code = "<pdf><page><content><image src=\"pdfl_test_img2.png\" rotation=\"20\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let images = vec![path.to_str().unwrap().to_string()];
        let pdft = to_pdft(node, &images);
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("pdfl_test_img2.png Do"));
        assert!(pdf_string.contains("17.10"));

        std::fs::remove_file(path).unwrap();
    }
}
