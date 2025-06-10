mod ast2pdft;
mod parser;
mod pdf_tree;

use std::io::{self, Read};

fn main() {
    let mut code = String::new();
    io::stdin().read_to_string(&mut code).expect("Failed to read from stdin");

    let ast = parser::parse(&code).unwrap();

    let pdft = ast2pdft::to_pdft(ast);

    let node = pdft.to_buffer();

    let file = std::fs::File::create("output.pdf");
    match file {
        Ok(mut file) => {
            use std::io::Write;
            file.write_all(&node).unwrap();
            println!("PDF generated successfully!");
        }
        Err(e) => println!("Error creating file: {}", e),
    }
}
