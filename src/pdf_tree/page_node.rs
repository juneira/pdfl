use super::content_node::ContentNode;
use super::font_node::FontNode;
use super::image_xobject_node::ImageXObjectNode;
use std::collections::HashMap;

pub struct PageNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub fonts: HashMap<String, FontNode>,
    pub images: HashMap<String, ImageXObjectNode>,
    pub contents: ContentNode,
}

impl PageNode {
    pub fn to_buffer(&self, xref: &mut Vec<usize>) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.to_obj().as_bytes());

        for font in self.fonts.values() {
            xref.push(xref.last().unwrap() + buffer.len() + 1);
            buffer.extend(font.obj_bytes());
            if let Some(bytes) = font.file_bytes() {
                xref.push(xref.last().unwrap() + buffer.len() + 1);
                buffer.extend(bytes);
            }
        }

        for image in self.images.values() {
            xref.push(xref.last().unwrap() + buffer.len() + 1);
            buffer.extend(image.to_buffer());
        }

        xref.push(xref.last().unwrap() + buffer.len() + 1);
        buffer.extend(self.contents.to_buffer());

        return buffer;
    }

    fn to_obj(&self) -> String {
        let fonts_str: Vec<String> = self
            .fonts
            .iter()
            .map(|(key, font)| format!("/{} {} {} R", key, font.obj_num, font.gen_num))
            .collect();

        let images_str: Vec<String> = self
            .images
            .iter()
            .map(|(key, img)| format!("/{} {} {} R", key, img.obj_num, img.gen_num))
            .collect();

        let mut resources_parts = Vec::new();
        resources_parts.push(format!("/Font << {} >>", fonts_str.join(" ")));
        if !images_str.is_empty() {
            resources_parts.push(format!("/XObject << {} >>", images_str.join(" ")));
        }
        let resources_joined = resources_parts.join(" ");

        format!(
            "{} {} obj\n<< /Type /Page\n/Resources << {} >>\n/Contents {} {} R\n>>\nendobj\n",
            self.obj_num,
            self.gen_num,
            resources_joined,
            self.contents.obj_num,
            self.contents.gen_num
        )
    }
}
