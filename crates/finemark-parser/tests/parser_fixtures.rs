use finemark_ast::Element;
use finemark_parser::parse_document;
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/parser")
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}

fn parse_fixture(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parsed = parse_document(input);
    Ok(serde_json::to_string_pretty(&parsed)?)
}

fn should_update_expected() -> bool {
    std::env::var("FINEMARK_UPDATE_EXPECTED").as_deref() == Ok("1")
}

fn fixture_names(category: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let input_dir = fixture_root().join(category).join("input");
    let mut names = Vec::new();

    for entry in fs::read_dir(&input_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("fm") {
            continue;
        }

        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| format!("invalid fixture filename: {}", path.display()))?;
        names.push(name.to_string());
    }

    names.sort_unstable();
    Ok(names)
}

fn run_fixture(category: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture_root().join(category);
    let input_path = root.join("input").join(format!("{name}.fm"));
    let expected_path = root.join("expected").join(format!("{name}.json"));

    // Fixture files are normalized so checked-out CRLF does not change spans.
    let input = normalize_newlines(&fs::read_to_string(&input_path)?);
    let actual = parse_fixture(&input)?;
    let actual = normalize_newlines(&actual);

    if should_update_expected() {
        if let Some(parent) = expected_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&expected_path, &actual)?;
    }

    let expected = normalize_newlines(&fs::read_to_string(&expected_path)?);
    assert_eq!(
        actual.trim(),
        expected.trim(),
        "parser fixture {category}/{name} did not match expected output",
    );

    Ok(())
}

fn run_category(category: &str) -> Result<(), Box<dyn std::error::Error>> {
    let names = fixture_names(category)?;
    assert!(
        !names.is_empty(),
        "no parser fixtures found for category {category}",
    );

    for name in names {
        run_fixture(category, &name)?;
    }

    Ok(())
}

#[test]
fn at_command_fixtures_match_expected() {
    run_category("at").expect("at command fixtures should match expected output");
}

#[test]
fn inline_fixtures_match_expected() {
    run_category("inline").expect("inline fixtures should match expected output");
}

#[test]
fn bare_carriage_return_is_text() {
    let input = "a\rb\n";
    let parsed = parse_document(input);

    assert!(
        matches!(
            parsed.as_slice(),
            [Element::Text(text), Element::SoftBreak(_)] if text.value == "a\rb"
        ),
        "bare carriage return should stay inside text, got: {parsed:#?}",
    );
    assert_eq!(parsed[0].span().start, 0);
    assert_eq!(parsed[0].span().end, 3);
}
