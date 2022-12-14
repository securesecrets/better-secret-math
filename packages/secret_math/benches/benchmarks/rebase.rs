use better_secret_math::{BtrRebase, Rebase};
use cosmwasm_std::Uint256;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ethnum::U256;

pub fn rebase_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rebase");

    group.bench_function("add_base", |b| {
        b.iter(|| {
            let mut rebase = Rebase::init();
            rebase.add_base(black_box(Uint256::from_u128(1000000u128)), false);
        })
    });

    group.bench_function("better_add_base", |b| {
        b.iter(|| {
            let mut rebase = BtrRebase::init();
            rebase.add_base(black_box(U256::new(1000000u128)), false);
        })
    });
}

criterion_group!(benches, rebase_benchmark);
