use std::{env, time::Instant};

use sieve::calculate;

fn main() {
    let mut args = env::args();
    args.next();

    let mut limit = 1000;
    let mut print = false;
    let mut time = false;

    for i in args {
        if let Ok(i) = i.parse::<u64>() {
            limit = i;
        }

        match i.as_str() {
            "--print" => print = true,
            "--time" => time = true,
            _ => {}
        }
    }

    let now = Instant::now();

    let result = calculate(limit);

    let elapsed = now.elapsed();

    if print {
        for i in result.iter() {
            print!("{}, ", i);
        }

        println!("\n");
    }

    if time {
        println!("took {}s for {} primes", elapsed.as_secs_f32(), result.count_primes());
    }
}
