use rusty_infer::*;
use std::env;
use std::process;

fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match gen_anns(&cfg) {
        Ok(_) => (),
        Err(ge) => {
            eprintln!("Problem generating anns: {}", ge.msg);
            // TODO: run cleanup on dirs etc?
            process::exit(1);
        }
    }

    copy_imgs(&cfg);
}
