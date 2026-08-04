use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use aether_core::{
    apply_structural_edit, canonical_ast, compile_source, compile_to_bytecode, compile_with_seed,
    forge_bytecode, run_bytecode, structural_document_json, verify_bytecode, InvocationValue,
    LANGUAGE_NAME, LANGUAGE_VERSION,
};

fn usage() {
    eprintln!(
        "Usage:\n  aether check <source-file>\n  aether structure <source-file>\n  aether apply-edit <source-file> <edit-file> --output <source-file>\n  aether compile <source-file> --output <artifact-file> [--bootstrap]\n  aether forge <compiler-artifact> <source-file> --output <artifact-file>\n  aether run <artifact-file>\n  aether version\n\ncompile uses the Aether-written seed compiler by default.\nstructure emits aether.ast/v5 JSON. apply-edit accepts aether.edit/v5, validates canonical source, then seed-compiles before writing.\nPass --bootstrap to emit with the Rust bootstrap (seed rebuild / diagnostics)."
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

fn compile(source_path: &Path, output_path: &Path, use_bootstrap: bool) -> Result<(), String> {
    let source = read_source(source_path)?;
    let output = if use_bootstrap {
        compile_to_bytecode(&source).map_err(|error| error.to_string())?
    } else {
        compile_with_seed(&source).map_err(|error| error.to_string())?
    };
    write_artifact(output_path, output.bytecode)?;
    let engine = if use_bootstrap { "bootstrap" } else { "seed" };
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} compiled {} to {} ({engine})",
        source_path.display(),
        output_path.display()
    );
    Ok(())
}

fn structure(source_path: &Path) -> Result<(), String> {
    let source = read_source(source_path)?;
    let document = structural_document_json(&source).map_err(|error| error.to_string())?;
    println!("{document}");
    Ok(())
}

fn apply_edit(source_path: &Path, edit_path: &Path, output_path: &Path) -> Result<(), String> {
    let source = read_source(source_path)?;
    let edit = read_source(edit_path)?;
    let result = apply_structural_edit(&source, &edit).map_err(|error| error.to_string())?;
    compile_with_seed(&result.source).map_err(|error| {
        format!(
            "refusing to write structurally edited source because seed compilation failed: {error}"
        )
    })?;
    write_source(output_path, &result.source)?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} applied {} validated structural edit(s) from {} to {}",
        result.operation_count,
        edit_path.display(),
        output_path.display()
    );
    Ok(())
}

fn write_artifact(output_path: &Path, artifact: Vec<u8>) -> Result<(), String> {
    verify_bytecode(&artifact).map_err(|error| {
        format!(
            "refusing to write an invalid Aether artifact to {}: {error}",
            output_path.display()
        )
    })?;
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
    fs::write(output_path, artifact)
        .map_err(|error| format!("could not write {}: {error}", output_path.display()))?;
    Ok(())
}

fn write_source(output_path: &Path, source: &str) -> Result<(), String> {
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
    fs::write(output_path, source)
        .map_err(|error| format!("could not write {}: {error}", output_path.display()))?;
    Ok(())
}

