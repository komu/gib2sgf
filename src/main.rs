mod files;

use std::fs;
use std::env;
use std::path::Path;
use filetime::FileTime;
use gib2sgf::gib_to_sgf;
use crate::files::{collect_files, normalize_path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let files = collect_files(Path::new(args.get(1).unwrap_or(&String::from(".")))).unwrap();
    let mut converted = false;
    let test = env::var("TEST").is_ok();

    for gib_path in files {
        let sgf_path = normalize_path(&gib_path).with_extension("sgf");

        if test {
            let gib_contents = fs::read_to_string(&gib_path).unwrap();
            let sgf = gib_to_sgf(&gib_contents).unwrap();
            if let Ok(existing_sgf) = fs::read_to_string(&sgf_path) {
                let old = remove_app_version_from_sgf(&existing_sgf);
                let new = remove_app_version_from_sgf(&sgf);

                if old != new {
                    println!("Results differ on {:?}\n   old: {}\n   new: {}", gib_path, old, new);
                }
            }

            // TODO: check that output is equal to previously generated SGF file
        } else if !sgf_path.exists() {
            println!("Convert {:?} -> {:?}", gib_path, sgf_path);

            // Convert file
            let gib_contents = fs::read_to_string(&gib_path).unwrap();
            let sgf_contents = gib_to_sgf(&gib_contents).unwrap();
            fs::write(&sgf_path, sgf_contents).unwrap();

            // Set the same modification time as the original file
            let metadata = fs::metadata(&gib_path).unwrap();
            let mtime = FileTime::from_last_modification_time(&metadata);
            filetime::set_file_mtime(&sgf_path, mtime).unwrap();
            converted = true
        }
    }

    if !test && !converted {
        println!("no unconverted files")
    }
}

fn remove_app_version_from_sgf(str: &str) -> String {
    if let Some(start) = str.find("AP[gib2sgf:") {
        let suffix = &str[start..];
        if let Some(end) = suffix.find("]") {
            return format!("{}{}", &str[..start], &suffix[end + 1..]);
        }
    }
    return str.to_string();
}
