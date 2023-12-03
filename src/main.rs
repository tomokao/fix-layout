use std::process::Command;

use clap::Parser;
use fix_layout::backend::{x11::X11, Backend};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    regex: String,
    #[arg(short, long)]
    active_command: String,
    #[arg(short, long)]
    unactive_command: String,
}

fn command_from_string(s: &str) -> Command {
    if cfg!(target_os = "windows") {
        let mut command = Command::new("cmd");
        command.arg("/C").arg(s);
        command
    } else {
        let mut command = Command::new("sh");
        command.arg("-c").arg(s);
        command
    }
}

fn main() {
    let args = Args::parse();

    let mut backend = X11::create().unwrap();
    let regex = Regex::new(&args.regex).unwrap();
    loop {
        backend.wait_for_active_window();
        _ = (if backend.active_window_matches(|s| regex.is_match(s)) {
            command_from_string(&args.active_command)
        } else {
            command_from_string(&args.unactive_command)
        })
        .output();
    }
}
