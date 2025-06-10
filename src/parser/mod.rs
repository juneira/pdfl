mod constants;

pub use constants::*;
use crate::lexer::Token;

pub fn parse(tokens: &mut Vec<Token>) -> Result<PdfNode, String> {
    return start(tokens);
}

fn start(tokens: &mut Vec<Token>) -> Result<PdfNode, String> {
    let pdf_token = tokens.pop();
    if pdf_token == None || pdf_token != Some(Token::Pdf) {
        return Err("Expected a Pdf token".to_string());
    }

    let child_page = match page(tokens) {
        Ok(page_node) => page_node,
        Err(error) => return Err(error),
    };

    let epdf_token = tokens.pop();
    if epdf_token == None || epdf_token != Some(Token::EPdf) {
        return Err("Expected a EPdf token".to_string());
    }

    return Ok(PdfNode {
        child_page: child_page,
    });
}

fn page(tokens: &mut Vec<Token>) -> Result<PageNode, String> {
    let page_token = tokens.pop();
    if page_token == None || page_token != Some(Token::Page) {
        return Err("Expected a Page token".to_string());
    }

    let child_content = match content(tokens) {
        Ok(content_node) => content_node,
        Err(error) => return Err(error),
    };

    let epage_token = tokens.pop();
    if epage_token == None || epage_token != Some(Token::EPage) {
        return Err("Expected an EPage token".to_string());
    }

    if tokens.last() == Some(&Token::EPdf) {
        return Ok(PageNode {
            child_content: child_content,
            child_page: None,
        });
    }

    match page(tokens) {
        Ok(child_page) => {
            return Ok(PageNode {
                child_content: child_content,
                child_page: Some(Box::new(child_page)),
            });
        }
        Err(error) => return Err(error),
    }
}

fn content(tokens: &mut Vec<Token>) -> Result<ContentNode, String> {
    let content_token = tokens.pop();
    if content_token == None || content_token != Some(Token::Content) {
        return Err("Expected a Content token".to_string());
    }

    let child_text = match text(tokens) {
        Ok(text_node) => text_node,
        Err(error) => return Err(error),
    };

    let econtent_token = tokens.pop();
    if econtent_token == None || econtent_token != Some(Token::EContent) {
        return Err("Expected an EContent token".to_string());
    }

    return Ok(ContentNode {
        child_text: child_text,
    });
}

fn text(tokens: &mut Vec<Token>) -> Result<TextNode, String> {
    let text_token = tokens.pop();
    if text_token == None || text_token != Some(Token::Text) {
        return Err("Expected a Text token".to_string());
    }

    let child_string = match tokens.pop() {
        Some(Token::Str(s)) => Token::Str(s),
        _ => return Err("Expected a Str token".to_string()),
    };

    let etext_token = tokens.pop();
    if etext_token == None || etext_token != Some(Token::EText) {
        return Err("Expected an EText token".to_string());
    }

    return Ok(TextNode {
        child_string: child_string,
    });
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use crate::parser::{parse, PdfNode, PageNode, ContentNode, TextNode};

    #[test]
    fn test_with_one_page() {
        let mut tokens: Vec<Token> = vec![
            Token::Pdf,
            Token::Page,
            Token::Content,
            Token::Text,
            Token::Str("longer    text    here\n      pdf".to_string()),
            Token::EText,
            Token::EContent,
            Token::EPage,
            Token::EPdf,
        ];
        tokens.reverse();

        let root = parse(tokens.as_mut());

        assert_eq!(root.unwrap(),
            PdfNode {
                child_page: PageNode {
                    child_page: None,
                    child_content: ContentNode {
                        child_text: TextNode {
                            child_string: Token::Str("longer    text    here\n      pdf".to_string()),
                        }
                    }
                }
            }
        );
    }

    #[test]
    fn test_with_two_pages() {
        let mut tokens: Vec<Token> = vec![
            Token::Pdf,
            Token::Page,
            Token::Content,
            Token::Text,
            Token::Str("longer    text    here\n      pdf".to_string()),
            Token::EText,
            Token::EContent,
            Token::EPage,
            Token::Page,
            Token::Content,
            Token::Text,
            Token::Str("short text".to_string()),
            Token::EText,
            Token::EContent,
            Token::EPage,
            Token::EPdf,
        ];
        tokens.reverse();

        let root = parse(tokens.as_mut());

        assert_eq!(root.unwrap(),
            PdfNode {
                child_page: PageNode {
                    child_content: ContentNode {
                        child_text: TextNode {
                            child_string: Token::Str("longer    text    here\n      pdf".to_string()),
                        }
                    },
                    child_page: Some(
                        Box::new(
                            PageNode {
                                child_content: ContentNode {
                                    child_text: TextNode {
                                        child_string: Token::Str("short text".to_string()),
                                    }
                                },
                                child_page: None,
                            }
                        )
                    ),
                }
            }
        );
    }

    #[test]
    fn test_when_token_of_page_is_incorrect() {
        let mut tokens: Vec<Token> = vec![
            Token::Pdf,
            Token::Page,
            Token::Content,
            Token::Text,
            Token::Str("longer    text    here\n      pdf".to_string()),
            Token::EText,
            Token::EContent,
            Token::EPage,
            Token::Page,
            Token::Content,
            Token::Text,
            Token::Str("short text".to_string()),
            Token::Text,
            Token::EContent,
            Token::EPage,
            Token::EPdf,
        ];
        tokens.reverse();

        let root = parse(tokens.as_mut());

        assert!(root.is_err());
    }

    #[test]
    fn test_when_tokens_are_empty() {
        let mut tokens: Vec<Token> = vec![];

        let root = parse(tokens.as_mut());

        assert!(root.is_err());
    }
}
