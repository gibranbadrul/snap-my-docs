mod args;
mod cmd;

use clap::Parser;
use args::Args;

fn main() {
    let args = Args::parse();

    if args.init.is_some() {
        cmd::init::run(args.init);
    } else {
        cmd::generate::run(&args.config, &args.output);
    }
}
