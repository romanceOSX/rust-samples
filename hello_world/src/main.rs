use clap::Parser;
/*
 * I followed the guide from the CLI book
 *  --> https://rust-cli.github.io/book/index.html
 */

/* tool <pattern> <filepath> */
fn _print_environment_vars() {
    for (k, v) in std::env::vars() {
        println!("{k}: {v}");
    }
}

#[derive(Parser)]
struct CliArgument {
    pattern: String,
    path: std::path::PathBuf,
}

fn _print_cli_args() {
    println!("printing provided arguments...");
    for arg in std::env::args() {
        println!("{arg}");
    }
}

fn _handle_cli_args() {
    /* .nth is an iter funciton */
    let pattern = std::env::args().nth(1).expect("Please provide a pattern");
    let filepath = std::env::args().nth(2).expect("Please provide a filepath");

    println!("Looking for \"{pattern}\" in {filepath}");
}

fn _handle_cli_args_clap() {
    let args = CliArgument::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}

fn main() {
    println!("Hello, world!");
    //_print_environment_vars();
    _print_cli_args();
    //_handle_cli_args();
    _handle_cli_args_clap();
}

