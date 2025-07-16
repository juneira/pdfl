mod attribute_parser;
mod catalog_converter;
mod content_converter;
mod element_converter;
mod font_converter;
mod image_converter;
mod page_converter;
mod pages_converter;
mod pdf_converter_main;

pub use pdf_converter_main::PdfConverter;

pub fn to_pdft(
    pdf_node: crate::parser::PdfNode,
    images: &[String],
    fonts: &[String],
) -> crate::pdf_tree::PdfNode {
    let converter = PdfConverter::new();
    converter.convert(pdf_node, images, fonts)
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
        let pdft = to_pdft(node, &Vec::new(), &Vec::new());

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
%%EOF"
        );
    }

    #[test]
    fn test_text_position_attributes() {
        let code = "<pdf><page><resource><font key=\"F1\" /></resource><content><text font=\"F1\" pos_x=\"20\" pos_y=\"50\">a</text></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new(), &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("20 50 Td"));
    }

    #[test]
    fn test_text_font_size_attribute() {
        let code = "<pdf><page><resource><font key=\"F1\" /></resource><content><text font_size=\"30\">a</text></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new(), &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("/F1 30 Tf"));
    }

    #[test]
    fn test_rectangle_generation() {
        let code = "<pdf><page><content><rectangle pos_x=\"10\" pos_y=\"20\" width=\"30\" height=\"40\" color=\"#FF0000\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new(), &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("10 20 cm"));
        assert!(pdf_string.contains("0 0 30 40 re"));
    }

    #[test]
    fn test_rectangle_rotation() {
        let code = "<pdf><page><content><rectangle pos_x=\"10\" pos_y=\"20\" width=\"30\" height=\"40\" rotation=\"45\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new(), &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("0.707"));
    }

    #[test]
    fn test_line_generation() {
        let code = "<pdf><page><content><line pos_x=\"5\" pos_y=\"15\" width=\"25\" color=\"#00FF00\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new(), &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("5 15 cm"));
    }

    #[test]
    fn test_line_rotation() {
        let code = "<pdf><page><content><line pos_x=\"5\" pos_y=\"15\" width=\"25\" rotation=\"45\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new(), &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("0.707"));
    }

    #[test]
    fn test_text_rotation() {
        let code = "<pdf><page><resource><font key=\"F1\" /></resource><content><text font=\"F1\" rotation=\"20\">a</text></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new(), &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("0.939"));
    }

    #[test]
    fn test_circle_generation() {
        let code = "<pdf><page><content><circle pos_x=\"10\" pos_y=\"20\" width=\"30\" height=\"40\" color=\"#FF0000\" /></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let pdft = to_pdft(node, &Vec::new(), &Vec::new());
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
        let pdft = to_pdft(node, &images, &Vec::new());
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
        let pdft = to_pdft(node, &images, &Vec::new());
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("pdfl_test_img2.png Do"));
        assert!(pdf_string.contains("17.10"));

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_external_font() {
        let code = "<pdf><page><content><text font=\"matrix.ttf\">a</text></content></page></pdf>";
        let node = crate::parser::parse(code).unwrap();
        let fonts = vec!["docs/examples/matrix.ttf".to_string()];
        let pdft = to_pdft(node, &Vec::new(), &fonts);
        let buffer = pdft.to_buffer();
        let pdf_string = String::from_utf8_lossy(&buffer);
        assert!(pdf_string.contains("/FontFile2"));
    }
}
