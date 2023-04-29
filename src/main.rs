use text_colorizer::*;
use std::env;
use std::fs;

fn print_usage() {
    eprintln!("{} - Change occurences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

const ARGS_LENGTH:usize = 4;

#[derive(Debug)]
struct Arguments {
    target: String,
    replace: String,
    filename: String,
    output: String
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != ARGS_LENGTH {
        print_usage();
        eprintln!("{} wrong number of arguments: expected {}, got {}.", "Error".red().bold(), ARGS_LENGTH, args.len());
        std::process::exit(1);
    }

    Arguments { target: args[0].clone(), replace: args[1].clone(), filename: args[2].clone(), output: args[3].clone() }
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}' : {:?}", "Error".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}' : {:?}", "Error".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };
}
