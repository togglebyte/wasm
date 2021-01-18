use wasmrun::*;

fn main() {
    let lib = inst_wasmtime();
    let add = add_wasmtime(&lib);
    unsafe {
        eprintln!("{:?}", add(1, 3).unwrap());
    }
}
