mod lexer;
mod parser;
mod pdf_tree;
mod ast2pdft;

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
        Ok(mut tokens) => {
            tokens.reverse();

            let node = parser::parse(tokens.as_mut());

            let pdft = ast2pdft::to_pdft(node.unwrap());

            let node = pdft.to_buffer();

            println!("{}", String::from_utf8(node).unwrap());

        },
        Err(e) => println!("{}", e)
    }
}
