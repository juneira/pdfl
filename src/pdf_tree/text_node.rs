pub struct TextNode {
    pub font: String,
    pub font_size: usize,
    pub x_pos: usize,
    pub y_pos: usize,
    pub text: String,
    pub color: (u8, u8, u8),
    pub rotation: f32,
}

impl TextNode {
    pub fn to_obj(&self) -> String {
        let (r, g, b) = self.color;
        let color_str = format!("{} {} {} rg", r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

        if self.rotation == 0.0 {
            return format!(
                "BT\n/{} {} Tf\n{}\n{} {} Td\n({}) Tj\nET",
                self.font, self.font_size, color_str, self.x_pos, self.y_pos, self.text
            );
        }

        let theta = self.rotation.to_radians();
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        return format!(
            "q\n{} {} {} {} {} {} cm\nBT\n/{} {} Tf\n{}\n0 0 Td\n({}) Tj\nET\nQ",
            cos_t,
            sin_t,
            -sin_t,
            cos_t,
            self.x_pos,
            self.y_pos,
            self.font,
            self.font_size,
            color_str,
            self.text
        );
    }
}
