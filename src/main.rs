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

    for gib_path in files {
        let sgf_path = normalize_path(&gib_path).with_extension("sgf");

        if !sgf_path.exists() {
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

    if !converted {
        println!("no unconverted files")
    }
}
