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

#[test]
fn closing_fence_must_be_alone_on_line() {
    let elems = parse_document("```\nnot closed\n``` trailing\n");
    assert!(
        elems
            .iter()
            .all(|elem| !matches!(elem, Element::CodeBlock(_)))
    );
}

#[test]
fn closing_fence_allows_leading_spaces() {
    let elems = parse_document("```\ncode\n   ```");
    assert!(matches!(elems[0], Element::CodeBlock(_)));
}

#[test]
fn closing_fence_allows_more_than_three_spaces() {
    let elems = parse_document("```\ncode\n       ```");
    assert!(matches!(elems[0], Element::CodeBlock(_)));
}

#[test]
fn parses_inline_tex() {
    let elems = parse_document("before $x + y$ after");
    let Element::TeX(tex) = &elems[1] else {
        panic!("expected TeX")
    };

    assert!(!tex.is_block);
    assert_eq!(tex.value, "x + y");
}

#[test]
fn inline_tex_does_not_cross_newline() {
    let elems = parse_document("before $x\ny$ after");
    assert!(
        elems
            .iter()
            .all(|elem| !matches!(elem, Element::TeX(tex) if !tex.is_block))
    );
}

#[test]
fn parses_block_tex() {
    let elems = parse_document("before $$\nx + y\n$$ after");
    let Element::TeX(tex) = &elems[1] else {
        panic!("expected TeX")
    };

    assert!(tex.is_block);
    assert_eq!(tex.value, "\nx + y\n");
}
