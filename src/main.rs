use structopt::StructOpt;
use termimad;

#[derive(StructOpt)]
#[structopt(name = "termd", about = "Markdown terminal viewer.")]
struct Options {
    input: std::path::PathBuf,
}

fn main() {
    let options = Options::from_args();
    let text = std::fs::read_to_string(options.input).expect("Unable to read input file");
    termimad::print_text(&text);
}
