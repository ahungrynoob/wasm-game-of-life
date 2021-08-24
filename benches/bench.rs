#![feature(test)]
extern crate test; // https://github.com/rust-analyzer/rust-analyzer/issues/6714

use wasm_game_of_life;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = wasm_game_of_life::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}
