use std::env;

// thread should be added
pub struct Argument {
    pub pattern: String,
    pub files: Vec<String>,
    pub ln: bool,
    pub depth: Option<usize>,
    pub invert_match: bool,
} 

pub fn parse_argument() -> Argument{
    let args:Vec<String> = env::args().collect();
    println!("first {}",args[1]);
    let mut pattern = String::new();
    let mut files:Vec<String> = Vec::new();
    let mut ln = false;
    let mut depth:Option<usize> = None;
    let mut invert_match = false;

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
                invert_match = true;
            }

            arg => {
                if pattern.is_empty() {
                    pattern = arg.to_owned();
                } else {
                    files.push(arg.to_owned());
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
        invert_match
    }

}