fn forge(compiler_path: &Path, source_path: &Path, output_path: &Path) -> Result<(), String> {
    let compiler = read_artifact(compiler_path)?;
    let source = read_source(source_path)?;
    let output = forge_bytecode(&compiler, &source).map_err(|error| error.to_string())?;
    if !output.stdout.is_empty() {
        eprint!("{}", output.stdout);
    }
    let InvocationValue::Bytes(artifact) = output.value else {
        return Err("the compiler weave must yield Bytes".to_owned());
    };
    write_artifact(output_path, artifact)?;
    println!(
        "{LANGUAGE_NAME} {LANGUAGE_VERSION} forged {} with {} to {}",
        source_path.display(),
        compiler_path.display(),
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
        "structure" => {
            let source = next_argument(&mut arguments, "source file")?;
            if arguments.next().is_some() {
                return Err("structure accepts exactly one source file".to_owned());
            }
            structure(Path::new(&source))
        }
        "apply-edit" => {
            let source = next_argument(&mut arguments, "source file")?;
            let edit = next_argument(&mut arguments, "edit file")?;
            let output_flag = next_argument(&mut arguments, "--output flag")?;
            if output_flag != "--output" {
                return Err("apply-edit requires --output <source-file>".to_owned());
            }
            let output = next_argument(&mut arguments, "source output file")?;
            if arguments.next().is_some() {
                return Err(
                    "apply-edit accepts one source file, one edit file, and --output <source-file>"
                        .to_owned(),
                );
            }
            apply_edit(Path::new(&source), Path::new(&edit), Path::new(&output))
        }
        "compile" => {
            let source = next_argument(&mut arguments, "source file")?;
            let output_flag = next_argument(&mut arguments, "--output flag")?;
            if output_flag != "--output" {
                return Err("compile requires --output <artifact-file>".to_owned());
            }
            let output = next_argument(&mut arguments, "artifact output file")?;
            let mut use_bootstrap = false;
            if let Some(extra) = arguments.next() {
                if extra == "--bootstrap" {
                    use_bootstrap = true;
                    if arguments.next().is_some() {
                        return Err(
                            "compile accepts optional --bootstrap after --output <file>".to_owned()
                        );
                    }
                } else {
                    return Err(
                        "compile accepts one source file, --output <file>, and optional --bootstrap"
                            .to_owned(),
                    );
                }
            }
            compile(Path::new(&source), Path::new(&output), use_bootstrap)
        }
        "forge" => {
            let compiler = next_argument(&mut arguments, "compiler artifact")?;
            let source = next_argument(&mut arguments, "source file")?;
            let output_flag = next_argument(&mut arguments, "--output flag")?;
            if output_flag != "--output" {
                return Err("forge requires --output <artifact-file>".to_owned());
            }
            let output = next_argument(&mut arguments, "artifact output file")?;
            if arguments.next().is_some() {
                return Err(
                    "forge accepts one compiler artifact, one source file, and one Aether artifact output file"
                        .to_owned(),
                );
            }
            forge(Path::new(&compiler), Path::new(&source), Path::new(&output))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEMPORARY_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

    struct TemporaryDirectory {
        path: PathBuf,
    }

    impl TemporaryDirectory {
        fn create() -> Self {
            let sequence = NEXT_TEMPORARY_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = env::temp_dir().join(format!(
                "aether-cli-forge-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("temporary forge directory should be created");
            Self { path }
        }
    }

    impl Drop for TemporaryDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn forge_invokes_compile_and_writes_only_a_verified_artifact() {
        let temporary = TemporaryDirectory::create();
        let target = compile_to_bytecode(
            "world target\n\nweave main [] -> Whole:\n  speak \"built by forge\"\n  yield 0\n",
        )
        .expect("target source should compile")
        .bytecode;
        let compiler_source = format!(
            "world forge\n\nweave compile [borrow source: Text] -> Bytes:\n  bind target <- bytes \"{}\"\n  yield move target\n\nweave main [] -> Whole:\n  yield 0\n",
            hex_encode(&target)
        );
        let compiler = compile_to_bytecode(&compiler_source)
            .expect("compiler fixture should compile")
            .bytecode;
        let compiler_path = temporary.path.join("compiler.aeth");
        let source_path = temporary.path.join("input.ae");
        let output_path = temporary.path.join("output.aeth");
        fs::write(&compiler_path, compiler).expect("compiler artifact should be written");
        fs::write(&source_path, "world supplied\n").expect("source input should be written");

        forge(&compiler_path, &source_path, &output_path)
            .expect("forge should write the compiler result");

        let generated = fs::read(&output_path).expect("forge artifact should be readable");
        assert_eq!(generated, target);
        verify_bytecode(&generated).expect("forge output must verify");
    }

    #[test]
    fn apply_edit_writes_only_canonical_seed_validated_source() {
        let temporary = TemporaryDirectory::create();
        let source_path = temporary.path.join("input.ae");
        let edit_path = temporary.path.join("edit.json");
        let output_path = temporary.path.join("output.ae");
        let source = "world cli\n\nweave main [] -> Whole:\n  yield 0\n";
        let edit = r#"{
  "protocol": "aether.edit/v5",
  "schema": "aether.ast/v5",
  "baseSource": "world cli\n\nweave main [] -> Whole:\n  yield 0\n",
  "operations": [{
    "op": "replace",
    "target": "weave:main",
    "declaration": {
      "kind": "Weave",
      "name": "main",
      "parameters": [],
      "result": "Whole",
      "effect": "Total",
      "body": [{
        "kind": "Yield",
        "value": {"kind": "Atom", "atom": {"kind": "Whole", "value": 9}}
      }]
    }
  }]
}"#;
        fs::write(&source_path, source).expect("source fixture should write");
        fs::write(&edit_path, edit).expect("edit fixture should write");

        apply_edit(&source_path, &edit_path, &output_path)
            .expect("validated structural edit should write");

        let written = fs::read_to_string(&output_path).expect("edited source should be readable");
        assert_eq!(written, "world cli\n\nweave main [] -> Whole:\n  yield 9\n");
        compile_with_seed(&written).expect("written source must remain seed compilable");
    }

    fn hex_encode(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0F)]));
        }
        output
    }
}
