use better_secret_math::{
    common::{muldiv, muldiv18},
    ud60x18::{constants::UNIT, mul},
    UNIT_U128,
};
use cosmwasm_std::{Decimal256, StdResult, Uint256};
use criterion::{black_box, criterion_group, Criterion};
use ethnum::U256;

fn math(x: Uint256, y: Uint256) -> Uint256 {
    let x = U256::from_be_bytes(x.to_be_bytes());
    let y = U256::from_be_bytes(y.to_be_bytes());
    Uint256::from_be_bytes((x * y * y * y).to_be_bytes())
}

fn checked_math(x: Uint256, y: Uint256) -> Uint256 {
    let x = U256::from_be_bytes(x.to_be_bytes());
    let y = U256::from_be_bytes(y.to_be_bytes());
    Uint256::from_be_bytes(
        x.checked_mul(y)
            .unwrap()
            .checked_mul(y)
            .unwrap()
            .checked_mul(y)
            .unwrap()
            .to_be_bytes(),
    )
}

fn mul_muldiv(x: U256, y: U256) -> StdResult<U256> {
    mul(mul(mul(x, y)?, y)?, y)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mul");

    let sml = [
        U256::from(0x000025a13a5aa_u128),
        U256::from(0x0000000000000082991369928_u128),
        U256::from(0x1EEE6A1E77D9838707Fu128),
        U256::from(0x28BE875E01341EA89311u128),
    ];

    let sml_uint = [
        Uint256::from_u128(0x000025a13a5aa_u128),
        Uint256::from_u128(0x0000000000000082991369928_u128),
        Uint256::from_u128(0x1EEE6A1E77D9838707Fu128),
        Uint256::from_u128(0x28BE875E01341EA89311u128),
    ];

    group.bench_function("U256::mul", |b| {
        b.iter(|| black_box(sml[0]) * black_box(sml[1]) * black_box(sml[1]) * black_box(sml[1]))
    });

    group.bench_function("U256::mul via muldiv", |b| {
        b.iter(|| mul_muldiv(black_box(sml[0]), black_box(sml[1])))
    });

    group.bench_function("Uint256::mul", |b| {
        b.iter(|| {
            black_box(sml_uint[0])
                * black_box(sml_uint[1])
                * black_box(sml_uint[1])
                * black_box(sml_uint[1])
        })
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
        b.iter(|| U256::from_be_bytes(black_box(sml_uint[1].to_be_bytes())))
    });

    group.bench_function("U256::decimals", |b| {
        b.iter(|| {
            U256::from_be_bytes(black_box(
                Decimal256::new(sml_uint[1]).atomics().to_be_bytes(),
            ))
        })
    });

    group.finish();

    let mut group = c.benchmark_group("Muldiv");

    group.bench_function("U256::muldiv", |b| {
        b.iter(|| muldiv(black_box(sml[2]), black_box(sml[3]), black_box(UNIT)))
    });

    group.bench_function("Uint256::multiply_ratio", |b| {
        b.iter(|| {
            black_box(sml_uint[2]).multiply_ratio(
                black_box(sml_uint[3]),
                black_box(Uint256::from_u128(UNIT_U128)),
            )
        })
    });

    group.bench_function("Decimal256::checked_mul", |b| {
        b.iter(|| {
            Decimal256::new(black_box(sml_uint[2]))
                .checked_mul(Decimal256::new(black_box(sml_uint[3])))
        })
    });

    group.bench_function("U256::muldiv18", |b| {
        b.iter(|| muldiv18(black_box(sml[2]), black_box(sml[3])))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
