use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use finemark_parser::core::parse_document;
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;
use std::time::Duration;

fn load_input() -> String {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("../../target.fm");

    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn bench_parse_document(c: &mut Criterion) {
    let input = load_input();

    let cases = [
        ("1x", input.clone()),
        ("10x", input.repeat(10)),
        ("100x", input.repeat(100)),
    ];

    let mut group = c.benchmark_group("parse_document");

    group
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(5))
        .sample_size(100);

    for (name, content) in cases {
        group.throughput(Throughput::Bytes(content.len() as u64));

        group.bench_with_input(
            BenchmarkId::new("full_parse", name),
            &content,
            |b, content| {
                b.iter(|| {
                    let result = parse_document(black_box(content.as_str()));
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_parse_document);
criterion_main!(benches);