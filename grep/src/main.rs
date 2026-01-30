use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target = args.get(1).expect("specify a word");
    let filename = match args.get(2) {
        Some(v) => v,
        None => panic!("Searching in a directory is not supported yet."),
    };
    let contents = fs::read_to_string(filename).expect("file not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    for (index, line) in lines.into_iter().enumerate() {
        if let Some(result) = search_in_a_line(target, line, index) {
            println!("{result}")
        }
    }
}

fn search_in_a_line(target: &str, line: &str, index: usize) -> Option<String> {
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let mut found = false;
    let mut results: Vec<String> = Vec::new();
    results.push(format!("\x1b[32m{}\x1b[0m:", (index + 1).to_string()));
    for word in words.into_iter() {
        if word == target {
            found = true;
            results.push(format!("\x1b[31m{}\x1b[0m", word))
        } else {
            results.push(word.to_string());
        }
    }
    let mut ans = String::new();
    for word in results.into_iter() {
        ans.push_str(word.as_str());
        ans.push(' ');
    }
    if found {
        return Some(ans);
    } else {
        return None;
    }
}
