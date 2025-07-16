mod ast2pdft;
mod parser;
mod pdf_tree;

use std::io::Write;
use std::fs;
use clap::Parser;

#[derive(Parser)]
#[command(name = "pdfl")]
#[command(about = "A PDF generator from custom language")]
struct Args {
    /// Path to the input code file
    input_file: String,

    /// Output PDF filename
    output_file: String,

    /// Image paths (can be specified multiple times)
    #[arg(short, long = "image", action = clap::ArgAction::Append)]
    images: Vec<String>,

    /// Font paths (can be specified multiple times)
    #[arg(short, long = "font", action = clap::ArgAction::Append)]
    fonts: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let code = fs::read_to_string(&args.input_file)
        .expect("Failed to read input file");

    let ast = parser::parse(&code).unwrap();

    let pdft = ast2pdft::to_pdft(ast, &args.images, &args.fonts);

    let node = pdft.to_buffer();

    let file = std::fs::File::create(&args.output_file);
    match file {
        Ok(mut file) => {
            file.write_all(&node).unwrap();
            println!("PDF generated successfully!");
        }
        Err(e) => println!("Error creating file: {}", e),
    }
}
