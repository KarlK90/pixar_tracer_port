#[macro_use]
extern crate criterion;
extern crate pathtracer;

use criterion::black_box;
use criterion::Criterion;

use pathtracer::{query_database, query_database_new, Vec3d};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("query_database-default", |b| {
        b.iter(|| {
            query_database(black_box(Vec3d {
                x: -22.0,
                y: 5.0,
                z: 25.0,
            }))
        })
    });
    c.bench_function("query_database-lazy", |b| {
        b.iter(|| {
            query_database_new(black_box(Vec3d {
                x: -22.0,
                y: 5.0,
                z: 25.0,
            }))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
