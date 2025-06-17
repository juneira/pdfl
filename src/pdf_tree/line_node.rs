pub struct LineNode {
    pub x_pos: usize,
    pub y_pos: usize,
    pub width: usize,
    pub color: (u8, u8, u8),
}

impl LineNode {
    pub fn to_obj(&self) -> String {
        let (r, g, b) = self.color;
        let color_str = format!("{} {} {} RG", r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let end_x = self.x_pos + self.width;
        format!("{}\n{} {} m\n{} {} l\nS", color_str, self.x_pos, self.y_pos, end_x, self.y_pos)
    }
}
