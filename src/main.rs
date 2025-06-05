mod lexer;
mod parser;
mod pdf_tree;

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

            println!("{:?}", node);
        },
        Err(e) => println!("{}", e)
    }
}
