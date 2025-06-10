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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_pdf() {
        let input = "        <pdf>

        <page>     <content>

<text>  one   two
three</text></content></page></pdf>";
        let result = parse(input);

        match result {
            Ok(pdf_node) => {
                assert_eq!(pdf_node.child_page.child_content.child_text.child_string, "one   two\nthree".to_string());
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
}
