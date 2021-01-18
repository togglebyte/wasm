#[no_mangle]
pub fn add(one: i32, two: i32) -> i32 {
    let nonsense = format!("{} {}", one, two);
    let d = md5::compute(nonsense.as_bytes());
    one + two
}
