
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Result, Write};

const SKIP_STR: &str = "，。；\n";

fn main() -> Result<()> {
    let mut learned: String = String::new();
    File::open("learned.txt")?.read_to_string(&mut learned)?;

    let mut unlearned: String = String::new();
    File::open("unlearned.txt")?.read_to_string(&mut unlearned)?;

    let unique_learned = get_unique(&learned);

    let unique_unlearned_origin = get_unique(&unlearned);

    let rt = get_diff(unique_unlearned_origin, unique_learned);

    output(rt);

    Ok(())
}

fn output(rt: Vec<char>) -> Result<()> {
    if let Err(e) = fs::remove_file("output.txt") {
        println!("{:?}", e);
    }
    let mut output_file = OpenOptions::new().write(true).create(true).append(true).open("output.txt")?;
    for c in rt.iter() {
        print!("{}", c.to_string());
        output_file.write_all(format!("{}{}", c.to_string(), "\n").as_bytes())?;
    }

    Ok(())
}

fn get_diff(p0: Vec<char>, p1: Vec<char>) -> Vec<char> {
    let mut rt = Vec::new();

    for to_add in p0 {
        if !p1.contains(&to_add) {
            rt.push(to_add);
        }
    }

    rt
}

fn get_unique(input: &str) -> Vec<char> {
    let mut rt = Vec::new();
    for char in input.chars() {
        if SKIP_STR.contains(char) {
            continue;
        }
        if rt.contains(&char) {
            continue;
        }
        rt.push(char);
    }
    rt
}
