use super::pages_node::PagesNode;
use super::content_node::ContentNode;
use super::font_node::FontNode;
use std::collections::HashMap;

pub struct PageNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub resources: HashMap<String, FontNode>,
    pub contents: ContentNode,
    pub parent: Option<Box<PagesNode>>,
}

impl PageNode {
    pub fn to_buffer(&self, xref: &mut Vec<usize>) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.to_obj().as_bytes());

        for font in self.resources.values() {
            xref.push(xref.last().unwrap() + buffer.len() + 1);
            buffer.extend(font.to_buffer());
        }

        xref.push(xref.last().unwrap() + buffer.len() + 1);
        buffer.extend(self.contents.to_buffer());

        return buffer;
    }

    fn to_obj(&self) -> String {
        let resources_str: Vec<String> = self.resources.iter()
            .map(|(key, font)| format!("/{} {} {} R", key, font.obj_num, font.gen_num))
            .collect();

        return format!(
            "{} {} obj\n<< /Type /Page\n/Resources << /Font << {} >> >>\n/Contents {} {} R\n>>\nendobj\n",
            self.obj_num,
            self.gen_num,
            resources_str.join(" "),
            self.contents.obj_num,
            self.contents.gen_num
        );
    }
}
