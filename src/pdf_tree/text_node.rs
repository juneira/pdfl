pub struct TextNode {
    pub font: String,
    pub font_size: usize,
    pub x_pos: usize,
    pub y_pos: usize,
    pub text: String,
}

impl TextNode {
    pub fn to_obj(&self) -> String {
        return format!(
            "BT\n/{} {} Tf\n{} {} Td\n({}) Tj\nET",
            self.font, self.font_size, self.x_pos, self.y_pos, self.text
        );
    }
}
