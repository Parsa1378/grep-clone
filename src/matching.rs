use std::fs;
use regex::Regex;
use std::thread;
use std::path::{Path};

use crate::cli::Argument;

fn search_pattern_file(pattern: &str, file: &Path, ln:bool, invert_match:bool) {
    if let Ok(content) = fs::read_to_string(file) {
        let lines = content.lines();

        let regex = Regex::new(&format!(r"\b{}\b", pattern)).unwrap();

        for(line_number, line) in lines.enumerate() {
            let pattern_found = regex.is_match(line);
            if (!invert_match && pattern_found) || (invert_match && !pattern_found) {
                let colored_line = regex.replace_all(line, |caps: &regex::Captures| {
                    format!("\x1B[32m{}\x1B[0m", caps.get(0).unwrap().as_str())
                });
                if ln {
                    println!("{}:{}: {}", file.display(), line_number + 1, colored_line)
                } else {
                    print!("{}: {}",file.display(), colored_line);
                }
            }
        }
    } else {
        eprintln!("Failed to read file: {}", file.display());
    }
}

fn search_pattern_directory(pattern: &str, dir: &Path, ln:bool, depth:usize, invert_match:bool) {
    if depth == 0 {
        return;
    }

    let mut threads:Vec<std::thread::JoinHandle<()>> = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let pattern_clone = pattern.to_owned();
                    let thread = thread::spawn(move || {
                        search_pattern_directory(&pattern_clone, &path, ln, depth - 1, invert_match)
                    });
                    threads.push(thread);
                } else if path.is_file() {
                    search_pattern_file(&pattern, &path, ln,invert_match);
                }
            }
        }
    } else {
        eprintln!("Failed to read file: {}", dir.display());
    }

    for thread in threads {
        thread.join().unwrap();
    }
}

pub fn search(args:Argument) {
    if args.files.is_empty() {
        println!("No files specified");
        return;
    }
    if args.files.len() == 1 && args.files[0].is_dir() {
        search_pattern_directory(&args.pattern, &args.files[0], args.ln, args.depth.unwrap_or_default(), args.invert_match)
    } else {
        let files = args.files.clone();
        let mut threads:Vec<std::thread::JoinHandle<()>> = Vec::new();
        for file in files {
            let pattern_clone = args.pattern.to_owned();
            let thread = thread::spawn(move || {
                search_pattern_file(&pattern_clone, &file, args.ln, args.invert_match);
            });
            threads.push(thread);
        }
        for thread in threads {
            thread.join().unwrap();
        }
    }
}
