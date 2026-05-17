use finemark_ast::Element;
use finemark_parser::parse_document;

// ── Escape sequences ──────────────────────────────────────────────────────

#[test]
fn escape_asterisk() {
    let elems = parse_document("\\*");
    let Element::Escape(e) = &elems[0] else { panic!("expected Escape") };
    assert_eq!(e.value, "*");
}

#[test]
fn escape_at_sign() {
    let elems = parse_document("\\@");
    let Element::Escape(e) = &elems[0] else { panic!("expected Escape") };
    assert_eq!(e.value, "@");
}

#[test]
fn escape_backslash() {
    let elems = parse_document("\\\\");
    let Element::Escape(e) = &elems[0] else { panic!("expected Escape") };
    assert_eq!(e.value, "\\");
}

#[test]
fn escape_opening_brace() {
    let elems = parse_document("\\{");
    let Element::Escape(e) = &elems[0] else { panic!("expected Escape") };
    assert_eq!(e.value, "{");
}

#[test]
fn escape_closing_brace() {
    let elems = parse_document("\\}");
    let Element::Escape(e) = &elems[0] else { panic!("expected Escape") };
    assert_eq!(e.value, "}");
}

// ── Text boundaries ───────────────────────────────────────────────────────

#[test]
fn text_stops_at_at_sign() {
    let elems = parse_document("hello @h1{world}");
    let Element::Text(t) = &elems[0] else { panic!("expected Text") };
    assert_eq!(t.value, "hello ");
    assert!(matches!(elems[1], Element::Heading(_)));
}

#[test]
fn text_stops_at_inline_style_delimiter() {
    let elems = parse_document("hello **bold**");
    assert!(matches!(elems[0], Element::Text(_)));
    assert!(matches!(elems[1], Element::Bold(_)));
}

#[test]
fn plain_text_is_single_node() {
    let elems = parse_document("just plain text");
    assert_eq!(elems.len(), 1);
    let Element::Text(t) = &elems[0] else { panic!("expected Text") };
    assert_eq!(t.value, "just plain text");
}

// ── Balanced brace body ───────────────────────────────────────────────────

#[test]
fn body_with_nested_braces() {
    let elems = parse_document("@comment{outer {inner} end}");
    let Element::Comment(c) = &elems[0] else { panic!("expected Comment") };
    assert_eq!(c.value, "outer {inner} end");
}

#[test]
fn body_with_escaped_closing_brace() {
    let elems = parse_document("@comment{literal \\} brace}");
    let Element::Comment(c) = &elems[0] else { panic!("expected Comment") };
    assert!(c.value.contains("\\}"));
}

// ── Soft / Hard breaks ────────────────────────────────────────────────────

#[test]
fn newline_produces_soft_break() {
    let elems = parse_document("line one\nline two");
    assert!(matches!(elems[1], Element::SoftBreak(_)));
}

#[test]
fn multiple_consecutive_newlines_produce_multiple_soft_breaks() {
    let elems = parse_document("a\n\nb");
    assert!(matches!(elems[1], Element::SoftBreak(_)));
    assert!(matches!(elems[2], Element::SoftBreak(_)));
}

#[test]
fn bare_carriage_return_stays_as_text() {
    let elems = parse_document("a\rb");
    let Element::Text(t) = &elems[0] else { panic!("expected Text") };
    assert_eq!(t.value, "a\rb");
}
