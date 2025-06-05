pub struct FontNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub subtype: String,
    pub base_font: String,
}

impl FontNode {
    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.to_obj().as_bytes());

        return buffer;
    }

    fn to_obj(&self) -> String {
        return format!(
            "{} {} obj\n<< /Type /Font\n/Subtype /{}\n/BaseFont /{}\n>>\nendobj\n",
            self.obj_num, self.gen_num, self.subtype, self.base_font
        );
    }
}
