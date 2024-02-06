use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dna = &args[1];
    let revc: String = dna
        .chars()
        .rev()
        .map(|base| match base {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            c => c,
        })
        .collect();
    println!("{}", revc);
}
