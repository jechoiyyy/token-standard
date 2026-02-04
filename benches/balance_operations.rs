use criterion::{BatchSize, Criterion, black_box, criterion_group, criterion_main};
use token_standard::*;

fn benchmark_balance_of(c: &mut Criterion) {
    let creator = "alice".to_string();
    let token = TokenState::new(creator.clone(), 1_000_000);

    // 1. 존재하는 주소 조회
    c.bench_function("balance_of existing address", |b| {
        b.iter(|| token.balance_of(black_box(&creator)));
    });

    // 2. 존재하지 않는 주소 조회
    let unknown = "unknown".to_string();
    c.bench_function("balance_of non-existing address", |b| {
        b.iter(|| token.balance_of(black_box(&unknown)));
    });
}

fn benchmark_transfer(c: &mut Criterion) {
    let creator = "alice".to_string();
    let recipient = "bob".to_string();

    // 성공 케이스
    c.bench_function("transfer success", |b| {
        b.iter_batched(
            || TokenState::new(creator.clone(), 1_000_000),
            |mut token| token.transfer(black_box(&creator), black_box(&recipient), black_box(100)),
            BatchSize::SmallInput,
        );
    });

    // 실패 케이스
    c.bench_function("transfer insufficient balance", |b| {
        b.iter_batched(
            || TokenState::new(creator.clone(), 100),
            |mut token| token.transfer(black_box(&creator), black_box(&recipient), 200),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, benchmark_balance_of, benchmark_transfer);
criterion_main!(benches);
