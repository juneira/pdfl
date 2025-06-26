mod constants;

pub use constants::*;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub fn parse(input: &str) -> Result<PdfNode, String> {
    let parser = grammar::PdfParser::new();

    match parser.parse(input) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

pub fn parse_attributes(input: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    for part in input.split_whitespace() {
        if let Some((k, v)) = part.split_once('=') {
            let v = v.trim_matches('"');
            map.insert(k.to_string(), v.to_string());
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_pdf() {
        let input = "        <pdf>

        <page>     <resource><font key=\"f1\" /></resource> <content>

<text font=\"f1\">  one   two
three</text></content></page></pdf>";
        let result = parse(input);

        match result {
            Ok(pdf_node) => {
                match &pdf_node.child_page.child_content.children[0] {
                    ContentElement::Text(t) => assert_eq!(t.child_string, "one   two\nthree".to_string()),
                    _ => panic!("Expected text node"),
                }
            }
            Err(e) => panic!("Expected Ok, got Err: {}", e),
        }
    }

    #[test]
    fn test_parse_invalid_pdf() {
        let input = "<pdf><page></pdf>";
        let result = parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_multiple_texts() {
        let input = "<pdf><page><resource><font key=\"f1\" /></resource><content><text font=\"f1\">one</text><text font=\"f1\">two</text></content></page></pdf>";
        let result = parse(input).unwrap();
        assert_eq!(result.child_page.child_content.children.len(), 2);
        match &result.child_page.child_content.children[0] {
            ContentElement::Text(t) => assert_eq!(t.child_string, "one"),
            _ => panic!("Expected text"),
        }
        match &result.child_page.child_content.children[1] {
            ContentElement::Text(t) => assert_eq!(t.child_string, "two"),
            _ => panic!("Expected text"),
        }
    }

    #[test]
    fn test_parse_text_with_attributes() {
        let input = "<pdf><page><resource><font key=\"f1\" /></resource><content><text font=\"f1\" pos_x=\"20\" pos_y=\"50\">hello</text></content></page></pdf>";
        let result = parse(input).unwrap();
        match &result.child_page.child_content.children[0] {
            ContentElement::Text(text) => {
                assert_eq!(text.child_string, "hello");
                assert_eq!(text.attributes.get("pos_x"), Some(&"20".to_string()));
                assert_eq!(text.attributes.get("pos_y"), Some(&"50".to_string()));
            }
            _ => panic!("Expected text"),
        }
    }

    #[test]
    fn test_parse_rectangle() {
        let input = "<pdf><page><content><rectangle pos_x=\"10\" pos_y=\"20\" width=\"30\" height=\"40\" /></content></page></pdf>";
        let result = parse(input).unwrap();
        match &result.child_page.child_content.children[0] {
            ContentElement::Rectangle(rect) => {
                assert_eq!(rect.attributes.get("pos_x"), Some(&"10".to_string()));
                assert_eq!(rect.attributes.get("pos_y"), Some(&"20".to_string()));
                assert_eq!(rect.attributes.get("width"), Some(&"30".to_string()));
                assert_eq!(rect.attributes.get("height"), Some(&"40".to_string()));
            }
            _ => panic!("Expected rectangle"),
        }
    }

    #[test]
    fn test_parse_line() {
        let input = "<pdf><page><content><line pos_x=\"15\" pos_y=\"25\" width=\"35\" /></content></page></pdf>";
        let result = parse(input).unwrap();
        match &result.child_page.child_content.children[0] {
            ContentElement::Line(line) => {
                assert_eq!(line.attributes.get("pos_x"), Some(&"15".to_string()));
                assert_eq!(line.attributes.get("pos_y"), Some(&"25".to_string()));
                assert_eq!(line.attributes.get("width"), Some(&"35".to_string()));
            }
            _ => panic!("Expected line"),
        }
    }

    #[test]
    fn test_parse_circle() {
        let input = "<pdf><page><content><circle pos_x=\"5\" pos_y=\"10\" width=\"20\" height=\"20\" /></content></page></pdf>";
        let result = parse(input).unwrap();
        match &result.child_page.child_content.children[0] {
            ContentElement::Circle(circ) => {
                assert_eq!(circ.attributes.get("pos_x"), Some(&"5".to_string()));
                assert_eq!(circ.attributes.get("pos_y"), Some(&"10".to_string()));
                assert_eq!(circ.attributes.get("width"), Some(&"20".to_string()));
                assert_eq!(circ.attributes.get("height"), Some(&"20".to_string()));
            }
            _ => panic!("Expected circle"),
        }
    }

    #[test]
    fn test_parse_image() {
        let input = "<pdf><page><content><image src=\"img.png\" pos_x=\"1\" pos_y=\"2\" width=\"3\" height=\"4\" /></content></page></pdf>";
        let result = parse(input).unwrap();
        match &result.child_page.child_content.children[0] {
            ContentElement::Image(img) => {
                assert_eq!(img.attributes.get("src"), Some(&"img.png".to_string()));
                assert_eq!(img.attributes.get("pos_x"), Some(&"1".to_string()));
                assert_eq!(img.attributes.get("pos_y"), Some(&"2".to_string()));
                assert_eq!(img.attributes.get("width"), Some(&"3".to_string()));
                assert_eq!(img.attributes.get("height"), Some(&"4".to_string()));
            }
            _ => panic!("Expected image"),
        }
    }

    #[test]
    fn test_parse_image_rotation() {
        let input = "<pdf><page><content><image src=\"img.png\" rotation=\"15\" /></content></page></pdf>";
        let result = parse(input).unwrap();
        match &result.child_page.child_content.children[0] {
            ContentElement::Image(img) => {
                assert_eq!(img.attributes.get("rotation"), Some(&"15".to_string()));
            }
            _ => panic!("Expected image"),
        }
    }
}
