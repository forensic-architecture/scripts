use rusty_infer::*;
use std::env;
use std::process;

fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    gen_anns(&cfg).unwrap_or_else(|err| {
        eprintln!("Problem generating anns: {}", err);
        // TODO: run cleanup on dirs etc?
        process::exit(1);
    });

    copy_imgs(&cfg);
}
