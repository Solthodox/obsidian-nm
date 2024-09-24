#[allow(unused_imports)]
use clap::Parser;
#[allow(unused_imports)]
use obsidiannm::commands::Args;
#[allow(unused_imports)]
use yansi::Paint as _;

pub fn main() {
    let args = Args::parse();
    match obsidiannm::run(args.command) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err.to_string().red())
        }
    }
}
