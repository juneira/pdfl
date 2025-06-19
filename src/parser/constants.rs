use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct PdfNode {
    pub child_page: PageNode,
}

#[derive(Debug, PartialEq)]
pub struct PageNode {
    pub resources: Option<ResourceNode>,
    pub child_content: ContentNode,
    pub child_page: Option<Box<PageNode>>,
}

#[derive(Debug, PartialEq)]
pub struct ResourceNode {
    pub fonts: Vec<FontNode>,
}

#[derive(Debug, PartialEq)]
pub struct FontNode {
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum ContentElement {
    Text(TextNode),
    Rectangle(RectangleNode),
    Line(LineNode),
    Circle(CircleNode),
}

#[derive(Debug, PartialEq)]
pub struct ContentNode {
    pub children: Vec<ContentElement>,
}

#[derive(Debug, PartialEq)]
pub struct TextNode {
    pub child_string: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub struct RectangleNode {
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub struct LineNode {
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub struct CircleNode {
    pub attributes: HashMap<String, String>,
}
