use matching::search;
mod cli;
mod matching;
fn main() {
    let args = cli::parse_argument();
    if args.files.is_empty() {
        println!("No files specified");
    }
    search(args);
}
