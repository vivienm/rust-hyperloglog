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

macro_rules! bench_merge {
    ($meth:ident) => {
        bench_merge!($meth, $meth);
    };
    ($name:ident, $meth:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut hll1 = HyperLogLog::<i32>::new();
            for i in 0..75_000 {
                hll1.insert(&i);
            }
            let mut hll2 = HyperLogLog::<i32>::new();
            for i in 25_000..100_000 {
                hll2.insert(&i);
            }
            b.iter(|| hll1.$meth(&hll2))
        }
    };
}

bench_merge!(try_merge);

bench_merge!(try_merge_from);
