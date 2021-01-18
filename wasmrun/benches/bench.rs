#![feature(test)]
extern crate test;
use test::bench::{black_box, Bencher};
use wasmrun::*;

#[bench]
fn wasmtime(b: &mut Bencher) {
    let inst = inst_wasmtime();
    let add = add_wasmtime(&inst);
    b.iter(|| {
        let val = add(2, 5).unwrap();
        assert_eq!(val, 7);
        val
    });
}

#[bench]
fn wasmer(b: &mut Bencher) {
    let inst = inst_wasmer();
    let add = add_wasmer(&inst);
    b.iter(|| {
        let val = add.call(2, 5).unwrap();
        assert_eq!(val, 7);
        val
    });
}

#[bench]
fn c(b: &mut Bencher) {
    unsafe {
        let mut func = dlopen();
        let f = (&mut func as *mut _) as *mut fn(i32, i32) -> i32;
        b.iter(|| {
            let val = c_add(f);
            assert_eq!(val, 7);
            val
        });
    }
}

#[bench]
fn c2(b: &mut Bencher) {
    unsafe {
        let lib = libload();
        let add = add_lib(&lib);
        b.iter(|| {
            let val = add(2, 5);
            assert_eq!(val, 7);
            val
        });
    }
}

#[bench]
fn lua(b: &mut Bencher) {
    let mut lua = rlua::Lua::new();

    let lua_code = r#"
    s = string.format("%d %d", one, two)
    res = one + two
        "#;

    let one = 2;
    let two = 5;

    let val = lua.context(|ctx| {
        b.iter(|| {
            ctx.globals().set("one", one);
            ctx.globals().set("two", two);
            ctx.load(lua_code).exec();
            let val: i32 = ctx.globals().get("res").unwrap();
            assert_eq!(val, 7);
        });
    });
}
