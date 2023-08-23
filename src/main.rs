use core::num;
use std::time::Instant;

mod fibonachi;
mod guess_game;
mod temp_conv;
fn main() {
    let now = Instant::now();
    let mut result = 0;

    for number in 1..1_000_000 {
        result = number;
    }


    let elapsed_time = now.elapsed().as_millis();
    println!("Elapsed:{elapsed_time}  Result:{result}")
}
