use arjp::JsonParser;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Benchmark for parsing a complex JSON string.
fn bench_parse(c: &mut Criterion) {
    let big_file = std::fs::read_to_string("data/twitter.json").expect("error");

    c.bench_function("parse_complex_json", |b| {
        b.iter(|| {
            let mut parser = JsonParser::new(black_box(big_file.as_str()));
            parser.parse().unwrap()
        })
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
