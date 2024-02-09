use std::fs::File;
use std::io::{self, BufRead, Error};

type LinesIter = std::iter::Peekable<io::Lines<io::BufReader<File>>>;

pub struct FASTAFile {
    lines: io::Lines<io::BufReader<File>>,
}

impl FASTAFile {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let file = File::open(filename);
        file.map(|f| {
            let reader = io::BufReader::new(f);
            let lines = reader.lines();
            FASTAFile { lines }
        })
    }
}

pub struct Entry {
    pub label: String,
    pub sequence: String,
}

impl IntoIterator for FASTAFile {
    type Item = Entry;
    type IntoIter = FASTAIterator;

    fn into_iter(self) -> Self::IntoIter {
        FASTAIterator {
            iter: self.lines.peekable(),
        }
    }
}

fn is_label(line: &String) -> bool {
    line.starts_with('>')
}

fn read_label(iter: &mut LinesIter) -> Option<String> {
    let line = iter.next()?;
    let res = line.and_then(|l| Ok(is_label(&l).then_some(l[1..].to_string())));
    match res {
        Ok(l) => l,
        Err(_) => None,
    }
}

fn read_sequence(iter: &mut LinesIter) -> Option<String> {
    let mut sequence = vec![];
    while let Some(line) = iter.next() {
        if let Ok(l) = line {
            if !is_label(&l) {
                sequence.push(l.clone());
            }
            let next = iter.peek();
            match next {
                Some(l) => {
                    if l.as_ref().is_ok_and(|l| is_label(&l)) {
                        break;
                    }
                }
                _ => (),
            }
        }
    }
    if sequence.is_empty() {
        None
    } else {
        Some(sequence.join(&""))
    }
}

pub struct FASTAIterator {
    iter: LinesIter,
}

impl Iterator for FASTAIterator {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let label = read_label(&mut self.iter);
        let sequence = read_sequence(&mut self.iter);
        match (label, sequence) {
            (Some(label), Some(sequence)) => Some(Entry { label, sequence }),
            _ => None,
        }
    }
}
