pub struct RectangleNode {
    pub x_pos: usize,
    pub y_pos: usize,
    pub width: usize,
    pub height: usize,
    pub color: (u8, u8, u8),
}

impl RectangleNode {
    pub fn to_obj(&self) -> String {
        let (r, g, b) = self.color;
        let color_str = format!("{} {} {} rg", r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        format!("{}\n{} {} {} {} re\nf", color_str, self.x_pos, self.y_pos, self.width, self.height)
    }
}
