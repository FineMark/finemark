mod block;
mod inline;
mod list;
mod parameter;
mod table;

pub use block::*;
pub use inline::*;
pub use list::*;
pub use parameter::*;
pub use table::*;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Element {
    // Inline elements
    Text(TextElement),
    Comment(CommentElement),
    Escape(EscapeElement),
    Error(ErrorElement),
    Link(LinkElement),
    InlineCode(InlineCodeElement),
    TeX(TeXElement),
    Bold(TextStyleElement),
    Italic(TextStyleElement),
    Strikethrough(TextStyleElement),
    Underline(TextStyleElement),
    Superscript(TextStyleElement),
    Subscript(TextStyleElement),
    SoftBreak(SoftBreakElement),
    HardBreak(HardBreakElement),

    // Block elements
    Heading(HeadingElement),
    BlockQuote(BlockQuoteElement),
    List(ListElement),
    HLine(HLineElement),
    CodeBlock(CodeBlockElement),
    Table(TableElement),
    TableRow(TableRowElement),
    TableColumn(TableColumnElement),
    ParagraphBreak(ParagraphBreakElement),
}

impl Element {
    pub fn span(&self) -> &crate::Span {
        match self {
            Element::Text(element) => &element.span,
            Element::Comment(element) => &element.span,
            Element::Escape(element) => &element.span,
            Element::Error(element) => &element.span,
            Element::Link(element) => &element.span,
            Element::InlineCode(element) => &element.span,
            Element::TeX(element) => &element.span,
            Element::Bold(element) => &element.span,
            Element::Italic(element) => &element.span,
            Element::Strikethrough(element) => &element.span,
            Element::Underline(element) => &element.span,
            Element::Superscript(element) => &element.span,
            Element::Subscript(element) => &element.span,
            Element::SoftBreak(e) => &e.span,
            Element::HardBreak(e) => &e.span,
            Element::Heading(element) => &element.span,
            Element::BlockQuote(element) => &element.span,
            Element::List(element) => &element.span,
            Element::HLine(element) => &element.span,
            Element::CodeBlock(element) => &element.span,
            Element::Table(element) => &element.span,
            Element::TableRow(element) => &element.span,
            Element::TableColumn(element) => &element.span,
            Element::ParagraphBreak(e) => &e.span,
        }
    }
}
