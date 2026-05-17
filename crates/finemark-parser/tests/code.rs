use finemark_ast::Element;
use finemark_parser::parse_document;

#[test]
fn parses_inline_code() {
    let elems = parse_document("before `code` after");
    assert!(matches!(elems[1], Element::InlineCode(_)));
}

#[test]
fn parses_inline_code_with_matching_backtick_count() {
    let elems = parse_document("before ``one ` tick`` after");
    let Element::InlineCode(code) = &elems[1] else {
        panic!("expected InlineCode")
    };

    assert_eq!(code.value, "one ` tick");
}

#[test]
fn inline_code_does_not_cross_newline() {
    let elems = parse_document("before `code\nafter`");
    assert!(
        elems
            .iter()
            .all(|elem| !matches!(elem, Element::InlineCode(_)))
    );
}

#[test]
fn parses_fenced_code_with_parameters() {
    let elems = parse_document("```(lang=\"rust\")\nfn main() {\n}\n```\n");
    let Element::CodeBlock(code) = &elems[0] else {
        panic!("expected CodeBlock")
    };

    assert!(code.parameters.contains_key("lang"));
    assert_eq!(code.value, "fn main() {\n}\n");
}

#[test]
fn longer_fence_allows_shorter_fence_inside() {
    let elems = parse_document("````(lang=\"markdown\")\n```rust\nfn main() {}\n```\n````\n");
    let Element::CodeBlock(code) = &elems[0] else {
        panic!("expected CodeBlock")
    };

    assert!(code.value.contains("```rust"));
}

#[test]
fn fenced_code_must_start_at_line_start() {
    let elems = parse_document("before ```\nnot code\n```\n");
    assert!(
        elems
            .iter()
            .all(|elem| !matches!(elem, Element::CodeBlock(_)))
    );
}
