use std::env;
use std::process;

// cargo clean removed the red line when re-arranging code into the lib
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // eprintln! prints to the standard error stream rather than the standard output
        // which is output.txt when running cargo run to poem.text > output.txt
        // rather than printing everything to the terminal.
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
