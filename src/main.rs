use std::time::Instant;

fn main() {
    let before_instant: Instant = Instant::now();
    println!("Elapsed Time: {:.2?}", before_instant.elapsed());
}