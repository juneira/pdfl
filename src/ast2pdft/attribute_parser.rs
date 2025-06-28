use std::collections::HashMap;

pub struct AttributeParser;

impl AttributeParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_usize(&self, attributes: &HashMap<String, String>, key: &str, default: usize) -> usize {
        attributes
            .get(key)
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(default)
    }

    pub fn parse_f32(&self, attributes: &HashMap<String, String>, key: &str, default: f32) -> f32 {
        attributes
            .get(key)
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(default)
    }

    pub fn parse_string(&self, attributes: &HashMap<String, String>, key: &str, default: &str) -> String {
        attributes
            .get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    pub fn parse_color(&self, attributes: &HashMap<String, String>, key: &str, default: (u8, u8, u8)) -> (u8, u8, u8) {
        attributes
            .get(key)
            .map(|v| v.trim_start_matches('#'))
            .and_then(|v| u32::from_str_radix(v, 16).ok())
            .map(|rgb| (((rgb >> 16) & 0xff) as u8, ((rgb >> 8) & 0xff) as u8, (rgb & 0xff) as u8))
            .unwrap_or(default)
    }
}
