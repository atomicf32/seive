use std::env;

use seive::calculate;

fn main() {
    let mut args = env::args();
    args.next();

    for i in args {
        if let Ok(i) = i.parse::<u128>() {
            for i in calculate(i) {
                print!("{}, ", i);
            }

            println!();
            println!();

            return;
        }
    }
}
