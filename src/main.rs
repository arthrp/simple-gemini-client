use std::env;

use url::Url;

mod gemini;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let url = &Url::parse(&args[1]).unwrap();
    gemini::get_data(url);
}

fn print_usage(){
    println!("Usage: simple-gemini-client [url]");
}