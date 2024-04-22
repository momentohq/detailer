use criterion::{criterion_group, Criterion};
use detailer::{detail, new_detailer, scope};

#[allow(clippy::expect_used)] // this is a benchmark, lints like this don't matter.
fn detailing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Detailer");
    group.throughput(criterion::Throughput::Elements(1));

    group.bench_function("disabled", |bencher| {
        bencher.iter(|| {
            let mut detailer = new_detailer!(Off);
            detail!(detailer, "it does something");
            let _guard = scope!(detailer, "suspended");
            detail!(detailer, "it does something else");
            detail!(detailer, "it does something else again");
        })
    });

    group.bench_function("enabled no time", |bencher| {
        bencher.iter(|| {
            let mut detailer = new_detailer!(Info, WithoutTiming);
            detail!(detailer, "it does something");
            let _guard = scope!(detailer, "suspended");
            detail!(detailer, "it does something else");
            detail!(detailer, "it does something else again");
        })
    });

    group.bench_function("enabled with time", |bencher| {
        bencher.iter(|| {
            let mut detailer = new_detailer!(Info, WithTiming);
            detail!(detailer, "it does something");
            let _guard = scope!(detailer, "suspended");
            detail!(detailer, "it does something else");
            detail!(detailer, "it does something else again");
        })
    });

    group.bench_function("cached enabled with time", |bencher| {
        let mut detailer = new_detailer!(Info, WithTiming);
        bencher.iter(|| {
            detailer.reset();
            detail!(detailer, "it does something");
            let _guard = scope!(detailer, "suspended");
            detail!(detailer, "it does something else");
            detail!(detailer, "it does something else again");
            detailer.flush();
        })
    });
}

criterion_group!(benches, detailing);
