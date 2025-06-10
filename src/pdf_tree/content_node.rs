use super::text_node::TextNode;

pub struct ContentNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub contents: Vec<TextNode>,
}

impl ContentNode {
    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.to_obj().as_bytes());

        return buffer;
    }

    fn to_obj(&self) -> String {
        let texts: Vec<String> = self.contents.iter().map(|t| t.to_obj()).collect();
        let joined = texts.join("\n");

        return format!(
            "{} {} obj\n<< /Length {}>>\nstream\n{}\nendstream\nendobj\n",
            self.obj_num,
            self.gen_num,
            joined.as_bytes().len(),
            joined
        );
    }
}
