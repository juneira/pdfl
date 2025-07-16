pub struct FontNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub subtype: String,
    pub base_font: String,
    pub descriptor_obj_num: Option<usize>,
    pub file_obj_num: Option<usize>,
    pub data: Option<Vec<u8>>,
    pub length1: Option<usize>,
}

impl FontNode {
    pub fn obj_bytes(&self) -> Vec<u8> {
        self.to_obj().into_bytes()
    }

    pub fn descriptor_bytes(&self) -> Option<Vec<u8>> {
        if let (Some(desc_obj), Some(file_obj)) = (self.descriptor_obj_num, self.file_obj_num) {
            let desc = crate::pdf_tree::FontDescriptorNode {
                obj_num: desc_obj,
                gen_num: self.gen_num,
                font_name: self.base_font.clone(),
                file_obj_num: file_obj,
            };
            Some(desc.to_buffer())
        } else {
            None
        }
    }

    pub fn file_bytes(&self) -> Option<Vec<u8>> {
        if let (Some(obj_num), Some(data), Some(len1)) =
            (self.file_obj_num, &self.data, self.length1)
        {
            let mut buf = Vec::new();
            buf.extend(
                format!(
                    "{} {} obj\n<< /Length {} /Length1 {} /Filter /FlateDecode>>\nstream\n",
                    obj_num,
                    self.gen_num,
                    data.len(),
                    len1
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
        match (self.descriptor_obj_num, self.file_obj_num) {
            (Some(desc_obj), Some(_)) => format!(
                "{} {} obj\n<< /Type /Font\n/Subtype /{}\n/BaseFont /{}\n/FontDescriptor {} {} R\n>>\nendobj\n",
                self.obj_num, self.gen_num, self.subtype, self.base_font, desc_obj, self.gen_num
            ),
            _ => format!(
                "{} {} obj\n<< /Type /Font\n/Subtype /{}\n/BaseFont /{}\n>>\nendobj\n",
                self.obj_num, self.gen_num, self.subtype, self.base_font
            ),
        }
    }
}
