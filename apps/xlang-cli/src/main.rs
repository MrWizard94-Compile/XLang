use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use aether_core::{
    canonical_ast, compile_source, compile_to_bytecode, run_bytecode, LANGUAGE_NAME,
    LANGUAGE_VERSION,
};

fn usage() {
    eprintln!(
        "Usage:\n  aether check <source-file>\n  aether compile <source-file> --output <artifact-file>\n  aether run <artifact-file>\n  aether version"
    );
}

fn read_source(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("could not read {}: {error}", path.display()))
}

fn read_artifact(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("could not read {}: {error}", path.display()))
}

fn check(source_path: &Path) -> Result<(), String> {
    let source = read_source(source_path)?;
    let program = compile_source(&source).map_err(|error| error.to_string())?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} check passed: {} significant token(s) in {}",
        program.significant_token_count(),
        source_path.display()
    );
    println!("{}", canonical_ast(&program));
    Ok(())
}

fn compile(source_path: &Path, output_path: &Path) -> Result<(), String> {
    let source = read_source(source_path)?;
    let output = compile_to_bytecode(&source).map_err(|error| error.to_string())?;
    let parent = output_path
        .parent()
        .filter(|candidate| !candidate.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if !parent.is_dir() {
        return Err(format!(
            "output directory {} does not exist",
            parent.display()
        ));
    }
    fs::write(output_path, output.bytecode)
        .map_err(|error| format!("could not write {}: {error}", output_path.display()))?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} compiled {} to {}",
        source_path.display(),
        output_path.display()
    );
    Ok(())
}

fn execute_artifact(artifact_path: &Path) -> Result<(), String> {
    let artifact = read_artifact(artifact_path)?;
    let output = run_bytecode(&artifact).map_err(|error| error.to_string())?;
    print!("{}", output.stdout);
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} exited with {}",
        output.exit_code
    );
    Ok(())
}

fn next_argument(
    arguments: &mut impl Iterator<Item = OsString>,
    name: &str,
) -> Result<OsString, String> {
    arguments.next().ok_or_else(|| format!("missing {name}"))
}

fn run() -> Result<(), String> {
    let mut arguments = env::args_os();
    let _program = arguments.next();
    let command = next_argument(&mut arguments, "command")?;
    match command.to_string_lossy().as_ref() {
        "check" => {
            let source = next_argument(&mut arguments, "source file")?;
            if arguments.next().is_some() {
                return Err("check accepts exactly one source file".to_owned());
            }
            check(Path::new(&source))
        }
        "compile" => {
            let source = next_argument(&mut arguments, "source file")?;
            let output_flag = next_argument(&mut arguments, "--output flag")?;
            if output_flag != "--output" {
                return Err("compile requires --output <artifact-file>".to_owned());
            }
            let output = next_argument(&mut arguments, "artifact output file")?;
            if arguments.next().is_some() {
                return Err(
                    "compile accepts one source file and one Aether artifact output file"
                        .to_owned(),
                );
            }
            compile(Path::new(&source), Path::new(&output))
        }
        "run" => {
            let artifact = next_argument(&mut arguments, "artifact file")?;
            if arguments.next().is_some() {
                return Err("run accepts exactly one Aether artifact file".to_owned());
            }
            execute_artifact(Path::new(&artifact))
        }
        "version" => {
            if arguments.next().is_some() {
                return Err("version does not accept arguments".to_owned());
            }
            println!("{LANGUAGE_NAME} {LANGUAGE_VERSION}");
            Ok(())
        }
        _ => Err(format!("unknown command {command:?}")),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            usage();
            eprintln!("{LANGUAGE_NAME} failed: {error}");
            ExitCode::FAILURE
        }
    }
}
