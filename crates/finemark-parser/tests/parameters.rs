use finemark_ast::Element;
use finemark_parser::parse_document;

// ── Empty / flag / value params ───────────────────────────────────────────

#[test]
fn empty_parameter_list() {
    let elems = parse_document("@hline[]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    assert!(h.parameters.is_empty());
}

#[test]
fn flag_parameter_no_value() {
    let elems = parse_document("@hline[thick]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    assert!(h.parameters.contains_key("thick"));
    assert!(h.parameters["thick"].value.is_empty());
}

#[test]
fn value_parameter() {
    let elems = parse_document("@hline[class=\"hr\"]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    let val = &h.parameters["class"].value;
    let Element::Text(t) = &val[0] else { panic!("expected Text in param value") };
    assert_eq!(t.value, "hr");
}

#[test]
fn parameter_empty_value() {
    let elems = parse_document("@hline[key=\"\"]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    assert!(h.parameters["key"].value.is_empty());
}

// ── Multiple parameters ───────────────────────────────────────────────────

#[test]
fn multiple_flag_params() {
    let elems = parse_document("@hline[a, b]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    assert!(h.parameters.contains_key("a"));
    assert!(h.parameters.contains_key("b"));
}

#[test]
fn mixed_flag_and_value_params() {
    let elems = parse_document("@hline[a, b=\"v\"]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    assert!(h.parameters.contains_key("a"));
    assert!(h.parameters.contains_key("b"));
}

#[test]
fn parameter_order_preserved() {
    // IndexMap must maintain insertion order from source
    let elems = parse_document("@hline[z, a, m]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    let keys: Vec<&str> = h.parameters.keys().map(String::as_str).collect();
    assert_eq!(keys, ["z", "a", "m"]);
}

// ── Whitespace inside parameter list ─────────────────────────────────────

#[test]
fn whitespace_around_equals() {
    let elems = parse_document("@hline[class = \"val\"]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    assert!(h.parameters.contains_key("class"));
}

#[test]
fn whitespace_inside_brackets() {
    let elems = parse_document("@hline[ key = \"val\" ]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    assert!(h.parameters.contains_key("key"));
}

// ── Value content edge cases ──────────────────────────────────────────────

#[test]
fn value_with_spaces() {
    let elems = parse_document("@hline[label=\"hello world\"]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    let val = &h.parameters["label"].value;
    let Element::Text(t) = &val[0] else { panic!("expected Text") };
    assert_eq!(t.value, "hello world");
}

#[test]
fn value_containing_closing_bracket() {
    // `]` inside quoted value must not terminate the parameter list
    let elems = parse_document("@hline[label=\"a ] b\"]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    let val = &h.parameters["label"].value;
    let Element::Text(t) = &val[0] else { panic!("expected Text") };
    assert_eq!(t.value, "a ] b");
}

#[test]
fn value_containing_comma() {
    let elems = parse_document("@hline[label=\"a, b\"]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    let val = &h.parameters["label"].value;
    let Element::Text(t) = &val[0] else { panic!("expected Text") };
    assert_eq!(t.value, "a, b");
}

#[test]
fn hyphenated_parameter_key() {
    let elems = parse_document("@hline[data-id=\"1\"]");
    let Element::HLine(h) = &elems[0] else { panic!("expected HLine") };
    assert!(h.parameters.contains_key("data-id"));
}
