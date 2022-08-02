use cosmwasm_std::{Decimal256, OverflowError, StdResult, Uint256};
use criterion::{black_box, criterion_group, Criterion};
use ethnum::{I256, U256};
use better_secret_math::{core::muldiv, SCALE, SCALE_u128};

fn math(x: Uint256, y: Uint256) -> Uint256 {
    let x = U256::from_be_bytes(x.to_be_bytes());
    let y = U256::from_be_bytes(y.to_be_bytes());
    Uint256::from_be_bytes((x * y * y * y).to_be_bytes())
}

fn checked_math(x: Uint256, y: Uint256) -> Uint256 {
    let x = U256::from_be_bytes(x.to_be_bytes());
    let y = U256::from_be_bytes(y.to_be_bytes());
    Uint256::from_be_bytes(x.checked_mul(y).unwrap().checked_mul(y).unwrap().checked_mul(y).unwrap().to_be_bytes())
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mul");

    let sml = [
        U256::from(0x000025a13a5aa_u128),
        U256::from(0x0000000000000082991369928_u128),
    ];

    let sml_uint = [
        Uint256::from_u128(0x000025a13a5aa_u128),
        Uint256::from_u128(0x0000000000000082991369928_u128),
    ];

    group.bench_function("U256::mul", |b| {
        b.iter(|| black_box(sml[0]) * black_box(sml[1]) * black_box(sml[1]) * black_box(sml[1]))
    });

    group.bench_function("Uint256::mul", |b| {
        b.iter(|| black_box(sml_uint[0]) * black_box(sml_uint[1]) * black_box(sml_uint[1]) * black_box(sml_uint[1]))
    });

    group.bench_function("Uint256 & U256::mul", |b| {
        b.iter(|| math(black_box(sml_uint[0]), black_box(sml_uint[1])))
    });

    group.bench_function("Uint256 & U256::checked mul", |b| {
        b.iter(|| checked_math(black_box(sml_uint[0]), black_box(sml_uint[1])))
    });

    group.finish();

    let mut group = c.benchmark_group("Conversion");

    group.bench_function("U256::to_be_bytes", |b| {
        b.iter(|| black_box(U256::from_be_bytes(sml_uint[1].to_be_bytes())))
    });

    group.bench_function("U256::decimals", |b| {
        b.iter(|| black_box(U256::from_be_bytes(Decimal256::new(sml_uint[1]).atomics().to_be_bytes())))
    });

    group.finish();

    let mut group = c.benchmark_group("Muldiv");

    group.bench_function("U256::muldiv", |b| {
        b.iter(|| black_box(muldiv(sml[0], sml[1], SCALE)))
    });

    group.bench_function("Uint256::multiply_ratio", |b| {
        b.iter(|| black_box(sml_uint[0].multiply_ratio(sml_uint[1], Uint256::from_u128(SCALE_u128))))
    });

    group.finish();

}

criterion_group!(benches, criterion_benchmark);
