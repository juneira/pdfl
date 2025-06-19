pub struct CircleNode {
    pub x_pos: usize,
    pub y_pos: usize,
    pub width: usize,
    pub height: usize,
    pub color: (u8, u8, u8),
}

impl CircleNode {
    pub fn to_obj(&self) -> String {
        let (r, g, b) = self.color;
        let color_str = format!("{} {} {} rg", r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let cx = self.x_pos as f32 + self.width as f32 / 2.0;
        let cy = self.y_pos as f32 + self.height as f32 / 2.0;
        let rx = self.width as f32 / 2.0;
        let ry = self.height as f32 / 2.0;
        let k = 0.5522847498_f32;
        format!(
            "{}\n{} {} m\n{} {} {} {} {} {} c\n{} {} {} {} {} {} c\n{} {} {} {} {} {} c\n{} {} {} {} {} {} c\nf",
            color_str,
            cx + rx,
            cy,
            cx + rx,
            cy + ry * k,
            cx + rx * k,
            cy + ry,
            cx,
            cy + ry,
            cx - rx * k,
            cy + ry,
            cx - rx,
            cy + ry * k,
            cx - rx,
            cy,
            cx - rx,
            cy - ry * k,
            cx - rx * k,
            cy - ry,
            cx,
            cy - ry,
            cx + rx * k,
            cy - ry,
            cx + rx,
            cy - ry * k,
            cx + rx,
            cy
        )
    }
}
