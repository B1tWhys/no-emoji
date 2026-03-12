use std::io;
use std::process::{ExitCode, exit};
use std::{env, fs::File, io::{BufRead, BufReader}, path::PathBuf};

const RST: &'static str = "\x1b[m";
const RED: &'static str = "\x1b[31m";

fn main() -> Result<(), ExitCode> {
    let has_errors = env::args().skip(1)
        .map(|arg| !file_is_ascii(&arg.into())
            .unwrap_or(false))
        .any(|x| x);

    if has_errors {
        Err(ExitCode::FAILURE)
    } else {
        Ok(()) 
    }
}

fn file_is_ascii(path: &PathBuf) -> io::Result<bool> {
    let file = File::open(&path).inspect_err(|e| eprintln!("Failed to open file path: {path:?}: {}", e))?;
    let reader = BufReader::new(file);
    let mut is_ascii = true;

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let line_num = line_num + 1;
        if let Some((col, bad_char)) = line.chars().enumerate().find(|(_, c)| !c.is_ascii()) {
            eprintln!("{RED}Error:{RST} Non-ascii character {bad_char} found at: {path:?}:{line_num}:{col}");
            is_ascii = false;
        }
    }

    Ok(is_ascii)
}

#[cfg(test)]
mod test {
    use super::*;

    fn path_in_test_dir(rel_path: &str) -> PathBuf {
        let test_dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();
        test_dir.join("test_files").join(rel_path)
    }

    #[test]
    fn file_is_ascii_with_bogus_file_path() {
        let test_file_path = path_in_test_dir("bogus_file_path.txt");
        let actual = file_is_ascii(&test_file_path);
        assert!(actual.is_err());
    }

    
    #[test]
    fn file_is_ascii_for_all_ascii_file() {
        let test_file_path = path_in_test_dir("ascii_file.txt");
        let actual = file_is_ascii(&test_file_path);
        assert!(matches!(actual, Ok(true)));
    }

    #[test]
    fn file_is_ascii_with_unicode() {
        let test_file_path = path_in_test_dir("unicode_file.txt");
        let actual = file_is_ascii(&test_file_path);
        assert!(matches!(actual, Ok(false)));
    }
}
