use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Emanuel (evilenx)";

pub fn show_help() {
    println!("Usage: clock [options]");
    println!();
    println!("Options:");
    println!("    --help, -h       show this help");
    println!("    --version, -v    show version");
    println!();
    println!("Controls:");
    println!("    ESC              exit program");
    println!("    B                cycle background colors");
    println!("    F                cycle font colors");
    println!();
    println!("Configuration file: ~/.config/clock/config.toml");
    println!("Example:");
    println!("    [settings]");
    println!("    font_size = 80");
    println!("    padding = 20.0");
    println!("    auto_resize = true");
}

pub fn show_version() {
    println!("clock {} - by {}", VERSION, AUTHOR);
}

pub fn handle_args() -> bool {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                show_help();
                true
            },
            "--version" | "-v" => {
                show_version();
                true
            },
            _ => {
                eprintln!("clock: unknown option: {}", args[1]);
                eprintln!("Try 'clock --help' for more information.");
                true
            }
        }
    } else {
        false
    }
}
