
fn fundamental_types() {
    // u8 u16 u32 u64
    // i8 i16 i32 i64
    // usize isize bool char

    let tuple : (char, bool, u32) = ('a', true, 32);
    let unit : () = ();

    println!("tuple: {:?} is {:?}", tuple, unit);
}

fn main() {
    fundamental_types();
}
