use std::env;

fn main() {
    // Retrieve arguments from CLI
    let args: Vec<String> = env::args().skip(1).collect(); 

    // Join "args" together
    // Assuming no flags at present
    let echo = args.join(" ");

    // Return result to stdout
    println!("{}", echo);
}
