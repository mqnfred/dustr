use ::clap::Clap;

fn main() {
    let opts = Options::parse();
    if let Err(err) = run(opts) {
        eprintln!("error: {}", err);
        ::std::process::exit(1);
    }
}

fn run(opts: Options) -> ::anyhow::Result<()> {
    ::dustr::Package::new(opts.name, opts.crates)?.build(opts.destination)
}

#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "Louis Feuvrier")]
struct Options {
    #[clap(short = 'n', long = "name", about = "Dart library name")]
    name: String,
    #[clap(short = 'd', long = "dest", about = "Folder to initialize the dart library in")]
    destination: ::std::path::PathBuf,
    #[clap(name = "crates", about = "Crates to generate dart bindings for")]
    crates: Vec<::std::path::PathBuf>,
}
