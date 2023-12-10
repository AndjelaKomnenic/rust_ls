use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Nije uneta putanja.");
        return;
    }

    let path = match fs::read_dir(&args[1]).ok() {
        None => {
            println!("Nije validna putanja.");
            return;
        }
        Some(path) => path,
    };

    //.filter(|p| p.is_ok()).map(|p| p.unwrap())
    for p in path.flatten() {
        println!("Name: {}", p.path().display())
    }
}
