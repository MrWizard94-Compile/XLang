use aether_core::{compile_to_bytecode, invoke_bytecode, InvocationValue};

fn main() {
    let source = include_str!("../../../seed/aether_seed.ae");
    let compiler = compile_to_bytecode(source).expect("seed source must bootstrap");
    let output = invoke_bytecode(
        &compiler.bytecode,
        "compile",
        &[InvocationValue::Text(source.to_owned())],
    )
    .expect("seed compiler invocation must complete");
    let InvocationValue::Bytes(bytecode) = output.value else {
        panic!("seed compiler must return Bytes");
    };
    std::fs::write("seed/raw.aeth", bytecode).expect("raw artifact must be written");
}
