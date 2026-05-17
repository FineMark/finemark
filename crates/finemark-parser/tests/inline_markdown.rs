use finemark_ast::Element;
use finemark_parser::parse_document;

#[test]
fn parses_markdown_text_styles() {
    let elements = parse_document("**bold** *italic* ~~strike~~ __under__ ^^super^^ ,,sub,,");

    assert!(matches!(elements[0], Element::Bold(_)));
    assert!(matches!(elements[2], Element::Italic(_)));
    assert!(matches!(elements[4], Element::Strikethrough(_)));
    assert!(matches!(elements[6], Element::Underline(_)));
    assert!(matches!(elements[8], Element::Superscript(_)));
    assert!(matches!(elements[10], Element::Subscript(_)));
}

#[test]
fn parses_triple_asterisk_as_nested_bold_italic() {
    let elements = parse_document("***both***");
    let Element::Bold(element) = &elements[0] else {
        panic!("expected bold element");
    };
    let Element::Italic(element) = &element.children[0] else {
        panic!("expected nested italic element");
    };

    assert_eq!(element.open_span.start, 2);
    assert_eq!(element.open_span.end, 3);
    assert_eq!(element.close_span.start, 7);
    assert_eq!(element.close_span.end, 8);
}
