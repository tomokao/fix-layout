use std::process::Command;

use fix_layout::backend::{x11::X11, Backend};
use regex::Regex;

fn main() {
    let mut backend = X11::create().unwrap();
    let regex = Regex::new(r"arch").unwrap();
    loop {
        backend.wait_for_active_window();
        _ = Command::new("setxkbmap")
            .args([
                "-model",
                "abnt2",
                "-layout",
                "br",
                "-variant",
                if backend.active_window_matches(&regex) {
                    "abnt2"
                } else {
                    "dvorak"
                },
            ])
            .output();
    }
}
