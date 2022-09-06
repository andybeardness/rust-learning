use std::env;

fn main() {
    let args = env::args();

    for arg in args {
        println!("arg == {}", arg);
    }

    let collected: Vec<String> = env::args().skip(1).collect();

    for c in collected {
        println!("c = {}", c);
    }
}
