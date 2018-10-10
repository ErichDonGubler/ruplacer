#[macro_use]
extern crate structopt;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

extern crate ruplacer;

#[derive(Debug, StructOpt)]
#[structopt(name = "ruplacer")]
struct Opt {
    #[structopt(long = "go")]
    go: bool,

    #[structopt(help = "The pattern to search for")]
    pattern: String,

    #[structopt(help = "The replacement")]
    replacement: String,

    #[structopt(parse(from_os_str), help = "The source path. Defaults to the working directory")]
    path: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let dry_run = !opt.go;

    let path = opt.path.unwrap_or(Path::new(".").to_path_buf());
    let query = ruplacer::query::substring(&opt.pattern, &opt.replacement);
    let mut directory_patcher = ruplacer::DirectoryPatcher::new(path);
    directory_patcher.dry_run(dry_run);
    let outcome = directory_patcher.patch(query);
    if let Err(err) = outcome {
        eprintln!("{}", err);
        process::exit(1);
    }
}
