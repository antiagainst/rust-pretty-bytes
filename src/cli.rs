use ::converter;

use std::io;
use std::env;
use getopts::Options;
use libc::{isatty, STDIN_FILENO};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} <number>", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

fn stdin_isatty() -> bool {
    let istty = unsafe { isatty(STDIN_FILENO as i32) };
    istty == 1
}

pub fn run(args: env::Args) -> () {
    let num: f64 = if stdin_isatty() {
        let args: Vec<String> = args.collect();
        let ref program = args[0];

        let mut opts = Options::new();
        opts.optflag("h", "help", "print this help menu");
        opts.optflag("v", "version", "print version");

        let matches = match opts.parse(&args[1..]) {
            Ok(m)  => { m }
            Err(f) => { panic!(f.to_string()) }
        };

        if matches.opt_present("v") {
            return print_version();
        } else if matches.opt_present("h") || args.len() != 2 {
            return print_usage(&program, opts);
        }

        match args[1].parse::<f64>() {
            Ok(n) => n,
            Err(f) => { panic!(f.to_string()) }
        }
    } else {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Unable to read from console!");
        input.trim().parse::<f64>().unwrap()
    };

    // TODO change name
    println!("{}", converter::convert(num));
}
