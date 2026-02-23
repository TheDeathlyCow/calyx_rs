use calyx_rs::generation::Grammar;
use rand::SeedableRng;
use rand::prelude::StdRng;

fn main() {
    let rng = StdRng::seed_from_u64(12345);
    let _grammar = Grammar::from_rng(rng);
}
