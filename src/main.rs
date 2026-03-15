mod cli;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();
    println!("Input: {}", args.input);
    println!("Output dir: {}", args.output);
    println!("Format: {}", args.format);
    println!("Quality: {}", args.quality);
    println!("Verbose: {}", args.verbose);
}
