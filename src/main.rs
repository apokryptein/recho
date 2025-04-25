use clap::Parser;

#[derive(Parser,Debug)]
#[command(name="recho", version, about="A simple echo-like CLI took written in Rust")]
struct Args {
    /// Remove trailing newline
    #[arg(short = 'n', long)]
    no_newline: bool,

    /// Interpret escape sequence characters
    #[arg(short = 'e', long, overrides_with = "no_escape")]
    escape: bool,

    /// Disable interpretation of backslash escapes (default)
    #[arg(short = 'E', long, overrides_with = "escape")]
    no_escape: bool,

    /// String to echo 
    #[arg()]
    text: Vec<String>,

}

fn main() {
    // Retrieve clap-defined args
    let args = Args::parse();

    // Join remainder or arguments -> what to echo
    let mut echo = args.text.join(" ");

    // Parse escapes if escape flag present
    if args.escape {
        echo = parse_escapes(&echo);
    } 

    // Check for newline removal and 
    // return result to stdout
    if args.no_newline {
        print!("{}", echo);
    } else {
        println!("{}", echo);
    }
}

// Parses escape characters for provided echo output
fn parse_escapes(output: &str) -> String {
    output
        .replace("\\\\", "\\") // backslash
        .replace("\\n", "\n")   // newline
        .replace("\\r", "\r")   // carriage return
        .replace("\\t", "\t")   // tab
        .replace("\\\"", "\"")  // double quote
}
