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
                assert_eq!(pdf_node.child_page.child_content.child_texts[0].child_string, "one   two\nthree".to_string());
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
        assert_eq!(result.child_page.child_content.child_texts.len(), 2);
        assert_eq!(result.child_page.child_content.child_texts[0].child_string, "one");
        assert_eq!(result.child_page.child_content.child_texts[1].child_string, "two");
    }

    #[test]
    fn test_parse_text_with_attributes() {
        let input = "<pdf><page><resource><font key=\"f1\" /></resource><content><text font=\"f1\" pos_x=\"20\" pos_y=\"50\">hello</text></content></page></pdf>";
        let result = parse(input).unwrap();
        let text = &result.child_page.child_content.child_texts[0];
        assert_eq!(text.child_string, "hello");
        assert_eq!(text.attributes.get("pos_x"), Some(&"20".to_string()));
        assert_eq!(text.attributes.get("pos_y"), Some(&"50".to_string()));
    }
}
