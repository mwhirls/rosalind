mod util {
    pub mod fasta;
}
use std::env;
use util::fasta;

struct GCContent {
    label: String,
    ratio: f32,
}

impl GCContent {
    fn new(entry: fasta::Entry) -> Self {
        let gc_count = entry.sequence.matches(|c| matches!(c, 'G' | 'C')).count();
        let ratio = gc_count as f32 / entry.sequence.len() as f32;
        GCContent {
            label: entry.label,
            ratio,
        }
    }
    fn empty() -> Self {
        GCContent {
            label: String::new(),
            ratio: 0.0,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];
    let file = fasta::FASTAFile::new(fpath);
    if let Ok(fasta) = file {
        let highest_gc = fasta
            .into_iter()
            .map(|entry: fasta::Entry| GCContent::new(entry))
            .fold(GCContent::empty(), |acc, x| {
                if acc.ratio > x.ratio {
                    acc
                } else {
                    x
                }
            });
        println!("{} {}", highest_gc.label, 100.0 * highest_gc.ratio);
    }
}
