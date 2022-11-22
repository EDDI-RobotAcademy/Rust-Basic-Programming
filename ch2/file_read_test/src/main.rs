use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("test.txt").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break;
        }

        println!("{} ({} 바이트)", line, len);

        line.truncate(0);
    }

    println!("다시 읽기!");

    f = File::open("test.txt").unwrap();
    reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} 바이트)", line, line.len());
    }
}
