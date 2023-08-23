use std::time::Instant;

pub(crate) fn run() {
    let now = Instant::now();
    let mut result = 0;

    for number in 1..1_000_000_000 {
        result = number;
    }

    for number in 1..1_000_000_000 {
        result = number;
    }

    for number in 1..1_000_000_000 {
        result = number;
    }


    let elapsed_time = now.elapsed().as_millis();
    println!("Elapsed:{elapsed_time}  Result:{result}")
}
