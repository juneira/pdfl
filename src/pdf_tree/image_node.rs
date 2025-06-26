pub struct ImageNode {
    pub name: String,
    pub x_pos: usize,
    pub y_pos: usize,
    pub width: usize,
    pub height: usize,
    pub rotation: f32,
}

impl ImageNode {
    pub fn to_obj(&self) -> String {
        let theta = self.rotation.to_radians();
        let cos_t = theta.cos();
        let sin_t = theta.sin();
        let a = self.width as f32 * cos_t;
        let b = self.width as f32 * sin_t;
        let c = -(self.height as f32) * sin_t;
        let d = self.height as f32 * cos_t;
        format!(
            "q\n{} {} {} {} {} {} cm\n/{} Do\nQ",
            a, b, c, d, self.x_pos, self.y_pos, self.name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_obj() {
        let node = ImageNode {
            name: "img".to_string(),
            x_pos: 10,
            y_pos: 20,
            width: 30,
            height: 40,
            rotation: 0.0,
        };

        let obj = node.to_obj();
        assert!(obj.contains("/img Do"));
        assert!(obj.contains("30 0 -0 40 10 20 cm"));
    }
}
