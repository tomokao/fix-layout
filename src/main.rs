use std::process::Command;

use clap::Parser;
use fix_layout::backend::{x11::X11, Backend, WindowAttribute};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, conflicts_with = "class_regex")]
    name_regex: Option<String>,
    #[arg(short, long, conflicts_with = "name_regex")]
    class_regex: Option<String>,
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
    let class_regex = args
        .class_regex
        .map(|s| Regex::new(&s).expect("invalid class regex"));
    let name_regex = args
        .name_regex
        .map(|s| Regex::new(&s).expect("invalid name regex"));
    loop {
        backend.wait_for_active_window();
        let matches = if let Some(ref cr) = class_regex {
            backend.active_window_matches(WindowAttribute::Class, |s| cr.is_match(s))
        } else if let Some(ref nr) = name_regex {
            backend.active_window_matches(WindowAttribute::Name, |s| nr.is_match(s))
        } else {
            false
        };
        _ = command_from_string(if matches {
            &args.active_command
        } else {
            &args.unactive_command
        })
        .output();
    }
}
