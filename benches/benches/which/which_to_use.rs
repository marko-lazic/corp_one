use criterion::{black_box, criterion_group, criterion_main, Criterion};

use raw_data::RawData;

mod raw_data;

fn test_normal_4096(c: &mut Criterion) {
    let raw_data = RawData::new('a', 4096);
    let s = raw_data.to_str();
    let mut group = c.benchmark_group("normal 4096");
    group
        .warm_up_time(std::time::Duration::from_secs(10))
        .measurement_time(std::time::Duration::from_secs(15))
        .sample_size(1_000);
    group.bench_function("to_string", |b| b.iter(|| black_box(s.to_string())));
    group.bench_function("to_owned", |b| b.iter(|| black_box(s.to_owned())));
    group.finish();
}

fn test_utf8_4096(c: &mut Criterion) {
    let raw_data = RawData::new('🦀', 4096);
    let s = raw_data.to_str();
    let mut group = c.benchmark_group("utf8 4096");
    group
        .warm_up_time(std::time::Duration::from_secs(10))
        .measurement_time(std::time::Duration::from_secs(45))
        .sample_size(1_000);
    group.bench_function("to_string", |b| b.iter(|| black_box(s.to_string())));
    group.bench_function("to_owned", |b| b.iter(|| black_box(s.to_owned())));
    group.finish();
}

criterion_group!(benches, test_normal_4096, test_utf8_4096);
criterion_main!(benches);
