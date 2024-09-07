mod catr_cli;

use std::process;

use catr_cli::CatrCli;
use clap::Parser;

fn main() {
    let arg = CatrCli::parse();
    if let Err(_) = arg.print_content() {
        process::exit(1);
    }
}
