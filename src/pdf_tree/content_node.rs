use super::text_node::TextNode;

pub struct ContentNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub content: TextNode,
}

impl ContentNode {
    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.to_obj().as_bytes());

        return buffer;
    }

    fn to_obj(&self) -> String {
        let text = self.content.to_obj();

        return format!(
            "{} {} obj\n<< /Length {}>>\nstream\n{}\nendstream\nendobj\n",
            self.obj_num,
            self.gen_num,
            text.as_bytes().len(),
            text
        );
    }
}
