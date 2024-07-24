use asciigraph::Graph;
use clap::Parser;
use std::fs::{File, write};
use std::io::Read;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of an input json file
    #[arg(short, long)]
    input: String,

    /// Path of an output\
    /// If it's 'STDOUT', the result is dumped to stdout
    #[arg(short, long, default_value_t = String::from("STDOUT"))]
    output: String,
}

fn main() {
    let args = Args::parse();

    #[cfg(feature = "json")] {
        let mut s = String::new();
        let mut f = File::open(&args.input).unwrap();
        f.read_to_string(&mut s).unwrap();
        let g = Graph::from_json(&s).unwrap();

        if args.output == "STDOUT" {
            println!("{}", g.draw());
        }

        else {
            write(&args.output, g.draw().as_bytes()).unwrap();
        }
    }

    #[cfg(not(feature = "json"))] {
        panic!("`json` feature is not enabled!");
    }
}
