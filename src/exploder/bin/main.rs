use std::{collections::BTreeMap, fs::File, io::Write};

const BOOK: &str = include_str!("iliad.txt");
const JAN_01_2000_MS: u64 = 946645200000;
const REPEAT_N: u64 = 100;

fn write_line(file: &mut File, timestamp: u64, line: &str) {
    writeln!(file, "{timestamp}|{line}").expect("failed write");
}

fn main() {
    let mut outputs: BTreeMap<char, File> = BTreeMap::new();
    let mut other_file = None;
    let mut cur_timestamp = JAN_01_2000_MS;

    std::fs::create_dir_all("data").expect("failed to create data dir");

    for n in 0..REPEAT_N {
        if n % 10 == 0 {
            println!("Writing out the Iliad for the {n}th time...");
        }
        for line in BOOK.lines() {
            let first_letter = line.chars().next().and_then(|c| {
                if c.is_ascii_alphabetic() {
                    Some(c.to_ascii_lowercase())
                } else {
                    None
                }
            });
            let file = match first_letter {
                None => other_file.get_or_insert_with(|| {
                    File::create("data/other.txt").expect("failed to create other.txt")
                }),
                Some(letter) => outputs.entry(letter).or_insert_with(|| {
                    File::create(format!("data/{letter}.txt"))
                        .expect("failed to create letter file")
                }),
            };
            write_line(file, cur_timestamp, line);
            cur_timestamp += rand::random_range(1u64..1000u64);
        }
    }
    println!("Wrote out the Iliad {REPEAT_N} times.")
}
