mod constants;

use constants::*;
use std::collections::HashMap;

pub use constants::Token;

pub fn lex(code: &str) -> Result<Vec<Token>, String> {
    let dict: HashMap<char, usize> = DICT_ENTRIES.iter().cloned().collect();

    let mut current_node: i32 = 0;
    let mut buff: String = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    for (i, c) in code.chars().enumerate() {
        // Wait String or TAG
        if current_node == 0 {
            if c == '<' {
                // Start TAG's state
                current_node = 3;
                buff.push(c);
                continue;
            }

            if is_valid_char_to_token_string(c) {
                // Start String's state
                current_node = 1;
                buff.push(c);
                continue;
            }

            if is_ignored_char(c) {
                // Ignores whitespace characters and new lines
                continue;
            }

            return Err(invalid_char_error(c, i, current_node));
        }

        // String's state
        if current_node == 1 {
            if is_valid_char_to_token_string(c) || is_ignored_char(c) {
                buff.push(c);
                continue;
            }

            if c == '<' {
                tokens.push(buffer_to_token(&buff));
                buff.clear();
                buff.push(c);

                // Start TAG's state - "<"
                current_node = 3;
                continue;
            }

            return Err(invalid_char_error(c, i, current_node));
        }

        if let Some(&current_c) = dict.get(&c) {
            current_node = RULES[current_node as usize][current_c];
            buff.push(c);

            // State invalid
            if current_node == -1 {
                return Err(unexpected_token_error(c, i));
            }

            // End Tag's state - ">"
            if current_node == 21 {
                tokens.push(buffer_to_token(&buff));
                buff.clear();

                current_node = 0;
            }

            continue;
        }

        return Err(invalid_char_error(c, i, current_node));
    }

    if current_node == 1 && !buff.trim().is_empty() {
        tokens.push(buffer_to_token(&buff));
    }

    return Ok(tokens);
}

fn buffer_to_token(buffer: &String) -> Token {
    match buffer.as_str() {
        "<pdf>" => Token::Pdf,
        "</pdf>" => Token::EPdf,
        "<content>" => Token::Content,
        "</content>" => Token::EContent,
        "<page>" => Token::Page,
        "</page>" => Token::EPage,
        "<text>" => Token::Text,
        "</text>" => Token::EText,
        _ => Token::Str(buffer.trim().to_string()),
    }
}

fn is_valid_char_to_token_string(c: char) -> bool {
    c.is_ascii_graphic() && c != '<'
}

fn is_ignored_char(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\r' || c == '\t'
}

fn invalid_char_error(c: char, i: usize, current_node: i32) -> String {
    format!("Invalid character '{}' at position {} - State {}", c, i, current_node)
}

fn unexpected_token_error(c: char, i: usize) -> String {
    format!("Unexpected token '{}' at position {}", c, i)
}

// Testes
#[cfg(test)]
mod tests {
    use crate::lexer::{lex, Token};

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
