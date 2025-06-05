use super::catalog_node::CatalogNode;

pub struct PdfNode {
    pub version: String,
    pub total_obj: usize,
    pub root: CatalogNode,
}

impl PdfNode {
    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.header().as_bytes());

        let mut xref: Vec<usize> = Vec::new();
        xref.push(buffer.len() + 1);

        buffer.extend(self.root.to_buffer(&mut xref));

        let start_xref = xref.last().unwrap() + 1;

        buffer.extend(self.xref_section(&xref).as_bytes());
        buffer.extend(self.trailer().as_bytes());
        buffer.extend(self.start_xref_section(start_xref).as_bytes());

        return buffer
    }

    pub fn header(&self) -> String {
        return format!("%PDF-{}\n", self.version);
    }

    fn xref_section(&self, xref: &Vec<usize>) -> String {
        let mut xref_str = String::new();
        xref_str.push_str("xref\n");
        xref_str.push_str(format!("0 {}\n", self.total_obj).as_str());
        xref_str.push_str("0000000000 65535 f\n");

        for (_i, offset) in xref.iter().enumerate() {
            xref_str.push_str(format!("{:010} 00000 n\n", offset).as_str());
        }
        return xref_str
    }

    fn trailer(&self) -> String {
        return format!(
            "trailer\n<< /Size {}\n/Root {} {} R\n>>\n",
            self.total_obj, self.root.obj_num, self.root.gen_num
        );
    }

    fn start_xref_section(&self, start_xref: usize) -> String {
        return format!("startxref\n{}\n%%EOF", start_xref);
    }
}
