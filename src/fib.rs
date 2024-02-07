use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args[1].parse().unwrap();
    let k: u32 = args[2].parse().unwrap();
    let (mut f2, mut f1) = (1, 1);
    for _i in 2..n {
        (f2, f1) = (f1, f2 * k + f1);
    }
    println!("{}", f1);
}
