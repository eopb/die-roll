use rand::Rng;
use std::collections::HashMap;

const TESTS_TO_DO: u32 = 5000000;

fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    for x in 1..=6 {
        (&mut map).insert(x, 1);
    }
    for _ in 0..TESTS_TO_DO {
        (&mut map)
            .entry(die_roll())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    dbg!(map);
}

fn die_roll() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, 7)
}
