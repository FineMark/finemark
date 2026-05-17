use finemark_ast::Element;
use finemark_parser::parse_document;

// ── Whitespace between keyword / params / body ────────────────────────────

#[test]
fn no_whitespace() {
    let elems = parse_document("@link(href=\"u\"){body}");
    assert!(matches!(elems[0], Element::Link(_)));
}

#[test]
fn space_before_params() {
    let elems = parse_document("@link (href=\"u\"){body}");
    assert!(matches!(elems[0], Element::Link(_)));
}

#[test]
fn space_before_body() {
    let elems = parse_document("@link(href=\"u\") {body}");
    assert!(matches!(elems[0], Element::Link(_)));
}

#[test]
fn spaces_everywhere() {
    let elems = parse_document("@link (href=\"u\") {body}");
    assert!(matches!(elems[0], Element::Link(_)));
}

#[test]
fn newlines_between_parts() {
    let elems = parse_document("@link\n(href=\"u\")\n{body}");
    assert!(matches!(elems[0], Element::Link(_)));
}

// ── Body presence / absence ───────────────────────────────────────────────

#[test]
fn hline_no_params_no_body() {
    let elems = parse_document("@hline");
    assert!(matches!(elems[0], Element::HLine(_)));
    assert_eq!(elems.len(), 1);
}

#[test]
fn hline_with_params_no_body() {
    let elems = parse_document("@hline(class=\"thick\")");
    let Element::HLine(h) = &elems[0] else {
        panic!("expected HLine")
    };
    assert!(h.parameters.contains_key("class"));
}

#[test]
fn heading_without_body_produces_empty_children() {
    let elems = parse_document("@h1");
    let Element::Heading(h) = &elems[0] else {
        panic!("expected Heading")
    };
    assert_eq!(h.level, 1);
    assert!(h.children.is_empty());
}

#[test]
fn heading_with_body() {
    let elems = parse_document("@h2{Title}");
    let Element::Heading(h) = &elems[0] else {
        panic!("expected Heading")
    };
    assert_eq!(h.level, 2);
    assert!(matches!(h.children[0], Element::Text(_)));
}

// ── Heading levels ────────────────────────────────────────────────────────

#[test]
fn all_heading_levels_parse() {
    for (cmd, level) in [
        ("@h1", 1u8),
        ("@h2", 2),
        ("@h3", 3),
        ("@h4", 4),
        ("@h5", 5),
        ("@h6", 6),
    ] {
        let elems = parse_document(cmd);
        let Element::Heading(h) = &elems[0] else {
            panic!("{cmd} did not produce Heading")
        };
        assert_eq!(h.level, level, "{cmd} level mismatch");
    }
}

// ── Nested AT commands ────────────────────────────────────────────────────

#[test]
fn at_nested_inside_quote_body() {
    let elems = parse_document("@quote{@h1{inner}}");
    let Element::BlockQuote(q) = &elems[0] else {
        panic!("expected BlockQuote")
    };
    assert!(matches!(q.children[0], Element::Heading(_)));
}

#[test]
fn link_inside_quote_after_whitespace() {
    let src = "@quote {\n  plain @link(href=\"u\"){linked}\n}";
    let elems = parse_document(src);
    let Element::BlockQuote(q) = &elems[0] else {
        panic!("expected BlockQuote")
    };
    let has_link = q.children.iter().any(|c| matches!(c, Element::Link(_)));
    assert!(has_link, "expected Link child inside BlockQuote");
}

#[test]
fn deeply_nested_at_commands() {
    let elems = parse_document("@quote{@quote{@h1{deep}}}");
    let Element::BlockQuote(outer) = &elems[0] else {
        panic!()
    };
    let Element::BlockQuote(inner) = &outer.children[0] else {
        panic!()
    };
    assert!(matches!(inner.children[0], Element::Heading(_)));
}

// ── Unknown / invalid @command → Text fallback ───────────────────────────

#[test]
fn unknown_at_command_produces_text_at() {
    let elems = parse_document("@unknowncmd");
    let Element::Text(t) = &elems[0] else {
        panic!("expected Text, got {:?}", elems[0])
    };
    assert_eq!(t.value, "@");
}

#[test]
fn at_not_followed_by_identifier_is_text() {
    let elems = parse_document("@ ");
    let Element::Text(t) = &elems[0] else {
        panic!("expected Text")
    };
    assert_eq!(t.value, "@");
}

#[test]
fn text_before_at_command_stops_correctly() {
    let elems = parse_document("hello @h1{world}");
    let Element::Text(t) = &elems[0] else {
        panic!("expected Text")
    };
    assert_eq!(t.value, "hello ");
    assert!(matches!(elems[1], Element::Heading(_)));
}

#[test]
fn br_command_produces_hard_break() {
    let elems = parse_document("a@brb");
    assert!(matches!(elems[1], Element::HardBreak(_)));
}

// ── Comment - raw body preserved ─────────────────────────────────────────

#[test]
fn comment_preserves_raw_content() {
    let elems = parse_document("@comment{raw @not_parsed content}");
    let Element::Comment(c) = &elems[0] else {
        panic!("expected Comment")
    };
    assert_eq!(c.value, "raw @not_parsed content");
}

#[test]
fn comment_without_body_has_empty_value() {
    let elems = parse_document("@comment");
    let Element::Comment(c) = &elems[0] else {
        panic!("expected Comment")
    };
    assert!(c.value.is_empty());
    assert!(c.body_open_span.is_none());
}

// ── Table structure ───────────────────────────────────────────────────────

#[test]
fn table_with_row_and_column() {
    let elems = parse_document("@table{@row{@column{cell}}}");
    let Element::Table(tbl) = &elems[0] else {
        panic!("expected Table")
    };
    let Element::TableRow(row) = &tbl.children[0] else {
        panic!("expected TableRow")
    };
    assert!(matches!(row.children[0], Element::TableColumn(_)));
}

#[test]
fn empty_table_body() {
    let elems = parse_document("@table{}");
    let Element::Table(tbl) = &elems[0] else {
        panic!("expected Table")
    };
    assert!(tbl.children.is_empty());
}

// ── List structure ───────────────────────────────────────────────────────

#[test]
fn unordered_list_with_item() {
    let elems = parse_document("@list(unordered){@item{something}}");
    let Element::List(list) = &elems[0] else {
        panic!("expected List")
    };
    assert!(list.parameters.contains_key("unordered"));
    assert_eq!(list.items.len(), 1);
    assert!(matches!(list.items[0].children[0], Element::Text(_)));
}

#[test]
fn ordered_list_preserves_parameters() {
    let elems = parse_document("@list(ordered, style=\"I\", start=\"3\"){@item{third}}");
    let Element::List(list) = &elems[0] else {
        panic!("expected List")
    };
    assert!(list.parameters.contains_key("ordered"));
    assert!(list.parameters.contains_key("style"));
    assert!(list.parameters.contains_key("start"));
}
