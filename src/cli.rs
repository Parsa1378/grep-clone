use clap::{App, Arg};


pub struct Arguments {
    pub patterns: Vec<String>,
    pub files: Vec<String>,
    pub ln: bool,
    pub depth: Option<usize>,
    pub nthreads: usize,
    pub invert_match: bool,
} 

pub fn parse_argument() -> Arguments {
    let matches = App::new("grep")
                    .about("a grep clone written in Rust")
                    .arg(
                        Arg::new("patterns")
                            .multiple(true)
                            .required(true)
                            .about("the patterns to search for"),
                    )
                    .arg(
                        Arg::new("file")
                            .multiple(true)
                            .about("The files to search within"),
                    )
                    .arg(
                        Arg::new("line-number")
                            .short("ln")
                            .long("line-number")
                            .about("show line numbers of the matched lines"),
                    )
                    .arg(
                        Arg::new("depth")
                            .short('d')
                            .long("depth")
                            .takes_value(true)
                            .about("Set the depth of recursive directory search"),
                    )
                    .arg(
                        Arg::new("threads")
                            .short('t')
                            .long("threads")
                            .takes_value(true)
                            .about("Set the number of threads to use for search"),
                    )
                    .arg(
                        Arg::new("invert-match")
                            .short('v')
                            .long("invert-match")
                            .about("Select non-matching lines"),
                    )
                    .get_matches();
    let patters = matches.get_many("patterns").unwrap_or_default();
    let files = matches.get_many("files").unwrap_or_default();
    let line_number = matches.is_present("line-number");
    let depth = matches.get_one("depth").map(|val| val.parse().unwrap());
    let nthreads = matches.get_one("threads").map(|val| val.parse().unwrap_or(1));
    let invert_match = matches.is_present("invert-match");

    Arguments {
        patterns,
        files,
        line_number,
        depth,
        nthreads,
        invert_match,
    }
}