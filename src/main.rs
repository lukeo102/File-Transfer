mod server;
mod client;
mod common;

use std::env;
use std::process::exit;


fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = args[1].as_str();

    if args.len() < 2 {
        println!("Not enough parameters");
        help();
    }

    match arg1 {
        "client" => { client(args); }
        "server" => { server(args); }
        _ => {
            println!("{} command unknown", &arg1);
            help();
        }
    }

    client::client::test();

}

fn client(args: Vec<String>) {

    println!()

}

fn server(args: Vec<String>) {
    // Parse requires a type, here, the type is given in the variable definition
    let number: i32 = args[2].parse().unwrap();

    server::server::main();

}

fn help() {
    println!("Unkown command, see usage below");
    // Usage info
}