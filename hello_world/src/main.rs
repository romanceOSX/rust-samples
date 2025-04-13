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
    //_print_cli_args();
    //_handle_cli_args();
    //_handle_cli_args_clap();
    _test_pnet();
}

/* conditionally implement a method depending on the trait */
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{x, y}
    }
}

/*
 * Interfaces abstraction wrapper sample
 */
trait Interface {
    fn print_info(&self);
}

struct InterfacePnet<'a> {
    iface: &'a pnet::datalink::NetworkInterface,
}

impl<'a> InterfacePnet<'a> {
    fn new(pnet_iface: &'a pnet::datalink::NetworkInterface) -> Self {
        Self {
            iface: pnet_iface,
        }
    }
}

impl<'a> Interface for InterfacePnet<'a> {
    fn print_info(&self) {
        println!("Helloo!");
    }
}

fn interfaces<'a>() -> Vec<Box<dyn Interface>> {
    /* construct vector of interfaces */
    let mut interfaces: Vec<Box<dyn Interface>> = Vec::new();
    interfaces.push(
        Box::new(
            InterfacePnet {
                /* datalink::interfaces() creates an stack-allocated object */
                //iface: &pnet::datalink::interfaces().get(0).unwrap(),
            }
        )
    );

    /* 
     * from Vec<pnet::NetworkInterfaces> 
     *   to Vec<Interface> 
     */

    interfaces
}

/* 
 * function that expects a vector, and returns a reference value that has the same
 * lifetime as the vector, i.e., an element of the vector
 * note that the lifetime notation uses the same syntax as generics
 */
fn _test_vector_references<'a, T>(vector: &'a Vec<T>) -> &'a T {
    vector.iter().nth(0).unwrap()
}

fn _test_pnet() {
    let ifaces = pnet::datalink::interfaces();
    for iface in ifaces {
        println!("- {:?}", iface.ips);
    }

    let vec = vec![1, 2, 3, 4, 5];
    print!("This is the first thing {}", _test_vector_references(&vec))
}


