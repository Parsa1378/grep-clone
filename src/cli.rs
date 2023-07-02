use std::env;
use std::path::{PathBuf};

// thread should be added
pub struct Argument {
    pub pattern: String,
    pub files: Vec<PathBuf>,
    pub ln: bool,
    pub depth: Option<usize>,
    pub invert_match: bool,
    pub case_sensitive:bool,
} 

pub fn parse_argument() -> Argument{
    let args:Vec<String> = env::args().collect();
    let mut pattern = String::new();
    let mut files:Vec<PathBuf> = Vec::new();
    let mut ln = false;
    let mut depth:Option<usize> = None;
    let mut invert_match = false;
    let mut case_sensitive = true;

    let mut i = 1;
    while i<args.len() {
        match args[i].as_str() {
            "-ln" => {
                ln = true;
            }

            "-d" => {
                i+=1;
                depth = args[i].parse::<usize>().ok();
            }

            "-i" => {
                case_sensitive = false;
            }

            "-v" => {
                invert_match = true;
            }            

            arg => {
                if pattern.is_empty() {
                    pattern = arg.to_owned();
                } else {
                    files.push(PathBuf::from(arg));
                }
            }
        }
        i+=1;
    }

    Argument {
        pattern,
        files,
        ln,
        depth,
        invert_match,
        case_sensitive,
    }

}