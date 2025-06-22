pub struct ImageNode {
    pub attributes: std::collections::HashMap<String, String>,
}

impl ImageNode {
    pub fn to_obj(&self) -> String {
        "% TODO: implement image drawing".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_to_obj_placeholder() {
        let image = ImageNode { attributes: HashMap::new() };
        assert_eq!(image.to_obj(), "% TODO: implement image drawing");
    }
}
