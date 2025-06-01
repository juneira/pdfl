mod constants;

use constants::*;
use std::collections::HashMap;

pub use constants::Token;

pub fn lex(code: &'static str) -> Result<Vec<Token>, String> {
    let dict: HashMap<char, usize> = DICT_ENTRIES.iter().cloned().collect();

    let mut current_node: i32 = 0;
    let mut buff: String = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    for (i, c) in code.chars().enumerate() {
        // Estado de Esperada de String ou TAG
        if current_node == 0 {
            if c == '<' {
                // Inicia o estado de TAG
                current_node = 3;
                buff.push(c);
                continue;
            }

            if is_valid_char_to_token_string(c) {
                // Inicia o estado de String
                current_node = 1;
                buff.push(c);
                continue;
            }

            if is_ignored_char(c) {
                // Ignora espaços em branco
                continue;
            }

            return Err(invalid_char_error(c, i, current_node));
        }

        // Estado de String
        if current_node == 1 {
            if is_valid_char_to_token_string(c) || is_ignored_char(c) {
                buff.push(c);
                continue;
            }

            if c == '<' {
                tokens.push(buffer_to_token(&buff));
                buff.clear();
                buff.push(c);

                // Estado de Inicio de TAG - "<"
                current_node = 3;
                continue;
            }

            return Err(invalid_char_error(c, i, current_node));
        }

        if let Some(&current_c) = dict.get(&c) {
            current_node = RULES[current_node as usize][current_c];
            buff.push(c);

            // Estado inválido
            if current_node == -1 {
                return Err(unexcepted_token_error(c, i));
            }

            // Estado de Fim de TAG - ">"
            if current_node == 21 {
                tokens.push(buffer_to_token(&buff));
                buff.clear();

                current_node = 0;
            }

            continue;
        }

        return Err(invalid_char_error(c, i, current_node));
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
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

fn is_ignored_char(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\r' || c == '\t'
}

fn invalid_char_error(c: char, i: usize, current_node: i32) -> String {
    format!("Invalid Char '{}' on position {} - State {}", c, i, current_node)
}

fn unexcepted_token_error(c: char, i: usize) -> String {
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
        let code = "<pdf>\n  <page>\n    <content>\n      <text>texto    mais    longoooo\n      pdf\n    </text>\n    </content>\n</page>\n</pdf>";
        let tokens = lex(code).unwrap();

         assert_eq!(tokens,
            vec![
                Token::Pdf,
                Token::Page,
                Token::Content,
                Token::Text,
                Token::Str("texto    mais    longoooo\n      pdf".to_string()),
                Token::EText,
                Token::EContent,
                Token::EPage,
                Token::EPdf,
            ]
        );
    }

    #[test]
    fn test_lex_invalid_char() {
        let code = "<pdf>@</pdf>";
        let result = lex(code);
        assert!(result.is_err());
    }
}
