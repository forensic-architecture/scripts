use std::env;
use std::process;
use supervisely_rs::*;

fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    gen_anns(&cfg);
}
