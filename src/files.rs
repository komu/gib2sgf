use std::{fs, io};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

pub fn collect_files(dir: &Path) -> io::Result<Vec<Box<Path>>> {
    let mut result = Vec::new();
    collect_files_into(dir, &mut result)?;
    Ok(result)
}

fn collect_files_into(dir: &Path, result: &mut Vec<Box<Path>>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                collect_files_into(&path, result)?;
            } else if path.is_file() && path.extension() == Some(OsStr::new("gib")) {
                result.push(path.into_boxed_path());
            }
        }
    }
    Ok(())
}

/// For some reason Tygem files names can end up with all kinds of junk
pub fn normalize_path(path: &Path) -> PathBuf {
    if let Some(str) = path.file_name().map(|n| n.to_str()).flatten() {
        path.with_file_name(normalize_file_name(str))
    } else {
        path.to_path_buf()
    }
}

fn normalize_file_name(name: &str) -> String {
    if name.ends_with(".gib") || name.ends_with(".gib.dll") {
        let mut result = name;
        loop {
            let previous = result;
            result = result.trim_end_matches(".gib").trim_end_matches(".dll").trim_end_matches('.');
            if result.len() == previous.len() {
                break
            }
        }

        format!("{}.gib", result)
    } else {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_file_name() {
        assert_eq!(normalize_file_name(""), "");
        assert_eq!(normalize_file_name("foo"), "foo");
        assert_eq!(normalize_file_name("foo.gib"), "foo.gib");
        assert_eq!(normalize_file_name("foo.gib.gib"), "foo.gib");
        assert_eq!(normalize_file_name("foo.gib.dll"), "foo.gib");
        assert_eq!(normalize_file_name("foo.gib.dll.gib"), "foo.gib");
        assert_eq!(normalize_file_name("foo.gib..gib"), "foo.gib");
    }
}
