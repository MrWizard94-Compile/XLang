use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use xlang_core::compile_source;

fn usage() {
    eprintln!("Usage: xlang check <source-file>");
}

fn read_source(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("could not read {}: {error}", path.display()))
}

fn run() -> Result<(), String> {
    let mut arguments = env::args_os();
    let _program = arguments.next();
    let command = arguments.next().ok_or_else(|| {
        usage();
        "missing command".to_owned()
    })?;

    if command != "check" {
        usage();
        return Err("expected the check command".to_owned());
    }

    let path = arguments.next().ok_or_else(|| {
        usage();
        "missing source file".to_owned()
    })?;

    if arguments.next().is_some() {
        usage();
        return Err("expected exactly one source file".to_owned());
    }

    let source_path = Path::new(&path);
    let source = read_source(source_path)?;
    let program = compile_source(&source).map_err(|error| error.to_string())?;

    println!(
        "XLang check passed: {} top-level item(s) in {}",
        program.items.len(),
        source_path.display()
    );
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("XLang check failed: {error}");
            ExitCode::FAILURE
        }
    }
}
