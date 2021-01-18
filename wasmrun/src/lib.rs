use std::ffi::CStr;

use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx};
use wasmer_runtime::{
    imports,
    instantiate,
    Instance as WasmerInstance,
    Func as WasmerFunc,
};
use libloading::Library;

// -----------------------------------------------------------------------------
//     - Libloading -
// -----------------------------------------------------------------------------
pub fn libload() -> Library {
    Library::new("../wasmlib/target/release/libwasmlib.so").unwrap()
}

pub fn add_lib(lib: &Library) -> libloading::Symbol::<unsafe extern fn(i32, i32) -> i32> {
    unsafe {
        let add: libloading::Symbol::<unsafe extern fn(i32, i32) -> i32> = lib.get(b"add").unwrap();
        add
    }
}

// -----------------------------------------------------------------------------
//     - Libc::dlopen -
// -----------------------------------------------------------------------------
pub unsafe fn dlopen() -> *mut libc::c_void {
    let s = CStr::from_bytes_with_nul(b"../wasmlib/target/release/libwasmlib.so\0").unwrap();
    let fn_name = CStr::from_bytes_with_nul(b"add\0").unwrap();

    let mut lib = libc::dlopen(s.as_ptr(), libc::RTLD_NOW);
    let mut func = libc::dlsym(lib, fn_name.as_ptr());
    func
}

pub unsafe fn c_add(mut func: *mut fn(i32, i32) -> i32) -> i32 {
    let val = (*func)(2, 5);
    val
}

// -----------------------------------------------------------------------------
//     - Wasm time -
// -----------------------------------------------------------------------------
pub fn inst_wasmtime() -> Instance {
    let store = Store::default();
    let mut linker = Linker::new(&store);
    let module = Module::from_file(
        store.engine(),
        "../wasmlib/target/wasm32-unknown-unknown/release/wasmlib.wasm",
    )
    .unwrap();
    let instance = Instance::new(&store, &module, &[]).unwrap();
    instance
}

pub fn add_wasmtime(inst: &Instance) -> impl Fn(i32, i32) -> Result<i32, Trap> {
    inst.get_func("add").unwrap().get2::<i32, i32, i32>().unwrap()
}

// -----------------------------------------------------------------------------
//     - Wasmer -
// -----------------------------------------------------------------------------
pub fn inst_wasmer() -> WasmerInstance {
    static WASM: &[u8] = include_bytes!("../../wasmlib/target/wasm32-unknown-unknown/release/wasmlib.wasm");
    let instance = instantiate(WASM, &imports![]).unwrap();
    instance
}

pub fn add_wasmer(inst: &WasmerInstance) -> WasmerFunc<(i32, i32), i32> {
    let f = inst.func::<(i32, i32), i32>("add").unwrap();
    f
}

