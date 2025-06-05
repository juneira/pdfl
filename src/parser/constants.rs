use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct NodePdf {
    pub child_page: NodePage,
}

#[derive(Debug, PartialEq)]
pub struct NodePage {
    pub child_content: NodeContent,
    pub child_page: Option<Box<NodePage>>,
}

#[derive(Debug, PartialEq)]
pub struct NodeContent {
    pub child_text: NodeText,
}

#[derive(Debug, PartialEq)]
pub struct NodeText {
    pub child_string: Token,
}
