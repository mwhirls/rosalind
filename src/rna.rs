use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dna = &args[1];
    let rna = dna.replace('T', "U");
    println!("{}", rna);
}
