#![allow(non_snake_case)]

use ChessProject::engine_modules::{
    zobrist::Zobrist,
    game_state::GameState,
    moves::Moves,
    perft::Perft,
};
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

pub fn moveGenBenchmark(c: &mut Criterion) {
    let mut z: Zobrist = Zobrist::new();
    let gs = GameState::new(&mut z);
    let mut m: Moves = Moves::new();
    let mut p: Perft = Perft::new(3);
    p.perftRoot(&mut m, &mut z, gs.bitboards, gs.castle_rights, gs.hash_key, true, 0);
    c.bench_function(
        "Move Generation",
        |b| b.iter(|| p.perftRoot(black_box(&mut m), black_box(&mut z), black_box(gs.bitboards), black_box(gs.castle_rights), black_box(gs.hash_key), black_box(true), black_box(0)))
    );
}

criterion_group!(benches, moveGenBenchmark);
criterion_main!(benches);
