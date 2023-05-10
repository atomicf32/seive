use std::{env, time::Instant};

use sieve::calculate;

fn main() {
    let mut args = env::args();
    args.next();

    let mut limit = 10000;
    let mut print = false;
    let mut time = false;

    for i in args {
        if let Ok(i) = i.parse::<usize>() {
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
            print!("{}\n ", i);
        }

        println!("\n");
    }

    if time {
        println!("took {}s for {} primes", elapsed.as_secs_f64(), result.len());
    }
}
