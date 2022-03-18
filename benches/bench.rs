#![feature(test)]

extern crate test;

use hyperloglog::HyperLogLog;
use test::{black_box, Bencher};

#[bench]
fn new(b: &mut Bencher) {
    b.iter(|| black_box(HyperLogLog::<i32>::new()))
}

#[bench]
fn insert(b: &mut Bencher) {
    let mut hll = HyperLogLog::<i32>::new();
    b.iter(|| {
        for i in 0..100_000 {
            hll.insert(&i);
        }
    });
}

#[bench]
fn count(b: &mut Bencher) {
    let mut hll = HyperLogLog::<i32>::new();
    for i in 0..100_000 {
        hll.insert(&i);
    }
    b.iter(|| hll.len());
}
