use std::collections::HashSet;
mod const_util;
mod file_util;

fn main() {
    // read learned file, get the learned set of String
    let learned_result = file_util::read_file_into_set("learned.txt".to_string());
    let learned_set = match learned_result {
        Ok(set) => set,
        Err(_) => {
            eprintln!("Error reading learned");
            return;
        }
    };
    // read unlearned file, get the unlearned set of String
    let unlearned_result = file_util::read_file_into_set("unlearned.txt".to_string());
    let unlearned_set = match unlearned_result {
        Ok(set) => set,
        Err(_) => {
            eprintln!("Error reading unlearned");
            return;
        }
    };

    let mut diff = unlearned_set
        .difference(&learned_set)
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();

    // get skip char set
    let skip_char_set: HashSet<String> = const_util::SKIP_STR
        .to_string()
        .chars()
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();

    // skip the skip char
    diff = diff
        .difference(&skip_char_set)
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();

    match file_util::write_set_to_file("output.txt".to_string(), diff) {
        Ok(_) => return,
        Err(_) => {
            println!("failed to output!");
            return;
        }
    }
}
