use hyperloglog::{HyperLogLog, Precision};

fn main() {
    for precision in Precision::variants() {
        let mut hll = HyperLogLog::<u32>::with_precision(*precision);
        for i in 0..1_000_000 {
            hll.insert(&i);
        }
        println!("Precision: {}, distinct count: {}", precision, hll.len());
    }
}
