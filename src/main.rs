use notify::Watcher;
use std::time::Duration;
use structopt::StructOpt;
use termimad;
use termimad::MadSkin;

#[derive(StructOpt)]
#[structopt(name = "termd", about = "Markdown terminal viewer.")]
struct Options {
    input: std::path::PathBuf,

    #[structopt(short = "d", long = "daemon")]
    daemon: bool,
}

fn main() {
    let options = Options::from_args();
    // let text = std::fs::read_to_string(options.input.clone()).expect("Unable to read input file");
    // termimad::print_text(&text);

    if options.daemon {
        eprintln!("Running in daemon mode");
        // let mut watcher =
        //     notify::immediate_watcher(|res: notify::Result<notify::Event>| match res {
        //         Ok(ev) => match ev.kind {
        //             notify::EventKind::Modify(_modify_kind) => {
        //                 let text = std::fs::read_to_string(ev.paths.first().unwrap())
        //                     .expect("Unable to read input file");
        //                 termimad::print_text(&text);
        //             }
        //             _ => eprintln!("other event: {:?}", ev),
        //         },
        //         Err(e) => eprintln!("watch error: {:?}", e),
        //     })
        //     .unwrap();
        // eprintln!("Start watching {:?}", options.input);
        // // watcher.configure(notify::Config::PreciseEvents(true));
        // watcher
        //     .watch(options.input, notify::RecursiveMode::NonRecursive)
        //     .unwrap();

        let mut watcher: notify::RecommendedWatcher =
            Watcher::new_immediate(|res: notify::Result<notify::Event>| match res {
                Ok(ev) => match ev.kind {
                    notify::EventKind::Modify(_modify_kind) => {
                        let text = std::fs::read_to_string(ev.paths.first().unwrap())
                            .expect("Unable to read input file");
                        // termimad::print_text(&text);
                        let skin = MadSkin::default();
                        let area = termimad::Area::full_screen();
                        let text = skin.area_text(&text, &area);
                        let view = termimad::TextView::from(&area, &text);
                        view.write().unwrap();
                    }
                    _ => eprintln!("other event: {:?}", ev),
                },
                Err(e) => eprintln!("watch error: {:?}", e),
            })
            .unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher
            .watch(options.input, notify::RecursiveMode::NonRecursive)
            .unwrap();

        eprintln!("Stopped watching");
        std::thread::sleep(Duration::from_secs(120));
    }
}
