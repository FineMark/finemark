mod block;
mod leaf;
mod list;
mod parameter;
mod table;

pub use block::*;
pub use leaf::*;
pub use list::*;
pub use parameter::*;
use serde::Serialize;
pub use table::*;

#[derive(Debug, Clone, Serialize)]
pub enum Element {
    // Basic text elements
    Text(TextElement),
    Comment(CommentElement),
    Escape(EscapeElement),
    Error(ErrorElement),
    Link(LinkElement),

    // Block elements
    Heading(HeadingElement),
    InlineCode(InlineCodeElement),
    TeX(TeXElement),
    BlockQuote(BlockQuoteElement),
    List(ListElement),
    HLine(HLineElement),
    CodeBlock(CodeBlockElement),
    Table(TableElement),
    TableRow(TableRowElement),
    TableColumn(TableColumnElement),

    // Line elements
    SoftBreak(SoftBreakElement),
    HardBreak(HardBreakElement),
}

impl Element {
    pub fn span(&self) -> &crate::Span {
        match self {
            Element::Text(element) => &element.span,
            Element::Comment(element) => &element.span,
            Element::Escape(element) => &element.span,
            Element::Error(element) => &element.span,
            Element::Link(element) => &element.span,
            Element::Heading(element) => &element.span,
            Element::InlineCode(element) => &element.span,
            Element::TeX(element) => &element.span,
            Element::BlockQuote(element) => &element.span,
            Element::List(element) => &element.span,
            Element::HLine(element) => &element.span,
            Element::CodeBlock(element) => &element.span,
            Element::Table(element) => &element.span,
            Element::TableRow(element) => &element.span,
            Element::TableColumn(element) => &element.span,
            Element::SoftBreak(e) => &e.span,
            Element::HardBreak(e) => &e.span,
        }
    }
}
