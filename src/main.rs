mod lexer;

fn main() {
    let code: &'static str = "
    <pdf>
        <page>
            <content>
                <text>
                    texto    mais    longoooo
                    pdf
                </text>
            </content>
        </page>
        <page>
            <content>
                <text>
                    Outro texto
                </text>
            </content>
        </page>
    </pdf>
    ";

    match lexer::lex(code) {
        Ok(tokens) => {
            for token in &tokens {
                match token {
                    lexer::Token::Pdf => println!("PDF"),
                    lexer::Token::EPdf => println!("EPDF"),
                    lexer::Token::Content => println!("Content"),
                    lexer::Token::EContent => println!("EContent"),
                    lexer::Token::Page => println!("Page"),
                    lexer::Token::EPage => println!("EPage"),
                    lexer::Token::Text => println!("Text"),
                    lexer::Token::EText => println!("EText"),
                    lexer::Token::Str(str) => println!("Str({})", str)
                }
            }
        },
        Err(e) => println!("{}", e)
    }
}
