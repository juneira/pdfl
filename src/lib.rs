use wasm_bindgen::prelude::*;

mod ast2pdft;
mod parser;
mod pdf_tree;

#[wasm_bindgen]
pub fn compile_pdfl(input: &str) -> Result<Box<[u8]>, JsValue> {
    let ast = parser::parse(input).map_err(|e| JsValue::from_str(&e))?;
    let pdft = ast2pdft::to_pdft(ast, &Vec::new());
    Ok(pdft.to_buffer().into_boxed_slice())
}
