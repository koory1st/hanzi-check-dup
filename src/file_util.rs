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
            // read line every charactors and insert into result Hash set
            read_line_into_set(&mut ret, line);

            // line.chars().map(|ch| ch.to_string()).for_each(|ch| {
            //     ret.insert(ch);
            // })
        }
    }
    Ok(ret)
}

fn read_line_into_set(set: &mut HashSet<String>, line: String) {
    let mut tmp = String::new();
    for ch in line.chars() {
        let char_string = ch.to_string();
        // not alphabet,
        if char_string.bytes().len() > 1 {
            set.insert(char_string);
            continue;
        }

        // alphabet
        if char_string.le(&"z".to_string()) && char_string.ge(&"A".to_string()) {
            tmp.push_str(&char_string);
        } else {
            if tmp.chars().count() > 0 {
                set.insert(tmp.clone());
                tmp.clear();
            }
            continue;
        }
    }

    if tmp.chars().count() > 0 {
        set.insert(tmp);
    }
}

#[test]
fn test_read_line_into_set() {
    let mut set = HashSet::<String>::new();
    read_line_into_set(&mut set, "abc, efg,aaaa,ZZZ,AAA".to_string());
    println!("{:?}", set);
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
