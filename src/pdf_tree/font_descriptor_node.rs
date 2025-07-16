pub struct FontDescriptorNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub font_name: String,
    pub file_obj_num: usize,
}

impl FontDescriptorNode {
    pub fn to_buffer(&self) -> Vec<u8> {
        format!(
            "{} {} obj\n<< /Type /FontDescriptor\n/FontName /{}\n/Flags 0\n/FontBBox [0 0 0 0]\n/ItalicAngle 0\n/Ascent 0\n/Descent 0\n/CapHeight 0\n/StemV 0\n/FontFile2 {} {} R\n>>\nendobj\n",
            self.obj_num,
            self.gen_num,
            self.font_name,
            self.file_obj_num,
            self.gen_num
        )
        .into_bytes()
    }
}
