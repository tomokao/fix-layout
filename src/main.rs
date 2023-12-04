use std::path::PathBuf;

use clap::Parser;
use fix_layout::backend::{x11::X11, Backend, WindowAttribute};
use fix_layout::utils::{command_from_string, deserialize_regex};
use regex::Regex;
use serde::Deserialize;

const ABOUT: &str = "Run different commands based on active window.

Originally made for pinning certain application windows to a specific keyboard layout, but may have other uses.";

#[derive(Parser, Debug)]
#[command(author, version, about = ABOUT, long_about = None)]
struct Args {
    #[arg(short, long, exclusive = true, help = "Path to config file. Exclusive")]
    config_file: Option<PathBuf>,
    #[arg(
        short = 'N',
        long,
        conflicts_with = "class_regex",
        help = "Regex for window name (titlebar)"
    )]
    name_regex: Option<String>,
    #[arg(
        short = 'C',
        long,
        conflicts_with = "name_regex",
        help = "Regex for window class (application)"
    )]
    class_regex: Option<String>,
    #[arg(short, long, help = "Command to run when target window becomes active")]
    active_command: Option<String>,
    #[arg(
        short,
        long,
        help = "Command to run when another window becomes active"
    )]
    unactive_command: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    entries: Vec<Entry>,
}

#[derive(Deserialize)]
struct Entry {
    #[serde(deserialize_with = "deserialize_regex")]
    regex: Regex,
    #[serde(alias = "attribute")]
    target_attribute: WindowAttribute,
    #[serde(alias = "active")]
    active_command: Option<String>,
    #[serde(alias = "unactive")]
    unactive_command: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut entries;

    match args.config_file {
        Some(path) => {
            let config: Config =
                toml::from_str(&std::fs::read_to_string(path).expect("could not read config"))
                    .expect("invalid config");
            entries = config.entries;
        }
        None => {
            entries = Vec::new();
            let target_attribute = if args.class_regex.is_some() {
                WindowAttribute::Class
            } else {
                WindowAttribute::Name
            };
            entries.push(Entry {
                regex: Regex::new(&args.class_regex.or(args.name_regex).unwrap())
                    .expect("invalid regex"),
                target_attribute,
                active_command: args.active_command,
                unactive_command: args.unactive_command,
            });
        }
    }

    let mut backend = X11::create().unwrap();
    loop {
        backend.wait_for_active_window();
        for entry in entries.iter() {
            _ = command_from_string(
                if backend
                    .active_window_matches(entry.target_attribute, |s| entry.regex.is_match(s))
                {
                    if entry.active_command.is_none() {
                        continue;
                    }
                    entry.active_command.as_ref().unwrap()
                } else {
                    if entry.unactive_command.is_none() {
                        continue;
                    }
                    entry.unactive_command.as_ref().unwrap()
                },
            )
            .output();
        }
    }
}
