use crate::ast2pdft::attribute_parser::AttributeParser;

pub struct ElementConverter;

impl ElementConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert(&self, element: &crate::parser::ContentElement) -> Option<crate::pdf_tree::ContentItem> {
        match element {
            crate::parser::ContentElement::Text(t) => {
                Some(crate::pdf_tree::ContentItem::Text(self.convert_text(t)))
            }
            crate::parser::ContentElement::Rectangle(r) => {
                Some(crate::pdf_tree::ContentItem::Rectangle(self.convert_rectangle(r)))
            }
            crate::parser::ContentElement::Line(l) => {
                Some(crate::pdf_tree::ContentItem::Line(self.convert_line(l)))
            }
            crate::parser::ContentElement::Circle(c) => {
                Some(crate::pdf_tree::ContentItem::Circle(self.convert_circle(c)))
            }
            crate::parser::ContentElement::Image(i) => {
                Some(crate::pdf_tree::ContentItem::Image(self.convert_image(i)))
            }
        }
    }

    fn convert_text(&self, ast_text: &crate::parser::TextNode) -> crate::pdf_tree::TextNode {
        let attribute_parser = AttributeParser::new();

        let x_pos = attribute_parser.parse_usize(&ast_text.attributes, "pos_x", 100);
        let y_pos = attribute_parser.parse_usize(&ast_text.attributes, "pos_y", 700);
        let font_size = attribute_parser.parse_usize(&ast_text.attributes, "font_size", 24);
        let font = attribute_parser.parse_string(&ast_text.attributes, "font", "F1");
        let color = attribute_parser.parse_color(&ast_text.attributes, "color", (0, 0, 0));
        let rotation = attribute_parser.parse_f32(&ast_text.attributes, "rotation", 0.0);

        crate::pdf_tree::TextNode {
            font,
            font_size,
            x_pos,
            y_pos,
            text: ast_text.child_string.clone(),
            color,
            rotation,
        }
    }

    fn convert_rectangle(&self, ast_rect: &crate::parser::RectangleNode) -> crate::pdf_tree::RectangleNode {
        let attribute_parser = AttributeParser::new();

        let x_pos = attribute_parser.parse_usize(&ast_rect.attributes, "pos_x", 50);
        let y_pos = attribute_parser.parse_usize(&ast_rect.attributes, "pos_y", 50);
        let width = attribute_parser.parse_usize(&ast_rect.attributes, "width", 50);
        let height = attribute_parser.parse_usize(&ast_rect.attributes, "height", 50);
        let rotation = attribute_parser.parse_f32(&ast_rect.attributes, "rotation", 0.0);
        let color = attribute_parser.parse_color(&ast_rect.attributes, "color", (0, 0, 0));

        crate::pdf_tree::RectangleNode {
            x_pos,
            y_pos,
            width,
            height,
            rotation,
            color,
        }
    }

    fn convert_line(&self, ast_line: &crate::parser::LineNode) -> crate::pdf_tree::LineNode {
        let attribute_parser = AttributeParser::new();

        let x_pos = attribute_parser.parse_usize(&ast_line.attributes, "pos_x", 50);
        let y_pos = attribute_parser.parse_usize(&ast_line.attributes, "pos_y", 50);
        let width = attribute_parser.parse_usize(&ast_line.attributes, "width", 50);
        let rotation = attribute_parser.parse_f32(&ast_line.attributes, "rotation", 0.0);
        let color = attribute_parser.parse_color(&ast_line.attributes, "color", (0, 0, 0));

        crate::pdf_tree::LineNode {
            x_pos,
            y_pos,
            width,
            color,
            rotation,
        }
    }

    fn convert_circle(&self, ast_circle: &crate::parser::CircleNode) -> crate::pdf_tree::CircleNode {
        let attribute_parser = AttributeParser::new();

        let x_pos = attribute_parser.parse_usize(&ast_circle.attributes, "pos_x", 50);
        let y_pos = attribute_parser.parse_usize(&ast_circle.attributes, "pos_y", 50);
        let width = attribute_parser.parse_usize(&ast_circle.attributes, "width", 50);
        let height = attribute_parser.parse_usize(&ast_circle.attributes, "height", 50);
        let color = attribute_parser.parse_color(&ast_circle.attributes, "color", (0, 0, 0));

        crate::pdf_tree::CircleNode {
            x_pos,
            y_pos,
            width,
            height,
            color,
        }
    }

    fn convert_image(&self, ast_image: &crate::parser::ImageNode) -> crate::pdf_tree::ImageNode {
        let attribute_parser = AttributeParser::new();

        let name = ast_image
            .attributes
            .get("src")
            .expect("src attribute missing")
            .to_string();
        let x_pos = attribute_parser.parse_usize(&ast_image.attributes, "pos_x", 50);
        let y_pos = attribute_parser.parse_usize(&ast_image.attributes, "pos_y", 50);
        let width = attribute_parser.parse_usize(&ast_image.attributes, "width", 50);
        let height = attribute_parser.parse_usize(&ast_image.attributes, "height", 50);
        let rotation = attribute_parser.parse_f32(&ast_image.attributes, "rotation", 0.0).clamp(0.0, 360.0);

        crate::pdf_tree::ImageNode {
            name,
            x_pos,
            y_pos,
            width,
            height,
            rotation,
        }
    }
}
