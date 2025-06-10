use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\r]+")]
pub enum Token {
    #[token("<pdf>")]
    Pdf,
    #[token("</pdf>")]
    EPdf,
    #[token("<page>")]
    Page,
    #[token("</page>")]
    EPage,
    #[token("<content>")]
    Content,
    #[token("</content>")]
    EContent,
    #[token("<text>")]
    Text,
    #[token("</text>")]
    EText,
    #[regex(r"[^\s<][^<]*", |lex| {
        let slice = lex.slice();
        if slice.is_ascii() {
            Ok(slice.trim().to_string())
        } else {
            Err(())
        }
    })]
    Str(String),
}

pub fn lex(code: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(code);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(tok) => tokens.push(tok),
            Err(_) => {
                return Err(format!("Invalid token '{}' at position {}", lexer.slice(), lexer.span().start));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_simple_tags() {
        let code = "<pdf><page><content><text>abc</text></content></page></pdf>";
        let tokens = lex(code).unwrap();
        assert_eq!(tokens,
            vec![
                Token::Pdf,
                Token::Page,
                Token::Content,
                Token::Text,
                Token::Str("abc".to_string()),
                Token::EText,
                Token::EContent,
                Token::EPage,
                Token::EPdf,
            ]
        );
    }

    #[test]
    fn test_lex_with_spaces_and_newlines() {
        let code = "<pdf>\n  <page>\n    <content>\n      <text>text    too    loooong\n      pdf\n    </text>\n    </content>\n</page>\n</pdf>";
        let tokens = lex(code).unwrap();

         assert_eq!(tokens,
            vec![
                Token::Pdf,
                Token::Page,
                Token::Content,
                Token::Text,
                Token::Str("text    too    loooong\n      pdf".to_string()),
                Token::EText,
                Token::EContent,
                Token::EPage,
                Token::EPdf,
            ]
        );
    }

    #[test]
    fn test_lex_string_without_tags() {
        let code = "      string without tags    ";
        let tokens = lex(code).unwrap();

         assert_eq!(tokens,
            vec![
                Token::Str("string without tags".to_string()),
            ]
        );
    }

    #[test]
    fn test_lex_invalid_char() {
        let code = "<pdf>é</pdf>";
        assert!(lex(code).is_err());
    }

    #[test]
    fn test_lex_string_with_numbers_and_punctuation() {
        let code = "<pdf><page><content><text>Hello 123!?.</text></content></page></pdf>";
        let tokens = lex(code).unwrap();

        assert_eq!(tokens,
            vec![
                Token::Pdf,
                Token::Page,
                Token::Content,
                Token::Text,
                Token::Str("Hello 123!?.".to_string()),
                Token::EText,
                Token::EContent,
                Token::EPage,
                Token::EPdf,
            ]
        );
    }
}
