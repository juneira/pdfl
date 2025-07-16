use crate::ast2pdft::content_converter::ContentConverter;
use crate::ast2pdft::font_converter::FontConverter;
use crate::ast2pdft::image_converter::ImageConverter;
use std::collections::HashMap;

pub struct PageConverter;

impl PageConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert(
        &self,
        ast_page: &crate::parser::PageNode,
        obj_num: usize,
        gen_num: usize,
        images: &[String],
        fonts_paths: &[String],
    ) -> (crate::pdf_tree::PageNode, usize) {
        let mut fonts = HashMap::new();
        let mut images_map = HashMap::new();
        let mut next_obj = obj_num + 1;

        let font_converter = FontConverter::new();
        if let Some(res) = &ast_page.resources {
            for font in &res.fonts {
                let (key, font_node, used) = font_converter.convert(font, next_obj, gen_num);
                fonts.insert(key, font_node);
                next_obj += used;
            }
        } else {
            fonts.insert(
                "F1".to_string(),
                font_converter.create_default(next_obj, gen_num),
            );
            next_obj += 1;
        }

        for font_path in fonts_paths {
            let (name, font_node, used) =
                font_converter.convert_file(font_path, next_obj, gen_num);
            if !fonts.contains_key(&name) {
                fonts.insert(name, font_node);
                next_obj += used;
            }
        }

        let image_converter = ImageConverter::new();
        for img_path in images {
            let (name, image_xobject) =
                image_converter.convert_xobject(img_path, next_obj, gen_num);
            images_map.insert(name, image_xobject);
            next_obj += 1;
        }

        let content_converter = ContentConverter::new();
        let content_node = content_converter.convert(&ast_page.child_content, next_obj, gen_num);
        next_obj += 1;

        (
            crate::pdf_tree::PageNode {
                obj_num,
                gen_num,
                fonts,
                images: images_map,
                contents: content_node,
            },
            next_obj - obj_num,
        )
    }
}
