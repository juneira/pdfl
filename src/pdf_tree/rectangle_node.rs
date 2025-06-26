pub struct RectangleNode {
    pub x_pos: usize,
    pub y_pos: usize,
    pub width: usize,
    pub height: usize,
    pub color: (u8, u8, u8),
    pub rotation: f32,
}

impl RectangleNode {
    pub fn to_obj(&self) -> String {
        let (r, g, b) = self.color;
        let color_str = format!("{} {} {} rg", r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let theta = self.rotation.to_radians();
        let cos_t = theta.cos();
        let sin_t = theta.sin();
        format!(
            "q\n{}\n{} {} {} {} {} {} cm\n0 0 {} {} re\nf\nQ",
            color_str,
            cos_t,
            sin_t,
            -sin_t,
            cos_t,
            self.x_pos,
            self.y_pos,
            self.width,
            self.height,
        )
    }
}
