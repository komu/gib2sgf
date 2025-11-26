mod files;

use std::fs;
use std::env;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use filetime::FileTime;
use gib2sgf::gib_to_sgf;
use crate::files::{collect_files, normalize_path};
use clap::Parser;
use encoding_rs::{EUC_KR, UTF_8};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

#[derive(Parser)]
#[command(name = "gib2sgf")]
#[command(about = "Convert GIB files to SGF format", long_about = None)]
struct Args {
    /// Path to GIB file or directory containing GIB files
    #[arg(default_value = ".")]
    path: String,

    /// Upload converted files to AI Sensei
    #[arg(long)]
    upload: bool,
}

fn read_file_with_encoding(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Try UTF-8 first
    if let Ok(s) = String::from_utf8(bytes.clone()) {
        return Ok(s);
    }

    // Try EUC-KR (common for Korean GIB files)
    let (decoded, _, had_errors) = EUC_KR.decode(&bytes);
    if !had_errors {
        return Ok(decoded.into_owned());
    }

    // Last resort: use UTF-8 with lossy conversion
    let (decoded, _, _) = UTF_8.decode(&bytes);
    Ok(decoded.into_owned())
}

fn main() {
    let args = Args::parse();

    let files = collect_files(Path::new(&args.path)).unwrap();
    let mut converted_files: Vec<PathBuf> = Vec::new();
    let test = env::var("TEST").is_ok();

    for gib_path in files {
        let sgf_path = normalize_path(&gib_path).with_extension("sgf");

        if test {
            let gib_contents = match read_file_with_encoding(&gib_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading {:?}: {}", gib_path, e);
                    continue;
                }
            };
            let sgf = gib_to_sgf(&gib_contents).unwrap();
            if let Ok(existing_sgf_bytes) = fs::read(&sgf_path) {
                if let Ok(existing_sgf) = String::from_utf8(existing_sgf_bytes) {
                    let old = remove_app_version_from_sgf(&existing_sgf);
                    let new = remove_app_version_from_sgf(&sgf);

                    if old != new {
                        println!("Results differ on {:?}\n   old: {}\n   new: {}", gib_path, old, new);
                    }
                }
            }

            // TODO: check that output is equal to previously generated SGF file
        } else if !sgf_path.exists() {
            println!("Convert {:?} -> {:?}", gib_path, sgf_path);

            // Convert file
            let gib_contents = match read_file_with_encoding(&gib_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading {:?}: {}", gib_path, e);
                    continue;
                }
            };
            let sgf_contents = gib_to_sgf(&gib_contents).unwrap();
            fs::write(&sgf_path, sgf_contents).unwrap();

            // Set the same modification time as the original file
            let metadata = fs::metadata(&gib_path).unwrap();
            let mtime = FileTime::from_last_modification_time(&metadata);
            filetime::set_file_mtime(&sgf_path, mtime).unwrap();

            converted_files.push(sgf_path);
        }
    }

    if !test && converted_files.is_empty() {
        println!("no unconverted files")
    }

    // Handle AI Sensei upload
    if !test && args.upload && !converted_files.is_empty() {
        upload_to_ai_sensei(&converted_files);
    }
}

fn upload_to_ai_sensei(converted_files: &[PathBuf]) {
    println!("\nUploading {} file(s) to AI Sensei:", converted_files.len());

    // Process each file one at a time
    for (idx, path) in converted_files.iter().enumerate() {
        println!("\n[{}/{}] Processing: {}", idx + 1, converted_files.len(), path.display());

        // Read SGF content (SGF files should be UTF-8)
        let sgf_content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("✗ Failed to read SGF file: {}", e);
                continue;
            }
        };

        // URL-encode the SGF content and build the URL
        let encoded_sgf = utf8_percent_encode(&sgf_content, NON_ALPHANUMERIC).to_string();
        let url = format!("https://ai-sensei.com/upload?sgf={}", encoded_sgf);

        println!("✓ Opening AI Sensei with SGF data...");

        // Open browser with SGF data in URL
        if let Err(e) = webbrowser::open(&url) {
            eprintln!("✗ Failed to open browser: {}", e);
            eprintln!("  Please manually visit: https://ai-sensei.com/upload");
            continue;
        }

        println!("✓ Browser opened - review and submit the upload");

        // Wait before processing next file
        if idx + 1 < converted_files.len() {
            println!("\nWaiting 3 seconds before next upload...");
            thread::sleep(Duration::from_secs(3));
        }
    }

    println!("\n✓ All uploads completed!");
}

fn remove_app_version_from_sgf(str: &str) -> String {
    if let Some(start) = str.find("AP[gib2sgf:") {
        let suffix = &str[start..];
        if let Some(end) = suffix.find("]") {
            return format!("{}{}", &str[..start], &suffix[end + 1..]);
        }
    }
    str.to_string()
}
