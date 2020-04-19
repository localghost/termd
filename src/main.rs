use structopt::StructOpt;
use termimad;

#[derive(StructOpt)]
#[structopt(name = "termd", about = "Markdown terminal viewer.")]
struct Options {
    input: std::path::PathBuf,

    #[structopt(short = "d", long = "daemon")]
    daemon: bool,
}

fn main() {
    let options = Options::from_args();
    let text = std::fs::read_to_string(options.input).expect("Unable to read input file");
    termimad::print_text(&text);

    if options.daemon {
        let mut watcher =
            notify::immediate_watcher(|res: notify::Result<notify::Event>| match res {
                Ok(ev) => match ev.kind {
                    notify::EventKind::Modify(_modify_kind) => {
                        let text = std::fs::read_to_string(options.input.clone())
                            .expect("Unable to read input file");
                        // termimad::print_text(&text);
                    }
                    _ => eprintln!("other event: {:?}", ev),
                },
                Err(e) => eprintln!("watch error: {:?}", e),
            })
            .unwrap();
    }
}
