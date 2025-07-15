pub struct FontNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub subtype: String,
    pub base_font: String,
    pub file_obj_num: Option<usize>,
    pub data: Option<Vec<u8>>,
}

impl FontNode {
    pub fn obj_bytes(&self) -> Vec<u8> {
        self.to_obj().into_bytes()
    }

    pub fn file_bytes(&self) -> Option<Vec<u8>> {
        if let (Some(obj_num), Some(data)) = (self.file_obj_num, &self.data) {
            let mut buf = Vec::new();
            buf.extend(
                format!(
                    "{} {} obj\n<< /Length {}>>\nstream\n",
                    obj_num,
                    self.gen_num,
                    data.len()
                )
                .as_bytes(),
            );
            buf.extend(data);
            buf.extend(b"\nendstream\nendobj\n");
            Some(buf)
        } else {
            None
        }
    }

    fn to_obj(&self) -> String {
        match self.file_obj_num {
            Some(file_obj) => format!(
                "{} {} obj\n<< /Type /Font\n/Subtype /{}\n/BaseFont /{}\n/FontFile2 {} {} R\n>>\nendobj\n",
                self.obj_num, self.gen_num, self.subtype, self.base_font, file_obj, self.gen_num
            ),
            None => format!(
                "{} {} obj\n<< /Type /Font\n/Subtype /{}\n/BaseFont /{}\n>>\nendobj\n",
                self.obj_num, self.gen_num, self.subtype, self.base_font
            ),
        }
    }
}
