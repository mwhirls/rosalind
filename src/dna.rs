use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let sequence = &args[1];
    let result: Vec<usize> = "ACGT"
        .chars()
        .map(|base| sequence.chars().filter(|c| *c == base).count())
        .collect();
    println!("{} {} {} {}", result[0], result[1], result[2], result[3]);
}
