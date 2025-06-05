use super::page_node::PageNode;

pub struct PagesNode {
    pub obj_num: usize,
    pub gen_num: usize,
    pub count: usize,
    pub kids: Vec<PageNode>,
}

impl PagesNode {
    pub fn to_buffer(&self, xref: &mut Vec<usize>) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.to_obj().as_bytes());

        for kid in &self.kids {
            xref.push(xref.last().unwrap() + buffer.len() + 1);
            buffer.extend(kid.to_buffer(xref));
        }

        return buffer;
    }

    fn to_obj(&self) -> String {
        let kids_str: Vec<String> = self.kids.iter()
            .map(|kid| format!("{} {} R", kid.obj_num, kid.gen_num))
            .collect();

        return format!(
            "{} {} obj\n<< /Type /Pages\n/Count {}\n/Kids [{}]\n>>\nendobj\n",
            self.obj_num,
            self.gen_num,
            self.count,
            kids_str.join(" ")
        );
    }
}
