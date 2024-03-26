use resplit::Cli;
use clap::Parser;
use create_new_library::read_stdin;

fn main() {
    let cli = Cli::parse();
    let buffer = read_stdin();
    let result = resplit::split(buffer,&cli);
    println!("{}", result);
}