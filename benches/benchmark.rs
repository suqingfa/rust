use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        (b, a) = (a + b, b);
    }
    b
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 30", |b| b.iter(|| fibonacci(black_box(30))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
