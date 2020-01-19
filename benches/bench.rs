#[macro_use]
extern crate criterion;
extern crate pathtracer;

use criterion::black_box;
use criterion::Criterion;

use pathtracer::{box_test, query_database, ray_marching, trace, Vec3d};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("query_database-lazy", |b| {
        b.iter(|| query_database(black_box(Vec3d::new(-22.0, 5.0, 25.0))))
    });
    c.bench_function("trace", |b| {
        b.iter(|| {
            trace(
                black_box(Vec3d::new(-22.0, 5.0, 25.0)),
                black_box(Vec3d::new(0.184649229, 0.215958387, -0.958783984)),
            )
        })
    });

    c.bench_function("box_test", |b| {
        b.iter(|| {
            box_test(
                black_box(Vec3d::new(-22.0, 5.0, 25.0)),
                black_box(Vec3d::new(-30.0, -0.5, -30.0)),
                black_box(Vec3d::new(-30.0, 18.0, 30.0)),
            )
        })
    });
    c.bench_function("ray_marching", |b| {
        b.iter(|| {
            ray_marching(
                black_box(Vec3d::new(-22.0, 5.0, 25.0)),
                black_box(Vec3d::new(0.184649229, 0.215958387, -0.958783984)),
                black_box(Vec3d::new(0.0, 0.0, 0.0)),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
