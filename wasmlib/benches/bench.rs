#![feature(test)]
extern crate test;
use test::bench::{Bencher, black_box};

#[bench]
fn wasmer(b: &mut Bencher) {
    b.iter(|| {
    });
}

#[bench]
fn wasmtime(b: &mut Bencher) {
    b.iter(|| {
    });
}
