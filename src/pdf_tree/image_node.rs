pub struct ImageNode {
    pub name: String,
    pub x_pos: usize,
    pub y_pos: usize,
    pub width: usize,
    pub height: usize,
}

impl ImageNode {
    pub fn to_obj(&self) -> String {
        format!(
            "q\n{} 0 0 {} {} {} cm\n/{} Do\nQ",
            self.width, self.height, self.x_pos, self.y_pos, self.name
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
        };

        let obj = node.to_obj();
        assert!(obj.contains("/img Do"));
        assert!(obj.contains("30 0 0 40 10 20 cm"));
    }
}
