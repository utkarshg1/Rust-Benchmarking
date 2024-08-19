use benchmark::{sort_algo_1, sort_algo_2};
use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers: Vec<i32> = vec![
        1, 2, 3, 100, 25, 54, 5, 5, 7, 8, 9, 10, 11, 12, 13, 25, 26, 45,
    ];

    c.bench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_1(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
