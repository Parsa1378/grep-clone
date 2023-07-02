use std::fs;
use regex::Regex;
use std::path::{Path};

use crate::cli::Argument;

fn search_pattern_file(pattern: &str, file: &Path, ln:bool, invert_match:bool) {
    if let Ok(content) = fs::read_to_string(file) {
        let lines = content.lines();

        let regex = Regex::new(&format!(r"\b{}\b", pattern)).unwrap();

        for(line_number, line) in lines.enumerate() {
            let pattern_found = regex.is_match(line);
            if (!invert_match && pattern_found) || (invert_match && !pattern_found) {
                if ln {
                    println!("{}:{}: {}", file.display(), line_number + 1, line)
                } else {
                    print!("{}: {}",file.display(), line);
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

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    search_pattern_directory(pattern, &path, ln, depth - 1, invert_match);
                } else if path.is_file() {
                    search_pattern_file(pattern, &path, ln,invert_match);
                }
            }
        }
    } else {
        eprintln!("Failed to read file: {}", dir.display());
    }
}

pub fn search(arg:Argument) {
    if arg.files.len() == 1 && arg.files[0].is_dir() {
        search_pattern_directory(&arg.pattern, &arg.files[0], arg.ln, arg.depth.unwrap_or_default(), arg.invert_match)
    } else {
        for file in &arg.files {
            search_pattern_file(&arg.pattern, file, arg.ln, arg.invert_match)
        }
    }
}

