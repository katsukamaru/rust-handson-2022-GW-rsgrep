use std::fs::read_to_string;
use rayon::prelude::*;

use structopt::StructOpt;

/*
 * file_nameとともに検索結果を表示させる
 */
fn grep(state: &GrepArgs, content: String, file_name: &str) {
    for (index, line) in content.lines().enumerate() {
        if line.contains(state.pattern.as_str()) {
            println!("{}:{} {}", file_name, index + 1, line);
        }
    }
}

fn run(state: GrepArgs) {
    state.path.par_iter().for_each(|file| match read_to_string(file) {
        Ok(content) => grep(&state, content, file),
        Err(reason) => print!("{}: {}\n", file, reason),
    })
}

fn main() {
    run(GrepArgs::from_args());
}

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>,
}

impl GrepArgs {
    fn print_pattern(self, user: String) {
        let pattern = self.pattern;
        println!("from: {}, pattern: {}", user, pattern)
    }
}
