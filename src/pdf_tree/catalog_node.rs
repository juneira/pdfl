use super::pages_node::PagesNode;

pub struct CatalogNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub pages: PagesNode,
}

impl CatalogNode {
    pub fn to_buffer(&self, xref: &mut Vec<usize>) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.to_obj().as_bytes());

        xref.push(xref.last().unwrap() + buffer.len() + 1);
        buffer.extend(self.pages.to_buffer(xref ));

        return buffer;
    }

    fn to_obj(&self) -> String {
        return format!(
            "{} {} obj\n<< /Type /Catalog\n/Pages {} {} R\n>>\nendobj\n",
            self.obj_num, self.gen_num, self.pages.obj_num, self.pages.gen_num
        )
    }
}
