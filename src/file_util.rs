use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, Error, Write},
};

pub fn read_file_into_set(file_path: String) -> Result<std::collections::HashSet<String>, Error> {
    let file = File::open(file_path)?;

    let mut ret: HashSet<String> = HashSet::new();

    let lines = io::BufReader::new(file).lines();

    for line_result in lines {
        if let Ok(line) = line_result {
            line.chars().map(|ch| ch.to_string()).for_each(|ch| {
                ret.insert(ch);
            })
        }
    }
    Ok(ret)
}

#[test]
fn test_read_file_into_set() {
    read_file_into_set("tests/learned.txt".to_string());
}

pub(crate) fn write_set_to_file(file_path: String, diff: HashSet<String>) -> Result<(), Error> {
    if let Err(e) = fs::remove_file(&file_path) {
        println!("{:?}", e);
    }
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;
    for c in diff {
        print!("{} ", c.to_string());
        output_file.write_all(format!("{}{}", c.to_string(), "\n").as_bytes())?;
    }

    Ok(())
}
