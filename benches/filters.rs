use criterion::{black_box, criterion_group, criterion_main, Criterion};

static TEXT: &str = include_str!("../resources/sample.md");

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("filters");

    group.bench_function("full copy", |b| {
        b.iter(|| {
            let text = black_box(TEXT.to_string());
            black_box(filter_measurements::filter_comments(&text));
        })
    });

    group.bench_function("copy within", |b| {
        b.iter(|| {
            let mut text = black_box(TEXT.to_string());
            black_box(filter_measurements::filter_comments_copy_within(&mut text));
        })
    });

    group.bench_function("custom copy within", |b| {
        b.iter(|| {
            let text = black_box(TEXT.to_string());
            black_box(filter_measurements::filter_comments_custom_copy_wihin(text));
        })
    });
}

criterion_group!(filter, benchmark);
criterion_main!(filter);
