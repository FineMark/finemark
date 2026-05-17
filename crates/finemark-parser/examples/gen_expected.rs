use finemark_parser::parse_document;
use std::fs;
use std::path::Path;

fn main() {
    let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/parser");

    let categories: Vec<String> = fs::read_dir(&fixture_root)
        .expect("failed to read fixtures/parser directory")
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    if categories.is_empty() {
        eprintln!("No fixture categories found under {}", fixture_root.display());
        return;
    }

    let mut generated = 0usize;

    for category in &categories {
        let input_dir = fixture_root.join(category).join("input");
        let expected_dir = fixture_root.join(category).join("expected");

        if !input_dir.exists() {
            eprintln!("  [{category}] input directory not found, skipping");
            continue;
        }

        fs::create_dir_all(&expected_dir).expect("failed to create expected directory");

        let mut entries: Vec<_> = fs::read_dir(&input_dir)
            .expect("failed to read input directory")
            .flatten()
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "fm"))
            .collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let path = entry.path();
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("invalid fixture filename");

            // Normalize CRLF to LF for consistent byte offsets across platforms.
            let input = fs::read_to_string(&path)
                .expect("failed to read input file")
                .replace("\r\n", "\n");

            let result = parse_document(&input);
            let json = serde_json::to_string_pretty(&result).expect("failed to serialize AST");

            let out_path = expected_dir.join(format!("{stem}.json"));
            fs::write(&out_path, &json).expect("failed to write expected file");

            println!("  generated: {}/{stem}.json", category);
            generated += 1;
        }
    }

    println!("\nDone — {generated} fixture(s) generated.");
}
