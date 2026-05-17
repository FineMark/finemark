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
pub enum Element<'i> {
    // Inline elements
    Text(TextElement<'i>),
    Comment(CommentElement<'i>),
    Escape(EscapeElement<'i>),
    Error(ErrorElement<'i>),
    Link(LinkElement<'i>),
    InlineCode(InlineCodeElement<'i>),
    TeX(TeXElement<'i>),
    Bold(TextStyleElement<'i>),
    Italic(TextStyleElement<'i>),
    Strikethrough(TextStyleElement<'i>),
    Underline(TextStyleElement<'i>),
    Superscript(TextStyleElement<'i>),
    Subscript(TextStyleElement<'i>),
    SoftBreak(SoftBreakElement),
    HardBreak(HardBreakElement),

    // Block elements
    Heading(HeadingElement<'i>),
    BlockQuote(BlockQuoteElement<'i>),
    List(ListElement<'i>),
    HLine(HLineElement<'i>),
    CodeBlock(CodeBlockElement<'i>),
    Table(TableElement<'i>),
    TableRow(TableRowElement<'i>),
    TableColumn(TableColumnElement<'i>),
    ParagraphBreak(ParagraphBreakElement),
}

impl Element<'_> {
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
